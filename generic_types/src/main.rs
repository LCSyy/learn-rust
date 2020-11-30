//! Learn Rust - Generic types.
//! 
//! ## 通用类型 - 函数、结构、枚举以及方法
//! 
//! ## 特征 (trait) - 使用 trait 约束通用类型的行为
//! - trait作为参数 `impl Trait`
//! - 多个特征约束，使用加号
//! - 使用 `where` 子句
//! - 返回实现特定特征的类型值
//! 
//! **blanket implementations**
//! 
//! 为符合特定特征约束的所有类型实现其他特征。
//! 
//! ## 声明周期，一种通用类型，告知编译器引用参数之间的关系
//! 
//! 用于指明引用类型的合法作用域。大多数时候，编译器能够自动推断生命周期，正如类型推断。在一些
//! 无法自动推断生命周期的情况下，则需要软件工程师显式地标注生命周期。
//! 

fn main() {
    // 如果Rust中没有通用类型机制，当我们需要定义一个仅类型不同，但功能相同的函数时，就无法实现。
    // 没有通用类型机制，相同功能的函数需要为每个类型都单独编写各自的版本。
    println!("{}", sumi(1,2));
    println!("{}", sumf(1.0, 2.0));

    let rect = Rect {x: 0., y: 0., w: 0., h: 0. };
    let circle = Circle {x:10., y: 2., r: 1.0 };
    draw(&rect);
    draw1(&circle);
    draw2(&circle);
    rect_shape().draw();
    rect.print();
}

fn sumi(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}

fn sumf(lhs: f64, rhs: f64) -> f64 {
    lhs + rhs
}

// =======

// 定义特征
trait Drawable {
    fn draw(&self);
}

fn draw(item: &impl Drawable) {
    item.draw();
}

// impl Drawable 实际是 <T: Drawable> 的简便写法
fn draw1<T: Drawable>(item: &T) {
    item.draw();
}

// 使用 where 子句
fn draw2<T>(item: &T) where T: Drawable {
    item.draw();
}

// 返回实现了指定特征的类型值
// 使用 impl Drawable 只能返回一种类型
fn rect_shape() -> impl Drawable {
    Rect {x: 0., y: 0., w: 0., h: 0. }
}

struct Item {}

impl Drawable for Item {
    fn draw(&self)  {

    }
}

struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Drawable for Rect {
    fn draw(&self) {
        println!("There is a rectangle at: ({}, {}), size: ({},{}).", self.x, self.y, self.w, self.h);
    }
}

struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Circle at ({},{}), radius is: {}.", self.x, self.y, self.r);
    }
}

trait Printable {
    fn print(&self);
}

// Blanket implementations
impl<T> Printable for T
    where T: Drawable
{
    fn print(&self) {
        println!("---------");
        self.draw();
        println!("---------");
    }
}
