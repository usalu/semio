//! 🧮️ Affine of the geometry primitives: the six coefficient affine transform with its composition and point application. See [`super`].

use super::*;

// #region 🔖️Shapes
/// 🧮️ Six-coefficient 2D affine `[a, b, c, d, e, f]` — same layout as `kurbo::Affine`:
/// `x' = a*x + c*y + e`, `y' = b*x + d*y + f`. `Mul` composes "self after other" (matrix product
/// `self * other`), matching `kurbo::Affine`'s own composition order — see `affine_tests`'s
/// differential proof against the real `kurbo::Affine`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    pub(crate) coeffs: [f64; 6],
}

impl Affine {
    pub const IDENTITY: Self = Self { coeffs: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] };
    pub fn new(coeffs: [f64; 6]) -> Self {
        Self { coeffs }
    }
    pub(super) fn translation(offset: Vec2) -> Self {
        Self { coeffs: [1.0, 0.0, 0.0, 1.0, offset.x, offset.y] }
    }
    fn scaling(s: f64) -> Self {
        Self { coeffs: [s, 0.0, 0.0, s, 0.0, 0.0] }
    }
    pub(super) fn rotation(angle: f64) -> Self {
        let (sin_a, cos_a) = angle.sin_cos();
        Self { coeffs: [cos_a, sin_a, -sin_a, cos_a, 0.0, 0.0] }
    }
    pub fn translate(self, offset: impl Into<Vec2>) -> Self {
        self * Self::translation(offset.into())
    }
    pub fn scale(self, s: f64) -> Self {
        self * Self::scaling(s)
    }
    pub fn rotate(self, angle: f64) -> Self {
        self * Self::rotation(angle)
    }
    /// 🔢️ Raw `[a, b, c, d, e, f]` matrix coefficients, for callers that need direct numeric access (e.g. north/south orientation checks) without importing `kurbo` themselves.
    pub fn as_coeffs(&self) -> [f64; 6] {
        self.coeffs
    }
}

impl std::ops::Mul for Affine {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let [a1, b1, c1, d1, e1, f1] = self.coeffs;
        let [a2, b2, c2, d2, e2, f2] = rhs.coeffs;
        Self {
            coeffs: [
                a1 * a2 + c1 * b2,
                b1 * a2 + d1 * b2,
                a1 * c2 + c1 * d2,
                b1 * c2 + d1 * d2,
                a1 * e2 + c1 * f2 + e1,
                b1 * e2 + d1 * f2 + f1,
            ],
        }
    }
}

impl std::ops::Mul<Point> for Affine {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        let [a, b, c, d, e, f] = self.coeffs;
        Point::new(a * rhs.x + c * rhs.y + e, b * rhs.x + d * rhs.y + f)
    }
}
// #endregion 🔖️Shapes
