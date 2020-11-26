//! Learn Rust - Common basic concepts about Rust.
//! 
//! ## 变量与可变性
//! 
//! Rust中变量默认是不可改变的。这样能增强代码的安全性以及更易于并发访问（不用担心数据同步问题）。
//! 在定义变量时，Rust将这个过程称为将一个值和名称绑定。
//! 变量必须初始化；不可变的变量不能赋值两次。
//! 
//! ## 常量
//! 
//! 常量定义方式：`const <CONST_NAME>: <type> = <const value>;`。常量在整个程序
//! 运行期间都存在，在其作用域内可访问。
//! 
//! 常量的行为和不可变变量类似，都是只能初始化而不能改变值。但它们有很多区别：
//! 1. 不允许在常量定义中添加`mut`；
//! 2. 常量必须标注类型；
//! 3. 常量可以在任何作用域定义，包括全局作用域；
//! 4. 常量只能通过常量表达式赋值，因为常量是在编译时计算其值的。
//! 
//! ## 隐藏(Shadowing)
//! 
//! 在同一作用域下，同一个变量名可多次绑定值，这些值可以是不同类型。在逻辑中需要
//! 一些中间变量的时候很方便，不必纠结取不同变量名以区别或者将变量定义为`mut`。
//! 

const DATA_SIZE: usize = 100;

fn main() {

    // Rust中变量是默认不可变的，也就是其关联的值不能被修改。
    let a = 1;
    // a += 1; // 错误：不可修改不可变变量的值
    println!("immutable: \t{}",a);

    // 要定义一个可修改变量，需在变量名前加上mut关键字。
    let mut b = 1;
    println!("before: \t{}", b);
    b += 1;
    println!("after: \t\t{}", b);

    // 常量在整个程序运行期间都是有效的，但在其定义的作用域内才可访问。
    println!("constant: \t{}", DATA_SIZE);
    {
        const INNER_CONST: i32 = 1234;
        println!("a constant: \t{}", INNER_CONST);
    }
    // println!("{}", INNER_CONST); // 错误：常量INNER_CONST不在该作用域中

    // 在Rust中，同一作用下，变量名可以多次定义，绑定到不同值、不同类型上。之前的定义将失效。
    let c = 1;
    println!("origin: \t{}", c);
    let c = c + 1;
    println!("shadow: \t{}", c);
    let c = "Hello, Rust!";
    println!("shadow again: \t{}", c);
}
