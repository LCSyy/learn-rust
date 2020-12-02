//! point2d
//! ## 性质
//! 
//! ## 运算
//! 
//! - 距离
//! 
//! ## 与向量的关系以及转换
//! 
#[derive(Debug,Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).abs().powi(2) + (self.y + other.y).abs().powi(2)).sqrt()
    }
}
