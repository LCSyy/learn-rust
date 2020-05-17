use std::ops::*;

/// # 二维向量。
#[derive(Default, Debug)]
pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector2 {

    /// 创建Vector2对象。
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    // 向量的模。
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    /// 单位向量
    pub fn normalized(&self) -> Vector2 {
        let len = self.length();
        if len == 0.0 {
            Vector2::default()
        } else {
            Vector2::new(self.x / len, self.y / len)
        }
    }

    /// 点乘。
    /// 向量点乘计算公式是：对应相乘再相加；公式描述为：`ab = |a||b|cos(t)`；t为向量a,b的夹角。
    pub fn dot(&self, other: Vector2) -> f64 {
        self.x + other.x + self.y * other.y
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs:Self) -> Self {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self {
        self * (-1.0)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Vector2 {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}