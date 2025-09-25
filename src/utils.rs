use base64::{self, Engine};
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONNECTION, CONTENT_TYPE, HeaderMap, HeaderValue,
    USER_AGENT,
};
use std::io::ErrorKind;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{env, fs};
use text_colorizer::Colorize;

/// 解析命令行参数并返回要使用的 Tinify Key
///
/// 解析规则：
/// - 如果第一个参数是 "set"，则把第二个参数作为 KEY 保存到本地并退出程序（用于持久化 KEY）
/// - 如果提供了第一个参数且不是 "set"，则把该参数当作临时 KEY 返回
/// - 否则尝试读取已保存的 KEY；若不存在则打印使用说明并退出程序（返回值不会发生）
///
/// 返回值：
/// 成功时返回要使用的 Key 字符串；在处理 "set" 或错误情形时会直接退出程序。
pub fn parse_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    // 如果是设置命令：tinifycli set <KEY>
    if args.len() > 0 && args[0] == "set" {
        if args.len() < 2 {
            eprintln!("{} 使用: tinifycli set <KEY>", "Error:".red().bold());
            std::process::exit(1);
        }
        let key = args[1].trim();
        if let Err(e) = save_key(key) {
            eprintln!("{} 保存 KEY 失败: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
        eprintln!(
            "{} 成功保存 KEY 到 {}",
            "Ok:".green().bold(),
            key_file().display()
        );
        std::process::exit(0);
    }

    // 如果命令行直接传入 KEY，则优先使用
    if args.len() > 0 {
        return args[0].clone();
    }

    // 无参数时尝试读取已保存的 key
    match read_key() {
        Ok(k) => k,
        Err(_) => {
            print_usage();
            eprintln!(
                "{} 缺少关键参数，Tinify Key。可用命令：{} 或 {}",
                "Error:".red().bold(),
                "tinifycli set <KEY>".green(),
                "tinifycli <KEY>".green()
            );
            std::process::exit(1);
        }
    }
}

/// 打印程序使用说明到标准错误
fn print_usage() {
    eprintln!("{} 压缩当前目录所有图片...", "tinify_cli".green());
    eprintln!("使用方式：");
    eprintln!("  tinifycli set <TINIFY KEY>    # 保存 KEY 到本地（~/.tinifycli/key）");
    eprintln!("  tinifycli <TINIFY KEY>        # 本次使用提供的 KEY");
    eprintln!("  tinifycli                     # 使用已保存的 KEY");
}

/// 返回配置目录路径（通常是 ~/.tinifycli）
///
/// 若无法获取 HOME 环境变量，则退回到当前目录下的 .tinifycli（极少发生）。
fn config_dir() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        let mut p = PathBuf::from(home);
        p.push(".tinifycli");
        p
    } else {
        // 回退到当前目录下的 .tinifycli（极少发生）
        let mut p = PathBuf::from(".");
        p.push(".tinifycli");
        p
    }
}

fn key_file() -> PathBuf {
    let mut p = config_dir();
    p.push("key");
    p
}

/// 将 KEY 保存到配置文件中，并在 Unix 系统上将权限设置为 600（仅文件所有者可读写）
///
/// 返回 Result：Ok(()) 表示保存成功，Err 表示出现 I/O 等错误。
fn save_key(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    let kf = key_file();
    fs::write(&kf, key.as_bytes())?;
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&kf)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&kf, perms)?;
    }
    Ok(())
}

/// 读取已保存的 KEY 并返回字符串
///
/// 如果文件不存在或读取失败，返回 Err 包装的错误。
fn read_key() -> Result<String, Box<dyn std::error::Error>> {
    let kf = key_file();
    match fs::read_to_string(&kf) {
        Ok(s) => Ok(s.trim().to_string()),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                Err(Box::new(e))
            } else {
                Err(Box::new(e))
            }
        }
    }
}

/// 对当前目录下的图片逐个调用 Tinify API 进行压缩
///
/// 行为说明：
/// - 仅在当前目录（非递归）查找常见图片扩展名文件（png/jpg/jpeg/gif/webp/bmp）
/// - 对每个图片先上传到 https://api.tinify.com/shrink，然后从返回的 output.url 下载压缩后的数据
/// - 将压缩后的文件保存为 compressed_<原文件名>，并在 stderr 输出结果和压缩比
///
/// 返回 Result：Ok(()) 表示全部处理完毕；遇到网络或 I/O 错误会返回 Err。
pub fn get_request(tkey: &str) -> Result<(), Box<dyn std::error::Error>> {
    let authorization = get_authorization(tkey);
    // eprintln!("authorization: {authorization}");

    // headers
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&authorization)?);
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (iPhone; CPU iPhone OS 18_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Mobile/15E148 Safari/604.1"));

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    // 只在当前目录（非递归）查找图片
    let exts = ["png", "jpg", "jpeg", "gif", "webp", "bmp"];
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if !exts.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                continue;
            }
        } else {
            continue;
        }

        let fname = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        eprintln!("图片 {} 正在压缩...", fname);

        // 读取文件字节并作为请求 body 上传
        let bytes = fs::read(&path)?;
        let orig_size = bytes.len();
        let res = client
            .post("https://api.tinify.com/shrink")
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(bytes)
            .send()?;

        let body = res.text()?;
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            if let Some(output) = json.get("output") {
                if let Some(url) = output.get("url").and_then(|v| v.as_str()) {
                    match client.get(url).send() {
                        Ok(get_res) => {
                            let compressed_bytes = get_res.bytes()?;
                            let compressed_size = compressed_bytes.len();
                            let compressed_fname = format!("compressed_{}", fname);
                            fs::write(&compressed_fname, &compressed_bytes)?;
                            eprintln!(
                                "输出: {} （{} bytes） 原始: {} bytes，压缩比: {:.2}%",
                                compressed_fname.green(),
                                compressed_size.to_string().green(),
                                orig_size,
                                100.0 * (compressed_size as f64) / (orig_size as f64)
                            );
                        }
                        Err(e) => {
                            eprintln!("下载压缩图片失败 ({}): {}", url, e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// 根据给定的 Tinify Key 构造 HTTP Basic Authorization 头值
///
/// 返回形如 "Basic <base64(api:KEY)>" 的字符串。
fn get_authorization(key: &str) -> String {
    let user = "api";
    let auth_str = format!("{}:{}", user, key);
    let auth_base64 = base64::engine::general_purpose::STANDARD.encode(auth_str.as_bytes());
    let authorization = format!("Basic {}", auth_base64);
    // eprintln!("{}", authorization);
    authorization
}
