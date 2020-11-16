//! Learn Rust Series
//! 
//! Learn Rust - Macros
//! 
//! Rust中宏的类型：
//! - `macro_rules!` 声明式宏；也被称为"macros by example","`macro_rules!`宏"；
//! - 过程式宏
//!   - 自定义#[derive]宏，根据声明的`derive`属性添加代码到结构或枚举；
//!   - 类属性宏，自定义属性；
//!   - 类函数宏。
//! 
//! 函数与宏的却别：
//! - 函数签名的参数是固定的，宏的参数数量可以不定；
//! - 宏在编译时生成代码；
//! - 在使用宏之前必须先定义或引入作用域中；而函数可以在任何地方定义且可以在任意地方调用；（？？函数不也需要引入作用域吗）
//! 
//! 宏的缺点：
//! - 复杂。
//! 
//! **macro_rules!**
//! 
//! ```rust
//! macro_rules! <name> {
//!     <body>
//! }
//! ```
//! 
//! [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)
//! 

#[macro_export]
macro_rules! me_str {
    ( $( $key:tt : $val:expr ),* ) => {
        $(
            println!("{}: {}",$key,$val);
        )*
    };
}

