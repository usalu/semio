//! 📍️ Point and vector of the geometry primitives: the point and vector value types with their arithmetic operators and conversions. See [`super`].

// #region 🔖️Shapes
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn distance(self, other: Self) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl std::ops::Add<Vec2> for Point {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub<Vec2> for Point {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::Sub for Point {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Vec2 {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn hypot(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl std::ops::Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        rhs * self
    }
}

impl std::ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
    }
}
// #endregion 🔖️Shapes
