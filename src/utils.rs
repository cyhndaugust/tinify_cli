use base64::{self, Engine};
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONNECTION, CONTENT_TYPE, HeaderMap, HeaderValue,
    USER_AGENT,
};
use std::{env, fs};
use text_colorizer::Colorize;

#[derive(Debug)]
pub struct Arguments {
    pub tkey: String, // Tinify Key
}

/// 解析参数
pub fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        print_usage();
        eprintln!("{} 缺少关键参数，Tinify Key", "Error:".red().bold());
        std::process::exit(1);
    }

    Arguments {
        tkey: args[0].clone(),
    }
}

/// 使用提示
fn print_usage() {
    eprintln!("{} 压缩当前目录所有图片...", "tinify_cli".green());
    eprintln!("使用方式：tinify_cli <TINIFY KEY>");
}

/// 发送请求
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

        // 打印状态、头和响应体（优先 JSON）
        // eprintln!("{} -> 状态: {}", fname, res.status());
        // for (k, v) in res.headers().iter() {
        //     eprintln!("Header: {}: {:?}", k, v);
        // }

        let body = res.text()?;
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            // eprintln!("Body (json): {}", serde_json::to_string_pretty(&json)?);

            // 如果有 output.url，则下载并保存为 compressed_<原文件名>
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

/// Authorization
fn get_authorization(key: &str) -> String {
    let user = "api";
    let auth_str = format!("{}:{}", user, key);
    let auth_base64 = base64::engine::general_purpose::STANDARD.encode(auth_str.as_bytes());
    let authorization = format!("Basic {}", auth_base64);
    // eprintln!("{}", authorization);
    authorization
}
