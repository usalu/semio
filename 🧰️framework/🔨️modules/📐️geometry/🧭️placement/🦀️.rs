//! 🧭️ 3D placement: a double-precision 3x4 affine transform and a sloped height plane.
//!
//! Matrices are row-major `[[m00, m01, m02, tx], [m10, ...], [m20, ...]]`; `then` composes in application order.

use crate::vector::{add3, cross3, dot3, normalize3, scale3, Xyz};
use crate::Point;

/// 🧭️ Affine map `p -> M p + t` in 3D.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine3 {
    pub m: [[f64; 4]; 3],
}

impl Affine3 {
    pub const IDENTITY: Self = Self { m: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]] };

    /// ➡️ Translation by `t`.
    pub fn translation(t: Xyz) -> Self {
        Self { m: [[1.0, 0.0, 0.0, t[0]], [0.0, 1.0, 0.0, t[1]], [0.0, 0.0, 1.0, t[2]]] }
    }

    /// 🔍️ Per-axis scale (negative components mirror).
    pub fn scaling(s: Xyz) -> Self {
        Self { m: [[s[0], 0.0, 0.0, 0.0], [0.0, s[1], 0.0, 0.0], [0.0, 0.0, s[2], 0.0]] }
    }

    /// 🔄️ Counter-clockwise rotation about +Z.
    pub fn rotation_z(angle: f64) -> Self {
        Self::rotation_axis([0.0, 0.0, 1.0], angle)
    }

    /// 🔄️ Rodrigues rotation about a (not necessarily unit) axis through the origin.
    pub fn rotation_axis(axis: Xyz, angle: f64) -> Self {
        let [x, y, z] = normalize3(axis);
        let (s, c) = angle.sin_cos();
        let k = 1.0 - c;
        Self { m: [[c + x * x * k, x * y * k - z * s, x * z * k + y * s, 0.0], [y * x * k + z * s, c + y * y * k, y * z * k - x * s, 0.0], [z * x * k - y * s, z * y * k + x * s, c + z * z * k, 0.0]] }
    }

    /// 🧭️ Frame mapping the local axes `x`, `y`, `z` (columns) and origin into world space.
    pub fn from_frame(origin: Xyz, x: Xyz, y: Xyz, z: Xyz) -> Self {
        Self { m: [[x[0], y[0], z[0], origin[0]], [x[1], y[1], z[1], origin[1]], [x[2], y[2], z[2], origin[2]]] }
    }

    /// 🔗️ Transform applying `self` first, then `next`.
    pub fn then(&self, next: &Self) -> Self {
        let mut m = [[0.0; 4]; 3];
        for (r, row) in m.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                *cell = (0..3).map(|k| next.m[r][k] * self.m[k][c]).sum::<f64>() + if c == 3 { next.m[r][3] } else { 0.0 };
            }
        }
        Self { m }
    }

    /// 📍️ Image of a point.
    pub fn apply_point(&self, p: Xyz) -> Xyz {
        let row = |r: usize| self.m[r][0] * p[0] + self.m[r][1] * p[1] + self.m[r][2] * p[2] + self.m[r][3];
        [row(0), row(1), row(2)]
    }

    /// ➡️ Image of a direction (translation ignored).
    pub fn apply_vector(&self, v: Xyz) -> Xyz {
        let row = |r: usize| self.m[r][0] * v[0] + self.m[r][1] * v[1] + self.m[r][2] * v[2];
        [row(0), row(1), row(2)]
    }

    /// 🧲️ Image of a surface normal (inverse transpose of the linear part, normalised, orientation-correct under mirrors).
    pub fn apply_normal(&self, n: Xyz) -> Xyz {
        let c0 = [self.m[0][0], self.m[1][0], self.m[2][0]];
        let c1 = [self.m[0][1], self.m[1][1], self.m[2][1]];
        let c2 = [self.m[0][2], self.m[1][2], self.m[2][2]];
        let (a, b, c) = (cross3(c1, c2), cross3(c2, c0), cross3(c0, c1));
        let sign = self.determinant().signum();
        scale3(normalize3(add3(add3(scale3(a, n[0]), scale3(b, n[1])), scale3(c, n[2]))), if sign == 0.0 { 1.0 } else { sign })
    }

    /// 🧮️ Determinant of the linear part (negative for mirrors).
    pub fn determinant(&self) -> f64 {
        let col = |c: usize| [self.m[0][c], self.m[1][c], self.m[2][c]];
        dot3(col(0), cross3(col(1), col(2)))
    }
}

/// 📐️ Plane `z = a x + b y + c` (height field), used for sloped slabs and roof faces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZPlane {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl ZPlane {
    /// ➖️ Horizontal plane at height `z`.
    pub fn flat(z: f64) -> Self {
        Self { a: 0.0, b: 0.0, c: z }
    }

    /// 📉️ Plane through `anchor` (xy) at height `z` rising by `slope = tan(angle)` per metre towards `direction` (radians counter-clockwise from +X).
    pub fn sloped(anchor: Point, z: f64, direction: f64, slope: f64) -> Self {
        let (a, b) = (slope * direction.cos(), slope * direction.sin());
        Self { a, b, c: z - a * anchor.x - b * anchor.y }
    }

    /// 📍️ Height at `p`.
    pub fn at(&self, p: Point) -> f64 {
        self.a * p.x + self.b * p.y + self.c
    }

    /// ↕️ Same plane raised by `dz`.
    pub fn raised(&self, dz: f64) -> Self {
        Self { c: self.c + dz, ..*self }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
