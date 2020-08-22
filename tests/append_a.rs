//! 向 文件a 追加行
use std::io::prelude::*;

fn main() {
    let mut file_handle = std::fs::OpenOptions::new().append(true).open("tests/a").unwrap();
    loop {
        file_handle.write_all("aaaaaaaaaaaaaaaaaaaa\n".as_bytes()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}