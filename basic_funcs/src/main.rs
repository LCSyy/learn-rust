//! Learn Rust - function basics.
//! 
//! ## 函数参数
//! 
//! 函数的参数默认是移动的，即将外部变量的所有权移动到函数体内，外部变量将不再合法。
//! 
//! ## 返回值
//! 
//! Rust的函数体中没有分号的表达式表示返回值。当然也可以使用比较传统的 `return` 。
//! 
//! ## 其他
//! 
//! 1. 函数参数 `mut a: &Object` 与 `a: &mut Object` 的区别？
//!   前者表示引用类型的变量a的值是可变的，即它的引用对象是可以改变的。
//! 后者则表示a引用了值可变的对象。
//! 
fn main() {
    prompt();
    let a = 1024;
    let b1 = Commpund {
        id: 101,
        name: String::from("Learn Rust"),
    };
    // immutable_params() 将b移动到了函数作用域内，因此外部的b不再合法。
    immutable_params(a, b1);
    
    let b2 = Commpund {
        id: 101,
        name: String::from("Learn Rust"),
    };
    // immutable_borrow() 借用b2变量，b2仍然合法。
    immutable_borrow(a,&b2);
    // 移动b2.
    mutable_params(a, b2);

    let mut b3 = Commpund {
        id: 102,
        name: String::from("Hello Variable"),
    };
    mutable_borrow(&mut b3);
    println!("{}", b3.name);

    let a = 1001;
    println!("{}", is_number_bigger_than_thousand(a));
}

struct Commpund {
    id: u64,
    name: String,
}

fn prompt() {
    println!("Hello, world!");
}

// 输入参数是不可变的。
// 基本标量类型都是直接拷贝，而对复合类型则是移动的（移动后外部变量将不可用）。
fn immutable_params(a: i32, b: Commpund) {
    println!("{}, {}", a, b.name);
}

// 对复合类型使用引用(reference)作为函数参数，此时成为借用(borrow)。
fn immutable_borrow(a: i32, b: &Commpund) {
    println!("a:{}, b.id: {}", a, b.id);
}

// 输入参数是可变的。
fn mutable_params(mut a: i32, mut b: Commpund) {
    println!("before: {}, {}",a, b.name);
    a += 1;
    b.name.push_str(" - Rust function basics");
    println!("after: {}, {}", a, b.name);
}

// 可变引用。
fn mutable_borrow(a: &mut Commpund) {
    a.name.push_str(" - I borrowed, and modified.");
}

// 返回值。
fn is_number_bigger_than_thousand(a: i32) -> bool {
    if a > 1000 {
        true
    } else {
        false
    }
}
