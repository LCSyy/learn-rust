//! Learn Rust - std::fs.
//! `fs` 模块中封装了跨平台的文件操作接口。平台专有的文件相关功能在 `std::os:$paltform` 中。
//! 
use std::fs::{File};
use std::io::Write;
use rand;

const BUF_SIZE: usize = 2048;

fn main() -> std::io::Result<()> {

    let mut file = File::create("learn-rust.db")?;
    let mut times = 100_000_000;
    while times > 0 {
        if times % 1_000_000 == 0 {
            println!("finished: {}", times);
            file.flush()?;
            file.sync_all()?;
        }
        file.write(&[rand::random::<u8>()])?;
        times -= 1;
    }

    Ok(())
}
