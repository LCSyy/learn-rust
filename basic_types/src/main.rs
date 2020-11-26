//! Learn Rust - basic types.
//! 
//! ## 数据类型
//! 
//! Rust是一门静态类型语言，也就是说每一个变量的值在编译时，必须明确其类型。
//! 
//! Rust中每一个值都有特定的数据类型。明确一个值的数据类型后，Rust就能知道可以对这个数据
//! 做哪些操作。Rust的编译器一般能够自动推导变脸值的类型。因此在一般编写代码过程中，不需要显
//! 式标注类型。如果在一些情况下，编译器无法自动推导，则就需要为变量提供类型说明。
//! 
//! ## 整型
//! 
//! 整型默认类型为i32。
//! 
//! ## 浮点型
//! 
//! 浮点数默认类型为f64。
//! 
//! ## 布尔类型
//! 
//! 占一个字节，有 `true`, `false` 两种值。
//! 
//! ## 不同数值类型之间的转换
//! 
//! trait From<T>, ...
//! 
//! ## 字符类型
//! 
//! Rust字符字面值使用单引号表示。
//! 
//! Rust的字符类型 `char` 占四个字节，使用 `Unicode Scalar Value` 来表示字符值。
//! 它能表示ASCII，汉字，emoji表情等等。它与Unicode的概念有所不同。
//! 
//! ## 元组类型
//! 
//! 元素类型将任意数量的各种类型的值组合在一起组成一个类型，该类型就是元组类型。
//! 元素类型具有固定长度，即一旦声明，其所占空间就固定了。
//! 
//! ## 数组类型
//! 
//! Rust中，数组具有固定长度。
//! 
//! ## 其他
//! 
//! **数组、切片与Vec<T>**
//! **字符、str与String**
//! 

fn main() {
    let r: u8 = 100;
    let g = 200u8;
    let b = 255u8;

    let multi = i32::from(r) * i32::from(g) * i32::from(b);
    println!("{} x {} x {} = {}", r, g, b, multi);

    let big_num = 100_000_0001_u64;
    println!("{}",big_num);

    // byte不能使用类型后缀。
    let byte = b'A';
    println!("{}", byte);

    const PI: f64 = 3.1415926;
    // 不是默认类型f64吗？为什么不标注类型，编译时会报类型不明确的错误呢？
    let radius: f64 = 1.234;
    let area = PI * radius.powi(2);
    println!("area: {}", area);

    let rust_ok: bool = true;
    if rust_ok {
        println!("Rust OK!");
    }

    let a = 'A';
    println!("char: {}", a);
    let b: char = 'B';
    println!("char: {}", b);
    let c = char::from(b'C');
    println!("char: {}", c);

    let tup = (1, 'A', b'A', 3.12);
    let (id, ch, cb, cost) = tup;
    println!("{}, {}, {}, {}", id, ch, cb, cost);
    println!("({}, {}, {}, {})", tup.0, tup.1, tup.2, tup.3);

    let arr = [3,2,1];
    println!("{}", arr.len());
    let arr: [u8; 4] = [0, 255, b'A', 0xff];
    println!("{}", arr.len());
    let arr = [7;5];
    println!("{}", arr.len());
}
