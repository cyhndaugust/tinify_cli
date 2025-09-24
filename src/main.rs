mod utils;
// use text_colorizer::Colorize;
use utils::{get_request, parse_args};

fn main() {
    let args = parse_args();
    // eprintln!("参数key：{}", args.tkey.yellow());

    if let Err(e) = get_request(&args.tkey) {
        eprintln!("请求失败: {}", e);
    }
}
