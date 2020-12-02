//! math3d
//! 
//! ## 2D
//! 
//! **基本单位** 点、向量。
//! 
//! ## 3D
//! 

pub mod math2d;

pub trait EchoItem {
    fn draw(&self);
}

pub struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rectangle {
    pub fn new(x:i32, y:i32, w:i32, h:i32) -> Self {
        Self {x,y,w,h}
    }
}

impl EchoItem for Rectangle {
    fn draw(&self) {
        for i in 0..self.w {
            for j in 0..self.h {
                print!("*");
            }
            println!("");
        }
    }
}
