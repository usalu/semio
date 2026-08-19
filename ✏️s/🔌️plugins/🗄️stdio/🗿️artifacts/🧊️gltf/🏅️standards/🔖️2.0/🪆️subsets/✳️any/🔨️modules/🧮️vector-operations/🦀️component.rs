//! 🧮️ Shared three-dimensional vector operations.

//#region 🧮️Vectors
pub(crate) type V3 = [f64; 3];

pub(crate) async fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub(crate) async fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub(crate) async fn mul(a: V3, scalar: f64) -> V3 {
    [a[0] * scalar, a[1] * scalar, a[2] * scalar]
}
pub(crate) async fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub(crate) async fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
pub(crate) async fn norm(a: V3) -> f64 {
    dot(a, a).sqrt()
}
pub(crate) async fn normalize(a: V3) -> V3 {
    let norm = norm(a);
    if norm > 0.0 {
        mul(a, 1.0 / norm)
    } else {
        [1.0, 0.0, 0.0]
    }
}
//#endregion 🧮️Vectors
