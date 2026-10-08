//! ➗️ Tiny f64 vector helpers shared by the planar curve, loop, triangulation, solid and section domains.

use crate::{Point, Vec2};

/// 📏️ Coincidence tolerance for lengths and distances, in the caller's unit (metres for the BIM domain): one nanometre.
pub const LENGTH_EPS: f64 = 1e-9;

/// 📐️ Coincidence tolerance for angles in radians.
pub const ANGLE_EPS: f64 = 1e-12;

/// 🧭️ Three f64 coordinates; the BIM-grade (double precision) counterpart of the render-space [`crate::Vec3`].
pub type Xyz = [f64; 3];

/// ✖️ 2D cross product (z component of the 3D cross product).
pub fn cross(a: Vec2, b: Vec2) -> f64 {
    a.x * b.y - a.y * b.x
}

/// 🔄️ Vector rotated a quarter turn counter-clockwise (the left-hand normal of a direction).
pub fn perp(a: Vec2) -> Vec2 {
    Vec2::new(-a.y, a.x)
}

/// 🧲️ Unit vector, or `None` when the length is below `eps`.
pub fn unit(a: Vec2, eps: f64) -> Option<Vec2> {
    let length = a.hypot();
    (length > eps).then(|| a / length)
}

/// 🎚️ Linear interpolation between two points.
pub fn lerp(a: Point, b: Point, t: f64) -> Point {
    Point::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t)
}

/// 🧭️ Signed angle from `a` to `b` in `(-pi, pi]`.
pub fn angle_between(a: Vec2, b: Vec2) -> f64 {
    cross(a, b).atan2(a.dot(b))
}

/// ➕️ 3D sum.
pub fn add3(a: Xyz, b: Xyz) -> Xyz {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// ➖️ 3D difference.
pub fn sub3(a: Xyz, b: Xyz) -> Xyz {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// ✖️ 3D scale.
pub fn scale3(a: Xyz, s: f64) -> Xyz {
    [a[0] * s, a[1] * s, a[2] * s]
}

/// 🔸️ 3D dot product.
pub fn dot3(a: Xyz, b: Xyz) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// ✖️ 3D cross product.
pub fn cross3(a: Xyz, b: Xyz) -> Xyz {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// 📏️ 3D Euclidean length.
pub fn length3(a: Xyz) -> f64 {
    dot3(a, a).sqrt()
}

/// 🧲️ Unit 3D vector, or the zero vector when the length is zero.
pub fn normalize3(a: Xyz) -> Xyz {
    let length = length3(a);
    if length > 0.0 {
        scale3(a, 1.0 / length)
    } else {
        [0.0; 3]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
