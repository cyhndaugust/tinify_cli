mod utils;
use utils::{compressed_images, parse_args};

fn main() {
    let tkey = parse_args();

    if let Err(e) = compressed_images(&tkey) {
        eprintln!("压缩失败: {}", e);
    }
}
