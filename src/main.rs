mod utils;
use utils::{get_request, parse_args};

fn main() {
    let tkey = parse_args();

    if let Err(e) = get_request(&tkey) {
        eprintln!("压缩失败: {}", e);
    }
}
