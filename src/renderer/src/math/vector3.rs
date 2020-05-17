/// 三维向量。
#[derive(Debug,Default)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
}
