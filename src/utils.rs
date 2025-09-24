use std::{collections::HashMap, env};

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
pub fn print_usage() {
    eprintln!("{} 压缩当前目录所有图片...", "tinify_cli".green());
    eprintln!("使用方式：tinify_cli <TINIFY KEY>");
}

/// 发送请求
pub fn get_request() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::blocking::Client::new();
    let res = client.post("http://httpbin.org/post").json(&map).send()?;

    eprintln!("{res:#?}");
    Ok(())
}
