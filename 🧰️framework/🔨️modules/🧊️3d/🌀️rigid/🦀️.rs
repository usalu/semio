//! 🌀️ Rigid-body 3D algebra in single precision: vectors, points, unit quaternions and rigid
//! transforms (isometries) — the framework-owned replacement for the `nalgebra` surface plugins
//! used to reach for directly (`parry3d::na::{Vector3,Point3,UnitQuaternion,Quaternion,
//! Isometry3}`). Mirrors `nalgebra`'s `rotation_between`/`Isometry3` semantics geometrically;
//! pinned against `parry3d`'s `nalgebra` re-export as a dev-dependency oracle in
//! `🧿️collision`'s test module (same crate, different file — Rust dev-deps are crate-wide).

//#region 🔖️Vector3
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed: field names are the wire shape.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }

    pub fn norm(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn amax(self) -> f32 {
        self.x.abs().max(self.y.abs()).max(self.z.abs())
    }

    /// 🧭️ Unit-length copy, or `None` when `self` is shorter than `eps`.
    pub fn try_normalize(self, eps: f32) -> Option<Self> {
        let n = self.norm();
        (n > eps).then(|| self * (1.0 / n))
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}
//#endregion 🔖️Vector3

//#region 🔖️Point3
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `Vector3` above.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn coords(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn from_coords(v: Vector3) -> Self {
        Self::new(v.x, v.y, v.z)
    }

    pub fn inf(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y), self.z.min(other.z))
    }

    pub fn sup(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y), self.z.max(other.z))
    }
}

impl std::ops::Sub for Point3 {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Add<Vector3> for Point3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
//#endregion 🔖️Point3

//#region 🔖️Quaternion
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `Vector3` above.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct Quaternion {
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(w: f32, i: f32, j: f32, k: f32) -> Self {
        Self { i, j, k, w }
    }
}

const ROTATION_EPS: f32 = 1e-8;

/// 🧭️ A unit-norm `Quaternion` — the framework's rotation representation. Construction always
/// normalizes, so the invariant holds for every value that escapes this module.
// 🧬️ `value_derive::{ToValue, FromValue}` additive; `transparent` forwards to `Quaternion`'s own (just-added) impl.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value", transparent)]
pub struct UnitQuaternion(Quaternion);

impl UnitQuaternion {
    pub fn identity() -> Self {
        Self(Quaternion::new(1.0, 0.0, 0.0, 0.0))
    }

    /// 🌀️ Normalizes `q`; falls back to identity for a (numerically) zero quaternion.
    pub fn from_quaternion(q: Quaternion) -> Self {
        let n = (q.i * q.i + q.j * q.j + q.k * q.k + q.w * q.w).sqrt();
        if n > ROTATION_EPS {
            Self(Quaternion { i: q.i / n, j: q.j / n, k: q.k / n, w: q.w / n })
        } else {
            Self::identity()
        }
    }

    pub fn quaternion(self) -> Quaternion {
        self.0
    }

    fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
        let (sin_half, cos_half) = (angle * 0.5).sin_cos();
        Self(Quaternion { i: axis.x * sin_half, j: axis.y * sin_half, k: axis.z * sin_half, w: cos_half })
    }

    /// 🧭️ The shortest rotation carrying `from` onto `to`. `None` exactly when either vector is
    /// (numerically) zero, or the two are exactly anti-parallel — the rotation axis is undefined
    /// there, matching `nalgebra::UnitQuaternion::rotation_between`.
    pub fn rotation_between(from: Vector3, to: Vector3) -> Option<Self> {
        let na = from.try_normalize(ROTATION_EPS)?;
        let nb = to.try_normalize(ROTATION_EPS)?;
        let axis = na.cross(nb);
        let axis_norm = axis.norm();
        if axis_norm > ROTATION_EPS {
            let unit_axis = axis * (1.0 / axis_norm);
            let cos = na.dot(nb).clamp(-1.0, 1.0);
            Some(Self::from_axis_angle(unit_axis, cos.acos()))
        } else if na.dot(nb) < 0.0 {
            None
        } else {
            Some(Self::identity())
        }
    }

    /// 🔄️ Rotates `v` by this quaternion (Fabian Giesen's optimized `qvq⁻¹` expansion).
    pub fn apply(self, v: Vector3) -> Vector3 {
        let q = self.0;
        let axis = Vector3::new(q.i, q.j, q.k);
        let t = axis.cross(v) * 2.0;
        v + t * q.w + axis.cross(t)
    }

