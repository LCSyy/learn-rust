/// 四维向量。
#[derive(Debug,Default)]
pub struct Vector4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4 {
        Vector4 { x, y, z, w }
    }
}
