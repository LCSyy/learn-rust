//! Rust Basic - Packages, Crates and Modules
//! 
//! 一个包(package)可以包含多个 `binary crate` 和一个可选的 `lib crate` 。
//! 
//! 作用域、可见性。
//! 
//! - Packages
//! - Crates
//! - Modules
//! - Path
//! 
//! 一个 `crate` 表示一个库或者一个可执行文件。`crate` 的根是编译器开始
//! 处理该`crate`的根模块。
//! 
//! 一个 `package` 包含一个 `Cargo.toml` 文件。包至少包含一个 `crate` 。
//! 可以是库或者二进制。一个包中，库crate最多只能有一个，二进制crate可以
//! 有多个。
//! 
//! 使用命令 `Cargo new` 创建包时，默认创建包含一个二进制crate的包。该
//! 二进制crate的根模块是定义在 `src/main.rs` 中的。且该crate和包同名。
//! 类似地，如果源文件中有 `src/lib.rs` ，则表示该包中包含了一个同名的
//! 库crate。而该库crate的根模块就定义在 `src/lib.rs` 文件中。
//! 
//! 在编译构建时，Cargo会将crate的根模块文件传递给 `rustc` ，用以构建库
//! 或二进制文件。
//! 
//! 一个包中可以包含多个二进制crate。方法是在目录 `src/bin` 下放置
//! 这些二进制crate的源文件，每一个源文件表示一个独立的二进制crate。
//! 
//! ## module system
//! 
//! modules, paths, use, pub, as, external packages, glob operator.
//! 
//! 模块是在一个crate中组织代码的结构。它还控制模块中项的可见性，其可在模块外使用，还是
//! 只能在模块内使用。
//! 
//! 一个crate的根模块名称为 `crate` 。
//! 
//! ### path
//! 
//! 绝对路径 `crate::` ，相对路径 `self::`, `super::` 以及当前模块定义的名称。
//! 
//! `use` 将路径引入到当前作用域。
//! 
//! `as` 重命名引入的路径。
//! 
//! `pub use` 将引入作用域的路径重新导出。
//! 
//! `::*` 使用通配符将指定路径的公开内容全部导出。
//! 
//! 将一个crate中的不同模块拆分到各自的源文件中。该文件的名称即的模块名称。与该模块同名的目录下的
//! 模块是该模块的子模块。

// The binary crate name is: basic_crates
fn main() {
    println!("Hello, world!");
}