    /// ✳️ Hamilton product: applying `self.compose(other)` equals applying `other` then `self`.
    pub fn compose(self, other: Self) -> Self {
        let (a, b) = (self.0, other.0);
        Self(Quaternion {
            w: a.w * b.w - a.i * b.i - a.j * b.j - a.k * b.k,
            i: a.w * b.i + a.i * b.w + a.j * b.k - a.k * b.j,
            j: a.w * b.j - a.i * b.k + a.j * b.w + a.k * b.i,
            k: a.w * b.k + a.i * b.j - a.j * b.i + a.k * b.w,
        })
    }

    pub fn inverse(self) -> Self {
        let q = self.0;
        Self(Quaternion { i: -q.i, j: -q.j, k: -q.k, w: q.w })
    }
}
//#endregion 🔖️Quaternion

//#region 🔖️Isometry3
/// 🧷️ A rigid transform: rotate then translate, `nalgebra::Isometry3`-shaped.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `Vector3` above.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct Isometry3 {
    pub translation: Vector3,
    pub rotation: UnitQuaternion,
}

impl Isometry3 {
    pub fn identity() -> Self {
        Self { translation: Vector3::zero(), rotation: UnitQuaternion::identity() }
    }

    pub fn from_parts(translation: Vector3, rotation: UnitQuaternion) -> Self {
        Self { translation, rotation }
    }

    pub fn inverse(self) -> Self {
        let inverse_rotation = self.rotation.inverse();
        Self { translation: inverse_rotation.apply(-self.translation), rotation: inverse_rotation }
    }

    pub fn transform_point(self, point: Point3) -> Point3 {
        Point3::from_coords(self.rotation.apply(point.coords()) + self.translation)
    }

    /// ✳️ `self.compose(other)` applied to a point equals applying `other` then `self`
    /// (`nalgebra`'s `self * other`).
    pub fn compose(self, other: Self) -> Self {
        Self { translation: self.translation + self.rotation.apply(other.translation), rotation: self.rotation.compose(other.rotation) }
    }
}
//#endregion 🔖️Isometry3

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_rotation_leaves_vectors_unchanged() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(UnitQuaternion::identity().apply(v), v);
    }

    #[test]
    fn rotation_between_parallel_is_identity() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(UnitQuaternion::rotation_between(a, a * 2.0), Some(UnitQuaternion::identity()));
    }

    #[test]
    fn rotation_between_anti_parallel_is_none() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(UnitQuaternion::rotation_between(a, -a), None);
    }

    #[test]
    fn rotation_between_quarter_turn_carries_from_onto_to() {
        let from = Vector3::new(1.0, 0.0, 0.0);
        let to = Vector3::new(0.0, 1.0, 0.0);
        let rotation = UnitQuaternion::rotation_between(from, to).expect("defined axis");
        let rotated = rotation.apply(from);
        assert!((rotated.x - to.x).abs() < 1e-5 && (rotated.y - to.y).abs() < 1e-5 && (rotated.z - to.z).abs() < 1e-5);
    }

    #[test]
    fn isometry_inverse_round_trips_a_point() {
        let pose = Isometry3::from_parts(Vector3::new(3.0, -2.0, 5.0), UnitQuaternion::rotation_between(Vector3::new(0.0, 0.0, 1.0), Vector3::new(1.0, 0.0, 0.0)).expect("defined axis"));
        let point = Point3::new(1.0, 1.0, 1.0);
        let round_tripped = pose.inverse().transform_point(pose.transform_point(point));
        assert!((round_tripped.x - point.x).abs() < 1e-5 && (round_tripped.y - point.y).abs() < 1e-5 && (round_tripped.z - point.z).abs() < 1e-5);
    }

    #[test]
    fn isometry_compose_matches_sequential_application() {
        let a = Isometry3::from_parts(Vector3::new(1.0, 0.0, 0.0), UnitQuaternion::identity());
        let b = Isometry3::from_parts(Vector3::new(0.0, 2.0, 0.0), UnitQuaternion::identity());
        let point = Point3::new(0.0, 0.0, 0.0);
        let composed = a.compose(b).transform_point(point);
        let sequential = a.transform_point(b.transform_point(point));
        assert_eq!(composed, sequential);
    }
}
