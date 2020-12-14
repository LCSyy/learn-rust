//! Learn Rust - Error Handling
//! 
//! Rust将错误分为两类：可恢复、不可恢复。
//! 
//! Rust中没有异常机制。使用 `Result<T,E>` 处理可恢复的错误，
//! 使用 `panic!` 宏处理不可恢复的错误。
//! 
//! ## Unrecoverable errors
//! 
//! 调用 `panic!` 时，程序会先打印错误消息，然后展开以及清理栈，最后退出当前线程。
//! 展开的含义是返回函数调用堆栈并清理堆栈数据。程序在展开、清理时需要做很多事情，而且
//! 也会占用一定的程序体积。如果想要在发生错误时立即退出，清理工作交由操作系统完成，
//! 可以通过在 `Cargo.toml` 文件的特定`[profile]`中添加 `panic = 'abort'` 来指定。
//! 
//! ```toml
//! [profile.release]
//! panic = 'abort'
//! ```
//! 
//! ## Recoverable errors
//! 
//! 使用 `std::result::Result<T,E>` 处理错误。
//! 
//! 使用 `?` 操作符返回错误。
//! 
use std::fs::File;
use std::io::{Write,ErrorKind};

fn main() {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        },
    };

    if let Err(e) = f.write_all(b"What's the fuck?") {
        panic!("Problem writing file: {:?}", e);
    }
}
