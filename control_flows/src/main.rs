//! Learn Rust - Control Flow
//! 
//! ## `if` 表达式
//! 
//! 由于 `if` 是表达式，所以它是由返回值的。据此可以根据条件返回不同的值。条件是 `if` 表达式的所有
//! 分支要处理到所有可能情况，且分支中返回值类型要一致。
//! 
//! ## `loop` 表达式
//! 
//! `loop` 表达式相当于无限循环，如果要退出循环，必须在 `loop` 表达式内显式地调用 `break` 表达式跳出
//! 循环。
//! 
//! ## `while` 语句
//! 
//! `while` 语句通过判断条件是否为真来决定是否继续执行。它是语句，所以没有返回值。
//! 
//! ## `for` 语句
//! 
//! `for` 语句和 `trait Iterator` 结合使用，十分方便。
//! 
fn main() {
    let num = 3;
    if num < 1000 {
        println!("Number {} is small number.", num);
    } else if num < 10000 {
        println!("{} is a big number!", num);
    } else {
        println!("{} is a super big number!", num);
    }

    // 从 if 表达式返回值。
    let num2 = if num > 100_000 {
        true
    } else {
        false
    }; // 定义num2的是let语句，需要在末尾添加分号。
    println!("{}", num2);

    // loop
    let mut times = 0;
    loop {
        println!("{}", times);
        if times > 5 {
            break
        }
        times += 1;
    }

    // while
    let mut times = 10;
    while times > 0 {
        println!("{}", times);
        times -= 1;
    }

    // for
    let total = 10;
    for i in (0..total).rev() {
        println!("{}", i);
    }

    let arr = [1,356,347,3,46,23];
    for e in arr.iter() {
        println!("{}", e);
    }
}
