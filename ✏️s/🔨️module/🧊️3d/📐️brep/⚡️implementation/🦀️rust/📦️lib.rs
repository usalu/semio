//! 🔩️ Brepkit-backed implementation of [`kernel_3d_engine::BrepKernel`] (being replaced in place by
//! a dependency-free native kernel — see `.🦑️repo/🎫️tickets/26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`).
//! The native modules below are additive: they compile alongside the brepkit wrapper until the
//! ticket's Flip phase swaps consumers over and deletes the wrapper.

// #region 🔖️NativeModules
// #region 🔖️Error
pub mod error {
//! 🚨️ Flat, hand-rolled error enums for every kernel subsystem (no `thiserror` — matches the
//! `mathematical_wfc` convention). Each variant carries just enough context to explain *why* an
//! operation refused to produce a result; the kernel's hard invariant is "never wrong, fail loud"
//! rather than silently returning a plausible-looking but invalid shape.

// #region 🔖️Errors

/// 🚨️ Top-level error returned by every `Brep` mutating/query method.
#[derive(Clone, Debug, PartialEq)]
pub enum KernelError {
    /// 🚨️ A caller-supplied parameter is out of range or otherwise malformed.
    InvalidInput(String),
    /// 🚨️ A referenced entity id does not exist (or belongs to another `Body`).
    MissingEntity(String),
    /// 🚨️ The operation is well-formed but could not be completed.
    Operation(String),
    /// 🚨️ An intersection sub-problem could not be resolved to certified geometry.
    Intersect(IntersectError),
    /// 🚨️ A Boolean combination could not be completed.
    Boolean(BooleanError),
    /// 🚨️ STEP import/export failed.
    Step(StepError),
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KernelError::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            KernelError::MissingEntity(id) => write!(f, "missing entity: {id}"),
            KernelError::Operation(msg) => write!(f, "operation failed: {msg}"),
            KernelError::Intersect(e) => write!(f, "intersection failed: {e}"),
            KernelError::Boolean(e) => write!(f, "boolean failed: {e}"),
            KernelError::Step(e) => write!(f, "step failed: {e}"),
        }
    }
}

impl std::error::Error for KernelError {}

impl From<IntersectError> for KernelError {
    fn from(e: IntersectError) -> Self {
        KernelError::Intersect(e)
    }
}

impl From<BooleanError> for KernelError {
    fn from(e: BooleanError) -> Self {
        KernelError::Boolean(e)
    }
}

impl From<StepError> for KernelError {
    fn from(e: StepError) -> Self {
        KernelError::Step(e)
    }
}

/// 🚨️ Curve/curve, curve/surface and surface/surface intersection failure modes.
#[derive(Clone, Debug, PartialEq)]
pub enum IntersectError {
    /// 🚨️ The two operands are tangent within tolerance; the caller must use a dedicated
    /// tangency-aware path rather than the generic intersector.
    Tangent,
    /// 🚨️ The general (marching) path failed to converge or close a loop within its iteration budget.
    Unresolved(String),
    /// 🚨️ The operands are geometrically degenerate (zero-length curve, singular surface point, …).
    Degenerate(String),
}

impl std::fmt::Display for IntersectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntersectError::Tangent => write!(f, "tangent configuration"),
            IntersectError::Unresolved(msg) => write!(f, "unresolved: {msg}"),
            IntersectError::Degenerate(msg) => write!(f, "degenerate: {msg}"),
        }
    }
}

/// 🚨️ Boolean pipeline failure modes.
#[derive(Clone, Debug, PartialEq)]
pub enum BooleanError {
    /// 🚨️ Face imprinting could not resolve a consistent UV arrangement.
    ImprintFailed(String),
    /// 🚨️ A cell produced by the arrangement could not be classified with certainty.
    ClassificationAmbiguous(String),
    /// 🚨️ The stitched result failed shape validation.
    InvalidResult(String),
    /// 🚨️ An intersection sub-step failed.
    Intersect(IntersectError),
}

impl std::fmt::Display for BooleanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BooleanError::ImprintFailed(msg) => write!(f, "imprint failed: {msg}"),
            BooleanError::ClassificationAmbiguous(msg) => write!(f, "ambiguous classification: {msg}"),
            BooleanError::InvalidResult(msg) => write!(f, "invalid result: {msg}"),
            BooleanError::Intersect(e) => write!(f, "{e}"),
        }
    }
}

impl From<IntersectError> for BooleanError {
    fn from(e: IntersectError) -> Self {
        BooleanError::Intersect(e)
    }
}

/// 🚨️ Hand-rolled ISO 10303-21 (STEP) reader/writer failure modes.
#[derive(Clone, Debug, PartialEq)]
pub enum StepError {
    /// 🚨️ The Part-21 lexer/parser rejected the input text.
    Syntax(String),
    /// 🚨️ An instance reference (`#123`) does not resolve to any parsed entity.
    UnresolvedReference(u64),
    /// 🚨️ An entity type is recognized but not translatable by this subset reader.
    Unsupported(String),
}

impl std::fmt::Display for StepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepError::Syntax(msg) => write!(f, "syntax error: {msg}"),
            StepError::UnresolvedReference(id) => write!(f, "unresolved reference #{id}"),
            StepError::Unsupported(name) => write!(f, "unsupported entity: {name}"),
        }
    }
}

// #endregion 🔖️Errors

// #region 🔖️Issues

/// 🚨️ A single finding from `validate::validate_body`, scoped to one entity.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidationIssue {
    /// 🚨️ Human-readable entity label the issue is scoped to (e.g. `"edge-3"`).
    pub entity: String,
    /// 🚨️ Machine-readable, stable diagnostic code (e.g. `"same-parameter-violated"`).
    pub code: &'static str,
    /// 🚨️ One-line description of the failure, including the measured residual where relevant.
    pub message: String,
}

impl std::fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.code, self.entity, self.message)
    }
}

// #endregion 🔖️Issues

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_error_displays_readable_message() {
        let e = KernelError::InvalidInput("radius must be positive".to_string());
        assert_eq!(e.to_string(), "invalid input: radius must be positive");
    }

    #[test]
    fn intersect_error_converts_into_kernel_error() {
        let e: KernelError = IntersectError::Tangent.into();
        assert!(matches!(e, KernelError::Intersect(IntersectError::Tangent)));
    }

    #[test]
    fn boolean_error_wraps_intersect_error() {
        let e: BooleanError = IntersectError::Degenerate("zero length".to_string()).into();
        assert!(matches!(e, BooleanError::Intersect(IntersectError::Degenerate(_))));
    }

    #[test]
    fn validation_issue_displays_code_entity_message() {
        let issue = ValidationIssue { entity: "edge-3".to_string(), code: "same-parameter-violated", message: "residual 1e-3 exceeds tol 1e-6".to_string() };
        assert_eq!(issue.to_string(), "[same-parameter-violated] edge-3: residual 1e-3 exceeds tol 1e-6");
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Error

// #region 🔖️Vec
pub mod vec {
//! 📐️ Plain-`f64` 2D/3D vectors and points — no external linear-algebra crate. Points and vectors
//! are kept as distinct newtypes (a point minus a point is a vector; a vector has no fixed origin)
//! so geometric code cannot silently add two points or translate a direction. Every operation here
//! is exact IEEE-754 arithmetic; tolerance-aware comparison lives in [`crate::tolerance`].

// #region 🔖️Scalars

/// 📐️ True when `a` and `b` are within `ulps` representable steps of each other (handles the
/// `0.1+0.2 != 0.3` class of rounding noise without hiding genuinely different values).
pub fn nearly_equal_ulps(a: f64, b: f64, ulps: i64) -> bool {
    if a == b {
        return true;
    }
    if a.is_nan() || b.is_nan() {
        return false;
    }
    let ia = a.to_bits() as i64;
    let ib = b.to_bits() as i64;
    (ia - ib).unsigned_abs() as i64 <= ulps
}

/// 📐️ Normalizes an angle into `[0, 2π)`.
pub fn normalize_angle(theta: f64) -> f64 {
    let two_pi = std::f64::consts::TAU;
    let wrapped = theta % two_pi;
    if wrapped < 0.0 {
        wrapped + two_pi
    } else {
        wrapped
    }
}

/// 📐️ Signed angular difference `b - a`, wrapped into `(-π, π]`.
pub fn angle_diff(a: f64, b: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let d = (b - a) % std::f64::consts::TAU;
    if d > pi {
        d - std::f64::consts::TAU
    } else if d <= -pi {
        d + std::f64::consts::TAU
    } else {
        d
    }
}

// #endregion 🔖️Scalars

// #region 🔖️Vec2

/// 📐️ A free 2D direction/displacement (no fixed origin).
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

/// 📐️ A located 2D point, e.g. a parametric-domain (u, v) coordinate.
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pnt2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
    pub fn dot(self, o: Vec2) -> f64 {
        self.x * o.x + self.y * o.y
    }
    /// 📐️ The z-component of the 3D cross product `(x,y,0) × (o.x,o.y,0)`; its sign is the
    /// orientation of the turn from `self` to `o`.
    pub fn cross(self, o: Vec2) -> f64 {
        self.x * o.y - self.y * o.x
    }
    pub fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn norm_sq(self) -> f64 {
        self.dot(self)
    }
    /// 📐️ Returns `None` when the vector is (numerically) zero-length rather than dividing by
    /// zero into a `NaN`/`inf` direction.
    pub fn normalized(self) -> Option<Vec2> {
        let n = self.norm();
        if n <= f64::EPSILON {
            None
        } else {
            Some(self * (1.0 / n))
        }
    }
    pub fn perp(self) -> Vec2 {
        Vec2::new(-self.y, self.x)
    }
    pub fn lerp(self, o: Vec2, t: f64) -> Vec2 {
        self * (1.0 - t) + o * t
    }
    pub fn angle(self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x + o.x, self.y + o.y)
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x - o.x, self.y - o.y)
    }
}
impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f64) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}
impl std::ops::Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl Pnt2 {
    pub fn new(x: f64, y: f64) -> Self {
        Pnt2 { x, y }
    }
    pub fn to_vec(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    pub fn lerp(self, o: Pnt2, t: f64) -> Pnt2 {
        Pnt2::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t)
    }
    pub fn distance(self, o: Pnt2) -> f64 {
        (self - o).norm()
    }
    pub fn distance_sq(self, o: Pnt2) -> f64 {
        (self - o).norm_sq()
    }
}
impl std::ops::Sub for Pnt2 {
    type Output = Vec2;
    fn sub(self, o: Pnt2) -> Vec2 {
        Vec2::new(self.x - o.x, self.y - o.y)
    }
}
impl std::ops::Add<Vec2> for Pnt2 {
    type Output = Pnt2;
    fn add(self, v: Vec2) -> Pnt2 {
        Pnt2::new(self.x + v.x, self.y + v.y)
    }
}

// #endregion 🔖️Vec2

// #region 🔖️Vec3

/// 📐️ A free 3D direction/displacement (no fixed origin).
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 📐️ A located 3D point in model space.
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pnt3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const X: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn from_array(a: [f64; 3]) -> Self {
        Vec3::new(a[0], a[1], a[2])
    }
    pub fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    pub fn dot(self, o: Vec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }
    pub fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn norm_sq(self) -> f64 {
        self.dot(self)
    }
    pub fn normalized(self) -> Option<Vec3> {
        let n = self.norm();
        if n <= f64::EPSILON {
            None
        } else {
            Some(self * (1.0 / n))
        }
    }
    pub fn lerp(self, o: Vec3, t: f64) -> Vec3 {
        self * (1.0 - t) + o * t
    }
    /// 📐️ Any unit vector orthogonal to `self` — used to seed orthonormal frames. Picks the
    /// world axis least aligned with `self` before cross-producting, so the result stays
    /// well-conditioned even when `self` is nearly axis-aligned.
    pub fn any_orthogonal(self) -> Vec3 {
        let ax = self.x.abs();
        let ay = self.y.abs();
        let az = self.z.abs();
        let seed = if ax <= ay && ax <= az {
            Vec3::X
        } else if ay <= az {
            Vec3::Y
        } else {
            Vec3::Z
        };
        self.cross(seed).normalized().unwrap_or(Vec3::X)
    }
    pub fn angle_to(self, o: Vec3) -> f64 {
        let cross = self.cross(o).norm();
        let dot = self.dot(o);
        cross.atan2(dot)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f64) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Pnt3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Pnt3 { x, y, z }
    }
    pub fn from_array(a: [f64; 3]) -> Self {
        Pnt3::new(a[0], a[1], a[2])
    }
    pub fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    pub fn to_vec(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    pub fn lerp(self, o: Pnt3, t: f64) -> Pnt3 {
        Pnt3::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t, self.z + (o.z - self.z) * t)
    }
    pub fn distance(self, o: Pnt3) -> f64 {
        (self - o).norm()
    }
    pub fn distance_sq(self, o: Pnt3) -> f64 {
        (self - o).norm_sq()
    }
}
impl std::ops::Sub for Pnt3 {
    type Output = Vec3;
    fn sub(self, o: Pnt3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl std::ops::Add<Vec3> for Pnt3 {
    type Output = Pnt3;
    fn add(self, v: Vec3) -> Pnt3 {
        Pnt3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}
impl std::ops::Sub<Vec3> for Pnt3 {
    type Output = Pnt3;
    fn sub(self, v: Vec3) -> Pnt3 {
        Pnt3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

// #endregion 🔖️Vec3

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product_is_orthogonal_to_both_operands() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, 0.5, 4.0);
        let c = a.cross(b);
        assert!(c.dot(a).abs() < 1e-12);
        assert!(c.dot(b).abs() < 1e-12);
    }

    #[test]
    fn normalized_returns_none_for_zero_vector() {
        assert_eq!(Vec3::ZERO.normalized(), None);
        assert_eq!(Vec2::ZERO.normalized(), None);
    }

    #[test]
    fn normalized_has_unit_length() {
        let v = Vec3::new(3.0, 4.0, 0.0).normalized().unwrap();
        assert!((v.norm() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn any_orthogonal_is_perpendicular_for_all_axis_aligned_inputs() {
        for v in [Vec3::X, Vec3::Y, Vec3::Z, -Vec3::X, -Vec3::Y, -Vec3::Z, Vec3::new(1.0, 1.0, 1.0)] {
            let o = v.any_orthogonal();
            assert!(v.dot(o).abs() < 1e-9, "not orthogonal for {v:?}: dot={}", v.dot(o));
            assert!((o.norm() - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn vec2_cross_sign_matches_turn_direction() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert!(a.cross(b) > 0.0);
        assert!(b.cross(a) < 0.0);
    }

    #[test]
    fn point_minus_point_is_vector_and_point_plus_vector_is_point() {
        let p = Pnt3::new(1.0, 2.0, 3.0);
        let q = Pnt3::new(4.0, 6.0, 8.0);
        let v: Vec3 = q - p;
        assert_eq!(v, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(p + v, q);
    }

    #[test]
    fn normalize_angle_wraps_into_0_tau() {
        assert!((normalize_angle(-0.1) - (std::f64::consts::TAU - 0.1)).abs() < 1e-12);
        assert!((normalize_angle(std::f64::consts::TAU + 0.5) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn angle_diff_wraps_into_minus_pi_pi() {
        let d = angle_diff(0.1, std::f64::consts::TAU - 0.1);
        assert!((d - (-0.2)).abs() < 1e-9);
    }

    #[test]
    fn lerp_at_endpoints_returns_endpoints() {
        let a = Pnt3::new(0.0, 0.0, 0.0);
        let b = Pnt3::new(10.0, 20.0, 30.0);
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
    }

    mod quick {
        use super::*;

        #[test]
        fn cross_product_antisymmetric_on_random_vectors() {
            let mut rng = mathematical_random::Rng::from_seed(1);
            for _ in 0..200 {
                let a = Vec3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let b = Vec3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let ab = a.cross(b);
                let ba = b.cross(a);
                assert!((ab + ba).norm() < 1e-9);
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Vec

// #region 🔖️Mat
pub mod mat {
//! 🧭️ 3×3/4×4 matrices, unit quaternions, rigid+uniform-scale transforms and orthonormal frames —
//! plain `f64` arrays, no external linear-algebra crate. `Trsf` (not a raw `Mat4`) is the type
//! every kernel operation accepts for placement, so a transform can never silently carry
//! non-uniform shear that would break analytic-surface recognition downstream.

use crate::vec::{Pnt3, Vec3};

// #region 🔖️Mat

/// 🧭️ A 3×3 matrix in row-major order, used for rotations and normal-transform cofactors.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Mat3 {
    pub rows: [[f64; 3]; 3],
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Mat3 { rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

    pub fn from_rows(rows: [[f64; 3]; 3]) -> Self {
        Mat3 { rows }
    }
    pub fn from_columns(c0: Vec3, c1: Vec3, c2: Vec3) -> Self {
        Mat3 { rows: [[c0.x, c1.x, c2.x], [c0.y, c1.y, c2.y], [c0.z, c1.z, c2.z]] }
    }
    pub fn column(&self, i: usize) -> Vec3 {
        Vec3::new(self.rows[0][i], self.rows[1][i], self.rows[2][i])
    }
    pub fn transform(&self, v: Vec3) -> Vec3 {
        Vec3::new(self.rows[0][0] * v.x + self.rows[0][1] * v.y + self.rows[0][2] * v.z, self.rows[1][0] * v.x + self.rows[1][1] * v.y + self.rows[1][2] * v.z, self.rows[2][0] * v.x + self.rows[2][1] * v.y + self.rows[2][2] * v.z)
    }
    pub fn transpose(&self) -> Mat3 {
        Mat3::from_rows([[self.rows[0][0], self.rows[1][0], self.rows[2][0]], [self.rows[0][1], self.rows[1][1], self.rows[2][1]], [self.rows[0][2], self.rows[1][2], self.rows[2][2]]])
    }
    pub fn mul(&self, o: &Mat3) -> Mat3 {
        let mut out = [[0.0; 3]; 3];
        for (r, out_row) in out.iter_mut().enumerate() {
            for (c, out_cell) in out_row.iter_mut().enumerate() {
                *out_cell = (0..3).map(|k| self.rows[r][k] * o.rows[k][c]).sum();
            }
        }
        Mat3::from_rows(out)
    }
    pub fn determinant(&self) -> f64 {
        let m = &self.rows;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }
    /// 🧭️ From an axis-angle rotation (Rodrigues' formula). `axis` need not be normalized.
    pub fn from_axis_angle(axis: Vec3, angle: f64) -> Mat3 {
        let a = axis.normalized().unwrap_or(Vec3::Z);
        let (s, c) = angle.sin_cos();
        let t = 1.0 - c;
        Mat3::from_rows([
            [t * a.x * a.x + c, t * a.x * a.y - s * a.z, t * a.x * a.z + s * a.y],
            [t * a.x * a.y + s * a.z, t * a.y * a.y + c, t * a.y * a.z - s * a.x],
            [t * a.x * a.z - s * a.y, t * a.y * a.z + s * a.x, t * a.z * a.z + c],
        ])
    }
}

// #endregion 🔖️Mat

// #region 🔖️Quat

/// 🧭️ A unit quaternion `(w, x, y, z)` representing a pure rotation.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Quat {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quat {
    pub const IDENTITY: Quat = Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };

    pub fn from_axis_angle(axis: Vec3, angle: f64) -> Self {
        let a = axis.normalized().unwrap_or(Vec3::Z);
        let half = angle * 0.5;
        let (s, c) = half.sin_cos();
        Quat { w: c, x: a.x * s, y: a.y * s, z: a.z * s }
    }
    pub fn norm(self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalized(self) -> Quat {
        let n = self.norm();
        if n <= f64::EPSILON {
            Quat::IDENTITY
        } else {
            Quat { w: self.w / n, x: self.x / n, y: self.y / n, z: self.z / n }
        }
    }
    pub fn conjugate(self) -> Quat {
        Quat { w: self.w, x: -self.x, y: -self.y, z: -self.z }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, o: Quat) -> Quat {
        Quat {
            w: self.w * o.w - self.x * o.x - self.y * o.y - self.z * o.z,
            x: self.w * o.x + self.x * o.w + self.y * o.z - self.z * o.y,
            y: self.w * o.y - self.x * o.z + self.y * o.w + self.z * o.x,
            z: self.w * o.z + self.x * o.y - self.y * o.x + self.z * o.w,
        }
    }
    pub fn rotate(self, v: Vec3) -> Vec3 {
        let qv = Quat { w: 0.0, x: v.x, y: v.y, z: v.z };
        let r = self.mul(qv).mul(self.conjugate());
        Vec3::new(r.x, r.y, r.z)
    }
    pub fn to_mat3(self) -> Mat3 {
        let q = self.normalized();
        let (w, x, y, z) = (q.w, q.x, q.y, q.z);
        Mat3::from_rows([
            [1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y - z * w), 2.0 * (x * z + y * w)],
            [2.0 * (x * y + z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z - x * w)],
            [2.0 * (x * z - y * w), 2.0 * (y * z + x * w), 1.0 - 2.0 * (x * x + y * y)],
        ])
    }
    /// 🧭️ Spherical linear interpolation, taking the short arc between `self` and `o`.
    pub fn slerp(self, o: Quat, t: f64) -> Quat {
        let a = self.normalized();
        let mut b = o.normalized();
        let mut dot = a.w * b.w + a.x * b.x + a.y * b.y + a.z * b.z;
        if dot < 0.0 {
            b = Quat { w: -b.w, x: -b.x, y: -b.y, z: -b.z };
            dot = -dot;
        }
        if dot > 0.9995 {
            return Quat { w: a.w + (b.w - a.w) * t, x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t, z: a.z + (b.z - a.z) * t }.normalized();
        }
        let theta0 = dot.acos();
        let theta = theta0 * t;
        let (s, c) = theta.sin_cos();
        let s0 = theta0.sin();
        let scale_a = c - dot * s / s0;
        let scale_b = s / s0;
        Quat { w: a.w * scale_a + b.w * scale_b, x: a.x * scale_a + b.x * scale_b, y: a.y * scale_a + b.y * scale_b, z: a.z * scale_a + b.z * scale_b }
    }
}

// #endregion 🔖️Quat

// #region 🔖️Trsf

/// 🧭️ A rigid transform with optional uniform scale: `p ↦ rotation·(scale·p) + translation`.
/// Deliberately excludes shear/non-uniform scale so analytic surfaces stay analytic after any
/// transform the kernel applies.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trsf {
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: f64,
}

impl Trsf {
    pub const IDENTITY: Trsf = Trsf { rotation: Quat::IDENTITY, translation: Vec3::ZERO, scale: 1.0 };

    pub fn translation(t: Vec3) -> Self {
        Trsf { translation: t, ..Trsf::IDENTITY }
    }
    pub fn rotation(q: Quat) -> Self {
        Trsf { rotation: q, ..Trsf::IDENTITY }
    }
    pub fn uniform_scale(s: f64) -> Self {
        Trsf { scale: s, ..Trsf::IDENTITY }
    }
    pub fn apply_point(&self, p: Pnt3) -> Pnt3 {
        let scaled = p.to_vec() * self.scale;
        Pnt3::from_array(self.rotation.rotate(scaled).to_array()) + self.translation
    }
    pub fn apply_vector(&self, v: Vec3) -> Vec3 {
        self.rotation.rotate(v * self.scale)
    }
    /// 🧭️ Transforms a surface normal correctly under non-uniform-free `Trsf` (uniform scale
    /// leaves direction unchanged, so this is just the rotation — kept as its own method so
    /// callers never reach for `apply_vector` on a normal by habit).
    pub fn apply_normal(&self, n: Vec3) -> Vec3 {
        self.rotation.rotate(n)
    }
    pub fn semio_compose_rs(&self, inner: &Trsf) -> Trsf {
        Trsf { rotation: self.rotation.mul(inner.rotation), translation: self.apply_vector(inner.translation) + self.translation, scale: self.scale * inner.scale }
    }
    pub fn inverse(&self) -> Trsf {
        let inv_rot = self.rotation.conjugate();
        let inv_scale = 1.0 / self.scale;
        let inv_translation = inv_rot.rotate(-self.translation * inv_scale);
        Trsf { rotation: inv_rot, translation: inv_translation, scale: inv_scale }
    }
}

// #endregion 🔖️Trsf

// #region 🔖️Frame

/// 🧭️ A right-handed orthonormal frame: origin plus three unit axes with `z = x × y`.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Frame3 {
    pub origin: Pnt3,
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

impl Frame3 {
    pub const WORLD: Frame3 = Frame3 { origin: Pnt3 { x: 0.0, y: 0.0, z: 0.0 }, x: Vec3::X, y: Vec3::Y, z: Vec3::Z };

    /// 🧭️ Builds a frame from an origin and a normal `z`-axis; `x`/`y` are derived deterministically
    /// via [`Vec3::any_orthogonal`] so the same normal always produces the same frame.
    pub fn from_normal(origin: Pnt3, normal: Vec3) -> Option<Frame3> {
        let z = normal.normalized()?;
        let x = z.any_orthogonal();
        let y = z.cross(x);
        Some(Frame3 { origin, x, y, z })
    }
    /// 🧭️ Builds a frame from an origin, a preferred `x` direction and a `z` normal — `x` is
    /// Gram-Schmidt orthogonalized against `z`, `y` completes the right-handed triad.
    pub fn from_x_z(origin: Pnt3, x_hint: Vec3, z_hint: Vec3) -> Option<Frame3> {
        let z = z_hint.normalized()?;
        let x_proj = x_hint - z * x_hint.dot(z);
        let x = x_proj.normalized()?;
        let y = z.cross(x);
        Some(Frame3 { origin, x, y, z })
    }
    pub fn to_world(&self, local: Pnt3) -> Pnt3 {
        self.origin + self.x * local.x + self.y * local.y + self.z * local.z
    }
    pub fn to_world_vector(&self, local: Vec3) -> Vec3 {
        self.x * local.x + self.y * local.y + self.z * local.z
    }
    pub fn to_local(&self, world: Pnt3) -> Pnt3 {
        let v = world - self.origin;
        Pnt3::new(v.dot(self.x), v.dot(self.y), v.dot(self.z))
    }
    pub fn to_local_vector(&self, world: Vec3) -> Vec3 {
        Vec3::new(world.dot(self.x), world.dot(self.y), world.dot(self.z))
    }
}

// #endregion 🔖️Frame

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quat_from_axis_angle_rotates_correctly() {
        let q = Quat::from_axis_angle(Vec3::Z, std::f64::consts::FRAC_PI_2);
        let r = q.rotate(Vec3::X);
        assert!((r - Vec3::Y).norm() < 1e-9);
    }

    #[test]
    fn quat_conjugate_is_inverse_rotation() {
        let q = Quat::from_axis_angle(Vec3::new(1.0, 2.0, 3.0), 0.7);
        let v = Vec3::new(4.0, -1.0, 2.0);
        let round_trip = q.conjugate().rotate(q.rotate(v));
        assert!((round_trip - v).norm() < 1e-9);
    }

    #[test]
    fn quat_to_mat3_matches_direct_rotation() {
        let q = Quat::from_axis_angle(Vec3::new(0.3, 0.7, -0.2), 1.1);
        let v = Vec3::new(1.0, 0.0, 0.0);
        let via_quat = q.rotate(v);
        let via_mat = q.to_mat3().transform(v);
        assert!((via_quat - via_mat).norm() < 1e-9);
    }

    #[test]
    fn slerp_endpoints_match_inputs() {
        let a = Quat::from_axis_angle(Vec3::Z, 0.0);
        let b = Quat::from_axis_angle(Vec3::Z, 1.5);
        let s0 = a.slerp(b, 0.0);
        let s1 = a.slerp(b, 1.0);
        assert!((s0.rotate(Vec3::X) - a.rotate(Vec3::X)).norm() < 1e-9);
        assert!((s1.rotate(Vec3::X) - b.rotate(Vec3::X)).norm() < 1e-9);
    }

    #[test]
    fn trsf_inverse_round_trips_a_point() {
        let t = Trsf { rotation: Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0), 0.9), translation: Vec3::new(5.0, -2.0, 3.0), scale: 2.5 };
        let p = Pnt3::new(1.0, 2.0, 3.0);
        let round_trip = t.inverse().apply_point(t.apply_point(p));
        assert!(round_trip.distance(p) < 1e-9);
    }

    #[test]
    fn trsf_compose_matches_sequential_application() {
        let a = Trsf { rotation: Quat::from_axis_angle(Vec3::Z, 0.4), translation: Vec3::new(1.0, 0.0, 0.0), scale: 1.0 };
        let b = Trsf { rotation: Quat::from_axis_angle(Vec3::X, 0.9), translation: Vec3::new(0.0, 2.0, 0.0), scale: 1.5 };
        let p = Pnt3::new(3.0, -1.0, 2.0);
        let composed = a.semio_compose_rs(&b).apply_point(p);
        let sequential = a.apply_point(b.apply_point(p));
        assert!(composed.distance(sequential) < 1e-9);
    }

    #[test]
    fn frame_from_normal_round_trips_local_world() {
        let f = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 5.0)).unwrap();
        let local = Pnt3::new(2.0, -1.0, 0.5);
        let round_trip = f.to_local(f.to_world(local));
        assert!(round_trip.distance(local) < 1e-9);
    }

    #[test]
    fn frame_from_normal_is_right_handed() {
        let f = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        assert!((f.x.cross(f.y) - f.z).norm() < 1e-9);
    }

    #[test]
    fn frame_from_normal_deterministic_for_axis_aligned_normals() {
        let f1 = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::X).unwrap();
        let f2 = Frame3::from_normal(Pnt3::new(5.0, 5.0, 5.0), Vec3::X).unwrap();
        assert_eq!(f1.x, f2.x);
        assert_eq!(f1.y, f2.y);
    }

    #[test]
    fn mat3_determinant_of_identity_is_one() {
        assert!((Mat3::IDENTITY.determinant() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn mat3_from_axis_angle_matches_quat_rotation() {
        let axis = Vec3::new(0.2, -0.4, 0.9);
        let angle = 1.3;
        let m = Mat3::from_axis_angle(axis, angle);
        let q = Quat::from_axis_angle(axis, angle);
        let v = Vec3::new(1.0, 2.0, -3.0);
        assert!((m.transform(v) - q.rotate(v)).norm() < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn trsf_inverse_round_trips_random_points() {
            let mut rng = mathematical_random::Rng::from_seed(7);
            for _ in 0..200 {
                let axis = Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5);
                let angle = rng.next_f64() * std::f64::consts::TAU;
                let t = Trsf { rotation: Quat::from_axis_angle(axis, angle), translation: Vec3::new(rng.next_f64() * 10.0, rng.next_f64() * 10.0, rng.next_f64() * 10.0), scale: 0.1 + rng.next_f64() * 5.0 };
                let p = Pnt3::new(rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 10.0 - 5.0);
                let round_trip = t.inverse().apply_point(t.apply_point(p));
                assert!(round_trip.distance(p) < 1e-7, "round trip drifted: {round_trip:?} vs {p:?}");
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Mat

// #region 🔖️Tolerance
pub mod tolerance {
//! 🎚️ The kernel's tolerance model: a fixed global [`Resolution`], per-entity [`Tol`] values with
//! a containment ordering (vertex ≥ its edges ≥ their faces), and a certified interval type [`Iv`]
//! used by [`crate::predicates`] to decide when a fast `f64` computation is trustworthy versus
//! when it must escalate to exact arithmetic. Geometric decision code should never compare raw
//! `f64`s with `==`/`<` — it should go through a `Tol` or an `Iv`.

// #region 🔖️Resolution

/// 🎚️ Kernel-wide default resolutions, used to seed new tolerances before any entity-specific
/// tightening/loosening happens.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Resolution {
    /// 🎚️ Default linear tolerance (model units).
    pub linear: f64,
    /// 🎚️ Default angular tolerance (radians).
    pub angular: f64,
    /// 🎚️ Default curve/surface parametric tolerance.
    pub param: f64,
}

impl Resolution {
    pub const DEFAULT: Resolution = Resolution { linear: 1e-7, angular: 1e-9, param: 1e-9 };
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::DEFAULT
    }
}

// #endregion 🔖️Resolution

// #region 🔖️Tol

/// 🎚️ A single linear tolerance value attached to one entity (vertex containment ball radius,
/// edge tube radius, or face shell thickness).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Tol(pub f64);

impl Tol {
    pub const DEFAULT: Tol = Tol(Resolution::DEFAULT.linear);

    pub fn new(value: f64) -> Self {
        debug_assert!(value.is_finite(), "tolerance must be finite");
        Tol(value.max(0.0))
    }
    pub fn value(self) -> f64 {
        self.0
    }
    /// 🎚️ True when `distance` is within this tolerance of zero.
    pub fn contains(self, distance: f64) -> bool {
        distance.abs() <= self.0
    }
    /// 🎚️ The tighter (smaller) of two tolerances — used when an operation must satisfy both
    /// operands' requirements simultaneously.
    pub fn tighter(self, o: Tol) -> Tol {
        Tol(self.0.min(o.0))
    }
    /// 🎚️ The looser (larger) of two tolerances — used when propagating tolerance up the
    /// containment hierarchy (an edge's tolerance must cover every incident vertex tolerance).
    pub fn looser(self, o: Tol) -> Tol {
        Tol(self.0.max(o.0))
    }
    /// 🎚️ Scales the tolerance, clamping to zero rather than going negative on a negative factor.
    pub fn scaled(self, factor: f64) -> Tol {
        Tol((self.0 * factor).max(0.0))
    }
}

impl Default for Tol {
    fn default() -> Self {
        Tol::DEFAULT
    }
}

/// 🎚️ Checks the tolerance-containment invariant: every entry in `finer` must be ≤ the
/// corresponding bound in `coarser` (e.g. a vertex's tolerance must be ≥ zero and every incident
/// edge's tolerance ≥ the vertex's, every incident face's ≥ the edge's). Returns the first
/// violating pair, if any.
pub fn check_containment(finer_label: &str, finer: Tol, coarser_label: &str, coarser: Tol) -> Option<(String, String)> {
    if finer.value() > coarser.value() {
        Some((finer_label.to_string(), coarser_label.to_string()))
    } else {
        None
    }
}

// #endregion 🔖️Tol

// #region 🔖️Interval

/// 🎚️ A certified closed interval `[lo, hi]` used for filtered (interval-arithmetic) evaluation.
/// Arithmetic ops widen conservatively so that the true real-valued result is always contained in
/// the returned interval — the caller escalates to exact arithmetic exactly when `contains_zero()`
/// leaves ambiguity that matters (a sign test straddling zero).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Iv {
    pub lo: f64,
    pub hi: f64,
}

impl Iv {
    pub fn exact(v: f64) -> Self {
        Iv { lo: v, hi: v }
    }
    pub fn new(lo: f64, hi: f64) -> Self {
        debug_assert!(lo <= hi);
        Iv { lo, hi }
    }
    pub fn contains_zero(self) -> bool {
        self.lo <= 0.0 && self.hi >= 0.0
    }
    /// 🎚️ `Some(true)`/`Some(false)` when the sign is certain, `None` when the interval straddles
    /// zero and the caller must escalate to an exact recomputation.
    pub fn sign(self) -> Option<std::cmp::Ordering> {
        if self.hi < 0.0 {
            Some(std::cmp::Ordering::Less)
        } else if self.lo > 0.0 {
            Some(std::cmp::Ordering::Greater)
        } else if self.lo == 0.0 && self.hi == 0.0 {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, o: Iv) -> Iv {
        Iv::new(self.lo + o.lo, self.hi + o.hi)
    }
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, o: Iv) -> Iv {
        Iv::new(self.lo - o.hi, self.hi - o.lo)
    }
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, o: Iv) -> Iv {
        let candidates = [self.lo * o.lo, self.lo * o.hi, self.hi * o.lo, self.hi * o.hi];
        Iv::new(candidates.iter().copied().fold(f64::INFINITY, f64::min), candidates.iter().copied().fold(f64::NEG_INFINITY, f64::max))
    }
    #[allow(clippy::should_implement_trait)]
    pub fn neg(self) -> Iv {
        Iv::new(-self.hi, -self.lo)
    }
    pub fn widen(self, epsilon: f64) -> Iv {
        Iv::new(self.lo - epsilon, self.hi + epsilon)
    }
}

// #endregion 🔖️Interval

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tol_contains_checks_absolute_distance() {
        let t = Tol::new(0.01);
        assert!(t.contains(0.005));
        assert!(t.contains(-0.005));
        assert!(!t.contains(0.02));
    }

    #[test]
    fn tol_tighter_and_looser_pick_correctly() {
        let a = Tol::new(0.1);
        let b = Tol::new(0.5);
        assert_eq!(a.tighter(b), a);
        assert_eq!(a.looser(b), b);
    }

    #[test]
    fn negative_tolerance_clamps_to_zero() {
        assert_eq!(Tol::new(-1.0), Tol::new(0.0));
    }

    #[test]
    fn check_containment_flags_violation() {
        // A vertex whose own tolerance ball (0.1) is larger than its incident edge's tube (0.01)
        // violates the containment hierarchy: the finer (vertex) must fit inside the coarser (edge).
        let vertex_tol = Tol::new(0.1);
        let edge_tol = Tol::new(0.01);
        let violation = check_containment("vertex-1", vertex_tol, "edge-1", edge_tol);
        assert!(violation.is_some());
        let ok = check_containment("vertex-1", vertex_tol, "edge-1", edge_tol.looser(vertex_tol));
        assert!(ok.is_none());
    }

    #[test]
    fn interval_add_widens_conservatively() {
        let a = Iv::new(1.0, 2.0);
        let b = Iv::new(-1.0, 3.0);
        let sum = a.add(b);
        assert_eq!(sum, Iv::new(0.0, 5.0));
    }

    #[test]
    fn interval_sign_is_none_when_straddling_zero() {
        let iv = Iv::new(-0.001, 0.001);
        assert_eq!(iv.sign(), None);
    }

    #[test]
    fn interval_sign_certain_when_strictly_positive_or_negative() {
        assert_eq!(Iv::new(0.5, 1.0).sign(), Some(std::cmp::Ordering::Greater));
        assert_eq!(Iv::new(-1.0, -0.5).sign(), Some(std::cmp::Ordering::Less));
    }

    #[test]
    fn interval_mul_contains_true_product_for_mixed_signs() {
        let a = Iv::new(-2.0, 3.0);
        let b = Iv::new(-1.0, 4.0);
        let product = a.mul(b);
        assert!(product.lo <= -8.0 && product.hi >= 12.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn interval_arithmetic_always_contains_scalar_result() {
            let mut rng = mathematical_random::Rng::from_seed(3);
            for _ in 0..500 {
                let a = rng.next_f64() * 20.0 - 10.0;
                let b = rng.next_f64() * 20.0 - 10.0;
                let ia = Iv::exact(a);
                let ib = Iv::exact(b);
                let sum = ia.add(ib);
                assert!(sum.lo <= a + b && sum.hi >= a + b);
                let prod = ia.mul(ib);
                assert!(prod.lo <= a * b + 1e-12 && prod.hi >= a * b - 1e-12);
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Tolerance

// #region 🔖️Predicates
pub mod predicates {
//! 🎯️ Robust geometric predicates: a cheap `f64` evaluation plus a conservative forward
//! error bound decides the sign whenever possible; only when the true value could be smaller
//! than the accumulated roundoff does the predicate escalate to exact [`mathematical_number::Rational`]
//! arithmetic (lossless for any finite `f64`, per `Rational::from_f64`). This is deliberately
//! simpler than Shewchuk-style adaptive expansions — the exact path is cold, so raw simplicity
//! beats squeezing out its last microsecond. The hard invariant: a predicate here never returns a
//! wrong sign, only (rarely) pays for a certain one.

use crate::vec::{Pnt2, Pnt3, Vec3};
use mathematical_number::Rational;
use std::cmp::Ordering;

// #region 🔖️Filtered

/// 🎯️ The exact sign of a geometric test — kept distinct from [`Ordering`] so call sites read as
/// geometry (`Orient::Positive`) rather than arithmetic (`Ordering::Greater`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orient {
    Positive,
    Negative,
    Zero,
}

impl From<Ordering> for Orient {
    fn from(o: Ordering) -> Self {
        match o {
            Ordering::Greater => Orient::Positive,
            Ordering::Less => Orient::Negative,
            Ordering::Equal => Orient::Zero,
        }
    }
}

/// 🎯️ Decides the sign of `value` (a sum of `terms`) certainly, or returns `None` when
/// accumulated floating-point roundoff across `terms.len()` operations could plausibly have
/// flipped the sign — the caller must then escalate to exact arithmetic.
fn filtered_sign(value: f64, terms: &[f64]) -> Option<Orient> {
    let magnitude: f64 = terms.iter().map(|t| t.abs()).sum();
    let bound = (terms.len() as f64 + 1.0) * f64::EPSILON * magnitude;
    if value > bound {
        Some(Orient::Positive)
    } else if value < -bound {
        Some(Orient::Negative)
    } else {
        None
    }
}

fn to_rational(v: f64) -> Rational {
    Rational::from_f64(v).expect("finite f64 is always exactly representable as a Rational")
}

fn rational_sign(v: &Rational) -> Orient {
    Orient::from(v.cmp(&Rational::zero()))
}

// #endregion 🔖️Filtered

// #region 🔖️Exact

/// 🎯️ Orientation of three 2D points: [`Orient::Positive`] when `a → b → c` turns counterclockwise,
/// [`Orient::Negative`] clockwise, [`Orient::Zero`] when collinear.
pub fn orient2d(a: Pnt2, b: Pnt2, c: Pnt2) -> Orient {
    let acx = b.x - a.x;
    let acy = b.y - a.y;
    let bcx = c.x - a.x;
    let bcy = c.y - a.y;
    let det_left = acx * bcy;
    let det_right = acy * bcx;
    let det = det_left - det_right;
    filtered_sign(det, &[det_left, det_right]).unwrap_or_else(|| orient2d_exact(a, b, c))
}

fn orient2d_exact(a: Pnt2, b: Pnt2, c: Pnt2) -> Orient {
    let (ax, ay) = (to_rational(a.x), to_rational(a.y));
    let (bx, by) = (to_rational(b.x), to_rational(b.y));
    let (cx, cy) = (to_rational(c.x), to_rational(c.y));
    let acx = bx.sub(&ax);
    let acy = by.sub(&ay);
    let bcx = cx.sub(&ax);
    let bcy = cy.sub(&ay);
    let det = acx.mul(&bcy).sub(&acy.mul(&bcx));
    rational_sign(&det)
}

/// 🎯️ Orientation of four 3D points via the signed volume of tetrahedron `(a,b,c,d)`, computed as
/// the scalar triple product `(b-a) · ((c-a) × (d-a))`. [`Orient::Positive`] when `(b-a,c-a,d-a)`
/// form a right-handed frame; [`Orient::Zero`] when the four points are coplanar.
pub fn orient3d(a: Pnt3, b: Pnt3, c: Pnt3, d: Pnt3) -> Orient {
    let u = b - a;
    let v = c - a;
    let w = d - a;
    let t1 = u.x * v.y * w.z;
    let t2 = u.x * v.z * w.y;
    let t3 = u.y * v.z * w.x;
    let t4 = u.y * v.x * w.z;
    let t5 = u.z * v.x * w.y;
    let t6 = u.z * v.y * w.x;
    let det = t1 - t2 + t3 - t4 + t5 - t6;
    filtered_sign(det, &[t1, t2, t3, t4, t5, t6]).unwrap_or_else(|| orient3d_exact(a, b, c, d))
}

fn orient3d_exact(a: Pnt3, b: Pnt3, c: Pnt3, d: Pnt3) -> Orient {
    let ax = to_rational(a.x);
    let ay = to_rational(a.y);
    let az = to_rational(a.z);
    let ux = to_rational(b.x).sub(&ax);
    let uy = to_rational(b.y).sub(&ay);
    let uz = to_rational(b.z).sub(&az);
    let vx = to_rational(c.x).sub(&ax);
    let vy = to_rational(c.y).sub(&ay);
    let vz = to_rational(c.z).sub(&az);
    let wx = to_rational(d.x).sub(&ax);
    let wy = to_rational(d.y).sub(&ay);
    let wz = to_rational(d.z).sub(&az);
    let t1 = ux.mul(&vy).mul(&wz);
    let t2 = ux.mul(&vz).mul(&wy);
    let t3 = uy.mul(&vz).mul(&wx);
    let t4 = uy.mul(&vx).mul(&wz);
    let t5 = uz.mul(&vx).mul(&wy);
    let t6 = uz.mul(&vy).mul(&wx);
    let det = t1.sub(&t2).add(&t3).sub(&t4).add(&t5).sub(&t6);
    rational_sign(&det)
}

/// 🎯️ The incircle test: [`Orient::Positive`] when `d` lies strictly inside the circle through
/// `a, b, c` (assuming `a, b, c` are given counterclockwise), [`Orient::Negative`] outside,
/// [`Orient::Zero`] on the circle.
pub fn in_circle2d(a: Pnt2, b: Pnt2, c: Pnt2, d: Pnt2) -> Orient {
    let adx = a.x - d.x;
    let ady = a.y - d.y;
    let bdx = b.x - d.x;
    let bdy = b.y - d.y;
    let cdx = c.x - d.x;
    let cdy = c.y - d.y;
    let ad2 = adx * adx + ady * ady;
    let bd2 = bdx * bdx + bdy * bdy;
    let cd2 = cdx * cdx + cdy * cdy;
    let t1 = adx * (bdy * cd2 - cdy * bd2);
    let t2 = ady * (bdx * cd2 - cdx * bd2);
    let t3 = ad2 * (bdx * cdy - cdx * bdy);
    let det = t1 - t2 + t3;
    filtered_sign(det, &[t1, t2, t3]).unwrap_or_else(|| in_circle2d_exact(a, b, c, d))
}

fn in_circle2d_exact(a: Pnt2, b: Pnt2, c: Pnt2, d: Pnt2) -> Orient {
    let dx = to_rational(d.x);
    let dy = to_rational(d.y);
    let adx = to_rational(a.x).sub(&dx);
    let ady = to_rational(a.y).sub(&dy);
    let bdx = to_rational(b.x).sub(&dx);
    let bdy = to_rational(b.y).sub(&dy);
    let cdx = to_rational(c.x).sub(&dx);
    let cdy = to_rational(c.y).sub(&dy);
    let ad2 = adx.mul(&adx).add(&ady.mul(&ady));
    let bd2 = bdx.mul(&bdx).add(&bdy.mul(&bdy));
    let cd2 = cdx.mul(&cdx).add(&cdy.mul(&cdy));
    let t1 = adx.mul(&bdy.mul(&cd2).sub(&cdy.mul(&bd2)));
    let t2 = ady.mul(&bdx.mul(&cd2).sub(&cdx.mul(&bd2)));
    let t3 = ad2.mul(&bdx.mul(&cdy).sub(&cdx.mul(&bdy)));
    let det = t1.sub(&t2).add(&t3);
    rational_sign(&det)
}

/// 🎯️ True when `a, b, c` are collinear within the exact predicate (i.e. `orient2d` is exactly zero).
pub fn collinear2d(a: Pnt2, b: Pnt2, c: Pnt2) -> bool {
    orient2d(a, b, c) == Orient::Zero
}

/// 🎯️ True when `a, b, c, d` are coplanar within the exact predicate.
pub fn coplanar3d(a: Pnt3, b: Pnt3, c: Pnt3, d: Pnt3) -> bool {
    orient3d(a, b, c, d) == Orient::Zero
}

/// 🎯️ The certified sign of `u · v` — used to classify angles as acute/obtuse/right without a
/// raw `f64` comparison.
pub fn sign_of_dot(u: Vec3, v: Vec3) -> Orient {
    let tx = u.x * v.x;
    let ty = u.y * v.y;
    let tz = u.z * v.z;
    let dot = tx + ty + tz;
    filtered_sign(dot, &[tx, ty, tz]).unwrap_or_else(|| sign_of_dot_exact(u, v))
}

fn sign_of_dot_exact(u: Vec3, v: Vec3) -> Orient {
    let tx = to_rational(u.x).mul(&to_rational(v.x));
    let ty = to_rational(u.y).mul(&to_rational(v.y));
    let tz = to_rational(u.z).mul(&to_rational(v.z));
    let dot = tx.add(&ty).add(&tz);
    rational_sign(&dot)
}

// #endregion 🔖️Exact

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orient2d_detects_counterclockwise_and_clockwise() {
        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(0.0, 1.0);
        assert_eq!(orient2d(a, b, c), Orient::Positive);
        assert_eq!(orient2d(a, c, b), Orient::Negative);
    }

    #[test]
    fn orient2d_detects_exact_collinearity() {
        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 1.0);
        let c = Pnt2::new(2.0, 2.0);
        assert_eq!(orient2d(a, b, c), Orient::Zero);
        assert!(collinear2d(a, b, c));
    }

    /// 🎯️ The true next representable `f64` above/below `x` — unlike adding `f64::EPSILON`, this
    /// is a real one-bit perturbation regardless of `x`'s magnitude (ULP scales with exponent).
    fn next_up(x: f64) -> f64 {
        f64::from_bits(x.to_bits() + 1)
    }
    fn next_down(x: f64) -> f64 {
        f64::from_bits(x.to_bits() - 1)
    }

    #[test]
    fn orient2d_resolves_near_degenerate_case_correctly() {
        // c sits exactly on line a->b->(2,2); perturbing it by a single ULP must still resolve
        // to the geometrically correct sign, which the interval filter alone cannot certify.
        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 1.0);
        let c_on = Pnt2::new(2.0, 2.0);
        let c_left = Pnt2::new(2.0, next_up(2.0));
        let c_right = Pnt2::new(2.0, next_down(2.0));
        assert_eq!(orient2d(a, b, c_on), Orient::Zero);
        assert_eq!(orient2d(a, b, c_left), Orient::Positive);
        assert_eq!(orient2d(a, b, c_right), Orient::Negative);
    }

    #[test]
    fn orient3d_detects_right_handed_and_left_handed_tetrahedra() {
        let a = Pnt3::new(0.0, 0.0, 0.0);
        let b = Pnt3::new(1.0, 0.0, 0.0);
        let c = Pnt3::new(0.0, 1.0, 0.0);
        let d = Pnt3::new(0.0, 0.0, 1.0);
        assert_eq!(orient3d(a, b, c, d), Orient::Positive);
        assert_eq!(orient3d(a, c, b, d), Orient::Negative);
    }

    #[test]
    fn orient3d_detects_exact_coplanarity() {
        let a = Pnt3::new(0.0, 0.0, 0.0);
        let b = Pnt3::new(1.0, 0.0, 0.0);
        let c = Pnt3::new(0.0, 1.0, 0.0);
        let d = Pnt3::new(1.0, 1.0, 0.0);
        assert_eq!(orient3d(a, b, c, d), Orient::Zero);
        assert!(coplanar3d(a, b, c, d));
    }

    #[test]
    fn orient3d_resolves_near_degenerate_case_correctly() {
        let a = Pnt3::new(0.0, 0.0, 0.0);
        let b = Pnt3::new(1.0, 0.0, 0.0);
        let c = Pnt3::new(0.0, 1.0, 0.0);
        let tiny = f64::EPSILON;
        let d_above = Pnt3::new(0.3, 0.3, tiny);
        let d_below = Pnt3::new(0.3, 0.3, -tiny);
        assert_eq!(orient3d(a, b, c, d_above), Orient::Positive);
        assert_eq!(orient3d(a, b, c, d_below), Orient::Negative);
    }

    #[test]
    fn in_circle2d_detects_inside_and_outside_unit_circle() {
        let a = Pnt2::new(1.0, 0.0);
        let b = Pnt2::new(0.0, 1.0);
        let c = Pnt2::new(-1.0, 0.0);
        let inside = Pnt2::new(0.0, 0.0);
        let outside = Pnt2::new(0.0, 5.0);
        let on = Pnt2::new(0.0, -1.0);
        assert_eq!(in_circle2d(a, b, c, inside), Orient::Positive);
        assert_eq!(in_circle2d(a, b, c, outside), Orient::Negative);
        assert_eq!(in_circle2d(a, b, c, on), Orient::Zero);
    }

    #[test]
    fn sign_of_dot_classifies_acute_right_obtuse() {
        assert_eq!(sign_of_dot(Vec3::X, Vec3::X), Orient::Positive);
        assert_eq!(sign_of_dot(Vec3::X, Vec3::Y), Orient::Zero);
        assert_eq!(sign_of_dot(Vec3::X, -Vec3::X), Orient::Negative);
    }

    mod quick {
        use super::*;

        #[test]
        fn orient2d_filtered_agrees_with_exact_on_random_and_near_degenerate_triples() {
            let mut rng = mathematical_random::Rng::from_seed(11);
            for _ in 0..5000 {
                let a = Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let b = Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                // Bias half the sample toward near-collinear configurations, where the filter
                // is most likely to need the exact escalation path.
                let c = if rng.next_bool(0.5) {
                    let t = rng.next_f64() * 2.0 - 0.5;
                    let perturb = (rng.next_f64() - 0.5) * 1e-12;
                    Pnt2::new(a.x + (b.x - a.x) * t + perturb, a.y + (b.y - a.y) * t)
                } else {
                    Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0)
                };
                assert_eq!(orient2d(a, b, c), orient2d_exact(a, b, c), "mismatch for {a:?} {b:?} {c:?}");
            }
        }

        #[test]
        fn orient3d_filtered_agrees_with_exact_on_random_and_near_degenerate_quadruples() {
            let mut rng = mathematical_random::Rng::from_seed(13);
            for _ in 0..3000 {
                let a = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let b = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let c = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let d = if rng.next_bool(0.5) {
                    let u = rng.next_f64() * 2.0 - 0.5;
                    let v = rng.next_f64() * 2.0 - 0.5;
                    let perturb = (rng.next_f64() - 0.5) * 1e-12;
                    Pnt3::new(a.x + (b.x - a.x) * u + (c.x - a.x) * v + perturb, a.y + (b.y - a.y) * u + (c.y - a.y) * v, a.z + (b.z - a.z) * u + (c.z - a.z) * v)
                } else {
                    Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0)
                };
                assert_eq!(orient3d(a, b, c, d), orient3d_exact(a, b, c, d), "mismatch for {a:?} {b:?} {c:?} {d:?}");
            }
        }

        #[test]
        fn in_circle2d_filtered_agrees_with_exact_on_random_configurations() {
            let mut rng = mathematical_random::Rng::from_seed(17);
            for _ in 0..3000 {
                let pts: Vec<Pnt2> = (0..4).map(|_| Pnt2::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0)).collect();
                assert_eq!(in_circle2d(pts[0], pts[1], pts[2], pts[3]), in_circle2d_exact(pts[0], pts[1], pts[2], pts[3]));
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Predicates

// #region 🔖️Oracle
pub mod oracle {
//! 🔮️ Ground truth used only by tests, kept deliberately independent from the kernel's own
//! algorithms (WFC-crate convention: a brute-force oracle catches bugs a self-referential test
//! never could). This module grows alongside the kernel — [`Sdf`] lands in Phase 0 with the
//! primitives it can already describe; mass-property, watertightness and shape-generator oracles
//! land in the phases that need them.

use crate::mat::Trsf;
use crate::vec::Pnt3;

// #region 🔖️Sdf

/// 🔮️ A closed-form signed distance field: negative inside, zero on the boundary, positive
/// outside. Used to probe classification and Boolean results independently of the kernel's own
/// ray-casting/arrangement code.
#[derive(Clone, Debug, PartialEq)]
pub enum Sdf {
    /// 🔮️ Axis-aligned box of the given half-extents, centered at the origin before `placement`.
    Box { half_extents: Pnt3, placement: Trsf },
    /// 🔮️ Sphere of the given radius, centered at the origin before `placement`.
    Sphere { radius: f64, placement: Trsf },
    /// 🔮️ Cylinder of the given radius and half-height, axis along local `z`, centered at the
    /// origin before `placement`.
    Cylinder { radius: f64, half_height: f64, placement: Trsf },
    /// 🔮️ Boolean combination of two fields.
    Union(Box<Sdf>, Box<Sdf>),
    Intersect(Box<Sdf>, Box<Sdf>),
    Difference(Box<Sdf>, Box<Sdf>),
}

impl Sdf {
    /// 🔮️ Evaluates the field at a world-space point.
    pub fn eval(&self, p: Pnt3) -> f64 {
        match self {
            Sdf::Box { half_extents, placement } => {
                let local = placement.inverse().apply_point(p);
                let dx = local.x.abs() - half_extents.x;
                let dy = local.y.abs() - half_extents.y;
                let dz = local.z.abs() - half_extents.z;
                let outside = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2) + dz.max(0.0).powi(2)).sqrt();
                let inside = dx.max(dy).max(dz).min(0.0);
                outside + inside
            }
            Sdf::Sphere { radius, placement } => {
                let local = placement.inverse().apply_point(p);
                local.to_vec().norm() - radius
            }
            Sdf::Cylinder { radius, half_height, placement } => {
                let local = placement.inverse().apply_point(p);
                let radial = (local.x * local.x + local.y * local.y).sqrt() - radius;
                let axial = local.z.abs() - half_height;
                let outside = (radial.max(0.0).powi(2) + axial.max(0.0).powi(2)).sqrt();
                let inside = radial.max(axial).min(0.0);
                outside + inside
            }
            Sdf::Union(a, b) => a.eval(p).min(b.eval(p)),
            Sdf::Intersect(a, b) => a.eval(p).max(b.eval(p)),
            Sdf::Difference(a, b) => a.eval(p).max(-b.eval(p)),
        }
    }
    /// 🔮️ `true` when `p` is inside (or on, within `tol`) the field's boundary.
    pub fn contains(&self, p: Pnt3, tol: f64) -> bool {
        self.eval(p) <= tol
    }
    pub fn union(self, other: Sdf) -> Sdf {
        Sdf::Union(Box::new(self), Box::new(other))
    }
    pub fn intersect(self, other: Sdf) -> Sdf {
        Sdf::Intersect(Box::new(self), Box::new(other))
    }
    pub fn difference(self, other: Sdf) -> Sdf {
        Sdf::Difference(Box::new(self), Box::new(other))
    }
}

// #endregion 🔖️Sdf

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_sdf_is_negative_inside_and_positive_outside() {
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement: Trsf::IDENTITY };
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(5.0, 0.0, 0.0)) > 0.0);
        assert!((b.eval(Pnt3::new(1.0, 0.0, 0.0))).abs() < 1e-9);
    }

    #[test]
    fn sphere_sdf_matches_analytic_distance() {
        let s = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        assert!((s.eval(Pnt3::new(5.0, 0.0, 0.0)) - 3.0).abs() < 1e-9);
        assert!((s.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-2.0)).abs() < 1e-9);
    }

    #[test]
    fn cylinder_sdf_is_correct_on_axis_and_cap() {
        let c = Sdf::Cylinder { radius: 1.0, half_height: 2.0, placement: Trsf::IDENTITY };
        assert!((c.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-1.0)).abs() < 1e-9);
        assert!((c.eval(Pnt3::new(0.0, 0.0, 5.0)) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn union_is_the_min_and_matches_containment_of_either_operand() {
        let a = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::vec::Vec3::new(-1.0, 0.0, 0.0)) };
        let b = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::vec::Vec3::new(1.0, 0.0, 0.0)) };
        let u = a.union(b);
        assert!(u.contains(Pnt3::new(-1.0, 0.0, 0.0), 1e-9));
        assert!(u.contains(Pnt3::new(1.0, 0.0, 0.0), 1e-9));
        assert!(!u.contains(Pnt3::new(5.0, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn difference_removes_the_second_operand() {
        let big = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        let small = Sdf::Sphere { radius: 1.0, placement: Trsf::IDENTITY };
        let d = big.difference(small);
        assert!(!d.contains(Pnt3::new(0.0, 0.0, 0.0), 1e-9));
        assert!(d.contains(Pnt3::new(1.5, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn placed_box_sdf_respects_transform() {
        let placement = Trsf::translation(crate::vec::Vec3::new(10.0, 0.0, 0.0));
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement };
        assert!(b.eval(Pnt3::new(10.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Oracle

// #region 🔖️Poly
pub mod poly {
//! ∿ Univariate polynomials in monomial and Bernstein form, closed-form low-degree solvers, and a
//! certified general root isolator (Bernstein sign-variation subdivision + safeguarded Newton).
//! The Bernstein form is the workhorse for [`crate::bezier`] and [`crate::bspline`]: its control
//! polygon convex-hulls the curve, so a control-polygon sign change is a *necessary* condition for
//! a root, which is exactly what Descartes' rule of signs turns into a certified root count.

// #region 🔖️Poly

/// ∿ A polynomial in monomial basis: `coeffs[i]` is the coefficient of `x^i`.
#[derive(Clone, Debug, PartialEq)]
pub struct Poly {
    pub coeffs: Vec<f64>,
}

impl Poly {
    pub fn new(coeffs: Vec<f64>) -> Self {
        Poly { coeffs }
    }
    /// ∿ Degree of the polynomial after trimming trailing (highest-order) exact zeros; a
    /// constant zero polynomial has degree `0`.
    pub fn degree(&self) -> usize {
        let mut d = self.coeffs.len().saturating_sub(1);
        while d > 0 && self.coeffs[d] == 0.0 {
            d -= 1;
        }
        d
    }
    /// ∿ Horner evaluation.
    pub fn eval(&self, x: f64) -> f64 {
        self.coeffs.iter().rev().fold(0.0, |acc, &c| acc * x + c)
    }
    /// ∿ Simultaneous Horner evaluation of the polynomial and its derivative (one pass, no
    /// separate `derivative()` allocation on the hot Newton-iteration path).
    pub fn eval_with_derivative(&self, x: f64) -> (f64, f64) {
        let mut value = 0.0;
        let mut deriv = 0.0;
        for &c in self.coeffs.iter().rev() {
            deriv = deriv * x + value;
            value = value * x + c;
        }
        (value, deriv)
    }
    pub fn derivative(&self) -> Poly {
        if self.coeffs.len() <= 1 {
            return Poly::new(vec![0.0]);
        }
        Poly::new(self.coeffs.iter().enumerate().skip(1).map(|(i, &c)| c * i as f64).collect())
    }
}

// #endregion 🔖️Poly

// #region 🔖️ClosedForm

/// ∿ Real roots of `a·x² + b·x + c`, using the cancellation-safe form (`q = -½(b + sign(b)·√Δ)`,
/// roots `q/a` and `c/q`) rather than the naive quadratic formula.
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    if a == 0.0 {
        return if b == 0.0 { vec![] } else { vec![-c / b] };
    }
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return vec![];
    }
    if disc == 0.0 {
        return vec![-b / (2.0 * a)];
    }
    let sqrt_disc = disc.sqrt();
    let sign = if b >= 0.0 { 1.0 } else { -1.0 };
    let q = -0.5 * (b + sign * sqrt_disc);
    let mut roots = vec![q / a, c / q];
    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
    roots
}

/// ∿ Real roots of `a·x³ + b·x² + c·x + d` (`a ≠ 0`) via the depressed-cubic trigonometric method
/// for three real roots and Cardano's formula otherwise.
pub fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> Vec<f64> {
    if a == 0.0 {
        return solve_quadratic(b, c, d);
    }
    let (b, c, d) = (b / a, c / a, d / a);
    let shift = b / 3.0;
    let p = c - b * b / 3.0;
    let q = 2.0 * b * b * b / 27.0 - b * c / 3.0 + d;
    let mut roots = if p.abs() < 1e-14 && q.abs() < 1e-14 {
        vec![0.0]
    } else {
        let discriminant = (q / 2.0).powi(2) + (p / 3.0).powi(3);
        if discriminant > 0.0 {
            let sqrt_disc = discriminant.sqrt();
            let u = cbrt(-q / 2.0 + sqrt_disc);
            let v = cbrt(-q / 2.0 - sqrt_disc);
            vec![u + v]
        } else if p.abs() < 1e-300 {
            vec![cbrt(-q)]
        } else {
            let r = (-p / 3.0).sqrt();
            let cos_arg = (3.0 * q / (2.0 * p * r)).clamp(-1.0, 1.0);
            let theta = cos_arg.acos();
            (0..3).map(|k| 2.0 * r * ((theta - std::f64::consts::TAU * k as f64) / 3.0).cos()).collect()
        }
    };
    for r in roots.iter_mut() {
        *r -= shift;
    }
    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
    roots
}

fn cbrt(x: f64) -> f64 {
    x.signum() * x.abs().powf(1.0 / 3.0)
}

// #endregion 🔖️ClosedForm

// #region 🔖️Bernstein

/// ∿ A polynomial in Bernstein basis on `[0, 1]`: `coeffs[i]` is the `i`-th control ordinate
/// `b_i` in `Σ b_i · C(n,i) · t^i · (1-t)^(n-i)`. The control polygon (the piecewise-linear
/// interpolant of `coeffs` at parameters `i/n`) convex-hulls the curve — the geometric fact
/// [`sign_variations`] exploits.
#[derive(Clone, Debug, PartialEq)]
pub struct Bernstein {
    pub coeffs: Vec<f64>,
}

impl Bernstein {
    pub fn new(coeffs: Vec<f64>) -> Self {
        Bernstein { coeffs }
    }
    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }
    /// ∿ De Casteljau evaluation at `t` (need not lie in `[0, 1]`; the polynomial extends).
    pub fn eval(&self, t: f64) -> f64 {
        let mut work = self.coeffs.clone();
        let n = work.len();
        for level in 1..n {
            for i in 0..n - level {
                work[i] = work[i] * (1.0 - t) + work[i + 1] * t;
            }
        }
        work.first().copied().unwrap_or(0.0)
    }
    /// ∿ De Casteljau subdivision at `t`: returns the control points of the restriction to
    /// `[0, t]` and to `[t, 1]`, each reparameterized back onto `[0, 1]`.
    pub fn subdivide(&self, t: f64) -> (Bernstein, Bernstein) {
        let n = self.coeffs.len();
        let mut table = vec![self.coeffs.clone()];
        for level in 1..n {
            let prev = &table[level - 1];
            let next: Vec<f64> = (0..n - level).map(|i| prev[i] * (1.0 - t) + prev[i + 1] * t).collect();
            table.push(next);
        }
        let left: Vec<f64> = (0..n).map(|i| table[i][0]).collect();
        let right: Vec<f64> = (0..n).map(|i| table[n - 1 - i][i]).collect();
        (Bernstein::new(left), Bernstein::new(right))
    }
    /// ∿ Converts to monomial (power) basis via repeated finite differences of the control net:
    /// `coeff[k] = C(n,k) · Δ^k b_0`.
    pub fn to_monomial(&self) -> Poly {
        let n = self.degree();
        let mut diffs = self.coeffs.clone();
        let mut monomial = vec![0.0; n + 1];
        monomial[0] = diffs[0];
        #[allow(clippy::needless_range_loop)]
        for k in 1..=n {
            for i in 0..diffs.len() - 1 {
                diffs[i] = diffs[i + 1] - diffs[i];
            }
            diffs.truncate(diffs.len() - 1);
            monomial[k] = binomial(n, k) * diffs[0];
        }
        Poly::new(monomial)
    }
    /// ∿ Converts a monomial polynomial to Bernstein form on `[0, 1]` (inverse of [`Self::to_monomial`]).
    pub fn from_monomial(p: &Poly) -> Bernstein {
        let n = p.degree();
        let coeffs = (0..=n)
            .map(|i| {
                (0..=i).map(|j| p.coeffs.get(j).copied().unwrap_or(0.0) * binomial(i, j) / binomial(n, j)).sum::<f64>()
            })
            .collect();
        Bernstein::new(coeffs)
    }
    /// ∿ Descartes' rule of signs applied to the control polygon: the number of sign changes in
    /// `coeffs` (ignoring exact zeros) is an upper bound on, and has the same parity as, the
    /// number of real roots in `(0, 1)`. `0` sign changes certifies *no* root; `1` certifies
    /// *exactly one*.
    pub fn sign_variations(&self) -> usize {
        let nonzero: Vec<f64> = self.coeffs.iter().copied().filter(|c| *c != 0.0).collect();
        nonzero.windows(2).filter(|w| w[0].signum() != w[1].signum()).count()
    }
}

fn binomial(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
}

// #endregion 🔖️Bernstein

// #region 🔖️Isolation

/// ∿ Recursively subdivides `b` over `[0, 1]` until every sub-interval has `0` or `1` sign
/// variation (certified root-free or root-isolating), returning the isolating `(lo, hi)`
/// intervals in increasing order. `max_depth` bounds recursion for pathological clustered-root
/// inputs — see [`crate::error`] for how callers should react if isolation is incomplete
/// (the kernel's "never wrong, fail loud" invariant: a caller hitting `max_depth` should treat
/// the sub-interval as unresolved rather than guess).
pub fn isolate_roots(b: &Bernstein, max_depth: u32) -> Vec<(f64, f64)> {
    let mut intervals = Vec::new();
    isolate_recursive(b, 0.0, 1.0, max_depth, &mut intervals);
    intervals
}

fn isolate_recursive(b: &Bernstein, lo: f64, hi: f64, depth: u32, out: &mut Vec<(f64, f64)>) {
    let variations = b.sign_variations();
    if variations == 0 {
        return;
    }
    if variations == 1 || depth == 0 {
        out.push((lo, hi));
        return;
    }
    let mid = 0.5;
    let (left, right) = b.subdivide(mid);
    let mid_param = lo + (hi - lo) * mid;
    isolate_recursive(&left, lo, mid_param, depth - 1, out);
    isolate_recursive(&right, mid_param, hi, depth - 1, out);
}

// #endregion 🔖️Isolation

// #region 🔖️Refine

/// ∿ Safeguarded Newton (bisection fallback whenever a Newton step would leave the bracket or
/// fails to shrink it) — guaranteed to converge given a valid sign-changing bracket `[lo, hi]`.
pub fn refine_root(p: &Poly, mut lo: f64, mut hi: f64, tol: f64, max_iters: u32) -> f64 {
    let mut f_lo = p.eval(lo);
    let f_hi = p.eval(hi);
    if f_lo == 0.0 {
        return lo;
    }
    if f_hi == 0.0 {
        return hi;
    }
    debug_assert!(f_lo.signum() != f_hi.signum(), "refine_root requires a sign-changing bracket");
    let mut x = 0.5 * (lo + hi);
    for _ in 0..max_iters {
        let (fx, dfx) = p.eval_with_derivative(x);
        if fx.abs() <= tol {
            return x;
        }
        if fx.signum() == f_lo.signum() {
            lo = x;
            f_lo = fx;
        } else {
            hi = x;
        }
        let newton_step = if dfx.abs() > 1e-300 { x - fx / dfx } else { f64::NAN };
        x = if newton_step.is_finite() && newton_step > lo && newton_step < hi { newton_step } else { 0.5 * (lo + hi) };
        if (hi - lo).abs() < tol {
            return x;
        }
    }
    x
}

// #endregion 🔖️Refine

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poly_eval_matches_direct_computation() {
        let p = Poly::new(vec![1.0, -2.0, 3.0]); // 1 - 2x + 3x^2
        assert!((p.eval(2.0) - (1.0 - 4.0 + 12.0)).abs() < 1e-12);
    }

    #[test]
    fn poly_derivative_matches_power_rule() {
        let p = Poly::new(vec![1.0, -2.0, 3.0, 4.0]); // 1 - 2x + 3x^2 + 4x^3
        let d = p.derivative();
        assert_eq!(d.coeffs, vec![-2.0, 6.0, 12.0]);
    }

    #[test]
    fn eval_with_derivative_matches_separate_calls() {
        let p = Poly::new(vec![2.0, -1.0, 0.5, 3.0]);
        let (v, dv) = p.eval_with_derivative(1.5);
        assert!((v - p.eval(1.5)).abs() < 1e-12);
        assert!((dv - p.derivative().eval(1.5)).abs() < 1e-12);
    }

    #[test]
    fn solve_quadratic_finds_known_roots() {
        // (x-2)(x-3) = x^2 -5x+6
        let roots = solve_quadratic(1.0, -5.0, 6.0);
        assert_eq!(roots.len(), 2);
        assert!((roots[0] - 2.0).abs() < 1e-9);
        assert!((roots[1] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn solve_quadratic_handles_no_real_roots() {
        assert!(solve_quadratic(1.0, 0.0, 1.0).is_empty());
    }

    #[test]
    fn solve_quadratic_avoids_cancellation_for_large_b() {
        // Classic near-cancellation case: a=1, b=1e8, c=1. Naive formula loses precision.
        let roots = solve_quadratic(1.0, 1e8, 1.0);
        assert_eq!(roots.len(), 2);
        for r in &roots {
            let p = Poly::new(vec![1.0, 1e8, 1.0]);
            assert!(p.eval(*r).abs() / (1e8 * r.abs() + 1.0) < 1e-6, "root {r} not accurate enough");
        }
    }

    #[test]
    fn solve_cubic_finds_three_known_real_roots() {
        // (x-1)(x-2)(x-3) = x^3 -6x^2+11x-6
        let mut roots = solve_cubic(1.0, -6.0, 11.0, -6.0);
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(roots.len(), 3);
        assert!((roots[0] - 1.0).abs() < 1e-9);
        assert!((roots[1] - 2.0).abs() < 1e-9);
        assert!((roots[2] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn solve_cubic_finds_single_real_root() {
        // x^3 + x + 1 has exactly one real root (~-0.6823)
        let roots = solve_cubic(1.0, 0.0, 1.0, 1.0);
        assert_eq!(roots.len(), 1);
        let p = Poly::new(vec![1.0, 1.0, 0.0, 1.0]);
        assert!(p.eval(roots[0]).abs() < 1e-9);
    }

    #[test]
    fn bernstein_eval_matches_monomial_conversion() {
        let p = Poly::new(vec![1.0, 2.0, -3.0, 0.5]);
        let b = Bernstein::from_monomial(&p);
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!((b.eval(t) - p.eval(t)).abs() < 1e-9, "mismatch at t={t}");
        }
    }

    #[test]
    fn bernstein_to_monomial_round_trips_from_monomial() {
        let p = Poly::new(vec![3.0, -1.5, 2.0, 4.0, -0.25]);
        let b = Bernstein::from_monomial(&p);
        let back = b.to_monomial();
        assert_eq!(back.coeffs.len(), p.coeffs.len());
        for (a, c) in back.coeffs.iter().zip(p.coeffs.iter()) {
            assert!((a - c).abs() < 1e-8, "coefficient mismatch: {a} vs {c}");
        }
    }

    #[test]
    fn bernstein_subdivide_matches_original_at_shared_endpoints_and_split_point() {
        let b = Bernstein::new(vec![0.0, 3.0, -1.0, 2.0]);
        let t = 0.4;
        let (left, right) = b.subdivide(t);
        assert!((left.eval(0.0) - b.eval(0.0)).abs() < 1e-9);
        assert!((left.eval(1.0) - b.eval(t)).abs() < 1e-9);
        assert!((right.eval(0.0) - b.eval(t)).abs() < 1e-9);
        assert!((right.eval(1.0) - b.eval(1.0)).abs() < 1e-9);
        // Sample a mid-point of the left piece and confirm it agrees with the original curve.
        assert!((left.eval(0.5) - b.eval(t * 0.5)).abs() < 1e-9);
    }

    #[test]
    fn sign_variations_certifies_no_root_for_monotone_positive_control_polygon() {
        let b = Bernstein::new(vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(b.sign_variations(), 0);
    }

    #[test]
    fn sign_variations_detects_single_sign_change() {
        let b = Bernstein::new(vec![-1.0, -0.5, 1.0, 2.0]);
        assert_eq!(b.sign_variations(), 1);
    }

    #[test]
    fn isolate_roots_finds_single_root_of_linear_bernstein() {
        // Line from -1 at t=0 to 1 at t=1: root at t=0.5.
        let b = Bernstein::new(vec![-1.0, 1.0]);
        let intervals = isolate_roots(&b, 20);
        assert_eq!(intervals.len(), 1);
        assert!(intervals[0].0 <= 0.5 && intervals[0].1 >= 0.5);
    }

    #[test]
    fn isolate_roots_finds_no_intervals_for_root_free_polynomial() {
        let b = Bernstein::new(vec![1.0, 2.0, 3.0]);
        assert!(isolate_roots(&b, 20).is_empty());
    }

    #[test]
    fn refine_root_converges_to_known_root() {
        let p = Poly::new(vec![-6.0, 11.0, -6.0, 1.0]); // (x-1)(x-2)(x-3)
        let root = refine_root(&p, 2.5, 3.5, 1e-12, 100);
        assert!((root - 3.0).abs() < 1e-9);
    }

    mod quick {
        use super::*;

        /// 🔮️ Brute-force oracle: dense sampling + bisection finds every sign-change interval,
        /// independent of the Bernstein/Descartes machinery under test.
        fn bisection_oracle(p: &Poly, samples: usize) -> Vec<f64> {
            let mut roots = Vec::new();
            let xs: Vec<f64> = (0..=samples).map(|i| i as f64 / samples as f64).collect();
            for w in xs.windows(2) {
                let (a, b) = (w[0], w[1]);
                let (fa, fb) = (p.eval(a), p.eval(b));
                if fa == 0.0 {
                    roots.push(a);
                } else if fa.signum() != fb.signum() {
                    roots.push(refine_root(p, a, b, 1e-12, 100));
                }
            }
            roots
        }

        #[test]
        fn isolate_roots_plus_refine_matches_bisection_oracle_on_random_polynomials() {
            let mut rng = mathematical_random::Rng::from_seed(23);
            for _ in 0..200 {
                let degree = 1 + (rng.next_range(0, 4) as usize);
                let coeffs: Vec<f64> = (0..=degree).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
                let p = Poly::new(coeffs);
                if p.coeffs[p.degree()] == 0.0 {
                    continue;
                }
                let b = Bernstein::from_monomial(&p);
                let intervals = isolate_roots(&b, 30);
                let mut found: Vec<f64> = intervals.iter().map(|(lo, hi)| refine_root(&p, *lo, *hi, 1e-11, 100)).collect();
                found.sort_by(|a, c| a.partial_cmp(c).unwrap());
                let expected = bisection_oracle(&p, 4000);
                assert_eq!(found.len(), expected.len(), "root count mismatch for {:?}: found {found:?} expected {expected:?}", p.coeffs);
                for (f, e) in found.iter().zip(expected.iter()) {
                    assert!((f - e).abs() < 1e-6, "root mismatch: found {f} expected {e} for {:?}", p.coeffs);
                }
            }
        }

        #[test]
        fn bernstein_monomial_round_trip_holds_on_random_polynomials() {
            let mut rng = mathematical_random::Rng::from_seed(29);
            for _ in 0..200 {
                let degree = rng.next_range(0, 6) as usize;
                let coeffs: Vec<f64> = (0..=degree).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
                let p = Poly::new(coeffs);
                let b = Bernstein::from_monomial(&p);
                let back = b.to_monomial();
                for (a, c) in back.coeffs.iter().zip(p.coeffs.iter()) {
                    assert!((a - c).abs() < 1e-6, "round trip mismatch: {a} vs {c}");
                }
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Poly

// #region 🔖️Bezier
pub mod bezier {
//! 🎀️ Rational Bézier curve segments in 2D and 3D: de Casteljau evaluation/splitting, degree
//! elevation, a convex-hull-derived bounding box, and the Bézier-clipping primitive that
//! [`crate::int_cc`]/[`crate::int_cs`] build their NURBS intersectors on. Weighted (rational)
//! control points are the uniform representation — an unweighted Bézier is just every weight `1`.

use crate::vec::{Pnt2, Pnt3};

// #region 🔖️Bezier

/// 🎀️ A 3D rational Bézier segment: `n+1` control points with weights, parameter domain `[0, 1]`.
#[derive(Clone, Debug, PartialEq)]
pub struct RationalBezier3 {
    pub controls: Vec<Pnt3>,
    pub weights: Vec<f64>,
}

/// 🎀️ A 2D rational Bézier segment (used as the pcurve building block).
#[derive(Clone, Debug, PartialEq)]
pub struct RationalBezier2 {
    pub controls: Vec<Pnt2>,
    pub weights: Vec<f64>,
}

impl RationalBezier3 {
    pub fn new(controls: Vec<Pnt3>, weights: Vec<f64>) -> Self {
        debug_assert_eq!(controls.len(), weights.len());
        RationalBezier3 { controls, weights }
    }
    pub fn unweighted(controls: Vec<Pnt3>) -> Self {
        let weights = vec![1.0; controls.len()];
        RationalBezier3::new(controls, weights)
    }
    pub fn degree(&self) -> usize {
        self.controls.len().saturating_sub(1)
    }
    pub fn is_rational(&self) -> bool {
        self.weights.iter().any(|w| (w - 1.0).abs() > 1e-12)
    }
    /// 🎀️ De Casteljau evaluation via homogeneous (weighted) coordinates, so a single algorithm
    /// covers both the polynomial and rational cases.
    pub fn eval(&self, t: f64) -> Pnt3 {
        let n = self.controls.len();
        let mut hx: Vec<f64> = (0..n).map(|i| self.controls[i].x * self.weights[i]).collect();
        let mut hy: Vec<f64> = (0..n).map(|i| self.controls[i].y * self.weights[i]).collect();
        let mut hz: Vec<f64> = (0..n).map(|i| self.controls[i].z * self.weights[i]).collect();
        let mut hw: Vec<f64> = self.weights.clone();
        for level in 1..n {
            for i in 0..n - level {
                hx[i] = hx[i] * (1.0 - t) + hx[i + 1] * t;
                hy[i] = hy[i] * (1.0 - t) + hy[i + 1] * t;
                hz[i] = hz[i] * (1.0 - t) + hz[i + 1] * t;
                hw[i] = hw[i] * (1.0 - t) + hw[i + 1] * t;
            }
        }
        Pnt3::new(hx[0] / hw[0], hy[0] / hw[0], hz[0] / hw[0])
    }
    /// 🎀️ Splits into two segments at `t`, each reparameterized onto `[0, 1]`. Uses de Casteljau
    /// on the homogeneous control net so the split is exact for rational curves too.
    pub fn subdivide(&self, t: f64) -> (RationalBezier3, RationalBezier3) {
        let n = self.controls.len();
        let mut hx = vec![vec![0.0; n]; n];
        let mut hy = vec![vec![0.0; n]; n];
        let mut hz = vec![vec![0.0; n]; n];
        let mut hw = vec![vec![0.0; n]; n];
        for i in 0..n {
            hx[0][i] = self.controls[i].x * self.weights[i];
            hy[0][i] = self.controls[i].y * self.weights[i];
            hz[0][i] = self.controls[i].z * self.weights[i];
            hw[0][i] = self.weights[i];
        }
        for level in 1..n {
            for i in 0..n - level {
                hx[level][i] = hx[level - 1][i] * (1.0 - t) + hx[level - 1][i + 1] * t;
                hy[level][i] = hy[level - 1][i] * (1.0 - t) + hy[level - 1][i + 1] * t;
                hz[level][i] = hz[level - 1][i] * (1.0 - t) + hz[level - 1][i + 1] * t;
                hw[level][i] = hw[level - 1][i] * (1.0 - t) + hw[level - 1][i + 1] * t;
            }
        }
        let mut left_c = Vec::with_capacity(n);
        let mut left_w = Vec::with_capacity(n);
        let mut right_c = Vec::with_capacity(n);
        let mut right_w = Vec::with_capacity(n);
        for level in 0..n {
            let w_left = hw[level][0];
            left_c.push(Pnt3::new(hx[level][0] / w_left, hy[level][0] / w_left, hz[level][0] / w_left));
            left_w.push(w_left);
            // R_level = b[n-1-level][level] (the triangle's other diagonal) — not b[level][n-1-level].
            let row = n - 1 - level;
            let col = level;
            let w_right = hw[row][col];
            right_c.push(Pnt3::new(hx[row][col] / w_right, hy[row][col] / w_right, hz[row][col] / w_right));
            right_w.push(w_right);
        }
        (RationalBezier3::new(left_c, left_w), RationalBezier3::new(right_c, right_w))
    }
    /// 🎀️ An axis-aligned box guaranteed to contain the curve (the convex hull of the weighted
    /// control points contains an unweighted curve exactly; for rational curves with all-positive
    /// weights it still contains the curve, since the curve point is a convex combination of the
    /// control points).
    pub fn control_hull_box(&self) -> (Pnt3, Pnt3) {
        let mut lo = self.controls[0];
        let mut hi = self.controls[0];
        for p in &self.controls[1..] {
            lo = Pnt3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Pnt3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
        (lo, hi)
    }
    /// 🎀️ Degree-elevates a polynomial (non-rational) Bézier by one degree, preserving the exact
    /// curve. Rational elevation is not implemented (unneeded by the kernel: rational Béziers are
    /// only ever consumed at fixed degree by the conic/NURBS conversion paths).
    pub fn elevate(&self) -> RationalBezier3 {
        debug_assert!(!self.is_rational(), "degree elevation is only implemented for polynomial (unweighted) Beziers");
        let n = self.degree();
        let m = n + 1;
        let mut controls = Vec::with_capacity(m + 1);
        for i in 0..=m {
            let a = i as f64 / m as f64;
            let b = (m - i) as f64 / m as f64;
            let left = if i > 0 { self.controls[i - 1].to_vec() * a } else { crate::vec::Vec3::ZERO };
            let right = if i <= n { self.controls[i].to_vec() * b } else { crate::vec::Vec3::ZERO };
            controls.push(Pnt3::from_array((left + right).to_array()));
        }
        RationalBezier3::unweighted(controls)
    }
}

impl RationalBezier2 {
    pub fn new(controls: Vec<Pnt2>, weights: Vec<f64>) -> Self {
        debug_assert_eq!(controls.len(), weights.len());
        RationalBezier2 { controls, weights }
    }
    pub fn unweighted(controls: Vec<Pnt2>) -> Self {
        let weights = vec![1.0; controls.len()];
        RationalBezier2::new(controls, weights)
    }
    pub fn degree(&self) -> usize {
        self.controls.len().saturating_sub(1)
    }
    pub fn eval(&self, t: f64) -> Pnt2 {
        let n = self.controls.len();
        let mut hx: Vec<f64> = (0..n).map(|i| self.controls[i].x * self.weights[i]).collect();
        let mut hy: Vec<f64> = (0..n).map(|i| self.controls[i].y * self.weights[i]).collect();
        let mut hw: Vec<f64> = self.weights.clone();
        for level in 1..n {
            for i in 0..n - level {
                hx[i] = hx[i] * (1.0 - t) + hx[i + 1] * t;
                hy[i] = hy[i] * (1.0 - t) + hy[i + 1] * t;
                hw[i] = hw[i] * (1.0 - t) + hw[i + 1] * t;
            }
        }
        Pnt2::new(hx[0] / hw[0], hy[0] / hw[0])
    }
    pub fn subdivide(&self, t: f64) -> (RationalBezier2, RationalBezier2) {
        let n = self.controls.len();
        let mut hx = vec![vec![0.0; n]; n];
        let mut hy = vec![vec![0.0; n]; n];
        let mut hw = vec![vec![0.0; n]; n];
        for i in 0..n {
            hx[0][i] = self.controls[i].x * self.weights[i];
            hy[0][i] = self.controls[i].y * self.weights[i];
            hw[0][i] = self.weights[i];
        }
        for level in 1..n {
            for i in 0..n - level {
                hx[level][i] = hx[level - 1][i] * (1.0 - t) + hx[level - 1][i + 1] * t;
                hy[level][i] = hy[level - 1][i] * (1.0 - t) + hy[level - 1][i + 1] * t;
                hw[level][i] = hw[level - 1][i] * (1.0 - t) + hw[level - 1][i + 1] * t;
            }
        }
        let mut left_c = Vec::with_capacity(n);
        let mut left_w = Vec::with_capacity(n);
        let mut right_c = Vec::with_capacity(n);
        let mut right_w = Vec::with_capacity(n);
        for level in 0..n {
            let w_left = hw[level][0];
            left_c.push(Pnt2::new(hx[level][0] / w_left, hy[level][0] / w_left));
            left_w.push(w_left);
            // R_level = b[n-1-level][level] (the triangle's other diagonal) — not b[level][n-1-level].
            let row = n - 1 - level;
            let col = level;
            let w_right = hw[row][col];
            right_c.push(Pnt2::new(hx[row][col] / w_right, hy[row][col] / w_right));
            right_w.push(w_right);
        }
        (RationalBezier2::new(left_c, left_w), RationalBezier2::new(right_c, right_w))
    }
    pub fn control_hull_box(&self) -> (Pnt2, Pnt2) {
        let mut lo = self.controls[0];
        let mut hi = self.controls[0];
        for p in &self.controls[1..] {
            lo = Pnt2::new(lo.x.min(p.x), lo.y.min(p.y));
            hi = Pnt2::new(hi.x.max(p.x), hi.y.max(p.y));
        }
        (lo, hi)
    }
}

// #endregion 🔖️Bezier

// #region 🔖️Split

/// 🎀️ Recursively subdivides a 2D Bézier segment until every leaf's control hull is smaller than
/// `tol` in both axes or `max_depth` is reached — the "fat line" precursor to full clipping,
/// used directly by [`crate::int_cc`] for curve/curve intersection.
pub fn subdivide_until_flat(b: &RationalBezier2, tol: f64, max_depth: u32) -> Vec<RationalBezier2> {
    let mut leaves = Vec::new();
    subdivide_recursive(b.clone(), tol, max_depth, &mut leaves);
    leaves
}

fn subdivide_recursive(b: RationalBezier2, tol: f64, depth: u32, out: &mut Vec<RationalBezier2>) {
    let (lo, hi) = b.control_hull_box();
    if (hi.x - lo.x) <= tol && (hi.y - lo.y) <= tol || depth == 0 {
        out.push(b);
        return;
    }
    let (left, right) = b.subdivide(0.5);
    subdivide_recursive(left, tol, depth - 1, out);
    subdivide_recursive(right, tol, depth - 1, out);
}

// #endregion 🔖️Split

// #region 🔖️Clip

/// 🎀️ Axis-aligned bounding-box overlap test between two curves' control hulls — the cheap
/// rejection test every pairwise intersector runs before doing real work.
pub fn boxes_overlap2(a: (Pnt2, Pnt2), b: (Pnt2, Pnt2), tol: f64) -> bool {
    a.0.x - tol <= b.1.x && b.0.x - tol <= a.1.x && a.0.y - tol <= b.1.y && b.0.y - tol <= a.1.y
}

// #endregion 🔖️Clip

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unweighted_bezier_eval_matches_de_casteljau_by_hand() {
        // Quadratic bezier: (0,0),(1,2),(2,0) at t=0.5 -> (1, 1)
        let b = RationalBezier2::unweighted(vec![Pnt2::new(0.0, 0.0), Pnt2::new(1.0, 2.0), Pnt2::new(2.0, 0.0)]);
        let p = b.eval(0.5);
        assert!((p.x - 1.0).abs() < 1e-12);
        assert!((p.y - 1.0).abs() < 1e-12);
    }

    #[test]
    fn eval_at_endpoints_matches_first_and_last_control_point() {
        let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 5.0, -2.0), Pnt3::new(3.0, 1.0, 4.0)]);
        assert_eq!(b.eval(0.0), b.controls[0]);
        assert_eq!(b.eval(1.0), *b.controls.last().unwrap());
    }

    #[test]
    fn subdivide_matches_original_at_endpoints_and_split_point() {
        let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 2.0, 0.0), Pnt3::new(2.0, -1.0, 1.0), Pnt3::new(3.0, 0.0, 2.0)]);
        let t = 0.35;
        let (left, right) = b.subdivide(t);
        assert!(left.eval(0.0).distance(b.eval(0.0)) < 1e-9);
        assert!(left.eval(1.0).distance(b.eval(t)) < 1e-9);
        assert!(right.eval(0.0).distance(b.eval(t)) < 1e-9);
        assert!(right.eval(1.0).distance(b.eval(1.0)) < 1e-9);
    }

    #[test]
    fn subdivide_of_rational_bezier_preserves_curve_points() {
        // Quarter circle as a rational quadratic Bezier.
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let b = RationalBezier2::new(vec![Pnt2::new(1.0, 0.0), Pnt2::new(1.0, 1.0), Pnt2::new(0.0, 1.0)], vec![1.0, s, 1.0]);
        let t = 0.3;
        let expected = b.eval(t);
        let (left, right) = b.subdivide(t);
        assert!((left.eval(1.0).x - expected.x).abs() < 1e-9);
        assert!((left.eval(1.0).y - expected.y).abs() < 1e-9);
        assert!((right.eval(0.0).x - expected.x).abs() < 1e-9);
        // All points on a quarter unit circle satisfy x^2+y^2=1.
        let sample = b.eval(0.6);
        assert!((sample.x * sample.x + sample.y * sample.y - 1.0).abs() < 1e-9);
    }

    #[test]
    fn control_hull_box_contains_all_sampled_curve_points() {
        let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(5.0, 5.0, 5.0), Pnt3::new(-3.0, 2.0, -1.0), Pnt3::new(1.0, -2.0, 3.0)]);
        let (lo, hi) = b.control_hull_box();
        for i in 0..=20 {
            let p = b.eval(i as f64 / 20.0);
            assert!(p.x >= lo.x - 1e-9 && p.x <= hi.x + 1e-9);
            assert!(p.y >= lo.y - 1e-9 && p.y <= hi.y + 1e-9);
            assert!(p.z >= lo.z - 1e-9 && p.z <= hi.z + 1e-9);
        }
    }

    #[test]
    fn elevate_preserves_the_curve_exactly() {
        let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 3.0, -1.0), Pnt3::new(2.0, -1.0, 2.0)]);
        let elevated = b.elevate();
        assert_eq!(elevated.degree(), b.degree() + 1);
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!(b.eval(t).distance(elevated.eval(t)) < 1e-9, "mismatch at t={t}");
        }
    }

    #[test]
    fn subdivide_until_flat_leaves_cover_the_full_parameter_range() {
        let b = RationalBezier2::unweighted(vec![Pnt2::new(0.0, 0.0), Pnt2::new(1.0, 5.0), Pnt2::new(2.0, -3.0), Pnt2::new(3.0, 1.0)]);
        let leaves = subdivide_until_flat(&b, 0.1, 12);
        assert!(!leaves.is_empty());
        // Endpoints of the whole curve must be reproduced by the first/last leaf.
        assert!(leaves.first().unwrap().eval(0.0).distance(b.eval(0.0)) < 1e-9);
        assert!(leaves.last().unwrap().eval(1.0).distance(b.eval(1.0)) < 1e-9);
    }

    #[test]
    fn boxes_overlap_detects_disjoint_and_touching_boxes() {
        let a = (Pnt2::new(0.0, 0.0), Pnt2::new(1.0, 1.0));
        let b = (Pnt2::new(0.5, 0.5), Pnt2::new(2.0, 2.0));
        let c = (Pnt2::new(5.0, 5.0), Pnt2::new(6.0, 6.0));
        assert!(boxes_overlap2(a, b, 1e-9));
        assert!(!boxes_overlap2(a, c, 1e-9));
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Bezier

// #region 🔖️Bspline
pub mod bspline {
//! 🧵️ Knot vectors, B-spline basis functions and de Boor evaluation for rational curves and
//! tensor-product surfaces — the machinery [`crate::curve::Curve3::Nurbs`] and
//! [`crate::surface::Surface::Nurbs`] are built on. Curves and surfaces themselves stay in their
//! own modules; this file is purely the numerical core, independent of any particular dimension.

// #region 🔖️Knots

/// 🧵️ A non-decreasing knot vector for a degree-`p` B-spline with `n` control points, satisfying
/// `len == n + p + 1`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KnotVector {
    pub knots: Vec<f64>,
    pub degree: usize,
}

impl KnotVector {
    /// 🧵️ Builds and validates a knot vector: non-decreasing, correct length for `(n, degree)`.
    pub fn new(knots: Vec<f64>, degree: usize, control_point_count: usize) -> Option<Self> {
        if knots.len() != control_point_count + degree + 1 {
            return None;
        }
        if knots.windows(2).any(|w| w[0] > w[1]) {
            return None;
        }
        Some(KnotVector { knots, degree })
    }
    /// 🧵️ A clamped (open) uniform knot vector: the first and last knots repeat `degree+1` times,
    /// the standard choice so the curve interpolates its first/last control points.
    pub fn clamped_uniform(control_point_count: usize, degree: usize) -> Self {
        let n = control_point_count;
        let p = degree;
        let interior = n.saturating_sub(p + 1);
        let mut knots = vec![0.0; p + 1];
        for i in 1..=interior {
            knots.push(i as f64 / (interior + 1) as f64);
        }
        knots.extend(std::iter::repeat_n(1.0, p + 1));
        KnotVector { knots, degree: p }
    }
    pub fn domain(&self) -> (f64, f64) {
        (self.knots[self.degree], self.knots[self.knots.len() - self.degree - 1])
    }
    pub fn control_point_count(&self) -> usize {
        self.knots.len() - self.degree - 1
    }
    pub fn is_periodic_compatible(&self) -> bool {
        // A periodic (non-clamped) knot vector has no repeated end knots beyond multiplicity 1.
        self.multiplicity_at_index(0) == 1
    }
    fn multiplicity_at_index(&self, i: usize) -> usize {
        self.knots.iter().filter(|&&k| k == self.knots[i]).count()
    }
    /// 🧵️ Finds the knot span index `i` such that `knots[i] <= u < knots[i+1]` (or the last valid
    /// span if `u` equals the domain's upper bound), via binary search — O(log n) per evaluation.
    pub fn find_span(&self, u: f64) -> usize {
        let n = self.control_point_count() - 1;
        let p = self.degree;
        if u >= self.knots[n + 1] {
            return n;
        }
        if u <= self.knots[p] {
            return p;
        }
        let mut lo = p;
        let mut hi = n + 1;
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if u < self.knots[mid] {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        lo
    }
    /// 🧵️ The multiplicity of the knot value equal to `u`, or `0` if `u` is not an existing knot
    /// (within exact equality — callers should snap to a known knot value before calling this).
    pub fn multiplicity(&self, u: f64) -> usize {
        self.knots.iter().filter(|&&k| k == u).count()
    }
}

// #endregion 🔖️Knots

// #region 🔖️Basis

/// 🧵️ Evaluates all `degree+1` nonzero basis functions at `u` in the knot span `span` (the
/// Cox-de Boor triangular recurrence, computed bottom-up per the standard NURBS-book algorithm —
/// `O(p²)` and numerically stable, unlike the naive top-down recursive definition).
pub fn basis_functions(knots: &KnotVector, span: usize, u: f64) -> Vec<f64> {
    let p = knots.degree;
    let mut n = vec![0.0; p + 1];
    n[0] = 1.0;
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    for j in 1..=p {
        left[j] = u - knots.knots[span + 1 - j];
        right[j] = knots.knots[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() > 1e-300 { n[r] / denom } else { 0.0 };
            n[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n[j] = saved;
    }
    n
}

/// 🧵️ Evaluates the nonzero basis functions and their derivatives up to order `max_deriv` at `u`
/// in `span`. Returns `derivs[k][j]` = the `k`-th derivative of the `j`-th nonzero basis function.
pub fn basis_function_derivatives(knots: &KnotVector, span: usize, u: f64, max_deriv: usize) -> Vec<Vec<f64>> {
    let p = knots.degree;
    let max_deriv = max_deriv.min(p);
    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    ndu[0][0] = 1.0;
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];
    for j in 1..=p {
        left[j] = u - knots.knots[span + 1 - j];
        right[j] = knots.knots[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let denom = ndu[j][r];
            let temp = if denom.abs() > 1e-300 { ndu[r][j - 1] / denom } else { 0.0 };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }
    let mut derivs = vec![vec![0.0; p + 1]; max_deriv + 1];
    for j in 0..=p {
        derivs[0][j] = ndu[j][p];
    }
    for r in 0..=p {
        let mut a = vec![vec![0.0; p + 1]; 2];
        a[0][0] = 1.0;
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        for k in 1..=max_deriv {
            let mut d = 0.0;
            let rk = r as isize - k as isize;
            let pk = p - k;
            if r >= k {
                a[s2][0] = a[s1][0] / ndu[pk + 1][rk as usize];
                d = a[s2][0] * ndu[rk as usize][pk];
            }
            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if r as isize - 1 <= pk as isize { k - 1 } else { p - r };
            for j in j1..=j2 {
                a[s2][j] = (a[s1][j] - a[s1][j - 1]) / ndu[pk + 1][(rk + j as isize) as usize];
                d += a[s2][j] * ndu[(rk + j as isize) as usize][pk];
            }
            if r <= pk {
                a[s2][k] = -a[s1][k - 1] / ndu[pk + 1][r];
                d += a[s2][k] * ndu[r][pk];
            }
            derivs[k][r] = d;
            std::mem::swap(&mut s1, &mut s2);
        }
    }
    let mut factor = p as f64;
    #[allow(clippy::needless_range_loop)]
    for k in 1..=max_deriv {
        for v in derivs[k].iter_mut().take(p + 1) {
            *v *= factor;
        }
        factor *= (p - k) as f64;
    }
    derivs
}

// #endregion 🔖️Basis

// #region 🔖️DeBoor

/// 🧵️ De Boor's algorithm for a rational (homogeneous) curve, evaluating one weighted-coordinate
/// channel — call once per coordinate (x, y, z, w) and divide by the resulting `w` to dehomogenize.
pub fn de_boor(knots: &KnotVector, control_values: &[f64], u: f64) -> f64 {
    let span = knots.find_span(u);
    let p = knots.degree;
    let n = basis_functions(knots, span, u);
    (0..=p).map(|j| n[j] * control_values[span - p + j]).sum()
}

// #endregion 🔖️DeBoor

// #region 🔖️Refine

/// 🧵️ Inserts a single knot `u` (Boehm's algorithm), returning the new knot vector and the new
/// control values for one coordinate channel — geometrically a no-op (the curve is unchanged),
/// used to raise local control or to harmonize two curves onto a shared knot vector.
pub fn insert_knot(knots: &KnotVector, control_values: &[f64], u: f64) -> (KnotVector, Vec<f64>) {
    let p = knots.degree;
    let span = knots.find_span(u);
    let mut new_knots = knots.knots.clone();
    new_knots.insert(span + 1, u);
    let n = control_values.len();
    let mut new_values = vec![0.0; n + 1];
    let prefix_end = span.saturating_sub(p) + 1;
    new_values[..prefix_end].copy_from_slice(&control_values[..prefix_end]);
    new_values[span + 1..=n].copy_from_slice(&control_values[span..n]);
    for i in (span + 1 - p)..=span {
        let alpha = if knots.knots[i + p] != knots.knots[i] { (u - knots.knots[i]) / (knots.knots[i + p] - knots.knots[i]) } else { 0.0 };
        new_values[i] = alpha * control_values[i] + (1.0 - alpha) * control_values[i - 1];
    }
    (KnotVector { knots: new_knots, degree: p }, new_values)
}

/// 🧵️ Elevates a Bézier segment's degree by one via the shared [`crate::bezier`] elevation
/// formula, exposed here so B-spline code can raise a single-span curve's degree without
/// round-tripping through the `Bernstein`/`Poly` types.
pub fn elevate_bezier_span(control_values: &[f64]) -> Vec<f64> {
    let n = control_values.len() - 1;
    let m = n + 1;
    (0..=m)
        .map(|i| {
            let a = i as f64 / m as f64;
            let b = (m - i) as f64 / m as f64;
            let left = if i > 0 { control_values[i - 1] * a } else { 0.0 };
            let right = if i <= n { control_values[i] * b } else { 0.0 };
            left + right
        })
        .collect()
}

// #endregion 🔖️Refine

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn cubic_clamped_5cp() -> KnotVector {
        // degree 3, 5 control points -> knot vector length 9: [0,0,0,0, 0.5, 1,1,1,1]
        KnotVector::new(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3, 5).unwrap()
    }

    #[test]
    fn knot_vector_rejects_wrong_length() {
        assert!(KnotVector::new(vec![0.0, 0.0, 1.0, 1.0], 3, 5).is_none());
    }

    #[test]
    fn knot_vector_rejects_decreasing_sequence() {
        assert!(KnotVector::new(vec![0.0, 0.5, 0.2, 1.0, 1.0], 1, 3).is_none());
    }

    #[test]
    fn clamped_uniform_has_correct_domain_and_multiplicity() {
        let kv = KnotVector::clamped_uniform(5, 3);
        assert_eq!(kv.domain(), (0.0, 1.0));
        assert_eq!(kv.multiplicity(0.0), 4);
        assert_eq!(kv.multiplicity(1.0), 4);
        assert_eq!(kv.control_point_count(), 5);
    }

    #[test]
    fn find_span_matches_brute_force_scan() {
        let kv = cubic_clamped_5cp();
        for i in 0..=100 {
            let u = i as f64 / 100.0;
            let expected = brute_force_span(&kv, u);
            assert_eq!(kv.find_span(u), expected, "mismatch at u={u}");
        }
    }

    fn brute_force_span(kv: &KnotVector, u: f64) -> usize {
        let n = kv.control_point_count() - 1;
        for i in kv.degree..=n {
            if u >= kv.knots[i] && u < kv.knots[i + 1] {
                return i;
            }
        }
        n
    }

    #[test]
    fn basis_functions_sum_to_one_everywhere_in_domain() {
        let kv = cubic_clamped_5cp();
        for i in 0..=50 {
            let u = i as f64 / 50.0;
            let span = kv.find_span(u);
            let n = basis_functions(&kv, span, u);
            let sum: f64 = n.iter().sum();
            assert!((sum - 1.0).abs() < 1e-10, "partition of unity violated at u={u}: sum={sum}");
        }
    }

    #[test]
    fn basis_functions_are_nonnegative() {
        let kv = cubic_clamped_5cp();
        for i in 0..=50 {
            let u = i as f64 / 50.0;
            let span = kv.find_span(u);
            let n = basis_functions(&kv, span, u);
            assert!(n.iter().all(|&v| v >= -1e-12), "negative basis value at u={u}: {n:?}");
        }
    }

    #[test]
    fn de_boor_interpolates_endpoints_of_clamped_curve() {
        let kv = cubic_clamped_5cp();
        let values = vec![0.0, 1.0, -2.0, 3.0, 5.0];
        let (lo, hi) = kv.domain();
        assert!((de_boor(&kv, &values, lo) - values[0]).abs() < 1e-9);
        assert!((de_boor(&kv, &values, hi) - *values.last().unwrap()).abs() < 1e-9);
    }

    #[test]
    fn basis_function_derivatives_match_finite_differences() {
        let kv = cubic_clamped_5cp();
        let u = 0.37;
        let span = kv.find_span(u);
        let derivs = basis_function_derivatives(&kv, span, u, 1);
        let h = 1e-6;
        let n_plus = basis_functions(&kv, kv.find_span(u + h), u + h);
        let n_minus = basis_functions(&kv, kv.find_span(u - h), u - h);
        for j in 0..=kv.degree {
            let fd = (n_plus[j] - n_minus[j]) / (2.0 * h);
            assert!((derivs[1][j] - fd).abs() < 1e-4, "derivative mismatch at j={j}: analytic={} fd={fd}", derivs[1][j]);
        }
    }

    #[test]
    fn basis_function_derivatives_order_zero_matches_basis_functions() {
        let kv = cubic_clamped_5cp();
        let u = 0.63;
        let span = kv.find_span(u);
        let plain = basis_functions(&kv, span, u);
        let derivs = basis_function_derivatives(&kv, span, u, 2);
        for j in 0..=kv.degree {
            assert!((plain[j] - derivs[0][j]).abs() < 1e-12);
        }
    }

    #[test]
    fn insert_knot_does_not_change_the_curve() {
        let kv = cubic_clamped_5cp();
        let values = vec![0.0, 2.0, -1.0, 3.0, 1.0];
        let (new_kv, new_values) = insert_knot(&kv, &values, 0.3);
        assert_eq!(new_kv.control_point_count(), values.len() + 1);
        for i in 0..=20 {
            let u = i as f64 / 20.0;
            let before = de_boor(&kv, &values, u);
            let after = de_boor(&new_kv, &new_values, u);
            assert!((before - after).abs() < 1e-9, "curve changed after knot insertion at u={u}: {before} vs {after}");
        }
    }

    #[test]
    fn elevate_bezier_span_preserves_curve_value() {
        // Single bezier span is a B-spline with degree = n and a clamped, no-interior-knot vector.
        let control_values = vec![0.0, 3.0, -2.0, 5.0];
        let elevated = elevate_bezier_span(&control_values);
        assert_eq!(elevated.len(), control_values.len() + 1);
        let b = crate::bezier::RationalBezier2::unweighted(control_values.iter().map(|&v| crate::vec::Pnt2::new(v, 0.0)).collect());
        let be = crate::bezier::RationalBezier2::unweighted(elevated.iter().map(|&v| crate::vec::Pnt2::new(v, 0.0)).collect());
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!((b.eval(t).x - be.eval(t).x).abs() < 1e-9);
        }
    }

    mod quick {
        use super::*;

        #[test]
        fn de_boor_matches_bernstein_sum_oracle_on_random_bezier_span_curves() {
            // A single-span (no interior knots) clamped B-spline of degree p is exactly the
            // Bernstein-basis polynomial with the same control values — an independent oracle.
            let mut rng = mathematical_random::Rng::from_seed(41);
            for _ in 0..200 {
                let degree = 1 + rng.next_range(0, 5) as usize;
                let n_cp = degree + 1;
                let kv = KnotVector::clamped_uniform(n_cp, degree);
                let values: Vec<f64> = (0..n_cp).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
                let bernstein = crate::poly::Bernstein::new(values.clone());
                for i in 0..=20 {
                    let u = i as f64 / 20.0;
                    let via_de_boor = de_boor(&kv, &values, u);
                    let via_bernstein = bernstein.eval(u);
                    assert!((via_de_boor - via_bernstein).abs() < 1e-9, "mismatch at u={u} degree={degree}: de_boor={via_de_boor} bernstein={via_bernstein}");
                }
            }
        }

        #[test]
        fn knot_insertion_is_geometrically_a_no_op_on_random_curves() {
            let mut rng = mathematical_random::Rng::from_seed(43);
            for _ in 0..100 {
                let degree = 1 + rng.next_range(0, 4) as usize;
                let n_cp = degree + 2 + rng.next_range(0, 4) as usize;
                let kv = KnotVector::clamped_uniform(n_cp, degree);
                let values: Vec<f64> = (0..n_cp).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
                let (lo, hi) = kv.domain();
                let u = lo + (hi - lo) * rng.next_f64();
                if kv.multiplicity(u) > degree {
                    continue;
                }
                let (new_kv, new_values) = insert_knot(&kv, &values, u);
                for i in 0..=20 {
                    let t = lo + (hi - lo) * (i as f64 / 20.0);
                    let before = de_boor(&kv, &values, t);
                    let after = de_boor(&new_kv, &new_values, t);
                    assert!((before - after).abs() < 1e-7, "curve changed at t={t}: {before} vs {after}");
                }
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Bspline

// #region 🔖️Curve
pub mod curve {
//! 🌀️ Analytic and free-form 3D curves ([`Curve3`]) and their 2D parameter-space counterparts
//! ([`Curve2`], the pcurve representation). Every variant supports position/derivative evaluation
//! and an *exact* [`Curve3::to_nurbs`]/[`Curve2::to_nurbs`] conversion — the single representation
//! every downstream algorithm (intersection, tessellation, STEP export) can fall back to when it
//! doesn't have an analytic fast path for a particular curve kind.

use crate::bspline::{de_boor, KnotVector};
use crate::mat::Frame3;
use crate::vec::{Pnt2, Pnt3, Vec2, Vec3};

// #region 🔖️Curve3

/// 🌀️ A 3D curve. Each variant's *natural* domain is documented on the variant; a curve's actual
/// used range (e.g. an edge's `(t0, t1)`) is stored by the topology layer, not here — this keeps
/// geometry shareable between edges that trim the same underlying curve differently.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Curve3 {
    /// 🌀️ `origin + t·dir`. Natural domain `(-∞, ∞)`. `dir` need not be unit.
    Line { origin: Pnt3, dir: Vec3 },
    /// 🌀️ A full circle in `frame`'s xy-plane. Natural domain `[0, 2π)`, periodic.
    Circle { frame: Frame3, radius: f64 },
    /// 🌀️ A full ellipse in `frame`'s xy-plane (`frame.x` = major axis, `frame.y` = minor axis).
    /// Natural domain `[0, 2π)`, periodic.
    Ellipse { frame: Frame3, major_radius: f64, minor_radius: f64 },
    /// 🌀️ A rational B-spline curve. Natural domain = the knot vector's domain.
    Nurbs { knots: KnotVector, controls: Vec<Pnt3>, weights: Vec<f64> },
}

/// 🌀️ An explicit rational-NURBS representation, returned by [`Curve3::to_nurbs`] /
/// [`Curve2::to_nurbs`] — every curve kind's common denominator.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NurbsCurve3 {
    pub knots: KnotVector,
    pub controls: Vec<Pnt3>,
    pub weights: Vec<f64>,
}

impl Curve3 {
    pub fn domain(&self) -> (f64, f64) {
        match self {
            Curve3::Line { .. } => (f64::NEG_INFINITY, f64::INFINITY),
            Curve3::Circle { .. } | Curve3::Ellipse { .. } => (0.0, std::f64::consts::TAU),
            Curve3::Nurbs { knots, .. } => knots.domain(),
        }
    }
    pub fn is_periodic(&self) -> bool {
        matches!(self, Curve3::Circle { .. } | Curve3::Ellipse { .. })
    }
    pub fn period(&self) -> Option<f64> {
        if self.is_periodic() {
            Some(std::f64::consts::TAU)
        } else {
            None
        }
    }
    pub fn eval(&self, t: f64) -> Pnt3 {
        match self {
            Curve3::Line { origin, dir } => *origin + *dir * t,
            Curve3::Circle { frame, radius } => frame.to_world(Pnt3::new(radius * t.cos(), radius * t.sin(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world(Pnt3::new(major_radius * t.cos(), minor_radius * t.sin(), 0.0)),
            Curve3::Nurbs { knots, controls, weights } => eval_nurbs_curve(knots, controls, weights, t),
        }
    }
    /// 🌀️ First derivative `dC/dt`.
    pub fn d1(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { dir, .. } => *dir,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.sin(), radius * t.cos(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.sin(), minor_radius * t.cos(), 0.0)),
            Curve3::Nurbs { .. } => nurbs_derivative_finite(self, t, 1),
        }
    }
    /// 🌀️ Second derivative `d²C/dt²`.
    pub fn d2(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { .. } => Vec3::ZERO,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.cos(), -radius * t.sin(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.cos(), -minor_radius * t.sin(), 0.0)),
            Curve3::Nurbs { .. } => nurbs_derivative_finite(self, t, 2),
        }
    }
    pub fn tangent(&self, t: f64) -> Option<Vec3> {
        self.d1(t).normalized()
    }
    /// 🌀️ Signed curvature magnitude `|C' × C''| / |C'|³` (the standard space-curve formula).
    pub fn curvature(&self, t: f64) -> f64 {
        let d1 = self.d1(t);
        let d2 = self.d2(t);
        let speed = d1.norm();
        if speed <= f64::EPSILON {
            return 0.0;
        }
        d1.cross(d2).norm() / speed.powi(3)
    }
    /// 🌀️ An exact rational-NURBS representation over `domain` (required for [`Curve3::Line`],
    /// whose natural domain is unbounded). Arcs longer than 120° are split into equal spans of
    /// the standard rational-quadratic circular-arc construction for numerical conditioning.
    /// The returned curve traces exactly the same points over `domain` and agrees with `self` at
    /// `domain.0`/`domain.1` and every span breakpoint in between — but, as for any rational
    /// quadratic circle/ellipse representation, its *own* parametrization is not angle-linear
    /// except at those breakpoints (a well-known property of the construction, not an
    /// approximation: every point it produces still lies exactly on the circle/ellipse).
    pub fn to_nurbs(&self, domain: (f64, f64)) -> NurbsCurve3 {
        match self {
            Curve3::Line { origin, dir } => {
                let p0 = *origin + *dir * domain.0;
                let p1 = *origin + *dir * domain.1;
                NurbsCurve3 { knots: KnotVector::new(vec![domain.0, domain.0, domain.1, domain.1], 1, 2).unwrap(), controls: vec![p0, p1], weights: vec![1.0, 1.0] }
            }
            Curve3::Circle { frame, radius } => arc_to_nurbs(frame, *radius, *radius, domain),
            Curve3::Ellipse { frame, major_radius, minor_radius } => arc_to_nurbs(frame, *major_radius, *minor_radius, domain),
            Curve3::Nurbs { knots, controls, weights } => NurbsCurve3 { knots: knots.clone(), controls: controls.clone(), weights: weights.clone() },
        }
    }
}

fn eval_nurbs_curve(knots: &KnotVector, controls: &[Pnt3], weights: &[f64], t: f64) -> Pnt3 {
    let hx: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.x * w).collect();
    let hy: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.y * w).collect();
    let hz: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.z * w).collect();
    let w = de_boor(knots, weights, t);
    Pnt3::new(de_boor(knots, &hx, t) / w, de_boor(knots, &hy, t) / w, de_boor(knots, &hz, t) / w)
}

/// 🌀️ Central-difference derivative — used for NURBS curves as a robust, simple stand-in until a
/// dedicated rational-derivative (de Boor `A_k(u)` recurrence) implementation is needed; accurate
/// to ~1e-6, adequate for tangent/curvature use but not for tight Newton iterations on NURBS,
/// which should prefer analytic curves or accept the extra refinement step.
fn nurbs_derivative_finite(curve: &Curve3, t: f64, order: u32) -> Vec3 {
    let h = 1e-4;
    match order {
        1 => (curve.eval(t + h) - curve.eval(t - h)) * (1.0 / (2.0 * h)),
        2 => {
            let a = curve.eval(t + h).to_vec();
            let b = curve.eval(t).to_vec();
            let c = curve.eval(t - h).to_vec();
            (a - b * 2.0 + c) * (1.0 / (h * h))
        }
        _ => Vec3::ZERO,
    }
}

/// 🌀️ Converts a circular/elliptical arc over `domain` into an exact rational-quadratic NURBS,
/// splitting into `⌈span / 120°⌉` equal-angle spans (the standard well-conditioned construction:
/// each span's middle control point sits at `radius / cos(half-span)` with weight `cos(half-span)`).
fn arc_to_nurbs(frame: &Frame3, radius_x: f64, radius_y: f64, domain: (f64, f64)) -> NurbsCurve3 {
    let span = domain.1 - domain.0;
    let max_span = std::f64::consts::TAU / 3.0; // 120 degrees
    let n_spans = (span.abs() / max_span).ceil().max(1.0) as usize;
    let step = span / n_spans as f64;
    let mut controls = Vec::with_capacity(2 * n_spans + 1);
    let mut weights = Vec::with_capacity(2 * n_spans + 1);
    let local_point = |angle: f64, r_scale: f64| Pnt3::new(radius_x * angle.cos() * r_scale, radius_y * angle.sin() * r_scale, 0.0);
    for span_i in 0..n_spans {
        let a0 = domain.0 + step * span_i as f64;
        let a1 = a0 + step;
        let half = (a1 - a0) * 0.5;
        let mid = (a0 + a1) * 0.5;
        let w1 = half.cos();
        let p0 = local_point(a0, 1.0);
        let p2 = local_point(a1, 1.0);
        let p1 = local_point(mid, 1.0 / w1);
        if span_i == 0 {
            controls.push(frame.to_world(p0));
            weights.push(1.0);
        }
        controls.push(frame.to_world(p1));
        weights.push(w1);
        controls.push(frame.to_world(p2));
        weights.push(1.0);
    }
    let mut knots = vec![domain.0, domain.0, domain.0];
    for span_i in 1..n_spans {
        let knot = domain.0 + step * span_i as f64;
        knots.push(knot);
        knots.push(knot);
    }
    knots.push(domain.1);
    knots.push(domain.1);
    knots.push(domain.1);
    NurbsCurve3 { knots: KnotVector::new(knots, 2, controls.len()).unwrap(), controls, weights }
}

// #endregion 🔖️Curve3

// #region 🔖️Curve2

/// 🌀️ A 2D curve, used as the pcurve type: a curve living in a face's `(u, v)` parameter domain.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Curve2 {
    Line { origin: Pnt2, dir: Vec2 },
    Circle { center: Pnt2, radius: f64 },
    Ellipse { center: Pnt2, x_axis: Vec2, major_radius: f64, minor_radius: f64 },
    Nurbs { knots: KnotVector, controls: Vec<Pnt2>, weights: Vec<f64> },
}

impl Curve2 {
    pub fn domain(&self) -> (f64, f64) {
        match self {
            Curve2::Line { .. } => (f64::NEG_INFINITY, f64::INFINITY),
            Curve2::Circle { .. } | Curve2::Ellipse { .. } => (0.0, std::f64::consts::TAU),
            Curve2::Nurbs { knots, .. } => knots.domain(),
        }
    }
    pub fn eval(&self, t: f64) -> Pnt2 {
        match self {
            Curve2::Line { origin, dir } => *origin + *dir * t,
            Curve2::Circle { center, radius } => *center + Vec2::new(radius * t.cos(), radius * t.sin()),
            Curve2::Ellipse { center, x_axis, major_radius, minor_radius } => {
                let x = x_axis.normalized().unwrap_or(Vec2::new(1.0, 0.0));
                let y = x.perp();
                *center + x * (major_radius * t.cos()) + y * (minor_radius * t.sin())
            }
            Curve2::Nurbs { knots, controls, weights } => eval_nurbs_curve2(knots, controls, weights, t),
        }
    }
    pub fn d1(&self, t: f64) -> Vec2 {
        match self {
            Curve2::Line { dir, .. } => *dir,
            Curve2::Circle { radius, .. } => Vec2::new(-radius * t.sin(), radius * t.cos()),
            Curve2::Ellipse { x_axis, major_radius, minor_radius, .. } => {
                let x = x_axis.normalized().unwrap_or(Vec2::new(1.0, 0.0));
                let y = x.perp();
                x * (-major_radius * t.sin()) + y * (minor_radius * t.cos())
            }
            Curve2::Nurbs { .. } => {
                let h = 1e-5;
                (self.eval(t + h) - self.eval(t - h)) * (1.0 / (2.0 * h))
            }
        }
    }
}

fn eval_nurbs_curve2(knots: &KnotVector, controls: &[Pnt2], weights: &[f64], t: f64) -> Pnt2 {
    let hx: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.x * w).collect();
    let hy: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.y * w).collect();
    let w = de_boor(knots, weights, t);
    Pnt2::new(de_boor(knots, &hx, t) / w, de_boor(knots, &hy, t) / w)
}

// #endregion 🔖️Curve2

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn fd_d1(curve: &Curve3, t: f64) -> Vec3 {
        let h = 1e-6;
        (curve.eval(t + h) - curve.eval(t - h)) * (1.0 / (2.0 * h))
    }
    fn fd_d2(curve: &Curve3, t: f64) -> Vec3 {
        let h = 1e-4;
        let a = curve.eval(t + h).to_vec();
        let b = curve.eval(t).to_vec();
        let c = curve.eval(t - h).to_vec();
        (a - b * 2.0 + c) * (1.0 / (h * h))
    }

    #[test]
    fn line_eval_and_derivatives() {
        let l = Curve3::Line { origin: Pnt3::new(1.0, 2.0, 3.0), dir: Vec3::new(2.0, 0.0, 0.0) };
        assert_eq!(l.eval(0.5), Pnt3::new(2.0, 2.0, 3.0));
        assert_eq!(l.d1(0.5), Vec3::new(2.0, 0.0, 0.0));
        assert_eq!(l.d2(0.5), Vec3::ZERO);
        assert_eq!(l.curvature(0.5), 0.0);
    }

    #[test]
    fn circle_eval_stays_on_circle_and_derivatives_match_finite_differences() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 3.0 };
        for i in 0..10 {
            let t = i as f64 * 0.5;
            let p = c.eval(t);
            assert!((p.to_vec().norm() - 3.0).abs() < 1e-9);
            assert!((c.d1(t) - fd_d1(&c, t)).norm() < 1e-5);
            assert!((c.d2(t) - fd_d2(&c, t)).norm() < 1e-2);
        }
    }

    #[test]
    fn circle_curvature_equals_reciprocal_radius() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, 1.0, 1.0), Vec3::X).unwrap();
        let c = Curve3::Circle { frame, radius: 2.5 };
        for t in [0.0, 1.0, 3.0, 5.5] {
            assert!((c.curvature(t) - 1.0 / 2.5).abs() < 1e-6, "curvature mismatch at t={t}");
        }
    }

    #[test]
    fn ellipse_derivatives_match_finite_differences() {
        let frame = Frame3::WORLD;
        let e = Curve3::Ellipse { frame, major_radius: 4.0, minor_radius: 2.0 };
        for i in 0..8 {
            let t = i as f64 * 0.7;
            assert!((e.d1(t) - fd_d1(&e, t)).norm() < 1e-5, "d1 mismatch at t={t}");
        }
    }

    #[test]
    fn line_to_nurbs_matches_line_eval() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 2.0, 3.0) };
        let nurbs = l.to_nurbs((0.0, 2.0));
        for i in 0..=10 {
            let t = i as f64 / 10.0 * 2.0;
            let via_nurbs = eval_nurbs_curve(&nurbs.knots, &nurbs.controls, &nurbs.weights, t);
            assert!(via_nurbs.distance(l.eval(t)) < 1e-9, "mismatch at t={t}");
        }
    }

    /// 🌀️ The invariant a rational arc-to-NURBS conversion actually guarantees: every produced
    /// point lies exactly on the circle (radius match at the frame's own scale), and the curve
    /// agrees with the original at `domain.0`/`domain.1` — NOT pointwise parameter equality
    /// in between, since the standard construction is not angle-linear except at breakpoints
    /// (confirmed by hand + a standalone check: see phase-2 scope note).
    fn assert_nurbs_traces_circle(nurbs: &NurbsCurve3, frame: &Frame3, radius: f64, domain: (f64, f64), samples: usize) {
        for i in 0..=samples {
            let t = domain.0 + (domain.1 - domain.0) * (i as f64 / samples as f64);
            let p = eval_nurbs_curve(&nurbs.knots, &nurbs.controls, &nurbs.weights, t);
            let local = frame.to_local(p);
            assert!((local.to_vec().norm() - radius).abs() < 1e-8, "point at t={t} is not on the circle: radius {}", local.to_vec().norm());
        }
    }

    #[test]
    fn circle_to_nurbs_traces_the_circle_exactly_for_small_arc() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 5.0 };
        let domain = (0.2, 0.2 + std::f64::consts::FRAC_PI_3); // 60 degrees, single span
        let nurbs = c.to_nurbs(domain);
        assert_nurbs_traces_circle(&nurbs, &frame, 5.0, domain, 20);
        assert!(nurbs.controls[0].distance(c.eval(domain.0)) < 1e-9);
        assert!(nurbs.controls.last().unwrap().distance(c.eval(domain.1)) < 1e-9);
    }

    #[test]
    fn circle_to_nurbs_traces_the_circle_exactly_for_full_circle_multi_span() {
        let frame = Frame3::from_normal(Pnt3::new(2.0, -1.0, 0.5), Vec3::new(0.3, 0.2, 1.0)).unwrap();
        let c = Curve3::Circle { frame, radius: 1.7 };
        let domain = c.domain();
        let nurbs = c.to_nurbs(domain);
        assert!(nurbs.controls.len() > 3, "a full circle must be split into more than one span");
        assert_nurbs_traces_circle(&nurbs, &frame, 1.7, domain, 60);
        assert!(nurbs.controls[0].distance(c.eval(domain.0)) < 1e-8);
        assert!(nurbs.controls.last().unwrap().distance(c.eval(domain.1)) < 1e-8);
    }

    #[test]
    fn ellipse_to_nurbs_traces_the_ellipse_exactly() {
        let frame = Frame3::WORLD;
        let major = 3.0;
        let minor = 1.0;
        let e = Curve3::Ellipse { frame, major_radius: major, minor_radius: minor };
        let domain = (0.0, std::f64::consts::PI * 1.5);
        let nurbs = e.to_nurbs(domain);
        for i in 0..=30 {
            let t = domain.0 + (domain.1 - domain.0) * (i as f64 / 30.0);
            let p = eval_nurbs_curve(&nurbs.knots, &nurbs.controls, &nurbs.weights, t);
            let local = frame.to_local(p);
            let residual = (local.x / major).powi(2) + (local.y / minor).powi(2) - 1.0;
            assert!(residual.abs() < 1e-8, "point at t={t} is not on the ellipse: residual={residual}");
        }
        assert!(nurbs.controls[0].distance(e.eval(domain.0)) < 1e-9);
        assert!(nurbs.controls.last().unwrap().distance(e.eval(domain.1)) < 1e-9);
    }

    #[test]
    fn curve2_line_and_circle_eval() {
        let l = Curve2::Line { origin: Pnt2::new(0.0, 0.0), dir: Vec2::new(1.0, 1.0) };
        assert_eq!(l.eval(2.0), Pnt2::new(2.0, 2.0));
        let c = Curve2::Circle { center: Pnt2::new(1.0, 1.0), radius: 2.0 };
        let p = c.eval(0.0);
        assert!(((p - Pnt2::new(1.0, 1.0)).norm() - 2.0).abs() < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn circle_to_nurbs_traces_the_circle_exactly_for_random_arcs() {
            let mut rng = mathematical_random::Rng::from_seed(53);
            for _ in 0..100 {
                let frame = Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z)).unwrap();
                let radius = 0.1 + rng.next_f64() * 10.0;
                let c = Curve3::Circle { frame, radius };
                let a0 = rng.next_f64() * std::f64::consts::TAU;
                let span = rng.next_f64() * std::f64::consts::TAU * 1.5;
                let domain = (a0, a0 + span);
                let nurbs = c.to_nurbs(domain);
                assert_nurbs_traces_circle(&nurbs, &frame, radius, domain, 25);
                assert!(nurbs.controls[0].distance(c.eval(domain.0)) < 1e-7, "start point mismatch radius={radius} domain={domain:?}");
                assert!(nurbs.controls.last().unwrap().distance(c.eval(domain.1)) < 1e-7, "end point mismatch radius={radius} domain={domain:?}");
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Curve

// #region 🔖️Curveops
pub mod curve_ops {
//! 📏️ Curve algorithms that operate *on* a [`crate::curve::Curve3`] rather than being part of its
//! definition: arc length, closest-point projection, and the split/reverse/join operations edges
//! need when Euler operators cut a curve. Kept separate from `curve.rs` so that file stays a pure
//! evaluation interface and this one can grow numerically heavier machinery independently.

use crate::bspline::{basis_functions, insert_knot, KnotVector};
use crate::curve::{Curve3, NurbsCurve3};
use crate::vec::Pnt3;

// #region 🔖️Length

/// 📏️ 5-point Gauss-Legendre nodes/weights on `[-1, 1]`.
const GL5_NODES: [f64; 5] = [-0.906_179_845_938_664, -0.538_469_310_105_683_1, 0.0, 0.538_469_310_105_683_1, 0.906_179_845_938_664];
const GL5_WEIGHTS: [f64; 5] = [0.2369268850561891, 0.4786286704993665, 0.5688888888888889, 0.4786286704993665, 0.2369268850561891];

fn gauss_legendre5(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    GL5_NODES.iter().zip(GL5_WEIGHTS.iter()).map(|(&x, &w)| w * f(mid + half * x)).sum::<f64>() * half
}

/// 📏️ Adaptive-quadrature arc length of `curve` over `[t0, t1]`: recursively halves the interval
/// until the 5-point Gauss-Legendre estimate agrees with the sum of its two half-interval
/// estimates to within `tol` (Richardson-style error control), or `max_depth` is reached.
pub fn arc_length(curve: &Curve3, t0: f64, t1: f64, tol: f64) -> f64 {
    arc_length_recursive(curve, t0, t1, tol, 24)
}

fn arc_length_recursive(curve: &Curve3, t0: f64, t1: f64, tol: f64, depth: u32) -> f64 {
    let speed = |t: f64| curve.d1(t).norm();
    let whole = gauss_legendre5(speed, t0, t1);
    if depth == 0 {
        return whole;
    }
    let mid = 0.5 * (t0 + t1);
    let left = gauss_legendre5(speed, t0, mid);
    let right = gauss_legendre5(speed, mid, t1);
    if (whole - (left + right)).abs() < tol {
        left + right
    } else {
        arc_length_recursive(curve, t0, mid, tol * 0.5, depth - 1) + arc_length_recursive(curve, mid, t1, tol * 0.5, depth - 1)
    }
}

/// 📏️ Finds the parameter `t ∈ [t0, t1]` at which the arc length from `t0` equals `target_length`,
/// via bisection on the (monotonic, since speed ≥ 0) length function.
pub fn param_at_length(curve: &Curve3, t0: f64, t1: f64, target_length: f64, tol: f64) -> f64 {
    let total = arc_length(curve, t0, t1, tol);
    if target_length <= 0.0 {
        return t0;
    }
    if target_length >= total {
        return t1;
    }
    let mut lo = t0;
    let mut hi = t1;
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        let len = arc_length(curve, t0, mid, tol);
        if (len - target_length).abs() < tol {
            return mid;
        }
        if len < target_length {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

// #endregion 🔖️Length

// #region 🔖️Project

/// 📏️ Closest point on `curve` (restricted to `domain`) to `target`: coarse uniform sampling
/// (`samples` intervals) seeds a safeguarded Newton refinement of `f(t) = (C(t)-P)·C'(t) = 0`
/// (the standard first-order optimality condition for point-curve distance) from the best sample
/// and its neighbors, keeping the global best result found. Returns `(t, distance)`.
pub fn closest_point(curve: &Curve3, domain: (f64, f64), target: Pnt3, samples: usize) -> (f64, f64) {
    let mut best_t = domain.0;
    let mut best_d2 = curve.eval(domain.0).distance_sq(target);
    for i in 0..=samples {
        let t = domain.0 + (domain.1 - domain.0) * (i as f64 / samples as f64);
        let d2 = curve.eval(t).distance_sq(target);
        if d2 < best_d2 {
            best_d2 = d2;
            best_t = t;
        }
    }
    // For a periodic curve, the true minimum can sit just across the domain boundary from the
    // best coarse sample (e.g. near angle 0 when the closest point is actually at 2π-ε) — a hard
    // clamp would trap Newton exactly at that boundary. Wrap into the period instead of clamping.
    let refined = newton_closest_point(curve, target, best_t, domain, curve.period());
    let refined_d2 = curve.eval(refined).distance_sq(target);
    if refined_d2 < best_d2 {
        (refined, refined_d2.sqrt())
    } else {
        (best_t, best_d2.sqrt())
    }
}

fn wrap_into_domain(t: f64, domain: (f64, f64), period: Option<f64>) -> f64 {
    match period {
        Some(p) => {
            let mut x = (t - domain.0) % p;
            if x < 0.0 {
                x += p;
            }
            domain.0 + x
        }
        None => t.clamp(domain.0, domain.1),
    }
}

fn newton_closest_point(curve: &Curve3, target: Pnt3, mut t: f64, domain: (f64, f64), period: Option<f64>) -> f64 {
    for _ in 0..30 {
        let c = curve.eval(t);
        let d1 = curve.d1(t);
        let d2 = curve.d2(t);
        let delta = c - target;
        let f = delta.dot(d1);
        let fp = d1.dot(d1) + delta.dot(d2);
        if fp.abs() <= 1e-300 {
            break;
        }
        let step = f / fp;
        let next = wrap_into_domain(t - step, domain, period);
        if (next - t).abs() < 1e-13 {
            t = next;
            break;
        }
        t = next;
    }
    t
}

/// 📏️ All local extrema of distance-to-`target` on `curve` over `domain` (both minima and
/// maxima), found by sign changes of `f(t) = (C(t)-P)·C'(t)` across a uniform sample, each refined
/// by the same Newton step as [`closest_point`]. Used where a caller needs every critical point,
/// not just the global closest (e.g. offset self-intersection analysis in later phases).
pub fn all_extrema(curve: &Curve3, domain: (f64, f64), target: Pnt3, samples: usize) -> Vec<f64> {
    let f = |t: f64| (curve.eval(t) - target).dot(curve.d1(t));
    let mut roots = Vec::new();
    let mut prev_t = domain.0;
    let mut prev_f = f(prev_t);
    for i in 1..=samples {
        let t = domain.0 + (domain.1 - domain.0) * (i as f64 / samples as f64);
        let ft = f(t);
        if prev_f == 0.0 {
            roots.push(prev_t);
        } else if prev_f.signum() != ft.signum() {
            roots.push(newton_closest_point(curve, target, 0.5 * (prev_t + t), (prev_t, t), None));
        }
        prev_t = t;
        prev_f = ft;
    }
    if prev_f == 0.0 {
        roots.push(prev_t);
    }
    roots
}

// #endregion 🔖️Project

// #region 🔖️Fit

/// 📏️ Global cubic interpolation through `points` using centripetal parameterization (Lee's
/// method) — the standard, well-conditioned choice for interpolating scattered points without
/// the cusping chord-length parametrization can produce.
pub fn interpolate_centripetal(points: &[Pnt3]) -> Option<NurbsCurve3> {
    let n = points.len();
    if n < 2 {
        return None;
    }
    let degree = (n - 1).min(3);
    let mut chord_sqrt = vec![0.0; n];
    for i in 1..n {
        chord_sqrt[i] = points[i].distance(points[i - 1]).sqrt();
    }
    let total: f64 = chord_sqrt.iter().sum();
    if total <= 0.0 {
        return None;
    }
    let mut params = vec![0.0; n];
    let mut acc = 0.0;
    for i in 1..n {
        acc += chord_sqrt[i];
        params[i] = acc / total;
    }
    let mut knots = vec![0.0; degree + 1];
    for j in 1..n - degree {
        let avg: f64 = params[j..j + degree].iter().sum::<f64>() / degree as f64;
        knots.push(avg);
    }
    knots.extend(std::iter::repeat_n(1.0, degree + 1));
    let kv = KnotVector::new(knots, degree, n)?;
    let mut a = vec![vec![0.0; n]; n];
    for (row, &u) in params.iter().enumerate() {
        let span = kv.find_span(u);
        let basis = basis_functions(&kv, span, u);
        for (j, &b) in basis.iter().enumerate() {
            a[row][span - degree + j] = b;
        }
    }
    let solve_axis = |axis: fn(&Pnt3) -> f64| -> Vec<f64> {
        let rhs: Vec<f64> = points.iter().map(axis).collect();
        solve_linear_system(&a, &rhs)
    };
    let xs = solve_axis(|p| p.x);
    let ys = solve_axis(|p| p.y);
    let zs = solve_axis(|p| p.z);
    let controls = (0..n).map(|i| Pnt3::new(xs[i], ys[i], zs[i])).collect();
    Some(NurbsCurve3 { knots: kv, controls, weights: vec![1.0; n] })
}

/// 📏️ Plain Gaussian elimination with partial pivoting — the interpolation matrix is small
/// (control-point count) and banded but not worth a dedicated banded solver at this scale.
fn solve_linear_system(a: &[Vec<f64>], rhs: &[f64]) -> Vec<f64> {
    let n = rhs.len();
    let mut m: Vec<Vec<f64>> = a.to_vec();
    let mut b = rhs.to_vec();
    for col in 0..n {
        let pivot = (col..n).max_by(|&i, &j| m[i][col].abs().partial_cmp(&m[j][col].abs()).unwrap()).unwrap();
        m.swap(col, pivot);
        b.swap(col, pivot);
        let diag = m[col][col];
        let pivot_row = m[col].clone();
        for row in col + 1..n {
            let factor = m[row][col] / diag;
            for (k, cell) in m[row].iter_mut().enumerate().skip(col) {
                *cell -= factor * pivot_row[k];
            }
            b[row] -= factor * b[col];
        }
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let sum: f64 = (row + 1..n).map(|k| m[row][k] * x[k]).sum();
        x[row] = (b[row] - sum) / m[row][row];
    }
    x
}

// #endregion 🔖️Fit

// #region 🔖️Edit

/// 📏️ Reverses a NURBS curve's direction: reverses control points/weights and mirrors the knot
/// vector around the domain, so `reverse(c).eval(domain.1 - (t - domain.0)) == c.eval(t)`.
pub fn reverse_nurbs(curve: &NurbsCurve3) -> NurbsCurve3 {
    let (lo, hi) = curve.knots.domain();
    let mut controls = curve.controls.clone();
    controls.reverse();
    let mut weights = curve.weights.clone();
    weights.reverse();
    let knots: Vec<f64> = curve.knots.knots.iter().rev().map(|&k| lo + hi - k).collect();
    NurbsCurve3 { knots: KnotVector { knots, degree: curve.knots.degree }, controls, weights }
}

/// 📏️ Splits a NURBS curve at `t` into two curves, each covering one side of the original domain,
/// via repeated knot insertion until `t` reaches full multiplicity (`degree + 1`), then slicing
/// the (now Bezier-joined) control net at that knot.
pub fn split_nurbs(curve: &NurbsCurve3, t: f64) -> (NurbsCurve3, NurbsCurve3) {
    let degree = curve.knots.degree;
    let mut knots = curve.knots.clone();
    let mut hx: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.x * w).collect();
    let mut hy: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.y * w).collect();
    let mut hz: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.z * w).collect();
    let mut hw = curve.weights.clone();
    let needed = degree + 1 - knots.multiplicity(t);
    for _ in 0..needed {
        let (nk, nx) = insert_knot(&knots, &hx, t);
        let (_, ny) = insert_knot(&knots, &hy, t);
        let (_, nz) = insert_knot(&knots, &hz, t);
        let (_, nw) = insert_knot(&knots, &hw, t);
        knots = nk;
        hx = nx;
        hy = ny;
        hz = nz;
        hw = nw;
    }
    // Once t has full multiplicity (degree+1), it occupies consecutive knot indices [k, k+degree]
    // for some k; find_span(t) returns k+degree (the span ending exactly at t), so k = span-degree.
    // The control net splits cleanly there: the left piece owns points [0, k), the right [k, end).
    let k = knots.find_span(t) - degree;
    let dehomogenize = |i: usize| Pnt3::new(hx[i] / hw[i], hy[i] / hw[i], hz[i] / hw[i]);
    let left_controls: Vec<Pnt3> = (0..k).map(dehomogenize).collect();
    let left_weights: Vec<f64> = hw[0..k].to_vec();
    let right_controls: Vec<Pnt3> = (k..hx.len()).map(dehomogenize).collect();
    let right_weights: Vec<f64> = hw[k..].to_vec();
    let left_knot_count = left_controls.len() + degree + 1;
    let right_knot_count = right_controls.len() + degree + 1;
    let left_knots = knots.knots[0..left_knot_count].to_vec();
    let right_knots = knots.knots[knots.knots.len() - right_knot_count..].to_vec();
    (
        NurbsCurve3 { knots: KnotVector { knots: left_knots, degree }, controls: left_controls, weights: left_weights },
        NurbsCurve3 { knots: KnotVector { knots: right_knots, degree }, controls: right_controls, weights: right_weights },
    )
}

// #endregion 🔖️Edit

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::bspline::de_boor;
    use crate::curve::Curve3;
    use crate::mat::Frame3;
    use crate::vec::Vec3;

    #[test]
    fn arc_length_of_line_equals_euclidean_distance() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(3.0, 4.0, 0.0) };
        let len = arc_length(&l, 0.0, 1.0, 1e-9);
        assert!((len - 5.0).abs() < 1e-6);
    }

    #[test]
    fn arc_length_of_quarter_circle_matches_closed_form() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 2.0 };
        let len = arc_length(&c, 0.0, std::f64::consts::FRAC_PI_2, 1e-9);
        assert!((len - std::f64::consts::PI).abs() < 1e-6); // quarter of 2*pi*r=4pi, i.e. pi
    }

    #[test]
    fn param_at_length_round_trips_with_arc_length() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 3.0 };
        let total = arc_length(&c, 0.0, 2.0, 1e-9);
        let target = total * 0.4;
        let t = param_at_length(&c, 0.0, 2.0, target, 1e-9);
        let recomputed = arc_length(&c, 0.0, t, 1e-9);
        assert!((recomputed - target).abs() < 1e-6);
    }

    #[test]
    fn closest_point_on_circle_matches_radial_projection() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 2.0 };
        let target = Pnt3::new(10.0, 0.0, 0.0);
        let (t, dist) = closest_point(&c, (0.0, std::f64::consts::TAU), target, 64);
        assert!((dist - 8.0).abs() < 1e-6);
        let p = c.eval(t);
        assert!(p.distance(Pnt3::new(2.0, 0.0, 0.0)) < 1e-6);
    }

    #[test]
    fn closest_point_on_line_matches_perpendicular_foot() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
        let target = Pnt3::new(5.0, 3.0, 0.0);
        let (t, dist) = closest_point(&l, (-10.0, 10.0), target, 40);
        assert!((t - 5.0).abs() < 1e-6);
        assert!((dist - 3.0).abs() < 1e-6);
    }

    #[test]
    fn all_extrema_finds_both_near_and_far_points_on_circle() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 2.0 };
        let target = Pnt3::new(10.0, 0.0, 0.0);
        let extrema = all_extrema(&c, (0.0, std::f64::consts::TAU), target, 64);
        assert_eq!(extrema.len(), 2);
        let near = c.eval(extrema[0]).distance(Pnt3::new(2.0, 0.0, 0.0)) < 1e-6 || c.eval(extrema[1]).distance(Pnt3::new(2.0, 0.0, 0.0)) < 1e-6;
        let far = c.eval(extrema[0]).distance(Pnt3::new(-2.0, 0.0, 0.0)) < 1e-6 || c.eval(extrema[1]).distance(Pnt3::new(-2.0, 0.0, 0.0)) < 1e-6;
        assert!(near && far);
    }

    /// 📏️ Recomputes the same centripetal parameter values `interpolate_centripetal` assigns to
    /// each data point — an independent oracle so the test checks the actual interpolation
    /// property (curve(param[i]) == points[i]) instead of a dense-sampling proxy, which can show
    /// a spurious "gap" purely from sampling resolution near fast-moving parts of the curve.
    fn centripetal_params(points: &[Pnt3]) -> Vec<f64> {
        let n = points.len();
        let mut chord_sqrt = vec![0.0; n];
        for i in 1..n {
            chord_sqrt[i] = points[i].distance(points[i - 1]).sqrt();
        }
        let total: f64 = chord_sqrt.iter().sum();
        let mut params = vec![0.0; n];
        let mut acc = 0.0;
        for i in 1..n {
            acc += chord_sqrt[i];
            params[i] = acc / total;
        }
        params
    }

    #[test]
    fn interpolate_centripetal_passes_through_all_points() {
        let points = vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 2.0, 0.0), Pnt3::new(3.0, 1.0, 0.0), Pnt3::new(4.0, 3.0, 1.0)];
        let curve = interpolate_centripetal(&points).unwrap();
        let params = centripetal_params(&points);
        for (p, t) in points.iter().zip(params.iter()) {
            let on_curve = de_boor_pnt(&curve, *t);
            assert!(on_curve.distance(*p) < 1e-6, "point {p:?} not interpolated at its own parameter t={t}: got {on_curve:?}");
        }
    }

    fn de_boor_pnt(curve: &NurbsCurve3, t: f64) -> Pnt3 {
        let hx: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.x * w).collect();
        let hy: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.y * w).collect();
        let hz: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.z * w).collect();
        let w = de_boor(&curve.knots, &curve.weights, t);
        Pnt3::new(de_boor(&curve.knots, &hx, t) / w, de_boor(&curve.knots, &hy, t) / w, de_boor(&curve.knots, &hz, t) / w)
    }

    #[test]
    fn reverse_nurbs_reproduces_the_same_curve_reversed() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 1.0, 1.0) };
        let nurbs = l.to_nurbs((0.0, 4.0));
        let reversed = reverse_nurbs(&nurbs);
        let (lo, hi) = nurbs.knots.domain();
        for i in 0..=10 {
            let t = lo + (hi - lo) * i as f64 / 10.0;
            let original = de_boor_pnt(&nurbs, t);
            let via_reversed = de_boor_pnt(&reversed, hi - (t - lo));
            assert!(original.distance(via_reversed) < 1e-9, "mismatch at t={t}");
        }
    }

    #[test]
    fn split_nurbs_pieces_reproduce_the_original_curve() {
        let points = vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 3.0, 0.0), Pnt3::new(3.0, -1.0, 1.0), Pnt3::new(5.0, 2.0, 2.0), Pnt3::new(6.0, 0.0, 0.0)];
        let curve = interpolate_centripetal(&points).unwrap();
        let (lo, hi) = curve.knots.domain();
        let split_t = lo + (hi - lo) * 0.4;
        let (left, right) = split_nurbs(&curve, split_t);
        let (left_lo, left_hi) = left.knots.domain();
        let (right_lo, right_hi) = right.knots.domain();
        assert!((left_hi - split_t).abs() < 1e-9);
        assert!((right_lo - split_t).abs() < 1e-9);
        for i in 0..=15 {
            let t = left_lo + (left_hi - left_lo) * i as f64 / 15.0;
            assert!(de_boor_pnt(&left, t).distance(de_boor_pnt(&curve, t)) < 1e-7, "left mismatch at t={t}");
        }
        for i in 0..=15 {
            let t = right_lo + (right_hi - right_lo) * i as f64 / 15.0;
            assert!(de_boor_pnt(&right, t).distance(de_boor_pnt(&curve, t)) < 1e-7, "right mismatch at t={t}");
        }
        // The split point itself must match exactly from both sides.
        assert!(de_boor_pnt(&left, left_hi).distance(de_boor_pnt(&right, right_lo)) < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn closest_point_matches_brute_force_dense_sampling_oracle() {
            let mut rng = mathematical_random::Rng::from_seed(61);
            for _ in 0..100 {
                let frame = Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z)).unwrap();
                let radius = 0.5 + rng.next_f64() * 5.0;
                let c = Curve3::Circle { frame, radius };
                let target = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let (_, dist) = closest_point(&c, (0.0, std::f64::consts::TAU), target, 32);
                let oracle_dist = (0..=100000).map(|i| c.eval(std::f64::consts::TAU * i as f64 / 100000.0).distance(target)).fold(f64::INFINITY, f64::min);
                assert!((dist - oracle_dist).abs() < 1e-4, "mismatch: newton={dist} oracle={oracle_dist}");
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Curveops

// #region 🔖️Surface
pub mod surface {
//! 🗺️ Analytic and free-form parametric surfaces. Every variant supports position, first/second
//! partial derivatives, normal, and Gaussian/mean/principal curvature via the standard first- and
//! second-fundamental-form formulas — the common surface interface every face in the topology
//! layer evaluates through, regardless of whether it's a `Plane` or a full `Nurbs` patch.

use crate::bspline::{basis_function_derivatives, KnotVector};
use crate::mat::Frame3;
use crate::vec::{Pnt3, Vec3};

// #region 🔖️Surface

/// 🗺️ A parametric surface `S(u, v)`. Domain and periodicity are documented per variant; as with
/// [`crate::curve::Curve3`], a face's *used* trim domain is stored by the topology layer, not here.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Surface {
    /// 🗺️ `frame.origin + u·frame.x + v·frame.y`. Domain `(-∞, ∞) × (-∞, ∞)`.
    Plane { frame: Frame3 },
    /// 🗺️ Axis along `frame.z`. `u` = angle around the axis (periodic `[0, 2π)`), `v` = height
    /// along the axis (`(-∞, ∞)`).
    Cylinder { frame: Frame3, radius: f64 },
    /// 🗺️ Apex at `frame.origin`, axis `frame.z`. `u` = angle (periodic), `v` = signed distance
    /// along the axis from the apex; the radius at `v` is `v · tan(half_angle)`.
    Cone { frame: Frame3, half_angle: f64 },
    /// 🗺️ `u` = azimuth around `frame.z` (periodic `[0, 2π)`), `v` = elevation from the equator
    /// (`[-π/2, π/2]`).
    Sphere { frame: Frame3, radius: f64 },
    /// 🗺️ `u` = azimuth around the main axis `frame.z` (periodic), `v` = angle around the tube
    /// (periodic). `major_radius` is the distance from the axis to the tube center.
    Torus { frame: Frame3, major_radius: f64, minor_radius: f64 },
    /// 🗺️ A rational tensor-product B-spline surface; `controls[i][j]`/`weights[i][j]` indexed by
    /// `(u, v)` control-net position.
    Nurbs { u_knots: KnotVector, v_knots: KnotVector, controls: Vec<Vec<Pnt3>>, weights: Vec<Vec<f64>> },
}

/// 🗺️ First/second partial derivatives at a surface point, the common input to normal and
/// curvature computations.
pub struct SurfaceDerivatives {
    pub point: Pnt3,
    pub du: Vec3,
    pub dv: Vec3,
    pub duu: Vec3,
    pub duv: Vec3,
    pub dvv: Vec3,
}

impl Surface {
    pub fn domain(&self) -> ((f64, f64), (f64, f64)) {
        match self {
            Surface::Plane { .. } => ((f64::NEG_INFINITY, f64::INFINITY), (f64::NEG_INFINITY, f64::INFINITY)),
            Surface::Cylinder { .. } => ((0.0, std::f64::consts::TAU), (f64::NEG_INFINITY, f64::INFINITY)),
            Surface::Cone { .. } => ((0.0, std::f64::consts::TAU), (0.0, f64::INFINITY)),
            Surface::Sphere { .. } => ((0.0, std::f64::consts::TAU), (-std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2)),
            Surface::Torus { .. } => ((0.0, std::f64::consts::TAU), (0.0, std::f64::consts::TAU)),
            Surface::Nurbs { u_knots, v_knots, .. } => (u_knots.domain(), v_knots.domain()),
        }
    }
    pub fn is_u_periodic(&self) -> bool {
        matches!(self, Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. })
    }
    pub fn is_v_periodic(&self) -> bool {
        matches!(self, Surface::Torus { .. })
    }
    pub fn eval(&self, u: f64, v: f64) -> Pnt3 {
        match self {
            Surface::Plane { frame } => frame.to_world(Pnt3::new(u, v, 0.0)),
            Surface::Cylinder { frame, radius } => frame.to_world(Pnt3::new(radius * u.cos(), radius * u.sin(), v)),
            Surface::Cone { frame, half_angle } => {
                let r = v * half_angle.tan();
                frame.to_world(Pnt3::new(r * u.cos(), r * u.sin(), v))
            }
            Surface::Sphere { frame, radius } => frame.to_world(Pnt3::new(radius * v.cos() * u.cos(), radius * v.cos() * u.sin(), radius * v.sin())),
            Surface::Torus { frame, major_radius, minor_radius } => {
                let r = major_radius + minor_radius * v.cos();
                frame.to_world(Pnt3::new(r * u.cos(), r * u.sin(), minor_radius * v.sin()))
            }
            Surface::Nurbs { u_knots, v_knots, controls, weights } => eval_nurbs_point(u_knots, v_knots, controls, weights, u, v),
        }
    }
    /// 🗺️ First and second partial derivatives at `(u, v)`. Analytic surfaces use closed forms;
    /// NURBS surfaces use central finite differences (see [`crate::curve`]'s equivalent note —
    /// adequate for normal/curvature/tessellation use, not for tight Newton iterations).
    pub fn derivatives(&self, u: f64, v: f64) -> SurfaceDerivatives {
        match self {
            Surface::Plane { frame } => SurfaceDerivatives { point: self.eval(u, v), du: frame.x, dv: frame.y, duu: Vec3::ZERO, duv: Vec3::ZERO, dvv: Vec3::ZERO },
            Surface::Cylinder { frame, radius } => {
                let du = frame.to_world_vector(Vec3::new(-radius * u.sin(), radius * u.cos(), 0.0));
                let duu = frame.to_world_vector(Vec3::new(-radius * u.cos(), -radius * u.sin(), 0.0));
                SurfaceDerivatives { point: self.eval(u, v), du, dv: frame.z, duu, duv: Vec3::ZERO, dvv: Vec3::ZERO }
            }
            Surface::Cone { frame, half_angle } => {
                let r = v * half_angle.tan();
                let tan_a = half_angle.tan();
                let du = frame.to_world_vector(Vec3::new(-r * u.sin(), r * u.cos(), 0.0));
                let dv = frame.to_world_vector(Vec3::new(tan_a * u.cos(), tan_a * u.sin(), 1.0));
                let duu = frame.to_world_vector(Vec3::new(-r * u.cos(), -r * u.sin(), 0.0));
                let duv = frame.to_world_vector(Vec3::new(-tan_a * u.sin(), tan_a * u.cos(), 0.0));
                SurfaceDerivatives { point: self.eval(u, v), du, dv, duu, duv, dvv: Vec3::ZERO }
            }
            Surface::Sphere { frame, radius } => {
                let du = frame.to_world_vector(Vec3::new(-radius * v.cos() * u.sin(), radius * v.cos() * u.cos(), 0.0));
                let dv = frame.to_world_vector(Vec3::new(-radius * v.sin() * u.cos(), -radius * v.sin() * u.sin(), radius * v.cos()));
                let duu = frame.to_world_vector(Vec3::new(-radius * v.cos() * u.cos(), -radius * v.cos() * u.sin(), 0.0));
                let duv = frame.to_world_vector(Vec3::new(radius * v.sin() * u.sin(), -radius * v.sin() * u.cos(), 0.0));
                let dvv = frame.to_world_vector(Vec3::new(-radius * v.cos() * u.cos(), -radius * v.cos() * u.sin(), -radius * v.sin()));
                SurfaceDerivatives { point: self.eval(u, v), du, dv, duu, duv, dvv }
            }
            Surface::Torus { frame, major_radius, minor_radius } => {
                let r = major_radius + minor_radius * v.cos();
                let du = frame.to_world_vector(Vec3::new(-r * u.sin(), r * u.cos(), 0.0));
                let dv = frame.to_world_vector(Vec3::new(-minor_radius * v.sin() * u.cos(), -minor_radius * v.sin() * u.sin(), minor_radius * v.cos()));
                let duu = frame.to_world_vector(Vec3::new(-r * u.cos(), -r * u.sin(), 0.0));
                let duv = frame.to_world_vector(Vec3::new(minor_radius * v.sin() * u.sin(), -minor_radius * v.sin() * u.cos(), 0.0));
                let dvv = frame.to_world_vector(Vec3::new(-minor_radius * v.cos() * u.cos(), -minor_radius * v.cos() * u.sin(), -minor_radius * v.sin()));
                SurfaceDerivatives { point: self.eval(u, v), du, dv, duu, duv, dvv }
            }
            Surface::Nurbs { .. } => finite_difference_derivatives(self, u, v),
        }
    }
    /// 🗺️ Unit surface normal `du × dv` (falls back to `None` at a singular point, e.g. a sphere
    /// pole or a cone apex, where `du` degenerates to zero).
    pub fn normal(&self, u: f64, v: f64) -> Option<Vec3> {
        let d = self.derivatives(u, v);
        d.du.cross(d.dv).normalized()
    }
    /// 🗺️ Gaussian curvature `K = (LN - M²) / (EG - F²)` and mean curvature `H = (EN - 2FM + GL) /
    /// (2(EG - F²))`, from the first fundamental form `(E, F, G)` and second `(L, M, N)`.
    pub fn curvature(&self, u: f64, v: f64) -> Option<(f64, f64)> {
        let d = self.derivatives(u, v);
        let n = d.du.cross(d.dv).normalized()?;
        let e = d.du.dot(d.du);
        let f = d.du.dot(d.dv);
        let g = d.dv.dot(d.dv);
        let l = d.duu.dot(n);
        let m = d.duv.dot(n);
        let nn = d.dvv.dot(n);
        let denom = e * g - f * f;
        if denom.abs() <= 1e-300 {
            return None;
        }
        let gaussian = (l * nn - m * m) / denom;
        let mean = (e * nn - 2.0 * f * m + g * l) / (2.0 * denom);
        Some((gaussian, mean))
    }
    /// 🗺️ Principal curvatures `(κ1, κ2)` derived from Gaussian `K` and mean `H` curvature via
    /// `κ = H ± √(H² - K)`.
    pub fn principal_curvatures(&self, u: f64, v: f64) -> Option<(f64, f64)> {
        let (gaussian, mean) = self.curvature(u, v)?;
        let disc = (mean * mean - gaussian).max(0.0).sqrt();
        Some((mean + disc, mean - disc))
    }
    pub fn is_planar(&self) -> bool {
        matches!(self, Surface::Plane { .. })
    }
}

fn eval_nurbs_point(u_knots: &KnotVector, v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], u: f64, v: f64) -> Pnt3 {
    let u_span = u_knots.find_span(u);
    let v_span = v_knots.find_span(v);
    let nu = basis_function_derivatives(u_knots, u_span, u, 0);
    let nv = basis_function_derivatives(v_knots, v_span, v, 0);
    let up = u_knots.degree;
    let vp = v_knots.degree;
    let mut hx = 0.0;
    let mut hy = 0.0;
    let mut hz = 0.0;
    let mut hw = 0.0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..=up {
        for j in 0..=vp {
            let ci = u_span - up + i;
            let cj = v_span - vp + j;
            let b = nu[0][i] * nv[0][j];
            let w = weights[ci][cj];
            hx += b * w * controls[ci][cj].x;
            hy += b * w * controls[ci][cj].y;
            hz += b * w * controls[ci][cj].z;
            hw += b * w;
        }
    }
    Pnt3::new(hx / hw, hy / hw, hz / hw)
}

fn finite_difference_derivatives(surface: &Surface, u: f64, v: f64) -> SurfaceDerivatives {
    let h = 1e-4;
    let p = surface.eval(u, v);
    let du = (surface.eval(u + h, v) - surface.eval(u - h, v)) * (1.0 / (2.0 * h));
    let dv = (surface.eval(u, v + h) - surface.eval(u, v - h)) * (1.0 / (2.0 * h));
    let duu = (surface.eval(u + h, v).to_vec() - p.to_vec() * 2.0 + surface.eval(u - h, v).to_vec()) * (1.0 / (h * h));
    let dvv = (surface.eval(u, v + h).to_vec() - p.to_vec() * 2.0 + surface.eval(u, v - h).to_vec()) * (1.0 / (h * h));
    let duv = (surface.eval(u + h, v + h).to_vec() - surface.eval(u + h, v - h).to_vec() - surface.eval(u - h, v + h).to_vec() + surface.eval(u - h, v - h).to_vec()) * (1.0 / (4.0 * h * h));
    SurfaceDerivatives { point: p, du, dv, duu, duv, dvv }
}

// #endregion 🔖️Surface

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn fd_du(s: &Surface, u: f64, v: f64) -> Vec3 {
        let h = 1e-6;
        (s.eval(u + h, v) - s.eval(u - h, v)) * (1.0 / (2.0 * h))
    }
    fn fd_dv(s: &Surface, u: f64, v: f64) -> Vec3 {
        let h = 1e-6;
        (s.eval(u, v + h) - s.eval(u, v - h)) * (1.0 / (2.0 * h))
    }

    #[test]
    fn plane_eval_and_normal() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::Z).unwrap();
        let s = Surface::Plane { frame };
        let p = s.eval(2.0, 3.0);
        assert!((p.z - 3.0).abs() < 1e-9);
        assert!((s.normal(0.0, 0.0).unwrap() - Vec3::Z).norm() < 1e-9);
        assert!(s.is_planar());
    }

    #[test]
    fn cylinder_derivatives_match_finite_differences_and_lie_on_cylinder() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cylinder { frame, radius: 2.0 };
        for (u, v) in [(0.3, 1.0), (2.0, -3.0), (5.0, 0.5)] {
            let p = s.eval(u, v);
            let local = frame.to_local(p);
            assert!((local.x * local.x + local.y * local.y).sqrt() - 2.0 < 1e-9);
            let d = s.derivatives(u, v);
            assert!((d.du - fd_du(&s, u, v)).norm() < 1e-4, "du mismatch at {u},{v}");
            assert!((d.dv - fd_dv(&s, u, v)).norm() < 1e-4, "dv mismatch at {u},{v}");
        }
    }

    #[test]
    fn cylinder_gaussian_curvature_is_zero_and_mean_curvature_is_half_reciprocal_radius() {
        let frame = Frame3::WORLD;
        let s = Surface::Cylinder { frame, radius: 3.0 };
        let (gaussian, mean) = s.curvature(0.5, 1.0).unwrap();
        assert!(gaussian.abs() < 1e-9, "cylinder must be developable (K=0), got {gaussian}");
        assert!((mean.abs() - 1.0 / (2.0 * 3.0)).abs() < 1e-6);
    }

    #[test]
    fn sphere_gaussian_curvature_equals_reciprocal_radius_squared() {
        let frame = Frame3::WORLD;
        let s = Surface::Sphere { frame, radius: 4.0 };
        for (u, v) in [(0.0, 0.0), (1.0, 0.3), (4.0, -0.5)] {
            let (gaussian, _) = s.curvature(u, v).unwrap();
            assert!((gaussian - 1.0 / 16.0).abs() < 1e-6, "mismatch at {u},{v}: {gaussian}");
        }
    }

    #[test]
    fn sphere_eval_stays_on_sphere_and_derivatives_match_finite_differences() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, -1.0, 2.0), Vec3::new(0.2, 0.3, 1.0)).unwrap();
        let s = Surface::Sphere { frame, radius: 5.0 };
        for (u, v) in [(0.2, 0.1), (3.0, -0.4), (5.5, 0.7)] {
            let p = s.eval(u, v);
            assert!((p.distance(frame.origin) - 5.0).abs() < 1e-9);
            let d = s.derivatives(u, v);
            assert!((d.du - fd_du(&s, u, v)).norm() < 1e-4);
            assert!((d.dv - fd_dv(&s, u, v)).norm() < 1e-4);
        }
    }

    #[test]
    fn torus_eval_stays_at_correct_distance_from_main_circle() {
        let frame = Frame3::WORLD;
        let s = Surface::Torus { frame, major_radius: 5.0, minor_radius: 1.5 };
        for (u, v) in [(0.0, 0.0), (1.0, 2.0), (4.0, 5.0)] {
            let p = s.eval(u, v);
            let radial = (p.x * p.x + p.y * p.y).sqrt();
            let dist_to_tube_center = ((radial - 5.0).powi(2) + p.z * p.z).sqrt();
            assert!((dist_to_tube_center - 1.5).abs() < 1e-9, "mismatch at {u},{v}: {dist_to_tube_center}");
        }
    }

    #[test]
    fn torus_derivatives_match_finite_differences() {
        let frame = Frame3::WORLD;
        let s = Surface::Torus { frame, major_radius: 4.0, minor_radius: 1.0 };
        for (u, v) in [(0.3, 0.7), (2.0, 4.0)] {
            let d = s.derivatives(u, v);
            assert!((d.du - fd_du(&s, u, v)).norm() < 1e-4);
            assert!((d.dv - fd_dv(&s, u, v)).norm() < 1e-4);
        }
    }

    #[test]
    fn cone_radius_grows_linearly_with_v_and_derivatives_match_finite_differences() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let half_angle = std::f64::consts::FRAC_PI_6;
        let s = Surface::Cone { frame, half_angle };
        for (u, v) in [(0.5, 2.0), (3.0, 5.0)] {
            let d = s.derivatives(u, v);
            assert!((d.du - fd_du(&s, u, v)).norm() < 1e-4);
            assert!((d.dv - fd_dv(&s, u, v)).norm() < 1e-4);
            let p = s.eval(u, v);
            let radial = (p.x * p.x + p.y * p.y).sqrt();
            assert!((radial - v * half_angle.tan()).abs() < 1e-9, "expected radius {} at v={v}, got {radial}", v * half_angle.tan());
        }
    }

    #[test]
    fn plane_second_derivatives_are_zero_and_gaussian_curvature_is_zero() {
        let frame = Frame3::WORLD;
        let s = Surface::Plane { frame };
        let d = s.derivatives(0.5, 0.5);
        assert_eq!(d.duu, Vec3::ZERO);
        assert_eq!(d.dvv, Vec3::ZERO);
        assert_eq!(d.duv, Vec3::ZERO);
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Surface

// #region 🔖️Surfaceops
pub mod surface_ops {
//! 🧭️ Surface algorithms that operate *on* a [`crate::surface::Surface`]: closest-point
//! projection (with closed-form fast paths for the surfaces that admit one) and Coons-patch
//! transfinite interpolation from four boundary curves. Kept separate from `surface.rs` for the
//! same reason as [`crate::curve_ops`] versus [`crate::curve`].

use crate::surface::Surface;
use crate::vec::{Pnt3, Vec3};

// #region 🔖️Project

/// 🧭️ Closest point on `surface` (restricted to `domain`) to `target`. Uses an exact closed form
/// for [`Surface::Plane`] and [`Surface::Sphere`]; otherwise coarse-grid seeding followed by a 2D
/// Newton iteration on the first-order optimality conditions `(S(u,v)-P)·Su = 0`, `(S(u,v)-P)·Sv = 0`.
/// Returns `(u, v, distance)`.
pub fn closest_point(surface: &Surface, domain: ((f64, f64), (f64, f64)), target: Pnt3, samples: usize) -> (f64, f64, f64) {
    match surface {
        Surface::Plane { frame } => {
            let local = frame.to_local(target);
            let p = surface.eval(local.x, local.y);
            (local.x, local.y, p.distance(target))
        }
        Surface::Sphere { frame, .. } => {
            let local = frame.to_local(target).to_vec();
            let n = local.normalized().unwrap_or(Vec3::Z);
            let v = n.z.clamp(-1.0, 1.0).asin();
            let u = n.y.atan2(n.x).rem_euclid(std::f64::consts::TAU);
            let p = surface.eval(u, v);
            (u, v, p.distance(target))
        }
        _ => closest_point_numeric(surface, domain, target, samples),
    }
}

/// 🧭️ Wraps into a periodic domain (mirrors [`crate::curve_ops`]'s identical fix for closed
/// curves) rather than clamping — otherwise Newton can get trapped exactly at a domain boundary
/// when the true optimum sits just across the periodic seam.
fn wrap_or_clamp(x: f64, lo: f64, hi: f64, periodic: bool) -> f64 {
    if periodic {
        let period = hi - lo;
        let mut w = (x - lo) % period;
        if w < 0.0 {
            w += period;
        }
        lo + w
    } else {
        x.clamp(lo, hi)
    }
}

fn closest_point_numeric(surface: &Surface, domain: ((f64, f64), (f64, f64)), target: Pnt3, samples: usize) -> (f64, f64, f64) {
    let (u_dom, v_dom) = domain;
    let u_periodic = surface.is_u_periodic();
    let v_periodic = surface.is_v_periodic();
    let u_hi = if u_dom.1.is_finite() { u_dom.1 } else { u_dom.0 + std::f64::consts::TAU };
    let v_hi = if v_dom.1.is_finite() { v_dom.1 } else { v_dom.0 + 10.0 };
    let v_lo = if v_dom.0.is_finite() { v_dom.0 } else { v_hi - 20.0 };
    let mut best = (u_dom.0, v_lo, f64::INFINITY);
    for i in 0..=samples {
        for j in 0..=samples {
            let u = u_dom.0 + (u_hi - u_dom.0) * (i as f64 / samples as f64);
            let v = v_lo + (v_hi - v_lo) * (j as f64 / samples as f64);
            let d = surface.eval(u, v).distance_sq(target);
            if d < best.2 {
                best = (u, v, d);
            }
        }
    }
    best.2 = best.2.sqrt();
    let (mut u, mut v, _) = best;
    for _ in 0..30 {
        let d = surface.derivatives(u, v);
        let delta = d.point - target;
        let fu = delta.dot(d.du);
        let fv = delta.dot(d.dv);
        let fuu = d.du.dot(d.du) + delta.dot(d.duu);
        let fuv = d.du.dot(d.dv) + delta.dot(d.duv);
        let fvv = d.dv.dot(d.dv) + delta.dot(d.dvv);
        let det = fuu * fvv - fuv * fuv;
        if det.abs() <= 1e-300 {
            break;
        }
        let step_u = (fu * fvv - fv * fuv) / det;
        let step_v = (fv * fuu - fu * fuv) / det;
        let next_u = wrap_or_clamp(u - step_u, u_dom.0, u_hi, u_periodic);
        let next_v = wrap_or_clamp(v - step_v, v_lo, v_hi, v_periodic);
        if (next_u - u).abs() < 1e-13 && (next_v - v).abs() < 1e-13 {
            u = next_u;
            v = next_v;
            break;
        }
        u = next_u;
        v = next_v;
    }
    let refined_dist = surface.eval(u, v).distance(target);
    if refined_dist < best.2 {
        (u, v, refined_dist)
    } else {
        best
    }
}

// #endregion 🔖️Project

// #region 🔖️Coons

/// 🧭️ Bilinear Coons-patch transfinite interpolation from four boundary curves parametrized on
/// `[0, 1]`: `c0`/`c1` are the `v=0`/`v=1` boundaries (functions of `u`), `d0`/`d1` are the `u=0`/
/// `u=1` boundaries (functions of `v`). Requires the four curves to agree at shared corners
/// (`c0(0)==d0(0)`, `c0(1)==d1(0)`, `c1(0)==d0(1)`, `c1(1)==d1(1)`) — the caller is responsible for
/// that consistency; this function does not check it.
pub fn coons_patch_eval(c0: &dyn Fn(f64) -> Pnt3, c1: &dyn Fn(f64) -> Pnt3, d0: &dyn Fn(f64) -> Pnt3, d1: &dyn Fn(f64) -> Pnt3, u: f64, v: f64) -> Pnt3 {
    let p00 = c0(0.0);
    let p10 = c0(1.0);
    let p01 = c1(0.0);
    let p11 = c1(1.0);
    let ruled_uv = c0(u).to_vec() * (1.0 - v) + c1(u).to_vec() * v;
    let ruled_vu = d0(v).to_vec() * (1.0 - u) + d1(v).to_vec() * u;
    let bilinear_corners = (p00.to_vec() * (1.0 - u) * (1.0 - v) + p10.to_vec() * u * (1.0 - v) + p01.to_vec() * (1.0 - u) * v + p11.to_vec() * u * v).to_array();
    Pnt3::from_array((ruled_uv + ruled_vu).to_array()) - Vec3::from_array(bilinear_corners)
}

// #endregion 🔖️Coons

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mat::Frame3;

    #[test]
    fn closest_point_on_plane_matches_orthogonal_projection() {
        let frame = Frame3::WORLD;
        let s = Surface::Plane { frame };
        let target = Pnt3::new(2.0, 3.0, 5.0);
        let (u, v, d) = closest_point(&s, s.domain(), target, 10);
        assert!((u - 2.0).abs() < 1e-9);
        assert!((v - 3.0).abs() < 1e-9);
        assert!((d - 5.0).abs() < 1e-9);
    }

    #[test]
    fn closest_point_on_sphere_matches_radial_projection() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, 1.0, 1.0), Vec3::Z).unwrap();
        let s = Surface::Sphere { frame, radius: 3.0 };
        let target = Pnt3::new(1.0, 1.0, 21.0); // 20 units above the sphere along its axis
        let (_, _, d) = closest_point(&s, s.domain(), target, 10);
        assert!((d - 17.0).abs() < 1e-6);
    }

    #[test]
    fn closest_point_on_cylinder_matches_expected_geometry() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cylinder { frame, radius: 2.0 };
        let target = Pnt3::new(10.0, 0.0, 5.0);
        let (u, v, d) = closest_point(&s, ((0.0, std::f64::consts::TAU), (0.0, 10.0)), target, 32);
        assert!((d - 8.0).abs() < 1e-5, "distance mismatch: {d}");
        let p = s.eval(u, v);
        assert!(p.distance(Pnt3::new(2.0, 0.0, 5.0)) < 1e-5);
    }

    #[test]
    fn coons_patch_reproduces_boundary_curves_exactly() {
        let c0 = |u: f64| Pnt3::new(u, 0.0, 0.0);
        let c1 = |u: f64| Pnt3::new(u, 1.0, u * u);
        let d0 = |v: f64| Pnt3::new(0.0, v, 0.0);
        let d1 = |v: f64| Pnt3::new(1.0, v, v);
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, t, 0.0).distance(c0(t)) < 1e-9, "v=0 boundary mismatch at u={t}");
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, t, 1.0).distance(c1(t)) < 1e-9, "v=1 boundary mismatch at u={t}");
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, 0.0, t).distance(d0(t)) < 1e-9, "u=0 boundary mismatch at v={t}");
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, 1.0, t).distance(d1(t)) < 1e-9, "u=1 boundary mismatch at v={t}");
        }
    }

    #[test]
    fn coons_patch_of_planar_boundaries_is_the_bilinear_plane() {
        let c0 = |u: f64| Pnt3::new(u, 0.0, 0.0);
        let c1 = |u: f64| Pnt3::new(u, 1.0, 0.0);
        let d0 = |v: f64| Pnt3::new(0.0, v, 0.0);
        let d1 = |v: f64| Pnt3::new(1.0, v, 0.0);
        let p = coons_patch_eval(&c0, &c1, &d0, &d1, 0.3, 0.7);
        assert!(p.distance(Pnt3::new(0.3, 0.7, 0.0)) < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn closest_point_on_cylinder_matches_brute_force_grid_oracle() {
            let mut rng = mathematical_random::Rng::from_seed(71);
            for _ in 0..50 {
                let frame = Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z)).unwrap();
                let radius = 0.5 + rng.next_f64() * 5.0;
                let s = Surface::Cylinder { frame, radius };
                let target = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let domain = ((0.0, std::f64::consts::TAU), (-15.0, 15.0));
                let (_, _, dist) = closest_point(&s, domain, target, 24);
                let mut oracle = f64::INFINITY;
                for i in 0..2000 {
                    let u = std::f64::consts::TAU * i as f64 / 2000.0;
                    for j in 0..200 {
                        let v = -15.0 + 30.0 * j as f64 / 200.0;
                        oracle = oracle.min(s.eval(u, v).distance(target));
                    }
                }
                assert!(dist <= oracle + 1e-3, "newton found {dist} worse than oracle {oracle}");
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Surfaceops

// #region 🔖️Arena
pub mod arena {
//! 🗄️ A generic generational arena `Store<T, Id>` plus the [`define_id!`] macro that stamps out
//! one typed id per topology/geometry kind. Generational `(index, generation)` ids are chosen
//! over raw indices or `Rc`/`Arc` pointers because they are: serde-friendly (a `Body` round-trips
//! to plain JSON), deterministic (iteration walks slots in index order — required for
//! byte-identical output across runs of the same operation sequence), and self-detecting of stale
//! handles (a freed-and-reused slot's old id fails `get` instead of silently aliasing new data).

// #region 🔖️Ids

/// 🗄️ The (index, generation) pair every typed id newtype wraps. Implemented by [`define_id!`].
pub trait ArenaId: Copy + Eq + std::hash::Hash + std::fmt::Debug {
    fn from_raw(index: u32, generation: u32) -> Self;
    fn raw_index(self) -> u32;
    fn raw_generation(self) -> u32;
}

/// 🗄️ Declares a `Copy + Eq + Hash + Ord + Serialize` newtype id backed by `(u32, u32)`, with a
/// human-readable `"kind-index"` `Display`/`FromStr` pair (the textual encoding boundary layers —
/// flow dictionaries, document ids — key off of, per the plan's `EntityRef` design).
#[macro_export]
macro_rules! define_id {
    ($name:ident, $tag:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            index: u32,
            generation: u32,
        }
        impl $crate::arena::ArenaId for $name {
            fn from_raw(index: u32, generation: u32) -> Self {
                $name { index, generation }
            }
            fn raw_index(self) -> u32 {
                self.index
            }
            fn raw_generation(self) -> u32 {
                self.generation
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}-{}-{}", $tag, self.index, self.generation)
            }
        }
    };
}

define_id!(VertexId, "vertex");
define_id!(EdgeId, "edge");
define_id!(CoedgeId, "coedge");
define_id!(LoopId, "loop");
define_id!(FaceId, "face");
define_id!(ShellId, "shell");
define_id!(SolidId, "solid");
define_id!(Curve3Id, "curve3");
define_id!(Curve2Id, "curve2");
define_id!(SurfaceId, "surface");

// #endregion 🔖️Ids

// #region 🔖️Store

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

/// 🗄️ A generational arena: O(1) insert/get/remove, a LIFO free list so freed slots are reused
/// deterministically (identical operation sequences reuse slots in the same order, a precondition
/// for byte-identical serialized output), and index-ordered iteration. Serde bounds are pinned to
/// `T` only — `Id` never needs to be (de)serializable itself, it only appears inside a zero-sized
/// `PhantomData` marker.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound(serialize = "T: serde::Serialize", deserialize = "T: serde::de::DeserializeOwned"))]
pub struct Store<T, Id> {
    slots: Vec<Slot<T>>,
    free: Vec<u32>,
    _marker: std::marker::PhantomData<fn() -> Id>,
}

impl<T, Id: ArenaId> Default for Store<T, Id> {
    fn default() -> Self {
        Store::new()
    }
}

impl<T, Id: ArenaId> Store<T, Id> {
    pub fn new() -> Self {
        Store { slots: Vec::new(), free: Vec::new(), _marker: std::marker::PhantomData }
    }
    pub fn insert(&mut self, value: T) -> Id {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            slot.value = Some(value);
            Id::from_raw(index, slot.generation)
        } else {
            let index = self.slots.len() as u32;
            self.slots.push(Slot { generation: 0, value: Some(value) });
            Id::from_raw(index, 0)
        }
    }
    pub fn get(&self, id: Id) -> Option<&T> {
        let slot = self.slots.get(id.raw_index() as usize)?;
        if slot.generation == id.raw_generation() {
            slot.value.as_ref()
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        let slot = self.slots.get_mut(id.raw_index() as usize)?;
        if slot.generation == id.raw_generation() {
            slot.value.as_mut()
        } else {
            None
        }
    }
    pub fn contains(&self, id: Id) -> bool {
        self.get(id).is_some()
    }
    pub fn remove(&mut self, id: Id) -> Option<T> {
        let slot = self.slots.get_mut(id.raw_index() as usize)?;
        if slot.generation != id.raw_generation() {
            return None;
        }
        let value = slot.value.take()?;
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(id.raw_index());
        Some(value)
    }
    pub fn len(&self) -> usize {
        self.slots.iter().filter(|s| s.value.is_some()).count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// 🗄️ Deterministic index-order iteration over live entries.
    pub fn iter(&self) -> impl Iterator<Item = (Id, &T)> {
        self.slots.iter().enumerate().filter_map(|(i, slot)| slot.value.as_ref().map(|v| (Id::from_raw(i as u32, slot.generation), v)))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id, &mut T)> {
        self.slots.iter_mut().enumerate().filter_map(|(i, slot)| { let gen = slot.generation; slot.value.as_mut().map(|v| (Id::from_raw(i as u32, gen), v)) })
    }
    pub fn ids(&self) -> impl Iterator<Item = Id> + '_ {
        self.iter().map(|(id, _)| id)
    }
}

// #endregion 🔖️Store

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    define_id!(TestId, "test");

    #[test]
    fn insert_and_get_round_trips() {
        let mut store: Store<i32, TestId> = Store::new();
        let id = store.insert(42);
        assert_eq!(store.get(id), Some(&42));
    }

    #[test]
    fn remove_then_get_returns_none() {
        let mut store: Store<i32, TestId> = Store::new();
        let id = store.insert(1);
        assert_eq!(store.remove(id), Some(1));
        assert_eq!(store.get(id), None);
    }

    #[test]
    fn stale_handle_after_reuse_returns_none() {
        let mut store: Store<i32, TestId> = Store::new();
        let a = store.insert(1);
        store.remove(a);
        let b = store.insert(2);
        assert_eq!(b.raw_index(), a.raw_index(), "the freed slot should be reused (LIFO free list)");
        assert_ne!(b.raw_generation(), a.raw_generation());
        assert_eq!(store.get(a), None, "the stale handle must not alias the new value");
        assert_eq!(store.get(b), Some(&2));
    }

    #[test]
    fn iteration_is_index_ordered_and_skips_removed_slots() {
        let mut store: Store<i32, TestId> = Store::new();
        let a = store.insert(10);
        let _b = store.insert(20);
        let c = store.insert(30);
        store.remove(a);
        let collected: Vec<(TestId, i32)> = store.iter().map(|(id, v)| (id, *v)).collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].1, 20);
        assert_eq!(collected[1].1, 30);
        assert!(collected[1].0.raw_index() == c.raw_index());
    }

    #[test]
    fn len_reflects_only_live_entries() {
        let mut store: Store<i32, TestId> = Store::new();
        let a = store.insert(1);
        store.insert(2);
        assert_eq!(store.len(), 2);
        store.remove(a);
        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());
    }

    #[test]
    fn display_uses_readable_tag_index_generation_format() {
        let mut store: Store<i32, TestId> = Store::new();
        let id = store.insert(1);
        assert_eq!(id.to_string(), format!("test-{}-{}", id.raw_index(), id.raw_generation()));
    }

    #[test]
    fn serde_round_trips_an_id() {
        let mut store: Store<i32, TestId> = Store::new();
        let id = store.insert(1);
        let json = serde_json::to_string(&id).unwrap();
        let back: TestId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
    }

    mod quick {
        use super::*;

        #[test]
        fn random_insert_remove_sequence_never_aliases_a_removed_id() {
            let mut rng = mathematical_random::Rng::from_seed(83);
            let mut store: Store<u64, TestId> = Store::new();
            let mut live: Vec<(TestId, u64)> = Vec::new();
            let mut removed: Vec<TestId> = Vec::new();
            for i in 0..2000u64 {
                if !live.is_empty() && rng.next_bool(0.4) {
                    let idx = rng.next_range(0, live.len() as u64) as usize;
                    let (id, _) = live.remove(idx);
                    store.remove(id);
                    removed.push(id);
                } else {
                    let id = store.insert(i);
                    live.push((id, i));
                }
            }
            for (id, value) in &live {
                assert_eq!(store.get(*id), Some(value));
            }
            for id in &removed {
                if !live.iter().any(|(lid, _)| lid == id) {
                    assert_eq!(store.get(*id), None, "removed id {id:?} must not resolve");
                }
            }
        }
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Arena

// #region 🔖️History
pub mod history {
//! 📜️ Operation provenance: a [`PersistentLabel`] assigned once at an entity's birth and never
//! reused, plus the [`OpDelta`] every mutating operation in [`crate::euler`] returns. This is what
//! lets the document layer (a later phase) build exact `generated`/`modified`/`deleted` maps for
//! `backwards()` inversion without needing to diff whole-body snapshots.

// #region 🔖️Labels

/// 📜️ A stable identity for one topological entity, assigned from a per-`Body` monotonically
/// increasing counter at birth. Unlike an arena [`crate::arena::ArenaId`] (which can be reused
/// after removal once its generation increments), a label is never reused — it survives arena
/// compaction and is the identity the document layer's persistent naming keys off of.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct PersistentLabel(pub u64);

/// 📜️ Issues fresh, never-repeating labels for one `Body`.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LabelSource {
    next: u64,
}

impl LabelSource {
    pub fn new() -> Self {
        LabelSource { next: 0 }
    }
    pub fn next_label(&mut self) -> PersistentLabel {
        let label = PersistentLabel(self.next);
        self.next += 1;
        label
    }
}

// #endregion 🔖️Labels

// #region 🔖️Delta

/// 📜️ The provenance of one mutating operation, in terms of stable [`PersistentLabel`]s rather
/// than arena ids (which can be reused after removal): every entity the operation created, every
/// entity it modified (paired with its label so the same entity's before/after states are
/// linkable), and every entity it deleted.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OpDelta {
    pub generated: Vec<PersistentLabel>,
    pub modified: Vec<PersistentLabel>,
    pub deleted: Vec<PersistentLabel>,
}

impl OpDelta {
    pub fn is_empty(&self) -> bool {
        self.generated.is_empty() && self.modified.is_empty() && self.deleted.is_empty()
    }
    pub fn merge(&mut self, other: OpDelta) {
        self.generated.extend(other.generated);
        self.modified.extend(other.modified);
        self.deleted.extend(other.deleted);
    }
}

/// 📜️ Accumulates an [`OpDelta`] as a checked editor runs; passed by every [`crate::euler`]
/// operator so no operation can forget to log what it touched. `record_deleted` and friends are
/// idempotent against duplicate reporting within one operation, since some editors touch the same
/// entity more than once (e.g. splitting an edge modifies the vertex on both sides).
#[derive(Clone, Debug, Default)]
pub struct OpRecorder {
    delta: OpDelta,
}

impl OpRecorder {
    pub fn new() -> Self {
        OpRecorder::default()
    }
    pub fn record_generated(&mut self, label: PersistentLabel) {
        if !self.delta.generated.contains(&label) {
            self.delta.generated.push(label);
        }
    }
    pub fn record_modified(&mut self, label: PersistentLabel) {
        if !self.delta.modified.contains(&label) && !self.delta.generated.contains(&label) {
            self.delta.modified.push(label);
        }
    }
    pub fn record_deleted(&mut self, label: PersistentLabel) {
        self.delta.generated.retain(|l| *l != label);
        self.delta.modified.retain(|l| *l != label);
        if !self.delta.deleted.contains(&label) {
            self.delta.deleted.push(label);
        }
    }
    pub fn into_delta(self) -> OpDelta {
        self.delta
    }
}

// #endregion 🔖️Delta

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_source_never_repeats() {
        let mut source = LabelSource::new();
        let a = source.next_label();
        let b = source.next_label();
        assert_ne!(a, b);
        assert_eq!(a.0, 0);
        assert_eq!(b.0, 1);
    }

    #[test]
    fn recorder_generated_then_deleted_cancels_out() {
        let mut rec = OpRecorder::new();
        let label = PersistentLabel(5);
        rec.record_generated(label);
        rec.record_deleted(label);
        let delta = rec.into_delta();
        assert!(delta.generated.is_empty());
        assert_eq!(delta.deleted, vec![label]);
    }

    #[test]
    fn recorder_generated_entity_is_not_also_reported_modified() {
        let mut rec = OpRecorder::new();
        let label = PersistentLabel(1);
        rec.record_generated(label);
        rec.record_modified(label);
        let delta = rec.into_delta();
        assert_eq!(delta.generated, vec![label]);
        assert!(delta.modified.is_empty());
    }

    #[test]
    fn recorder_deduplicates_repeated_reports() {
        let mut rec = OpRecorder::new();
        let label = PersistentLabel(2);
        rec.record_modified(label);
        rec.record_modified(label);
        let delta = rec.into_delta();
        assert_eq!(delta.modified.len(), 1);
    }

    #[test]
    fn op_delta_merge_concatenates_all_three_lists() {
        let mut a = OpDelta { generated: vec![PersistentLabel(1)], modified: vec![PersistentLabel(2)], deleted: vec![] };
        let b = OpDelta { generated: vec![], modified: vec![], deleted: vec![PersistentLabel(3)] };
        a.merge(b);
        assert_eq!(a.generated, vec![PersistentLabel(1)]);
        assert_eq!(a.modified, vec![PersistentLabel(2)]);
        assert_eq!(a.deleted, vec![PersistentLabel(3)]);
    }

    #[test]
    fn empty_delta_reports_is_empty() {
        assert!(OpDelta::default().is_empty());
        assert!(!OpDelta { generated: vec![PersistentLabel(0)], ..Default::default() }.is_empty());
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️History

// #region 🔖️Topo
pub mod topo {
//! 🧱️ The B-Rep topology model: `Body` owns arenas of `Vertex/Edge/Coedge/Loop/Face/Shell/Solid`
//! plus geometry pools (`Curve3`/`Curve2`/`Surface`) that entities reference by id rather than
//! owning directly — two edges produced by splitting one edge share a `Curve3Id` with different
//! parameter ranges, geometry equality is id equality, and nothing here holds an `Rc`/`Arc` or a
//! back-pointer that would break serde round-tripping or determinism. Pcurves are first-class:
//! every `Coedge` on a non-planar face carries one, per the plan's "pcurves are not optional
//! cached projections" architectural rule.

use crate::arena::{CoedgeId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, Store, SurfaceId, VertexId};
use crate::curve::{Curve2, Curve3};
use crate::history::{LabelSource, PersistentLabel};
use crate::surface::Surface;
use crate::tolerance::Tol;
use crate::vec::Pnt3;

// #region 🔖️Entities

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vertex {
    pub position: Pnt3,
    pub tol: Tol,
    pub label: PersistentLabel,
}

/// 🧱️ An edge's `curve` is shared geometry; `range` is *this edge's* portion of that curve's
/// parameter domain, so two edges split from one original edge share `curve` with disjoint ranges.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Edge {
    pub curve: Curve3Id,
    pub range: (f64, f64),
    pub v0: VertexId,
    pub v1: VertexId,
    pub tol: Tol,
    pub label: PersistentLabel,
}

/// 🧱️ One face's use of one edge within one loop. `forward` is this use's orientation relative to
/// the edge's own `v0 → v1` direction. `pcurve`/`prange` are the edge's curve reparametrized into
/// the owning face's `(u, v)` domain — `None` only ever transiently, before a producer has filled
/// it in; a face with a missing pcurve on a non-planar surface fails validation (see `validate.rs`).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Coedge {
    pub edge: EdgeId,
    pub forward: bool,
    pub pcurve: Option<Curve2Id>,
    pub prange: (f64, f64),
    pub loop_id: LoopId,
    pub next: CoedgeId,
    pub prev: CoedgeId,
}

/// 🧱️ A closed cycle of coedges bounding one region of a face (the outer boundary, or one hole).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Loop {
    pub first: CoedgeId,
    pub face: FaceId,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Face {
    pub surface: SurfaceId,
    pub outer: Option<LoopId>,
    pub inners: Vec<LoopId>,
    /// 🧱️ `true` when the face's outward normal is `-normal(surface)` (the surface's own natural
    /// normal, reversed) rather than matching it directly.
    pub flipped: bool,
    pub tol: Tol,
    pub label: PersistentLabel,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shell {
    pub faces: Vec<FaceId>,
    pub label: PersistentLabel,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Solid {
    pub outer: ShellId,
    pub inners: Vec<ShellId>,
    pub label: PersistentLabel,
}

// #endregion 🔖️Entities

// #region 🔖️Body

/// 🧱️ One B-Rep model: topology arenas + geometry pools + the label counter that stamps every
/// newly-born entity with a [`PersistentLabel`].
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Body {
    pub vertices: Store<Vertex, VertexId>,
    pub edges: Store<Edge, EdgeId>,
    pub coedges: Store<Coedge, CoedgeId>,
    pub loops: Store<Loop, LoopId>,
    pub faces: Store<Face, FaceId>,
    pub shells: Store<Shell, ShellId>,
    pub solids: Store<Solid, SolidId>,
    pub curves3: Store<Curve3, Curve3Id>,
    pub curves2: Store<Curve2, Curve2Id>,
    pub surfaces: Store<Surface, SurfaceId>,
    pub labels: LabelSource,
}

impl Body {
    pub fn new() -> Self {
        Body::default()
    }
    pub fn new_label(&mut self) -> PersistentLabel {
        self.labels.next_label()
    }
}

// #endregion 🔖️Body

// #region 🔖️Traverse

impl Body {
    /// 🧱️ Walks a loop's coedge ring starting from `Loop::first`, following `next` until it
    /// returns to the start. Panics via a debug assertion in the euler layer's invariant checks
    /// if the ring is malformed; callers here get a plain `Vec` (empty if the loop id is stale).
    pub fn loop_coedges(&self, loop_id: LoopId) -> Vec<CoedgeId> {
        let Some(lp) = self.loops.get(loop_id) else { return Vec::new() };
        let mut result = Vec::new();
        let mut current = lp.first;
        loop {
            result.push(current);
            let Some(coedge) = self.coedges.get(current) else { break };
            current = coedge.next;
            if current == lp.first {
                break;
            }
            if result.len() > self.coedges.len() {
                break; // malformed ring guard: never loop forever on corrupt data
            }
        }
        result
    }
    pub fn face_loops(&self, face_id: FaceId) -> Vec<LoopId> {
        let Some(face) = self.faces.get(face_id) else { return Vec::new() };
        let mut result: Vec<LoopId> = face.outer.into_iter().collect();
        result.extend(face.inners.iter().copied());
        result
    }
    pub fn face_coedges(&self, face_id: FaceId) -> Vec<CoedgeId> {
        self.face_loops(face_id).into_iter().flat_map(|l| self.loop_coedges(l)).collect()
    }
    pub fn shell_faces(&self, shell_id: ShellId) -> Vec<FaceId> {
        self.shells.get(shell_id).map(|s| s.faces.clone()).unwrap_or_default()
    }
    pub fn solid_shells(&self, solid_id: SolidId) -> Vec<ShellId> {
        let Some(solid) = self.solids.get(solid_id) else { return Vec::new() };
        let mut result = vec![solid.outer];
        result.extend(solid.inners.iter().copied());
        result
    }
    pub fn solid_faces(&self, solid_id: SolidId) -> Vec<FaceId> {
        self.solid_shells(solid_id).into_iter().flat_map(|s| self.shell_faces(s)).collect()
    }
    /// 🧱️ The edge's endpoint vertices in `(start, end)` order as seen through `coedge`'s own
    /// orientation (i.e. respecting `forward`, not the underlying edge's raw `v0`/`v1`).
    pub fn coedge_endpoints(&self, coedge_id: CoedgeId) -> Option<(VertexId, VertexId)> {
        let coedge = self.coedges.get(coedge_id)?;
        let edge = self.edges.get(coedge.edge)?;
        Some(if coedge.forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) })
    }
    /// 🧱️ Every vertex incident to at least one edge that references it as `v0` or `v1`.
    pub fn vertex_edges(&self, vertex_id: VertexId) -> Vec<EdgeId> {
        self.edges.iter().filter(|(_, e)| e.v0 == vertex_id || e.v1 == vertex_id).map(|(id, _)| id).collect()
    }
    /// 🧱️ Every coedge that uses `edge_id` (both orientations, both faces if the edge is shared).
    pub fn edge_coedges(&self, edge_id: EdgeId) -> Vec<CoedgeId> {
        self.coedges.iter().filter(|(_, c)| c.edge == edge_id).map(|(id, _)| id).collect()
    }
}

// #endregion 🔖️Traverse

// #region 🔖️Remap

impl Body {
    /// 🧱️ A deep copy of the entire body: every arena's entries are copied into a fresh `Body`
    /// with (generally) different arena indices, but *the same* [`PersistentLabel`]s — used
    /// wherever a caller needs an independent, mutable working copy without disturbing the
    /// original (e.g. undo snapshots, before the document layer's smarter delta-based history).
    pub fn deep_copy(&self) -> Body {
        self.clone()
    }
}

// #endregion 🔖️Remap

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::ArenaId;
    use crate::mat::Frame3;
    use crate::vec::Vec3;

    fn null_coedge() -> CoedgeId {
        ArenaId::from_raw(0, 0)
    }
    fn null_loop() -> LoopId {
        ArenaId::from_raw(0, 0)
    }
    fn null_face() -> FaceId {
        ArenaId::from_raw(0, 0)
    }

    // Small test-only builders that pre-fetch `body.new_label()` into a local before the
    // `insert(...)` call — calling `body.new_label()` inline as an argument to `body.x.insert(..)`
    // is a double mutable borrow of `body` the borrow checker rejects even though the fields are
    // disjoint (the two calls are nested, not sequential).
    fn insert_vertex(body: &mut Body, position: Pnt3) -> VertexId {
        let label = body.new_label();
        body.vertices.insert(Vertex { position, tol: Tol::DEFAULT, label })
    }
    fn insert_edge(body: &mut Body, curve: Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId) -> EdgeId {
        let label = body.new_label();
        body.edges.insert(Edge { curve, range, v0, v1, tol: Tol::DEFAULT, label })
    }
    fn insert_face(body: &mut Body, surface: SurfaceId) -> FaceId {
        let label = body.new_label();
        body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: false, tol: Tol::DEFAULT, label })
    }
    fn insert_shell(body: &mut Body, faces: Vec<FaceId>) -> ShellId {
        let label = body.new_label();
        body.shells.insert(Shell { faces, label })
    }
    fn insert_solid(body: &mut Body, outer: ShellId, inners: Vec<ShellId>) -> SolidId {
        let label = body.new_label();
        body.solids.insert(Solid { outer, inners, label })
    }

    fn make_triangle_loop(body: &mut Body, face: FaceId, positions: [Pnt3; 3]) -> LoopId {
        let vertices: Vec<VertexId> = positions.iter().map(|&p| insert_vertex(body, p)).collect();
        let curves: Vec<Curve3Id> = (0..3)
            .map(|i| {
                let a = positions[i];
                let b = positions[(i + 1) % 3];
                body.curves3.insert(Curve3::Line { origin: a, dir: b - a })
            })
            .collect();
        let edges: Vec<EdgeId> = (0..3).map(|i| insert_edge(body, curves[i], (0.0, 1.0), vertices[i], vertices[(i + 1) % 3])).collect();
        let loop_id = body.loops.insert(Loop { first: null_coedge(), face });
        let coedge_ids: Vec<CoedgeId> = edges.iter().map(|&e| body.coedges.insert(Coedge { edge: e, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() })).collect();
        for i in 0..3 {
            let coedge = body.coedges.get_mut(coedge_ids[i]).unwrap();
            coedge.next = coedge_ids[(i + 1) % 3];
            coedge.prev = coedge_ids[(i + 2) % 3];
        }
        body.loops.get_mut(loop_id).unwrap().first = coedge_ids[0];
        loop_id
    }

    #[test]
    fn loop_coedges_walks_the_full_ring_once() {
        let mut body = Body::new();
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let face = insert_face(&mut body, surface);
        let loop_id = make_triangle_loop(&mut body, face, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)]);
        let coedges = body.loop_coedges(loop_id);
        assert_eq!(coedges.len(), 3);
        assert_eq!(coedges[0], body.loops.get(loop_id).unwrap().first);
    }

    #[test]
    fn face_loops_includes_outer_and_all_inner_loops() {
        let mut body = Body::new();
        let frame = Frame3::WORLD;
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let face = insert_face(&mut body, surface);
        let outer = make_triangle_loop(&mut body, face, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(10.0, 0.0, 0.0), Pnt3::new(0.0, 10.0, 0.0)]);
        let inner = make_triangle_loop(&mut body, face, [Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(2.0, 1.0, 0.0), Pnt3::new(1.0, 2.0, 0.0)]);
        body.faces.get_mut(face).unwrap().outer = Some(outer);
        body.faces.get_mut(face).unwrap().inners = vec![inner];
        let loops = body.face_loops(face);
        assert_eq!(loops.len(), 2);
        assert!(loops.contains(&outer));
        assert!(loops.contains(&inner));
        assert_eq!(body.face_coedges(face).len(), 6);
    }

    #[test]
    fn shell_and_solid_traversal_returns_all_members() {
        let mut body = Body::new();
        let frame = Frame3::WORLD;
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let f1 = insert_face(&mut body, surface);
        let f2 = insert_face(&mut body, surface);
        let shell = insert_shell(&mut body, vec![f1, f2]);
        let inner_shell = insert_shell(&mut body, vec![]);
        let solid = insert_solid(&mut body, shell, vec![inner_shell]);
        assert_eq!(body.shell_faces(shell), vec![f1, f2]);
        assert_eq!(body.solid_shells(solid), vec![shell, inner_shell]);
        assert_eq!(body.solid_faces(solid), vec![f1, f2]);
    }

    #[test]
    fn coedge_endpoints_respects_orientation() {
        let mut body = Body::new();
        let v0 = insert_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0));
        let v1 = insert_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0));
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
        let edge = insert_edge(&mut body, curve, (0.0, 1.0), v0, v1);
        let loop_id = body.loops.insert(Loop { first: null_coedge(), face: null_face() });
        let fwd = body.coedges.insert(Coedge { edge, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() });
        let rev = body.coedges.insert(Coedge { edge, forward: false, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() });
        assert_eq!(body.coedge_endpoints(fwd), Some((v0, v1)));
        assert_eq!(body.coedge_endpoints(rev), Some((v1, v0)));
    }

    #[test]
    fn vertex_edges_and_edge_coedges_find_all_incident_entries() {
        let mut body = Body::new();
        let v0 = insert_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0));
        let v1 = insert_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0));
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
        let edge = insert_edge(&mut body, curve, (0.0, 1.0), v0, v1);
        body.coedges.insert(Coedge { edge, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: null_loop(), next: null_coedge(), prev: null_coedge() });
        body.coedges.insert(Coedge { edge, forward: false, pcurve: None, prange: (0.0, 1.0), loop_id: null_loop(), next: null_coedge(), prev: null_coedge() });
        assert_eq!(body.vertex_edges(v0), vec![edge]);
        assert_eq!(body.vertex_edges(v1), vec![edge]);
        assert_eq!(body.edge_coedges(edge).len(), 2);
    }

    #[test]
    fn serde_round_trips_a_whole_body() {
        let mut body = Body::new();
        let frame = Frame3::WORLD;
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let face = insert_face(&mut body, surface);
        make_triangle_loop(&mut body, face, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)]);
        let json = serde_json::to_string(&body).unwrap();
        let back: Body = serde_json::from_str(&json).unwrap();
        assert_eq!(back.vertices.len(), body.vertices.len());
        assert_eq!(back.edges.len(), body.edges.len());
        assert_eq!(back.faces.len(), body.faces.len());
    }

    #[test]
    fn deep_copy_produces_an_independent_body() {
        let mut body = Body::new();
        let v = insert_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0));
        let mut copy = body.deep_copy();
        copy.vertices.get_mut(v).unwrap().position = Pnt3::new(9.0, 9.0, 9.0);
        assert_ne!(body.vertices.get(v).unwrap().position, copy.vertices.get(v).unwrap().position);
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Topo

// #region 🔖️Euler
pub mod euler {
//! ✂️ Checked topology editors — the *only* functions permitted to mutate a [`crate::topo::Body`].
//! Each takes an `&mut OpRecorder` so no operation can forget to log what it created/modified/
//! deleted; assembling a body exclusively through these (never by poking a `Store` directly) is
//! what keeps "public shapes cannot exist in a partially invalid state" true by construction.

use crate::arena::{ArenaId, CoedgeId, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, SurfaceId, VertexId};
use crate::history::OpRecorder;
use crate::tolerance::Tol;
use crate::topo::{Body, Coedge, Edge, Face, Loop, Shell, Solid, Vertex};
use crate::vec::Pnt3;

// #region 🔖️Make

fn dummy_coedge() -> CoedgeId {
    ArenaId::from_raw(0, 0)
}

/// ✂️ Creates a new vertex, recording it as generated.
pub fn make_vertex(body: &mut Body, position: Pnt3, tol: Tol, rec: &mut OpRecorder) -> VertexId {
    let label = body.new_label();
    rec.record_generated(label);
    body.vertices.insert(Vertex { position, tol, label })
}

/// ✂️ Creates a new edge referencing shared curve geometry, recording it as generated.
pub fn make_edge(body: &mut Body, curve: Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId, tol: Tol, rec: &mut OpRecorder) -> EdgeId {
    let label = body.new_label();
    rec.record_generated(label);
    body.edges.insert(Edge { curve, range, v0, v1, tol, label })
}

/// ✂️ Builds a closed coedge ring from `members` (one `(edge, forward)` pair per coedge, in ring
/// order) and links it into a new [`Loop`]. Loops/coedges have no [`crate::history::PersistentLabel`]
/// of their own (they are structural, not independently document-nameable), so nothing is recorded.
pub fn make_loop(body: &mut Body, face: FaceId, members: &[(EdgeId, bool)]) -> LoopId {
    let loop_id = body.loops.insert(Loop { first: dummy_coedge(), face });
    let coedge_ids: Vec<CoedgeId> = members.iter().map(|&(edge, forward)| body.coedges.insert(Coedge { edge, forward, pcurve: None, prange: (0.0, 0.0), loop_id, next: dummy_coedge(), prev: dummy_coedge() })).collect();
    let n = coedge_ids.len();
    for i in 0..n {
        let coedge = body.coedges.get_mut(coedge_ids[i]).unwrap();
        coedge.next = coedge_ids[(i + 1) % n];
        coedge.prev = coedge_ids[(i + n - 1) % n];
    }
    body.loops.get_mut(loop_id).unwrap().first = coedge_ids[0];
    loop_id
}

/// ✂️ Creates a new face, recording it as generated.
pub fn add_face(body: &mut Body, surface: SurfaceId, outer: Option<LoopId>, inners: Vec<LoopId>, flipped: bool, tol: Tol, rec: &mut OpRecorder) -> FaceId {
    let label = body.new_label();
    rec.record_generated(label);
    body.faces.insert(Face { surface, outer, inners, flipped, tol, label })
}

/// ✂️ Creates a new shell, recording it as generated.
pub fn add_shell(body: &mut Body, faces: Vec<FaceId>, rec: &mut OpRecorder) -> ShellId {
    let label = body.new_label();
    rec.record_generated(label);
    body.shells.insert(Shell { faces, label })
}

/// ✂️ Creates a new solid, recording it as generated.
pub fn add_solid(body: &mut Body, outer: ShellId, inners: Vec<ShellId>, rec: &mut OpRecorder) -> SolidId {
    let label = body.new_label();
    rec.record_generated(label);
    body.solids.insert(Solid { outer, inners, label })
}

// #endregion 🔖️Make

// #region 🔖️SplitJoin

/// ✂️ Splits `edge_id` at curve parameter `t` (which must lie strictly within the edge's current
/// range) into two edges sharing the same underlying curve, joined by a new vertex at `position`.
/// Every coedge that used the old edge is replaced by two coedges spliced into the same ring in
/// the correct order — including the degenerate case of a single self-referential coedge (a full
/// periodic edge, e.g. a closed circle, forming a one-coedge loop). Returns
/// `(first_half, second_half, new_vertex)`, where "first"/"second" are relative to the edge's own
/// `v0 → v1` direction (not any particular coedge's orientation).
pub fn split_edge(body: &mut Body, edge_id: EdgeId, t: f64, position: Pnt3, rec: &mut OpRecorder) -> (EdgeId, EdgeId, VertexId) {
    let old_edge = body.edges.get(edge_id).expect("split_edge requires a live edge id").clone();
    debug_assert!(t > old_edge.range.0 && t < old_edge.range.1, "split parameter must lie strictly within the edge's range");
    let new_vertex = make_vertex(body, position, old_edge.tol, rec);
    let e1 = make_edge(body, old_edge.curve, (old_edge.range.0, t), old_edge.v0, new_vertex, old_edge.tol, rec);
    let e2 = make_edge(body, old_edge.curve, (t, old_edge.range.1), new_vertex, old_edge.v1, old_edge.tol, rec);
    let affected: Vec<CoedgeId> = body.edge_coedges(edge_id);
    for coedge_id in affected {
        let coedge = body.coedges.get(coedge_id).unwrap().clone();
        let (first_edge, second_edge) = if coedge.forward { (e1, e2) } else { (e2, e1) };
        let self_loop = coedge.prev == coedge_id && coedge.next == coedge_id;
        let c1 = body.coedges.insert(Coedge { edge: first_edge, forward: coedge.forward, pcurve: None, prange: (0.0, 0.0), loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
        let c2 = body.coedges.insert(Coedge { edge: second_edge, forward: coedge.forward, pcurve: None, prange: (0.0, 0.0), loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
        if self_loop {
            body.coedges.get_mut(c1).unwrap().prev = c2;
            body.coedges.get_mut(c1).unwrap().next = c2;
            body.coedges.get_mut(c2).unwrap().prev = c1;
            body.coedges.get_mut(c2).unwrap().next = c1;
        } else {
            let prev_id = coedge.prev;
            let next_id = coedge.next;
            body.coedges.get_mut(c1).unwrap().prev = prev_id;
            body.coedges.get_mut(c1).unwrap().next = c2;
            body.coedges.get_mut(c2).unwrap().prev = c1;
            body.coedges.get_mut(c2).unwrap().next = next_id;
            body.coedges.get_mut(prev_id).unwrap().next = c1;
            body.coedges.get_mut(next_id).unwrap().prev = c2;
        }
        if let Some(lp) = body.loops.get_mut(coedge.loop_id) {
            if lp.first == coedge_id {
                lp.first = c1;
            }
        }
        body.coedges.remove(coedge_id);
    }
    body.edges.remove(edge_id);
    rec.record_deleted(old_edge.label);
    (e1, e2, new_vertex)
}

// #endregion 🔖️SplitJoin

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::Curve3;
    use crate::mat::Frame3;
    use crate::surface::Surface;
    use crate::vec::Vec3;

    /// ✂️ Builds the topology of a unit tetrahedron (4 vertices, 6 edges, 4 triangular faces, 1
    /// shell, 1 solid) purely through the checked editors above — the flagship "assemble a real
    /// closed solid from scratch" gate for this phase.
    fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> SolidId {
        let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
        let vertices: Vec<VertexId> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
        let edge_pairs = [(0, 1), (1, 2), (2, 0), (0, 3), (1, 3), (2, 3)];
        let mut edges = std::collections::HashMap::new();
        for &(a, b) in &edge_pairs {
            let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
            let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
            edges.insert((a, b), edge);
            edges.insert((b, a), edge);
        }
        // Four triangular faces of a tetrahedron with vertex indices 0,1,2,3.
        let face_defs = [[0, 1, 2], [0, 3, 1], [1, 3, 2], [2, 3, 0]];
        let mut faces = Vec::new();
        for tri in face_defs {
            let normal = (positions[tri[1]] - positions[tri[0]]).cross(positions[tri[2]] - positions[tri[0]]);
            let frame = Frame3::from_normal(positions[tri[0]], normal).unwrap();
            let surface = body.surfaces.insert(Surface::Plane { frame });
            let members: Vec<(EdgeId, bool)> = (0..3)
                .map(|i| {
                    let a = tri[i];
                    let b = tri[(i + 1) % 3];
                    let edge = edges[&(a, b)];
                    let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                    (edge, forward)
                })
                .collect();
            let outer = make_loop(body, FaceId::from_raw(0, 0), &members);
            let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
            body.loops.get_mut(outer).unwrap().face = face;
            faces.push(face);
        }
        let shell = add_shell(body, faces, rec);
        add_solid(body, shell, vec![], rec)
    }

    #[test]
    fn tetrahedron_satisfies_euler_poincare_formula() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let vertex_count = body.vertices.len() as i64;
        let edge_count = body.edges.len() as i64;
        let face_count = body.solid_faces(solid).len() as i64;
        assert_eq!(vertex_count, 4);
        assert_eq!(edge_count, 6);
        assert_eq!(face_count, 4);
        assert_eq!(vertex_count - edge_count + face_count, 2, "V - E + F must equal 2 for a genus-0 closed solid");
    }

    #[test]
    fn tetrahedron_build_records_every_entity_as_generated() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        build_tetrahedron(&mut body, &mut rec);
        let delta = rec.into_delta();
        assert_eq!(delta.generated.len(), 4 + 6 + 4 + 1 + 1, "vertices + edges + faces + shell + solid");
        assert!(delta.deleted.is_empty());
    }

    #[test]
    fn each_face_loop_is_a_closed_ring_of_three_coedges() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        for face in body.solid_faces(solid) {
            let outer = body.faces.get(face).unwrap().outer.unwrap();
            assert_eq!(body.loop_coedges(outer).len(), 3);
        }
    }

    #[test]
    fn split_edge_on_a_free_edge_creates_two_edges_and_a_vertex() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let v1 = make_vertex(&mut body, Pnt3::new(4.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
        let edge = make_edge(&mut body, curve, (0.0, 4.0), v0, v1, Tol::DEFAULT, &mut rec);
        let (e1, e2, new_vertex) = split_edge(&mut body, edge, 1.5, Pnt3::new(1.5, 0.0, 0.0), &mut rec);
        assert!(!body.edges.contains(edge));
        assert_eq!(body.edges.get(e1).unwrap().v0, v0);
        assert_eq!(body.edges.get(e1).unwrap().v1, new_vertex);
        assert_eq!(body.edges.get(e2).unwrap().v0, new_vertex);
        assert_eq!(body.edges.get(e2).unwrap().v1, v1);
        assert_eq!(body.edges.get(e1).unwrap().range, (0.0, 1.5));
        assert_eq!(body.edges.get(e2).unwrap().range, (1.5, 4.0));
    }

    #[test]
    fn split_edge_within_a_loop_ring_preserves_ring_validity() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let face = body.solid_faces(solid)[0];
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        let coedges_before = body.loop_coedges(outer);
        let target_coedge = coedges_before[0];
        let target_edge = body.coedges.get(target_coedge).unwrap().edge;
        let (t0, t1) = body.edges.get(target_edge).unwrap().range;
        let mid = 0.5 * (t0 + t1);
        let curve = body.edges.get(target_edge).unwrap().curve;
        let position = body.curves3.get(curve).unwrap().eval(mid);
        split_edge(&mut body, target_edge, mid, position, &mut rec);
        let coedges_after = body.loop_coedges(outer);
        assert_eq!(coedges_after.len(), coedges_before.len() + 1, "the ring gains exactly one coedge");
        // The ring must still be a single closed cycle covering every live coedge in the loop.
        let mut seen = std::collections::HashSet::new();
        for c in &coedges_after {
            assert!(seen.insert(*c), "ring must not repeat a coedge");
        }
        for c in &coedges_after {
            let co = body.coedges.get(*c).unwrap();
            assert!(coedges_after.contains(&co.next));
            assert!(coedges_after.contains(&co.prev));
        }
    }

    #[test]
    fn split_edge_on_a_self_referential_single_coedge_loop_produces_a_valid_two_coedge_ring() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let curve = body.curves3.insert(Curve3::Circle { frame, radius: 1.0 });
        let v = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let edge = make_edge(&mut body, curve, (0.0, std::f64::consts::TAU), v, v, Tol::DEFAULT, &mut rec);
        let loop_id = make_loop(&mut body, FaceId::from_raw(0, 0), &[(edge, true)]);
        let coedges_before = body.loop_coedges(loop_id);
        assert_eq!(coedges_before.len(), 1);
        let single = coedges_before[0];
        assert_eq!(body.coedges.get(single).unwrap().next, single);
        assert_eq!(body.coedges.get(single).unwrap().prev, single);
        split_edge(&mut body, edge, std::f64::consts::PI, Pnt3::new(-1.0, 0.0, 0.0), &mut rec);
        let coedges_after = body.loop_coedges(loop_id);
        assert_eq!(coedges_after.len(), 2);
        let a = body.coedges.get(coedges_after[0]).unwrap();
        let b = body.coedges.get(coedges_after[1]).unwrap();
        assert_eq!(a.next, coedges_after[1]);
        assert_eq!(b.next, coedges_after[0]);
        assert_eq!(a.prev, coedges_after[1]);
        assert_eq!(b.prev, coedges_after[0]);
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Euler

// #region 🔖️Validate
pub mod validate {
//! 🩺️ Shape validation: everything the kernel's "never wrong, fail loud" invariant needs a way to
//! check for. Every check returns [`ValidationIssue`]s rather than a bare bool so a caller (or a
//! human) can see exactly which entity failed and why; nothing here mutates the body.

use crate::arena::ArenaId;
use crate::error::ValidationIssue;
use crate::topo::Body;

// #region 🔖️Topology

fn check_loop_rings(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (loop_id, lp) in body.loops.iter() {
        let coedges = body.loop_coedges(loop_id);
        if coedges.is_empty() {
            issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "empty-loop", message: "loop has no coedges".to_string() });
            continue;
        }
        if coedges[0] != lp.first {
            issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "broken-ring", message: "walking next from Loop::first did not return to itself — the ring is broken or too long".to_string() });
            continue;
        }
        let n = coedges.len();
        for i in 0..n {
            let Some((_, end_a)) = body.coedge_endpoints(coedges[i]) else { continue };
            let Some((start_b, _)) = body.coedge_endpoints(coedges[(i + 1) % n]) else { continue };
            if end_a != start_b {
                issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "loop-not-closed", message: format!("coedge {i} ends at a different vertex than coedge {} starts at", (i + 1) % n) });
            }
            let coedge_a = body.coedges.get(coedges[i]).unwrap();
            let coedge_b = body.coedges.get(coedges[(i + 1) % n]).unwrap();
            if coedge_a.next != coedges[(i + 1) % n] || coedge_b.prev != coedges[i] {
                issues.push(ValidationIssue { entity: format!("loop-{}", loop_id.raw_index()), code: "next-prev-mismatch", message: format!("coedge {i}'s next/prev pointers are not symmetric with its ring neighbor") });
            }
        }
    }
}

/// 🩺️ Flags edges used by more than 2 coedges — valid for future non-manifold support but worth
/// surfacing explicitly (the boolean/sewing pipeline in later phases assumes 2-manifold input
/// unless a caller has opted into non-manifold handling).
fn check_edge_valence(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (edge_id, _) in body.edges.iter() {
        let valence = body.edge_coedges(edge_id).len();
        if valence > 2 {
            issues.push(ValidationIssue { entity: format!("edge-{}", edge_id.raw_index()), code: "non-manifold-edge", message: format!("edge is used by {valence} coedges (2-manifold shapes use at most 2)") });
        }
    }
}

// #endregion 🔖️Topology

// #region 🔖️Geometry

/// 🩺️ Every vertex's tolerance must fit inside every incident edge's tolerance, and every edge's
/// inside every face whose loop uses it — the containment hierarchy from the plan's tolerance model.
fn check_tolerance_containment(body: &Body, issues: &mut Vec<ValidationIssue>) {
    for (edge_id, edge) in body.edges.iter() {
        for v in [edge.v0, edge.v1] {
            let Some(vertex) = body.vertices.get(v) else { continue };
            if let Some((finer, coarser)) = crate::tolerance::check_containment(&format!("vertex-{}", v.raw_index()), vertex.tol, &format!("edge-{}", edge_id.raw_index()), edge.tol) {
                issues.push(ValidationIssue { entity: finer.clone(), code: "tolerance-containment-violated", message: format!("{finer}'s tolerance exceeds its containing {coarser}'s") });
            }
        }
    }
    for (face_id, face) in body.faces.iter() {
        for coedge_id in body.face_coedges(face_id) {
            let Some(coedge) = body.coedges.get(coedge_id) else { continue };
            let Some(edge) = body.edges.get(coedge.edge) else { continue };
            if let Some((finer, coarser)) = crate::tolerance::check_containment(&format!("edge-{}", coedge.edge.raw_index()), edge.tol, &format!("face-{}", face_id.raw_index()), face.tol) {
                issues.push(ValidationIssue { entity: finer.clone(), code: "tolerance-containment-violated", message: format!("{finer}'s tolerance exceeds its containing {coarser}'s") });
            }
        }
    }
}

/// 🩺️ Same-parameter check: samples a coedge's pcurve against its 3D edge curve at corresponding
/// parameters (mapped linearly from the pcurve's `prange` onto the edge's `range`) and confirms
/// the face's surface, evaluated at the pcurve point, agrees with the 3D curve within the edge's
/// tolerance. Skips coedges with no pcurve (only an issue on non-planar faces, which nothing
/// before Phase 4 produces yet, so this check is dormant until surfaces with pcurves exist).
fn check_same_parameter(body: &Body, issues: &mut Vec<ValidationIssue>) {
    const SAMPLES: usize = 5;
    for (face_id, face) in body.faces.iter() {
        let Some(surface) = body.surfaces.get(face.surface) else { continue };
        for coedge_id in body.face_coedges(face_id) {
            let Some(coedge) = body.coedges.get(coedge_id) else { continue };
            let Some(pcurve_id) = coedge.pcurve else { continue };
            let Some(pcurve) = body.curves2.get(pcurve_id) else { continue };
            let Some(edge) = body.edges.get(coedge.edge) else { continue };
            let Some(curve3) = body.curves3.get(edge.curve) else { continue };
            for i in 0..=SAMPLES {
                let s = i as f64 / SAMPLES as f64;
                let p = coedge.prange.0 + (coedge.prange.1 - coedge.prange.0) * s;
                let t = edge.range.0 + (edge.range.1 - edge.range.0) * s;
                let uv = pcurve.eval(p);
                let via_surface = surface.eval(uv.x, uv.y);
                let via_curve = curve3.eval(t);
                if via_surface.distance(via_curve) > edge.tol.value() {
                    issues.push(ValidationIssue { entity: format!("coedge-{}", coedge_id.raw_index()), code: "same-parameter-violated", message: format!("pcurve and 3D curve disagree by {} at s={s} (tol {})", via_surface.distance(via_curve), edge.tol.value()) });
                }
            }
        }
    }
}

// #endregion 🔖️Geometry

// #region 🔖️Report

/// 🩺️ Runs every structural and geometric check and returns every finding.
pub fn validate_body(body: &Body) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    check_loop_rings(body, &mut issues);
    check_edge_valence(body, &mut issues);
    check_tolerance_containment(body, &mut issues);
    check_same_parameter(body, &mut issues);
    issues
}

// #endregion 🔖️Report

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::Curve3;
    use crate::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
    use crate::history::OpRecorder;
    use crate::mat::Frame3;
    use crate::surface::Surface;
    use crate::tolerance::Tol;
    use crate::vec::{Pnt3, Vec3};

    fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> crate::arena::SolidId {
        let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
        let vertices: Vec<_> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
        let edge_pairs = [(0, 1), (1, 2), (2, 0), (0, 3), (1, 3), (2, 3)];
        let mut edges = std::collections::HashMap::new();
        for &(a, b) in &edge_pairs {
            let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
            let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
            edges.insert((a, b), edge);
            edges.insert((b, a), edge);
        }
        let face_defs = [[0, 1, 2], [0, 3, 1], [1, 3, 2], [2, 3, 0]];
        let mut faces = Vec::new();
        for tri in face_defs {
            let normal = (positions[tri[1]] - positions[tri[0]]).cross(positions[tri[2]] - positions[tri[0]]);
            let frame = Frame3::from_normal(positions[tri[0]], normal).unwrap();
            let surface = body.surfaces.insert(Surface::Plane { frame });
            let members: Vec<(crate::arena::EdgeId, bool)> = (0..3)
                .map(|i| {
                    let a = tri[i];
                    let b = tri[(i + 1) % 3];
                    let edge = edges[&(a, b)];
                    let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                    (edge, forward)
                })
                .collect();
            let outer = make_loop(body, ArenaId::from_raw(0, 0), &members);
            let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
            body.loops.get_mut(outer).unwrap().face = face;
            faces.push(face);
        }
        let shell = add_shell(body, faces, rec);
        add_solid(body, shell, vec![], rec)
    }

    #[test]
    fn a_cleanly_built_tetrahedron_validates_with_no_issues() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        build_tetrahedron(&mut body, &mut rec);
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "unexpected issues on a clean solid: {issues:?}");
    }

    #[test]
    fn a_broken_ring_pointer_is_detected() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let face = body.solid_faces(solid)[0];
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        let coedges = body.loop_coedges(outer);
        // Corrupt the ring: point the first coedge's `next` at itself instead of its real neighbor.
        let first = coedges[0];
        body.coedges.get_mut(first).unwrap().next = first;
        let issues = validate_body(&body);
        assert!(issues.iter().any(|i| i.code == "broken-ring" || i.code == "next-prev-mismatch"), "expected a ring issue, got {issues:?}");
    }

    #[test]
    fn a_vertex_tolerance_exceeding_its_edge_tolerance_is_detected() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        build_tetrahedron(&mut body, &mut rec);
        let (vertex_id, _) = body.vertices.iter().next().unwrap();
        body.vertices.get_mut(vertex_id).unwrap().tol = Tol::new(10.0);
        let issues = validate_body(&body);
        assert!(issues.iter().any(|i| i.code == "tolerance-containment-violated"), "expected a tolerance issue, got {issues:?}");
    }

    #[test]
    fn a_non_manifold_edge_is_flagged() {
        // Build a free-standing edge with three coedges referencing it (impossible in a clean
        // 2-manifold build, so constructed directly).
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let v1 = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
        let edge = make_edge(&mut body, curve, (0.0, 1.0), v0, v1, Tol::DEFAULT, &mut rec);
        for _ in 0..3 {
            body.coedges.insert(crate::topo::Coedge { edge, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: ArenaId::from_raw(0, 0), next: ArenaId::from_raw(0, 0), prev: ArenaId::from_raw(0, 0) });
        }
        let issues = validate_body(&body);
        assert!(issues.iter().any(|i| i.code == "non-manifold-edge"), "expected a non-manifold-edge issue, got {issues:?}");
    }

    #[test]
    fn same_parameter_violation_is_detected_when_pcurve_disagrees_with_3d_curve() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let face = body.solid_faces(solid)[0];
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        let coedge_id = body.loop_coedges(outer)[0];
        // Attach a pcurve that does NOT correspond to the face's surface at all — a constant,
        // clearly-wrong 2D point far from where the 3D edge actually projects.
        let bad_pcurve = body.curves2.insert(crate::curve::Curve2::Line { origin: crate::vec::Pnt2::new(500.0, 500.0), dir: crate::vec::Vec2::new(0.0, 0.0) });
        let coedge = body.coedges.get_mut(coedge_id).unwrap();
        coedge.pcurve = Some(bad_pcurve);
        coedge.prange = (0.0, 1.0);
        let issues = validate_body(&body);
        assert!(issues.iter().any(|i| i.code == "same-parameter-violated"), "expected a same-parameter issue, got {issues:?}");
    }
}
// #endregion 🔖️Tests
}
// #endregion 🔖️Validate

// #endregion 🔖️NativeModules

use std::f64::consts::TAU;

use async_trait::async_trait;
use brepkit_geometry::convert::{circle_to_nurbs, ellipse_to_nurbs, line_to_nurbs};
use brepkit_geometry::sampling::{sample_deflection, surface_grid};
use brepkit_io::stl::import_mesh;
use brepkit_math::curves::{Circle3D, Ellipse3D, Line3D};
use brepkit_math::frame::Frame3;
use brepkit_math::mat::Mat4;
use brepkit_math::nurbs::bezier_clip::curve_curve_intersect;
use brepkit_math::nurbs::curve::NurbsCurve;
use brepkit_math::nurbs::fitting::{approximate, interpolate};
use brepkit_math::nurbs::intersection::{intersect_curve_surface, intersect_nurbs_nurbs};
use brepkit_math::nurbs::surface::NurbsSurface;
use brepkit_math::nurbs::surface_fitting::interpolate_surface;
use brepkit_math::surfaces::{ConicalSurface, CylindricalSurface, SphericalSurface, ToroidalSurface};
use brepkit_math::vec::{Point3, Vec3 as BkVec3};
use brepkit_operations::blend_ops::fillet_v2;
use brepkit_operations::boolean::{boolean, compound_cut, BooleanOp};
use brepkit_operations::chamfer::{chamfer, chamfer_asymmetric};
use brepkit_operations::copy::copy_solid;
use brepkit_operations::defeature::defeature;
use brepkit_operations::draft::draft;
use brepkit_operations::extrude::extrude;
use brepkit_operations::fill_face::fill_coons_patch;
use brepkit_operations::fillet::{fillet_variable, FilletRadiusLaw};
use brepkit_operations::helix::{helical_sweep, make_helix_curve};
use brepkit_operations::loft::{loft, loft_smooth};
use brepkit_operations::measure;
use brepkit_operations::mesh_boolean::mesh_boolean;
use brepkit_operations::mirror::mirror;
use brepkit_operations::offset_face::offset_face;
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::pattern::{circular_pattern, grid_pattern, linear_pattern};
use brepkit_operations::pipe::pipe;
use brepkit_operations::primitives::{make_box, make_cone, make_convex_hull, make_cylinder, make_sphere, make_torus};
use brepkit_operations::revolve::revolve;
use brepkit_operations::section::section;
use brepkit_operations::sew::sew_faces;
use brepkit_operations::shell_op::shell;
use brepkit_operations::split::split;
use brepkit_operations::sweep::sweep;
use brepkit_operations::tessellate::{sample_solid_edges, tessellate_solid_with_tolerance, tessellate_with_tolerance};
use brepkit_operations::thicken::thicken;
use brepkit_operations::transform::transform_solid;
use brepkit_topology::builder;
use brepkit_topology::compound::CompoundId;
use brepkit_topology::edge::{Edge, EdgeCurve, EdgeId};
use brepkit_topology::explorer;
use brepkit_topology::face::{FaceId, FaceSurface};
use brepkit_topology::solid::SolidId;
use brepkit_topology::vertex::{Vertex, VertexId};
use brepkit_topology::wire::{OrientedEdge, Wire, WireId};
use brepkit_topology::{Topology, TopologyError};
use kernel_3d_engine::{BrepError, BrepKernel, BrepTopology, ClosestPoint, FaceGroup, GeometryHandle, GeometryKind, MeshTransfer, ParamDomain, PointClassification, Vec3};
use rayon::prelude::*;
use semio_framework_core::{MeshExporter, MeshImporter};

// #region Helpers
const TOL: f64 = 1e-6;

fn p3(v: Vec3) -> Point3 {
    Point3::new(v[0], v[1], v[2])
}

fn from_p3(p: Point3) -> Vec3 {
    [p.x(), p.y(), p.z()]
}

fn v3(v: Vec3) -> BkVec3 {
    BkVec3::new(v[0], v[1], v[2])
}

fn from_v3(v: BkVec3) -> Vec3 {
    [v.x(), v.y(), v.z()]
}
// #endregion Helpers

// #region 🔖️Registry
enum KernelCurve {
    Line(Line3D, f64),
    Circle(Circle3D, f64, f64),
    Ellipse(Ellipse3D, f64, f64),
    Nurbs(NurbsCurve),
}

enum KernelSurface {
    Plane { origin: Point3, normal: BkVec3 },
    Cylinder(CylindricalSurface),
    Cone(ConicalSurface),
    Sphere(SphericalSurface),
    Torus(ToroidalSurface),
    Nurbs(NurbsSurface),
}

enum Entity {
    Vertex(VertexId),
    Edge(EdgeId),
    Wire(WireId),
    Face(FaceId),
    Solid(SolidId),
    Compound(CompoundId),
    Curve(KernelCurve),
    Surface(KernelSurface),
}

struct Entry {
    kind: GeometryKind,
    entity: Entity,
}

pub struct BrepkitKernel {
    topo: Topology,
    seq: u32,
    registry: std::collections::HashMap<String, Entry>,
    /// 🐌️➡️⚡️ Coarse-tessellation cache for [`Self::boolean_mesh_sync`]'s torus fallback, keyed by
    /// `(SolidId, deflection_bits)` — repeated booleans against the same static operand (the
    /// slider-drag motivating case) skip re-tessellating that operand every call. Invalidated by
    /// [`Self::invalidate_solid_derived_caches`] wherever a `SolidId` is mutated in place.
    mesh_boolean_cache: std::collections::HashMap<(SolidId, u64), brepkit_operations::tessellate::TriangleMesh>,
}

impl Default for BrepkitKernel {
    fn default() -> Self {
        Self::new()
    }
}

impl BrepkitKernel {
    pub fn new() -> Self {
        Self { topo: Topology::new(), seq: 0, registry: std::collections::HashMap::new(), mesh_boolean_cache: std::collections::HashMap::new() }
    }

    /// 🧹️ Evicts derived-data caches for a `SolidId` that's about to be mutated in place
    /// (`translate`/`rotate`/`scale`/`heal_solid`/`convert_to_nurbs` reuse the same `SolidId`
    /// rather than registering a fresh one, unlike every other mutating operation).
    fn invalidate_solid_derived_caches(&mut self, solid: SolidId) {
        self.mesh_boolean_cache.retain(|(id, _), _| *id != solid);
    }

    /// ⚡️ Tessellates a solid at `deflection`, reusing a cached mesh when available.
    fn cached_tessellate_solid(&mut self, solid: SolidId, deflection: f64) -> Result<brepkit_operations::tessellate::TriangleMesh, BrepError> {
        let key = (solid, deflection.to_bits());
        if let Some(mesh) = self.mesh_boolean_cache.get(&key) {
            return Ok(mesh.clone());
        }
        let mesh = tessellate_solid_with_tolerance(&self.topo, solid, deflection, 0.2).map_err(Self::map_err)?;
        self.mesh_boolean_cache.insert(key, mesh.clone());
        Ok(mesh)
    }

    fn register_entity(&mut self, kind: GeometryKind, entity: Entity) -> GeometryHandle {
        self.seq += 1;
        let handle = GeometryHandle::new(kind, self.seq);
        self.registry.insert(handle.as_str().to_string(), Entry { kind, entity });
        handle
    }

    fn register_solid(&mut self, solid: SolidId) -> GeometryHandle {
        self.register_entity(GeometryKind::Solid, Entity::Solid(solid))
    }

    fn entry(&self, handle: &GeometryHandle) -> Result<&Entry, BrepError> {
        self.registry.get(handle.as_str()).ok_or_else(|| BrepError::MissingHandle(handle.as_str().to_string()))
    }

    fn solid_id(&self, handle: &GeometryHandle) -> Result<SolidId, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Solid(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a solid", handle.as_str()))),
        }
    }

    fn face_id(&self, handle: &GeometryHandle) -> Result<FaceId, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Face(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a face", handle.as_str()))),
        }
    }

    fn edge_id(&self, handle: &GeometryHandle) -> Result<EdgeId, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Edge(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not an edge", handle.as_str()))),
        }
    }

    fn wire_id(&self, handle: &GeometryHandle) -> Result<WireId, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Wire(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a wire", handle.as_str()))),
        }
    }

    fn solid_ids_from_handle(&self, handle: &GeometryHandle) -> Result<Vec<SolidId>, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Solid(id) => Ok(vec![*id]),
            Entity::Compound(id) => Ok(self.topo.compound(*id).map_err(Self::map_topo_err)?.solids().to_vec()),
            _ => Err(BrepError::InvalidInput(format!("{} is not a solid or compound", handle.as_str()))),
        }
    }

    fn map_err(error: brepkit_operations::OperationsError) -> BrepError {
        BrepError::Operation(error.to_string())
    }

    fn map_topo_err(error: TopologyError) -> BrepError {
        BrepError::Operation(error.to_string())
    }

    fn map_io_err(error: brepkit_io::IoError) -> BrepError {
        BrepError::Operation(error.to_string())
    }

    fn rotation_axis_matrix(axis: Vec3, angle: f64) -> Result<Mat4, BrepError> {
        let len = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        if len < 1e-12 {
            return Err(BrepError::InvalidInput("zero rotation axis".into()));
        }
        let (x, y, z) = (axis[0] / len, axis[1] / len, axis[2] / len);
        let (s, c) = angle.sin_cos();
        let one_c = 1.0 - c;
        Ok(Mat4([
            [one_c * x * x + c, one_c * x * y - s * z, one_c * x * z + s * y, 0.0],
            [one_c * x * y + s * z, one_c * y * y + c, one_c * y * z - s * x, 0.0],
            [one_c * x * z - s * y, one_c * y * z + s * x, one_c * z * z + c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    fn solid_has_torus_surface(&self, solid: SolidId) -> bool {
        explorer::solid_faces(&self.topo, solid).map(|faces| faces.into_iter().any(|face_id| self.topo.face(face_id).ok().is_some_and(|face| matches!(face.surface(), FaceSurface::Torus(_))))).unwrap_or(false)
    }

    fn solid_bounds_overlap(&self, a: SolidId, b: SolidId) -> bool {
        let Some(aabb_a) = measure::solid_bounding_box(&self.topo, a).ok() else {
            return true;
        };
        let Some(aabb_b) = measure::solid_bounding_box(&self.topo, b).ok() else {
            return true;
        };
        let margin = brepkit_math::tolerance::Tolerance::new().linear;
        aabb_a.min.x() <= aabb_b.max.x() + margin
            && aabb_a.max.x() + margin >= aabb_b.min.x()
            && aabb_a.min.y() <= aabb_b.max.y() + margin
            && aabb_a.max.y() + margin >= aabb_b.min.y()
            && aabb_a.min.z() <= aabb_b.max.z() + margin
            && aabb_a.max.z() + margin >= aabb_b.min.z()
    }

    fn boolean_mesh_sync(&mut self, operator: BooleanOp, a: SolidId, b: SolidId) -> Result<SolidId, BrepError> {
        // 🐌️ Coarser than the default render deflection on purpose: this only feeds the
        // triangle-triangle boolean, not the final mesh, and a finer value multiplies the
        // CDT/mesh-boolean triangle count enough to turn torus-involving cuts into a
        // multi-second (wasm: ~20s) synchronous stall on the caller's thread.
        let deflection = 0.1;
        let tol = brepkit_math::tolerance::Tolerance::new();
        let mesh_a = self.cached_tessellate_solid(a, deflection)?;
        let mesh_b = self.cached_tessellate_solid(b, deflection)?;
        let mb = match mesh_boolean(&mesh_a, &mesh_b, operator, tol.linear) {
            Ok(result) => result,
            Err(brepkit_operations::OperationsError::EmptyResult { .. }) if operator == BooleanOp::Intersect => {
                return Ok(self.topo.add_empty_solid());
            }
            Err(error) => return Err(Self::map_err(error)),
        };
        import_mesh(&mut self.topo, &mb.mesh, tol.linear).map_err(Self::map_io_err)
    }

    fn boolean_sync(&mut self, operator: BooleanOp, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let a_id = self.solid_id(a)?;
        let b_id = self.solid_id(b)?;
        let torus_involved = self.solid_has_torus_surface(a_id) || self.solid_has_torus_surface(b_id);
        let use_mesh = torus_involved && self.solid_bounds_overlap(a_id, b_id);
        let solid = if use_mesh { self.boolean_mesh_sync(operator, a_id, b_id)? } else { boolean(&mut self.topo, operator, a_id, b_id).map_err(Self::map_err)? };
        Ok(self.register_solid(solid))
    }

    fn edge_lines_flat(edges: &brepkit_operations::tessellate::EdgeLines) -> Vec<f32> {
        let mut flat = Vec::new();
        for index in 0..edges.offsets.len() {
            let start = edges.offsets[index];
            let end = edges.offsets.get(index + 1).copied().unwrap_or(edges.positions.len());
            let segment = &edges.positions[start..end];
            for pair in segment.windows(2) {
                let a = &pair[0];
                let b = &pair[1];
                flat.extend([a.x() as f32, a.y() as f32, a.z() as f32, b.x() as f32, b.y() as f32, b.z() as f32]);
            }
        }
        flat
    }

    fn sample_oriented_edge_lines(&self, edge: EdgeId, tol: f64) -> Result<Vec<f32>, BrepError> {
        let edge_data = self.topo.edge(edge).map_err(Self::map_topo_err)?;
        let start = self.topo.vertex(edge_data.start()).map_err(Self::map_topo_err)?.point();
        let end = self.topo.vertex(edge_data.end()).map_err(Self::map_topo_err)?.point();
        let delta = end - start;
        if delta.x() * delta.x() + delta.y() * delta.y() + delta.z() * delta.z() < 1e-18 {
            return Ok(Vec::new());
        }
        let nurbs = self.edge_to_nurbs(edge)?;
        let (a, b) = nurbs.domain();
        let samples = sample_deflection(&nurbs, a, b, tol);
        let mut edges = Vec::new();
        for pair in samples.windows(2) {
            let p0 = pair[0].1;
            let p1 = pair[1].1;
            edges.extend([p0.x() as f32, p0.y() as f32, p0.z() as f32, p1.x() as f32, p1.y() as f32, p1.z() as f32]);
        }
        Ok(edges)
    }

    fn sample_face_boundary_edge_lines(&self, face: FaceId, tol: f64) -> Result<Vec<f32>, BrepError> {
        let face_data = self.topo.face(face).map_err(Self::map_topo_err)?;
        let wire = self.topo.wire(face_data.outer_wire()).map_err(Self::map_topo_err)?;
        let mut edges = Vec::new();
        for oriented_edge in wire.edges() {
            edges.extend(self.sample_oriented_edge_lines(oriented_edge.edge(), tol)?);
        }
        Ok(edges)
    }

    fn curve_domain_inner(curve: &KernelCurve) -> ParamDomain {
        match curve {
            KernelCurve::Line(_, len) => ParamDomain { min: 0.0, max: *len },
            KernelCurve::Circle(_, a, b) | KernelCurve::Ellipse(_, a, b) => ParamDomain { min: *a, max: *b },
            KernelCurve::Nurbs(c) => {
                let (a, b) = c.domain();
                ParamDomain { min: a, max: b }
            }
        }
    }

    fn curve_evaluate(curve: &KernelCurve, t: f64) -> Point3 {
        match curve {
            KernelCurve::Line(line, _) => {
                let d = line.direction();
                let o = line.origin();
                Point3::new(o.x() + d.x() * t, o.y() + d.y() * t, o.z() + d.z() * t)
            }
            KernelCurve::Circle(c, _, _) => c.evaluate(t),
            KernelCurve::Ellipse(e, _, _) => e.evaluate(t),
            KernelCurve::Nurbs(c) => c.evaluate(t),
        }
    }

    fn curve_tangent_inner(curve: &KernelCurve, t: f64) -> BkVec3 {
        match curve {
            KernelCurve::Line(line, _) => line.tangent(),
            KernelCurve::Circle(c, _, _) => c.tangent(t),
            KernelCurve::Ellipse(e, _, _) => e.tangent(t),
            KernelCurve::Nurbs(c) => {
                let d = c.derivatives(t, 1);
                if d.len() > 1 {
                    d[1]
                } else {
                    BkVec3::new(1.0, 0.0, 0.0)
                }
            }
        }
    }

    fn curve_curvature_inner(curve: &KernelCurve, t: f64) -> f64 {
        match curve {
            KernelCurve::Line(_, _) => 0.0,
            KernelCurve::Circle(c, _, _) => 1.0 / c.radius(),
            KernelCurve::Ellipse(e, a, b) => {
                if let Ok(nurbs) = ellipse_to_nurbs(e, *a, *b) {
                    let d = nurbs.derivatives(t, 2);
                    if d.len() < 2 {
                        0.0
                    } else {
                        let tan = d[1];
                        let tan_len = tan.length();
                        if tan_len < 1e-15 {
                            0.0
                        } else {
                            let d2 = if d.len() > 2 { d[2] } else { BkVec3::new(0.0, 0.0, 0.0) };
                            tan.cross(d2).length() / tan_len.powi(3)
                        }
                    }
                } else {
                    0.0
                }
            }
            KernelCurve::Nurbs(c) => {
                let d = c.derivatives(t, 2);
                if d.len() < 2 {
                    return 0.0;
                }
                let tan = d[1];
                let tan_len = tan.length();
                if tan_len < 1e-15 {
                    return 0.0;
                }
                let d2 = if d.len() > 2 { d[2] } else { BkVec3::new(0.0, 0.0, 0.0) };
                tan.cross(d2).length() / tan_len.powi(3)
            }
        }
    }

    fn curve_to_nurbs(curve: &KernelCurve) -> Result<NurbsCurve, BrepError> {
        match curve {
            KernelCurve::Line(line, len) => {
                let end = line.evaluate(*len);
                line_to_nurbs(line.origin(), end).map_err(|e| BrepError::Operation(e.to_string()))
            }
            KernelCurve::Circle(c, a, b) => circle_to_nurbs(c, *a, *b).map_err(|e| BrepError::Operation(e.to_string())),
            KernelCurve::Ellipse(e, a, b) => ellipse_to_nurbs(e, *a, *b).map_err(|e| BrepError::Operation(e.to_string())),
            KernelCurve::Nurbs(c) => Ok(c.clone()),
        }
    }

    fn edge_to_nurbs(&self, edge: EdgeId) -> Result<NurbsCurve, BrepError> {
        let edge_data = self.topo.edge(edge).map_err(Self::map_topo_err)?;
        let start_pt = self.topo.vertex(edge_data.start()).map_err(Self::map_topo_err)?.point();
        let end_pt = self.topo.vertex(edge_data.end()).map_err(Self::map_topo_err)?.point();
        match edge_data.curve() {
            EdgeCurve::NurbsCurve(c) => Ok(c.clone()),
            EdgeCurve::Line => line_to_nurbs(start_pt, end_pt).map_err(|e| BrepError::Operation(e.to_string())),
            EdgeCurve::Circle(c) => {
                let (a, b) = if edge_data.start() == edge_data.end() {
                    (0.0, TAU)
                } else {
                    let ts = c.project(start_pt);
                    let mut te = c.project(end_pt);
                    if te <= ts {
                        te += TAU;
                    }
                    (ts, te)
                };
                circle_to_nurbs(c, a, b).map_err(|e| BrepError::Operation(e.to_string()))
            }
            EdgeCurve::Ellipse(e) => {
                let (a, b) = if edge_data.start() == edge_data.end() {
                    (0.0, TAU)
                } else {
                    let ts = e.project(start_pt);
                    let mut te = e.project(end_pt);
                    if te <= ts {
                        te += TAU;
                    }
                    (ts, te)
                };
                ellipse_to_nurbs(e, a, b).map_err(|e| BrepError::Operation(e.to_string()))
            }
        }
    }

    fn surface_to_nurbs(surface: &KernelSurface) -> Result<NurbsSurface, BrepError> {
        match surface {
            KernelSurface::Nurbs(s) => Ok(s.clone()),
            KernelSurface::Plane { origin, normal } => {
                let frame = Frame3::from_normal(*origin, *normal).map_err(|e| BrepError::Operation(e.to_string()))?;
                let u = frame.x;
                let v = frame.y;
                let grid = vec![vec![*origin, *origin + u, *origin + u + v, *origin + v], vec![*origin + u, *origin + u * 2.0, *origin + u * 2.0 + v, *origin + u + v]];
                interpolate_surface(&grid, 1, 1).map_err(|e| BrepError::Operation(e.to_string()))
            }
            KernelSurface::Cylinder(c) => c.to_nurbs(0.0, TAU).map_err(|e| BrepError::Operation(e.to_string())),
            KernelSurface::Cone(c) => c.to_nurbs(0.0, TAU).map_err(|e| BrepError::Operation(e.to_string())),
            KernelSurface::Sphere(s) => s.to_nurbs().map_err(|e| BrepError::Operation(e.to_string())),
            KernelSurface::Torus(t) => t.to_nurbs().map_err(|e| BrepError::Operation(e.to_string())),
        }
    }

    fn parse_points(points: &[Vec3]) -> Result<Vec<Point3>, BrepError> {
        Ok(points.iter().map(|p| p3(*p)).collect())
    }

    fn make_planar_face_points(&mut self, points: &[Point3]) -> Result<FaceId, BrepError> {
        builder::make_planar_face(&mut self.topo, points, TOL).map_err(Self::map_topo_err)
    }
}
// #endregion 🔖️Registry

impl BrepkitKernel {
    pub fn box_prim_sync(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_box(&mut self.topo, width, depth, height).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn sphere_prim_sync(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_sphere(&mut self.topo, radius, 24).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn cylinder_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_cylinder(&mut self.topo, radius, height).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn cone_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_cone(&mut self.topo, radius, 0.0, height).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn torus_prim_sync(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_torus(&mut self.topo, major, minor, 24).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn convex_hull_sync(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        if pts.len() < 4 {
            return Err(BrepError::InvalidInput("convex hull needs at least 4 points".into()));
        }
        let solid = make_convex_hull(&mut self.topo, &pts).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn line_curve_sync(&mut self, start: Vec3, end: Vec3) -> Result<GeometryHandle, BrepError> {
        let a = p3(start);
        let b = p3(end);
        let dir = b - a;
        let len = dir.length();
        if len < 1e-12 {
            return Err(BrepError::InvalidInput("coincident line endpoints".into()));
        }
        let line = Line3D::new(a, dir).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Line(line, len))))
    }

    pub fn circle_curve_sync(&mut self, center: Vec3, normal: Vec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        let circle = Circle3D::new(p3(center), v3(normal), radius).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Circle(circle, 0.0, TAU))))
    }

    pub fn arc_curve_sync(&mut self, center: Vec3, normal: Vec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        let circle = Circle3D::new(p3(center), v3(normal), radius).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Circle(circle, start_angle, end_angle))))
    }

    pub fn ellipse_curve_sync(&mut self, center: Vec3, normal: Vec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        let ellipse = Ellipse3D::new(p3(center), v3(normal), semi_major, semi_minor).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Ellipse(ellipse, 0.0, TAU))))
    }

    pub fn polyline_wire_sync(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        if pts.len() < 2 {
            return Err(BrepError::InvalidInput("polyline needs at least 2 points".into()));
        }
        let n = pts.len();
        let verts: Vec<VertexId> = pts.iter().map(|p| self.topo.add_vertex(Vertex::new(*p, TOL))).collect();
        let edges: Vec<EdgeId> = (0..n - 1).map(|i| self.topo.add_edge(Edge::new(verts[i], verts[i + 1], EdgeCurve::Line))).collect();
        let oriented: Vec<OrientedEdge> = edges.iter().map(|&e| OrientedEdge::new(e, true)).collect();
        let wire = Wire::new(oriented, false).map_err(Self::map_topo_err)?;
        let wid = self.topo.add_wire(wire);
        Ok(self.register_entity(GeometryKind::Wire, Entity::Wire(wid)))
    }

    pub fn rectangle_wire_sync(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let hw = width / 2.0;
        let hh = height / 2.0;
        let pts = [Point3::new(-hw, -hh, 0.0), Point3::new(hw, -hh, 0.0), Point3::new(hw, hh, 0.0), Point3::new(-hw, hh, 0.0)];
        let wire = builder::make_polygon_wire(&mut self.topo, &pts, TOL).map_err(Self::map_topo_err)?;
        Ok(self.register_entity(GeometryKind::Wire, Entity::Wire(wire)))
    }

    pub fn regular_polygon_wire_sync(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        if sides < 3 {
            return Err(BrepError::InvalidInput("polygon needs at least 3 sides".into()));
        }
        let wire = builder::make_regular_polygon_wire(&mut self.topo, radius, sides, TOL).map_err(Self::map_topo_err)?;
        Ok(self.register_entity(GeometryKind::Wire, Entity::Wire(wire)))
    }

    pub fn interpolate_curve_sync(&mut self, points: &[Vec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        if pts.len() < 2 {
            return Err(BrepError::InvalidInput("interpolate needs at least 2 points".into()));
        }
        let curve = interpolate(&pts, degree).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Nurbs(curve))))
    }

    pub fn approximate_curve_sync(&mut self, points: &[Vec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        let curve = approximate(&pts, degree, control_points).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Nurbs(curve))))
    }

    pub fn helix_curve_sync(&mut self, origin: Vec3, axis: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let curve = make_helix_curve(p3(origin), v3(axis), radius, pitch, turns, 8).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Nurbs(curve))))
    }

    pub fn plane_surface_sync(&mut self, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError> {
        Ok(self.register_entity(GeometryKind::Surface, Entity::Surface(KernelSurface::Plane { origin: p3(origin), normal: v3(normal) })))
    }

    pub fn planar_face_from_points_sync(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        if pts.len() < 3 {
            return Err(BrepError::InvalidInput("planar face needs at least 3 points".into()));
        }
        let face = self.make_planar_face_points(&pts)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn planar_face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let wire_id = self.wire_id(wire)?;
        let face = builder::make_planar_face_from_wire(&mut self.topo, wire_id).map_err(Self::map_topo_err)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn nurbs_surface_from_grid_sync(&mut self, points: &[Vec<Vec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        let grid: Vec<Vec<Point3>> = points.iter().map(|row| row.iter().map(|p| p3(*p)).collect()).collect();
        let surface = interpolate_surface(&grid, degree_u, degree_v).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Surface, Entity::Surface(KernelSurface::Nurbs(surface))))
    }

    pub fn coons_patch_sync(&mut self, curves: &[Vec<Vec3>]) -> Result<GeometryHandle, BrepError> {
        if curves.len() < 4 {
            return Err(BrepError::InvalidInput("coons patch needs 4 boundary curves".into()));
        }
        let polylines: Vec<Vec<Point3>> = curves.iter().map(|c| Self::parse_points(c)).collect::<Result<_, _>>()?;
        let face = fill_coons_patch(&mut self.topo, &polylines).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn offset_face_sync(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(face)?;
        let face = offset_face(&mut self.topo, face_id, distance, 16).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn thicken_face_sync(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(face)?;
        let solid = thicken(&mut self.topo, face_id, thickness).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn extrude_wire_sync(&mut self, wire: &GeometryHandle, vector: Vec3) -> Result<GeometryHandle, BrepError> {
        let distance = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
        if distance < 1e-12 {
            return Err(BrepError::InvalidInput("extrude vector magnitude must be positive".into()));
        }
        let direction = [vector[0] / distance, vector[1] / distance, vector[2] / distance];
        let face = self.planar_face_from_wire_sync(wire)?;
        self.extrude_sync(&face, direction, distance)
    }

    pub fn extrude_sync(&mut self, face: &GeometryHandle, direction: Vec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(face)?;
        let solid = extrude(&mut self.topo, face_id, v3(direction), distance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn revolve_sync(&mut self, face: &GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(face)?;
        let solid = revolve(&mut self.topo, face_id, p3(axis_origin), v3(axis_direction), angle).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn loft_sync(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        let face_ids: Vec<FaceId> = profiles.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = if smooth { loft_smooth(&mut self.topo, &face_ids).map_err(Self::map_err)? } else { loft(&mut self.topo, &face_ids).map_err(Self::map_err)? };
        Ok(self.register_solid(solid))
    }

    pub fn sweep_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(profile)?;
        let nurbs = match &self.entry(path)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            Entity::Wire(w) => {
                let wire = self.topo.wire(*w).map_err(Self::map_topo_err)?;
                let mut points = Vec::new();
                for oe in wire.edges() {
                    let edge = self.topo.edge(oe.edge()).map_err(Self::map_topo_err)?;
                    points.push(self.topo.vertex(edge.start()).map_err(Self::map_topo_err)?.point());
                    points.push(self.topo.vertex(edge.end()).map_err(Self::map_topo_err)?.point());
                }
                interpolate(&points, 3).map_err(|e| BrepError::Operation(e.to_string()))?
            }
            _ => return Err(BrepError::InvalidInput("sweep path must be curve, edge, or wire".into())),
        };
        let solid = sweep(&mut self.topo, face_id, &nurbs).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn pipe_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(profile)?;
        let path_curve = match &self.entry(path)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            _ => return Err(BrepError::InvalidInput("pipe path must be curve or edge".into())),
        };
        let guide_curve = if let Some(g) = guide {
            match &self.entry(g)?.entity {
                Entity::Curve(c) => Some(Self::curve_to_nurbs(c)?),
                Entity::Edge(e) => Some(self.edge_to_nurbs(*e)?),
                _ => None,
            }
        } else {
            None
        };
        let solid = pipe(&mut self.topo, face_id, &path_curve, guide_curve.as_ref()).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn helical_sweep_sync(&mut self, profile: &GeometryHandle, axis_origin: Vec3, axis_dir: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(profile)?;
        let solid = helical_sweep(&mut self.topo, face_id, p3(axis_origin), v3(axis_dir), radius, pitch, turns, 8).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn fuse_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.boolean_sync(BooleanOp::Fuse, a, b)
    }

    pub fn cut_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.boolean_sync(BooleanOp::Cut, a, b)
    }

    pub fn intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.boolean_sync(BooleanOp::Intersect, a, b)
    }

    pub fn compound_cut_sync(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let target_id = self.solid_id(target)?;
        let tool_ids: Vec<SolidId> = tools.iter().map(|h| self.solid_id(h)).collect::<Result<_, _>>()?;
        let solid = compound_cut(&mut self.topo, target_id, &tool_ids, brepkit_operations::boolean::BooleanOptions::default()).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn translate_sync(&mut self, shape: &GeometryHandle, offset: Vec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        transform_solid(&mut self.topo, solid, &Mat4::translation(offset[0], offset[1], offset[2])).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn rotate_sync(&mut self, shape: &GeometryHandle, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        transform_solid(&mut self.topo, solid, &Self::rotation_axis_matrix(axis, angle)?).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn scale_sync(&mut self, shape: &GeometryHandle, factor: f64, center: Vec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let to_origin = Mat4::translation(-center[0], -center[1], -center[2]);
        let scale = Mat4::scale(factor, factor, factor);
        let back = Mat4::translation(center[0], center[1], center[2]);
        transform_solid(&mut self.topo, solid, &(back * scale * to_origin)).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn mirror_sync(&mut self, shape: &GeometryHandle, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError> {
        let solid_id = self.solid_id(shape)?;
        let solid = mirror(&mut self.topo, solid_id, p3(origin), v3(normal)).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn copy_shape_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let solid = copy_solid(&mut self.topo, solid).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn linear_pattern_sync(&mut self, shape: &GeometryHandle, direction: Vec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let compound = linear_pattern(&mut self.topo, solid, v3(direction), spacing, count).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Compound, Entity::Compound(compound)))
    }

    pub fn circular_pattern_sync(&mut self, shape: &GeometryHandle, axis: Vec3, count: usize) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let compound = circular_pattern(&mut self.topo, solid, v3(axis), count).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Compound, Entity::Compound(compound)))
    }

    #[allow(clippy::too_many_arguments, reason = "mirrors kernel_3d_engine::BrepKernel::grid_pattern's shape 1:1 (that trait is out of this crate's scope to restructure)")]
    pub fn grid_pattern_sync(&mut self, shape: &GeometryHandle, dir_x: Vec3, dir_y: Vec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let compound = grid_pattern(&mut self.topo, solid, v3(dir_x), v3(dir_y), spacing_x, spacing_y, count_x, count_y).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Compound, Entity::Compound(compound)))
    }

    pub fn fillet_sync(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let solid = fillet_v2(&mut self.topo, solid, &edges, radius).map_err(Self::map_err)?.solid;
        Ok(self.register_solid(solid))
    }

    pub fn fillet_variable_sync(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let laws: Vec<(EdgeId, FilletRadiusLaw)> = edges.iter().map(|&e| (e, FilletRadiusLaw::Linear { start: radius_start, end: radius_end })).collect();
        let solid = fillet_variable(&mut self.topo, solid, &laws).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn chamfer_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let solid = chamfer(&mut self.topo, solid, &edges, distance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    /// 🎯️ Fillets only `edges` instead of every edge of the solid — brepkit's
    /// `fillet_v2` already accepts an explicit edge list, `fillet_sync` just always
    /// passes every edge; this exposes the selective-edge case directly.
    pub fn fillet_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edge_ids: Vec<EdgeId> = edges.iter().map(|handle| self.edge_id(handle)).collect::<Result<_, _>>()?;
        let solid = fillet_v2(&mut self.topo, solid, &edge_ids, radius).map_err(Self::map_err)?.solid;
        Ok(self.register_solid(solid))
    }

    /// 🎯️ Chamfers only `edges` instead of every edge of the solid.
    pub fn chamfer_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edge_ids: Vec<EdgeId> = edges.iter().map(|handle| self.edge_id(handle)).collect::<Result<_, _>>()?;
        let solid = chamfer(&mut self.topo, solid, &edge_ids, distance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn chamfer_asymmetric_sync(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let solid = chamfer_asymmetric(&mut self.topo, solid, &edges, d1, d2).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn shell_sync(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let open: Vec<FaceId> = open_faces.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = shell(&mut self.topo, solid, thickness, &open).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn draft_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: Vec3, neutral_point: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let face_ids: Vec<FaceId> = faces.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = draft(&mut self.topo, solid, &face_ids, v3(pull_direction), p3(neutral_point), angle).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn offset_solid_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let solid = offset_solid_v2(&mut self.topo, solid, distance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn defeature_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let face_ids: Vec<FaceId> = faces.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = defeature(&mut self.topo, solid, &face_ids).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn section_sync(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<Vec<GeometryHandle>, BrepError> {
        let solid_id = self.solid_id(solid)?;
        let result = section(&mut self.topo, solid_id, p3(plane_origin), v3(plane_normal)).map_err(Self::map_err)?;
        Ok(result.faces.into_iter().map(|f| self.register_entity(GeometryKind::Face, Entity::Face(f))).collect())
    }

    pub fn split_sync(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        let solid_id = self.solid_id(solid)?;
        let result = split(&mut self.topo, solid_id, p3(plane_origin), v3(plane_normal)).map_err(Self::map_err)?;
        Ok((self.register_solid(result.positive), self.register_solid(result.negative)))
    }

    pub fn curve_curve_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError> {
        let nurbs_a = match &self.entry(a)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            _ => return Err(BrepError::InvalidInput("curve a must be curve or edge".into())),
        };
        let nurbs_b = match &self.entry(b)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            _ => return Err(BrepError::InvalidInput("curve b must be curve or edge".into())),
        };
        let hits = curve_curve_intersect(&nurbs_a, &nurbs_b, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.iter().map(|h| from_p3(h.point)).collect())
    }

    pub fn curve_surface_intersect_sync(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError> {
        let nurbs_c = match &self.entry(curve)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            _ => return Err(BrepError::InvalidInput("curve must be curve or edge".into())),
        };
        let nurbs_s = match &self.entry(surface)?.entity {
            Entity::Surface(s) => Self::surface_to_nurbs(s)?,
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Nurbs(ns) => ns.clone(),
                    other => Self::surface_to_nurbs(&match other {
                        FaceSurface::Plane { normal, d } => KernelSurface::Plane { origin: Point3::new(normal.x() * *d, normal.y() * *d, normal.z() * *d), normal: *normal },
                        FaceSurface::Cylinder(c) => KernelSurface::Cylinder(c.clone()),
                        FaceSurface::Cone(c) => KernelSurface::Cone(c.clone()),
                        FaceSurface::Sphere(s) => KernelSurface::Sphere(s.clone()),
                        FaceSurface::Torus(t) => KernelSurface::Torus(t.clone()),
                        FaceSurface::Nurbs(_) => unreachable!(),
                    })?,
                }
            }
            _ => return Err(BrepError::InvalidInput("surface must be surface or face".into())),
        };
        let hits = intersect_curve_surface(&nurbs_c, &nurbs_s, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.iter().map(|h| from_p3(h.point)).collect())
    }

    pub fn surface_surface_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, _tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let nurbs_a = match &self.entry(a)?.entity {
            Entity::Surface(s) => Self::surface_to_nurbs(s)?,
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Nurbs(ns) => ns.clone(),
                    other => Self::surface_to_nurbs(&match other {
                        FaceSurface::Plane { normal, d } => {
                            let origin = Point3::new(normal.x() * *d, normal.y() * *d, normal.z() * *d);
                            KernelSurface::Plane { origin, normal: *normal }
                        }
                        FaceSurface::Cylinder(c) => KernelSurface::Cylinder(c.clone()),
                        FaceSurface::Cone(c) => KernelSurface::Cone(c.clone()),
                        FaceSurface::Sphere(s) => KernelSurface::Sphere(s.clone()),
                        FaceSurface::Torus(t) => KernelSurface::Torus(t.clone()),
                        FaceSurface::Nurbs(_) => unreachable!(),
                    })?,
                }
            }
            _ => return Err(BrepError::InvalidInput("a must be surface or face".into())),
        };
        let nurbs_b = match &self.entry(b)?.entity {
            Entity::Surface(s) => Self::surface_to_nurbs(s)?,
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Nurbs(ns) => ns.clone(),
                    other => Self::surface_to_nurbs(&match other {
                        FaceSurface::Plane { normal, d } => {
                            let origin = Point3::new(normal.x() * *d, normal.y() * *d, normal.z() * *d);
                            KernelSurface::Plane { origin, normal: *normal }
                        }
                        FaceSurface::Cylinder(c) => KernelSurface::Cylinder(c.clone()),
                        FaceSurface::Cone(c) => KernelSurface::Cone(c.clone()),
                        FaceSurface::Sphere(s) => KernelSurface::Sphere(s.clone()),
                        FaceSurface::Torus(t) => KernelSurface::Torus(t.clone()),
                        FaceSurface::Nurbs(_) => unreachable!(),
                    })?,
                }
            }
            _ => return Err(BrepError::InvalidInput("b must be surface or face".into())),
        };
        let curves = intersect_nurbs_nurbs(&nurbs_a, &nurbs_b, 32, 0.0).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(curves.into_iter().map(|ic| self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Nurbs(ic.curve)))).collect())
    }

    pub fn curve_point_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError> {
        match &self.entry(curve)?.entity {
            Entity::Curve(c) => Ok(from_p3(Self::curve_evaluate(c, parameter))),
            Entity::Edge(e) => {
                let edge = self.topo.edge(*e).map_err(Self::map_topo_err)?;
                let start = self.topo.vertex(edge.start()).map_err(Self::map_topo_err)?.point();
                let end = self.topo.vertex(edge.end()).map_err(Self::map_topo_err)?.point();
                match edge.curve() {
                    EdgeCurve::Line => {
                        let len = (end - start).length();
                        let frac = if len < 1e-15 { 0.0 } else { parameter / len };
                        Ok(from_p3(Point3::new(start.x() + (end.x() - start.x()) * frac, start.y() + (end.y() - start.y()) * frac, start.z() + (end.z() - start.z()) * frac)))
                    }
                    EdgeCurve::NurbsCurve(c) => Ok(from_p3(c.evaluate(parameter))),
                    EdgeCurve::Circle(c) => Ok(from_p3(c.evaluate(parameter))),
                    EdgeCurve::Ellipse(el) => Ok(from_p3(el.evaluate(parameter))),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", curve.as_str()))),
        }
    }

    pub fn curve_tangent_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError> {
        match &self.entry(curve)?.entity {
            Entity::Curve(c) => Ok(from_v3(Self::curve_tangent_inner(c, parameter))),
            Entity::Edge(e) => {
                let edge = self.topo.edge(*e).map_err(Self::map_topo_err)?;
                match edge.curve() {
                    EdgeCurve::Line => {
                        let start = self.topo.vertex(edge.start()).map_err(Self::map_topo_err)?.point();
                        let end = self.topo.vertex(edge.end()).map_err(Self::map_topo_err)?.point();
                        let dir = end - start;
                        let len = dir.length();
                        if len < 1e-15 {
                            Ok([1.0, 0.0, 0.0])
                        } else {
                            Ok([dir.x() / len, dir.y() / len, dir.z() / len])
                        }
                    }
                    EdgeCurve::NurbsCurve(c) => {
                        let d = c.derivatives(parameter, 1);
                        Ok(if d.len() > 1 { from_v3(d[1]) } else { [1.0, 0.0, 0.0] })
                    }
                    EdgeCurve::Circle(c) => Ok(from_v3(c.tangent(parameter))),
                    EdgeCurve::Ellipse(el) => Ok(from_v3(el.tangent(parameter))),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", curve.as_str()))),
        }
    }

    pub fn curve_domain_sync(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        match &self.entry(curve)?.entity {
            Entity::Curve(c) => Ok(Self::curve_domain_inner(c)),
            Entity::Edge(e) => {
                let edge = self.topo.edge(*e).map_err(Self::map_topo_err)?;
                match edge.curve() {
                    EdgeCurve::Line => {
                        let start = self.topo.vertex(edge.start()).map_err(Self::map_topo_err)?.point();
                        let end = self.topo.vertex(edge.end()).map_err(Self::map_topo_err)?.point();
                        Ok(ParamDomain { min: 0.0, max: (end - start).length() })
                    }
                    EdgeCurve::NurbsCurve(c) => {
                        let (a, b) = c.domain();
                        Ok(ParamDomain { min: a, max: b })
                    }
                    EdgeCurve::Circle(_) | EdgeCurve::Ellipse(_) => Ok(ParamDomain { min: 0.0, max: TAU }),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", curve.as_str()))),
        }
    }

    pub fn curve_curvature_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        match &self.entry(curve)?.entity {
            Entity::Curve(c) => Ok(Self::curve_curvature_inner(c, parameter)),
            Entity::Edge(e) => {
                let edge = self.topo.edge(*e).map_err(Self::map_topo_err)?;
                match edge.curve() {
                    EdgeCurve::Line => Ok(0.0),
                    EdgeCurve::Circle(c) => Ok(1.0 / c.radius()),
                    EdgeCurve::Ellipse(el) => {
                        let nurbs = ellipse_to_nurbs(el, 0.0, TAU).map_err(|e| BrepError::Operation(e.to_string()))?;
                        let d = nurbs.derivatives(parameter, 2);
                        if d.len() < 2 {
                            Ok(0.0)
                        } else {
                            let tan = d[1];
                            let tan_len = tan.length();
                            if tan_len < 1e-15 {
                                Ok(0.0)
                            } else {
                                let d2 = if d.len() > 2 { d[2] } else { BkVec3::new(0.0, 0.0, 0.0) };
                                Ok(tan.cross(d2).length() / tan_len.powi(3))
                            }
                        }
                    }
                    EdgeCurve::NurbsCurve(c) => {
                        let d = c.derivatives(parameter, 2);
                        if d.len() < 2 {
                            Ok(0.0)
                        } else {
                            let tan = d[1];
                            let tan_len = tan.length();
                            if tan_len < 1e-15 {
                                Ok(0.0)
                            } else {
                                let d2 = if d.len() > 2 { d[2] } else { BkVec3::new(0.0, 0.0, 0.0) };
                                Ok(tan.cross(d2).length() / tan_len.powi(3))
                            }
                        }
                    }
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", curve.as_str()))),
        }
    }

    pub fn surface_point_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError> {
        match &self.entry(surface)?.entity {
            Entity::Surface(KernelSurface::Plane { origin, normal }) => {
                let frame = Frame3::from_normal(*origin, *normal).map_err(|e| BrepError::Operation(e.to_string()))?;
                Ok(from_p3(frame.origin + frame.x * u + frame.y * v))
            }
            Entity::Surface(KernelSurface::Cylinder(c)) => Ok(from_p3(c.evaluate(u, v))),
            Entity::Surface(KernelSurface::Cone(c)) => Ok(from_p3(c.evaluate(u, v))),
            Entity::Surface(KernelSurface::Sphere(s)) => Ok(from_p3(s.evaluate(u, v))),
            Entity::Surface(KernelSurface::Torus(t)) => Ok(from_p3(t.evaluate(u, v))),
            Entity::Surface(KernelSurface::Nurbs(ns)) => Ok(from_p3(ns.evaluate(u, v))),
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Nurbs(ns) => Ok(from_p3(ns.evaluate(u, v))),
                    FaceSurface::Plane { normal, d } => {
                        let origin = Point3::new(normal.x() * *d, normal.y() * *d, normal.z() * *d);
                        let frame = Frame3::from_normal(origin, *normal).map_err(|e| BrepError::Operation(e.to_string()))?;
                        Ok(from_p3(frame.origin + frame.x * u + frame.y * v))
                    }
                    FaceSurface::Cylinder(c) => Ok(from_p3(c.evaluate(u, v))),
                    FaceSurface::Cone(c) => Ok(from_p3(c.evaluate(u, v))),
                    FaceSurface::Sphere(s) => Ok(from_p3(s.evaluate(u, v))),
                    FaceSurface::Torus(t) => Ok(from_p3(t.evaluate(u, v))),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a surface", surface.as_str()))),
        }
    }

    pub fn surface_normal_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError> {
        match &self.entry(surface)?.entity {
            Entity::Surface(KernelSurface::Plane { normal, .. }) => Ok(from_v3(*normal)),
            Entity::Surface(KernelSurface::Cylinder(c)) => Ok(from_v3(c.normal(u, v))),
            Entity::Surface(KernelSurface::Cone(c)) => Ok(from_v3(c.normal(u, v))),
            Entity::Surface(KernelSurface::Sphere(s)) => Ok(from_v3(s.normal(u, v))),
            Entity::Surface(KernelSurface::Torus(t)) => Ok(from_v3(t.normal(u, v))),
            Entity::Surface(KernelSurface::Nurbs(ns)) => {
                let d = ns.derivatives(u, v, 1);
                let du = d.get(1).and_then(|row| row.first()).copied();
                let dv = d.first().and_then(|row| row.get(1)).copied();
                if let (Some(du), Some(dv)) = (du, dv) {
                    Ok(from_v3(du.cross(dv).normalize().unwrap_or(BkVec3::new(0.0, 0.0, 1.0))))
                } else {
                    Ok([0.0, 0.0, 1.0])
                }
            }
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Plane { normal, .. } => Ok(from_v3(*normal)),
                    FaceSurface::Nurbs(ns) => {
                        let d = ns.derivatives(u, v, 1);
                        let du = d.get(1).and_then(|row| row.first()).copied();
                        let dv = d.first().and_then(|row| row.get(1)).copied();
                        if let (Some(du), Some(dv)) = (du, dv) {
                            Ok(from_v3(du.cross(dv).normalize().unwrap_or(BkVec3::new(0.0, 0.0, 1.0))))
                        } else {
                            Ok([0.0, 0.0, 1.0])
                        }
                    }
                    FaceSurface::Cylinder(c) => Ok(from_v3(c.normal(u, v))),
                    FaceSurface::Cone(c) => Ok(from_v3(c.normal(u, v))),
                    FaceSurface::Sphere(s) => Ok(from_v3(s.normal(u, v))),
                    FaceSurface::Torus(t) => Ok(from_v3(t.normal(u, v))),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a surface", surface.as_str()))),
        }
    }

    pub fn volume_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        let solids = self.solid_ids_from_handle(shape)?;
        let mut total = 0.0;
        for solid in solids {
            total += measure::solid_volume(&self.topo, solid, 0.1).map_err(Self::map_err)?;
        }
        Ok(total)
    }

    pub fn area_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match &self.entry(shape)?.entity {
            Entity::Face(f) => measure::face_area(&self.topo, *f, 0.1).map_err(Self::map_err),
            Entity::Solid(s) => measure::solid_surface_area(&self.topo, *s, 0.1).map_err(Self::map_err),
            Entity::Compound(c) => {
                let mut total = 0.0;
                for &s in self.topo.compound(*c).map_err(Self::map_topo_err)?.solids() {
                    total += measure::solid_surface_area(&self.topo, s, 0.1).map_err(Self::map_err)?;
                }
                Ok(total)
            }
            _ => Err(BrepError::InvalidInput(format!("{} cannot compute area", shape.as_str()))),
        }
    }

    pub fn length_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match &self.entry(shape)?.entity {
            Entity::Edge(e) => measure::edge_length(&self.topo, *e).map_err(Self::map_err),
            Entity::Wire(w) => {
                let wire = self.topo.wire(*w).map_err(Self::map_topo_err)?;
                let mut total = 0.0;
                for oe in wire.edges() {
                    total += measure::edge_length(&self.topo, oe.edge()).map_err(Self::map_err)?;
                }
                Ok(total)
            }
            Entity::Curve(c) => {
                let domain = Self::curve_domain_inner(c);
                let nurbs = Self::curve_to_nurbs(c)?;
                let (a, b) = nurbs.domain();
                let samples = sample_deflection(&nurbs, a, b, 0.01);
                Ok(if samples.len() < 2 {
                    domain.max - domain.min
                } else {
                    let mut len = 0.0;
                    for w in samples.windows(2) {
                        len += (w[1].1 - w[0].1).length();
                    }
                    len
                })
            }
            _ => Err(BrepError::InvalidInput(format!("{} cannot compute length", shape.as_str()))),
        }
    }

    pub fn center_of_mass_sync(&self, shape: &GeometryHandle) -> Result<Vec3, BrepError> {
        let solid = self.solid_id(shape)?;
        let com = measure::solid_center_of_mass(&self.topo, solid, 0.1).map_err(Self::map_err)?;
        Ok(from_p3(com))
    }

    pub fn bounding_box_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solids = self.solid_ids_from_handle(shape)?;
        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);
        for solid in solids {
            let aabb = measure::solid_bounding_box(&self.topo, solid).map_err(Self::map_err)?;
            min = Point3::new(min.x().min(aabb.min.x()), min.y().min(aabb.min.y()), min.z().min(aabb.min.z()));
            max = Point3::new(max.x().max(aabb.max.x()), max.y().max(aabb.max.y()), max.z().max(aabb.max.z()));
        }
        let dx = max.x() - min.x();
        let dy = max.y() - min.y();
        let dz = max.z() - min.z();
        let solid = make_box(&mut self.topo, dx.max(TOL), dy.max(TOL), dz.max(TOL)).map_err(Self::map_err)?;
        let cx = (min.x() + max.x()) / 2.0;
        let cy = (min.y() + max.y()) / 2.0;
        let cz = (min.z() + max.z()) / 2.0;
        transform_solid(&mut self.topo, solid, &Mat4::translation(cx, cy, cz)).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn distance_sync(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        let a_solids = self.solid_ids_from_handle(a)?;
        let b_solids = self.solid_ids_from_handle(b)?;
        let mut best = f64::MAX;
        for &sa in &a_solids {
            for &sb in &b_solids {
                let d = brepkit_operations::distance::solid_to_solid_distance(&self.topo, sa, sb).map_err(Self::map_err)?.distance;
                best = best.min(d);
            }
        }
        Ok(best)
    }

    pub fn closest_point_sync(&self, shape: &GeometryHandle, point: Vec3) -> Result<ClosestPoint, BrepError> {
        let solid = self.solid_id(shape)?;
        let result = brepkit_operations::distance::point_to_solid_distance(&self.topo, p3(point), solid).map_err(Self::map_err)?;
        Ok(ClosestPoint { distance: result.distance, point: from_p3(result.point_b), parameter: None, uv: None })
    }

    pub fn classify_point_sync(&self, solid: &GeometryHandle, point: Vec3) -> Result<PointClassification, BrepError> {
        let solid_id = self.solid_id(solid)?;
        let result = brepkit_operations::classify::classify_point(&self.topo, solid_id, p3(point), 0.1, TOL).map_err(Self::map_err)?;
        Ok(match result {
            brepkit_operations::classify::PointClassification::Inside => PointClassification::Inside,
            brepkit_operations::classify::PointClassification::Outside => PointClassification::Outside,
            brepkit_operations::classify::PointClassification::OnBoundary => PointClassification::OnBoundary,
        })
    }

    pub fn validate_sync(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        let solid = self.solid_id(shape)?;
        let report = brepkit_operations::validate::validate_solid_relaxed(&self.topo, solid).map_err(Self::map_err)?;
        if report.error_count() == 0 {
            Ok("valid".into())
        } else {
            Ok(format!("{} errors", report.error_count()))
        }
    }

    pub fn vertex_sync(&mut self, point: Vec3) -> Result<GeometryHandle, BrepError> {
        let id = self.topo.add_vertex(Vertex::new(p3(point), TOL));
        Ok(self.register_entity(GeometryKind::Vertex, Entity::Vertex(id)))
    }

    pub fn face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let wire_id = self.wire_id(wire)?;
        let face = builder::make_face_from_wire(&mut self.topo, wire_id).map_err(Self::map_topo_err)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn sew_faces_sync(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let face_ids: Vec<FaceId> = faces.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = sew_faces(&mut self.topo, &face_ids, tolerance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn heal_solid_sync(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        brepkit_operations::heal::heal_solid(&mut self.topo, solid, tolerance).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn convert_to_nurbs_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        brepkit_operations::heal::convert_to_bspline(&mut self.topo, solid).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn deconstruct_sync(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        let solids = self.solid_ids_from_handle(shape)?;
        let mut vertex_ids = Vec::new();
        let mut edge_ids = Vec::new();
        let mut face_ids = Vec::new();
        let mut seen_vertices = std::collections::HashSet::new();
        let mut seen_edges = std::collections::HashSet::new();
        let mut seen_faces = std::collections::HashSet::new();
        for solid in solids {
            for vertex in explorer::solid_vertices(&self.topo, solid).map_err(Self::map_topo_err)? {
                if seen_vertices.insert(vertex.index()) {
                    vertex_ids.push(vertex);
                }
            }
            for edge in explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)? {
                if seen_edges.insert(edge.index()) {
                    edge_ids.push(edge);
                }
            }
            for face in explorer::solid_faces(&self.topo, solid).map_err(Self::map_topo_err)? {
                if seen_faces.insert(face.index()) {
                    face_ids.push(face);
                }
            }
        }
        Ok(BrepTopology {
            vertices: vertex_ids.into_iter().map(|id| self.register_entity(GeometryKind::Vertex, Entity::Vertex(id))).collect(),
            edges: edge_ids.into_iter().map(|id| self.register_entity(GeometryKind::Edge, Entity::Edge(id))).collect(),
            faces: face_ids.into_iter().map(|id| self.register_entity(GeometryKind::Face, Entity::Face(id))).collect(),
        })
    }

    pub fn export_step_sync(&self, shapes: &[GeometryHandle]) -> Result<String, BrepError> {
        let mut solids = Vec::new();
        for h in shapes {
            solids.extend(self.solid_ids_from_handle(h)?);
        }
        brepkit_io::step::writer::write_step(&self.topo, &solids).map_err(Self::map_io_err)
    }

    pub fn export_stl_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let mut solids = Vec::new();
        for h in shapes {
            solids.extend(self.solid_ids_from_handle(h)?);
        }
        brepkit_io::stl::writer::write_stl(&self.topo, &solids, deflection, brepkit_io::stl::writer::StlFormat::Binary).map_err(Self::map_io_err)
    }

    pub fn export_obj_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        let mut solids = Vec::new();
        for h in shapes {
            solids.extend(self.solid_ids_from_handle(h)?);
        }
        brepkit_io::obj::write_obj(&self.topo, &solids, deflection).map_err(Self::map_io_err)
    }

    pub fn export_gltf_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let mut solids = Vec::new();
        for h in shapes {
            solids.extend(self.solid_ids_from_handle(h)?);
        }
        brepkit_io::gltf::write_glb(&self.topo, &solids, deflection).map_err(Self::map_io_err)
    }

    /// 🌉️ Tessellates `handle` via `tessellate_sync`'s `MeshTransfer` path and converts it straight to framework-core `MeshData`; the bridge that lets `GlbExporter`/`GlbImporter` (hand-rolled, dependency-free) serve GLB for B-Rep solids instead of `brepkit_io::gltf::write_glb`.
    pub fn tessellate_to_mesh_data_sync(&self, handle: &GeometryHandle, tolerance: f64) -> Result<semio_framework_core::MeshData, BrepError> {
        let transfer = self.tessellate_sync(handle, tolerance)?;
        Ok(mesh_data_from_mesh_transfer(&transfer))
    }

    /// 🌉️ GLB export standardized on the hand-rolled `GlbExporter` codec (see `tessellate_to_mesh_data_sync`), not `brepkit_io::gltf`.
    pub fn export_glb_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let mut mesh = semio_framework_core::MeshData::default();
        for handle in shapes {
            mesh.merge(&self.tessellate_to_mesh_data_sync(handle, deflection.max(1e-4))?);
        }
        semio_framework_core::GlbExporter.export(&mesh).map_err(BrepError::Operation)
    }

    /// 🌉️ GLB import standardized on the hand-rolled `GlbImporter` codec, converted into a solid the same way `import_dwg_sync`/`import_stl_sync` do.
    pub fn import_glb_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mesh = semio_framework_core::GlbImporter.import(data).map_err(BrepError::Operation)?;
        let positions: Vec<Point3> = mesh.positions.as_chunks::<3>().0.iter().map(|c| Point3::new(c[0] as f64, c[1] as f64, c[2] as f64)).collect();
        let normals: Vec<brepkit_math::vec::Vec3> = mesh.normals.as_chunks::<3>().0.iter().map(|c| brepkit_math::vec::Vec3::new(c[0] as f64, c[1] as f64, c[2] as f64)).collect();
        let triangle_mesh = brepkit_operations::tessellate::TriangleMesh { positions, normals, indices: mesh.indices.clone() };
        let solid = import_mesh(&mut self.topo, &triangle_mesh, tolerance).map_err(Self::map_io_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn import_step_sync(&mut self, data: &str) -> Result<Vec<GeometryHandle>, BrepError> {
        let solids = brepkit_io::step::reader::read_step(data, &mut self.topo).map_err(Self::map_io_err)?;
        Ok(solids.into_iter().map(|s| self.register_solid(s)).collect())
    }

    pub fn import_stl_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mesh = brepkit_io::stl::reader::read_stl(data).map_err(Self::map_io_err)?;
        let solid = import_mesh(&mut self.topo, &mesh, tolerance).map_err(Self::map_io_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn import_obj_sync(&mut self, data: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mesh = brepkit_io::obj::read_obj(data).map_err(Self::map_io_err)?;
        let solid = import_mesh(&mut self.topo, &mesh, tolerance).map_err(Self::map_io_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn export_dwg_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let mut mesh = semio_framework_core::MeshData::default();
        for h in shapes {
            for solid in self.solid_ids_from_handle(h)? {
                let tri = tessellate_solid_with_tolerance(&self.topo, solid, deflection.max(1e-4), 0.2).map_err(Self::map_err)?;
                let base = (mesh.positions.len() / 3) as u32;
                for p in &tri.positions {
                    mesh.positions.extend_from_slice(&[p.x() as f32, p.y() as f32, p.z() as f32]);
                }
                mesh.indices.extend(tri.indices.iter().map(|i| i + base));
            }
        }
        let drawing = semio_framework_core::mesh_to_dwg_drawing(&mesh);
        semio_framework_core::dwg_to_bytes(&drawing).map_err(BrepError::Operation)
    }

    pub fn import_dwg_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let drawing = semio_framework_core::dwg_from_bytes(data).map_err(BrepError::Operation)?;
        let mesh = semio_framework_core::dwg_drawing_to_mesh(&drawing);
        let positions: Vec<Point3> = mesh.positions.as_chunks::<3>().0.iter().map(|c| Point3::new(c[0] as f64, c[1] as f64, c[2] as f64)).collect();
        let normals: Vec<brepkit_math::vec::Vec3> = mesh.normals.as_chunks::<3>().0.iter().map(|c| brepkit_math::vec::Vec3::new(c[0] as f64, c[1] as f64, c[2] as f64)).collect();
        let triangle_mesh = brepkit_operations::tessellate::TriangleMesh { positions, normals, indices: mesh.indices.clone() };
        let solid = import_mesh(&mut self.topo, &triangle_mesh, tolerance).map_err(Self::map_io_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn kind_sync(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        Ok(self.entry(handle)?.kind)
    }

    /// 🧩️ Extracts the low-poly polygon soup of a solid from B-Rep face wires, using shared topology
    /// vertices (no per-face Steiner edge samples). Each entry is `(outer_loop, hole_loops)` indexed into
    /// `positions`. Simple faces become one n-gon; callers triangulate holed faces without filling openings.
    pub fn solid_face_loops_sync(&self, handle: &GeometryHandle) -> Result<(Vec<[f32; 3]>, Vec<(Vec<u32>, Vec<Vec<u32>>)>), BrepError> {
        let solid = self.solid_id(handle)?;
        let mut vertex_to_index: std::collections::HashMap<usize, u32> = std::collections::HashMap::new();
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut face_loops = Vec::new();
        for face_id in explorer::solid_faces(&self.topo, solid).map_err(Self::map_topo_err)? {
            let face = self.topo.face(face_id).map_err(Self::map_topo_err)?;
            let mut wires = vec![face.outer_wire()];
            wires.extend_from_slice(face.inner_wires());
            let mut loops: Vec<Vec<u32>> = Vec::new();
            for wire_id in wires {
                let wire = self.topo.wire(wire_id).map_err(Self::map_topo_err)?;
                let mut loop_indices = Vec::new();
                for oriented in wire.edges() {
                    let edge = self.topo.edge(oriented.edge()).map_err(Self::map_topo_err)?;
                    let vid = oriented.oriented_start(&edge);
                    let index = if let Some(&existing) = vertex_to_index.get(&vid.index()) {
                        existing
                    } else {
                        let point = self.topo.vertex(vid).map_err(Self::map_topo_err)?.point();
                        let next = positions.len() as u32;
                        positions.push([point.x() as f32, point.y() as f32, point.z() as f32]);
                        vertex_to_index.insert(vid.index(), next);
                        next
                    };
                    loop_indices.push(index);
                }
                loops.push(loop_indices);
            }
            let mut loops_iter = loops.into_iter();
            let Some(outer) = loops_iter.next() else { continue };
            if outer.len() < 3 {
                continue;
            }
            face_loops.push((outer, loops_iter.collect()));
        }
        Ok((positions, face_loops))
    }

    pub fn tessellate_sync(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError> {
        let tol = tolerance.max(1e-4);
        let entry = self.entry(handle)?;
        match &entry.entity {
            Entity::Solid(solid) => {
                // 🧵️ Tessellate each face in parallel (faces are independent); the triangle-index
                // merge below stays sequential since index offsets depend on prior faces' output.
                let faces = explorer::solid_faces(&self.topo, *solid).map_err(Self::map_topo_err)?;
                let face_meshes: Result<Vec<_>, BrepError> = faces.par_iter().map(|&face| tessellate_with_tolerance(&self.topo, face, tol, 0.2).map_err(Self::map_err).map(|mesh| (face, mesh))).collect();
                let mut transfer = MeshTransfer { position: Vec::new(), normal: Vec::new(), index: Vec::new(), edges: Vec::new(), points: Vec::new(), face_groups: Vec::new() };
                for (face, mesh) in face_meshes? {
                    let base = transfer.position.len() / 3;
                    transfer.position.extend(mesh.positions.iter().flat_map(|p| [p.x() as f32, p.y() as f32, p.z() as f32]));
                    transfer.normal.extend(mesh.normals.iter().flat_map(|n| [n.x() as f32, n.y() as f32, n.z() as f32]));
                    let tri_start = transfer.index.len() as u32;
                    let tri_count = mesh.indices.len() as u32;
                    for idx in mesh.indices {
                        transfer.index.push(idx + base as u32);
                    }
                    transfer.face_groups.push(FaceGroup { start: tri_start, count: tri_count, entity_id: face.index().to_string() });
                }
                let edges = sample_solid_edges(&self.topo, *solid, tol).map_err(Self::map_err)?;
                transfer.edges = Self::edge_lines_flat(&edges);
                Ok(transfer)
            }
            Entity::Compound(c) => {
                let mut transfer = MeshTransfer::default();
                for &solid in self.topo.compound(*c).map_err(Self::map_topo_err)?.solids() {
                    let mesh = tessellate_solid_with_tolerance(&self.topo, solid, tol, 0.2).map_err(Self::map_err)?;
                    let edges = sample_solid_edges(&self.topo, solid, tol).map_err(Self::map_err)?;
                    let base = transfer.position.len() / 3;
                    transfer.position.extend(mesh.positions.iter().flat_map(|p| [p.x() as f32, p.y() as f32, p.z() as f32]));
                    transfer.normal.extend(mesh.normals.iter().flat_map(|n| [n.x() as f32, n.y() as f32, n.z() as f32]));
                    let tri_start = transfer.index.len() as u32;
                    let tri_count = mesh.indices.len() as u32;
                    for idx in mesh.indices {
                        transfer.index.push(idx + base as u32);
                    }
                    transfer.edges.extend(Self::edge_lines_flat(&edges));
                    transfer.face_groups.push(FaceGroup { start: tri_start, count: tri_count, entity_id: handle.as_str().to_string() });
                }
                Ok(transfer)
            }
            Entity::Face(face) => {
                let mesh = tessellate_with_tolerance(&self.topo, *face, tol, 0.2).map_err(Self::map_err)?;
                let position: Vec<f32> = mesh.positions.iter().flat_map(|p| [p.x() as f32, p.y() as f32, p.z() as f32]).collect();
                let normal: Vec<f32> = mesh.normals.iter().flat_map(|n| [n.x() as f32, n.y() as f32, n.z() as f32]).collect();
                let triangle_count = mesh.indices.len() as u32;
                let edges = self.sample_face_boundary_edge_lines(*face, tol)?;
                Ok(MeshTransfer { position, normal, index: mesh.indices, edges, points: Vec::new(), face_groups: vec![FaceGroup { start: 0, count: triangle_count, entity_id: handle.as_str().to_string() }] })
            }
            Entity::Curve(c) => {
                let nurbs = Self::curve_to_nurbs(c)?;
                let (a, b) = nurbs.domain();
                let samples = sample_deflection(&nurbs, a, b, tol);
                let mut edges = Vec::new();
                for w in samples.windows(2) {
                    let p0 = w[0].1;
                    let p1 = w[1].1;
                    edges.extend([p0.x() as f32, p0.y() as f32, p0.z() as f32, p1.x() as f32, p1.y() as f32, p1.z() as f32]);
                }
                Ok(MeshTransfer { position: Vec::new(), normal: Vec::new(), index: Vec::new(), edges, points: Vec::new(), face_groups: Vec::new() })
            }
            Entity::Vertex(v) => {
                let p = self.topo.vertex(*v).map_err(Self::map_topo_err)?.point();
                Ok(MeshTransfer { position: Vec::new(), normal: Vec::new(), index: Vec::new(), edges: Vec::new(), points: vec![p.x() as f32, p.y() as f32, p.z() as f32], face_groups: Vec::new() })
            }
            Entity::Edge(_) | Entity::Wire(_) => {
                let mut edges = Vec::new();
                let edge_ids: Vec<EdgeId> = match &entry.entity {
                    Entity::Edge(e) => vec![*e],
                    Entity::Wire(w) => self.topo.wire(*w).map_err(Self::map_topo_err)?.edges().iter().map(|oe| oe.edge()).collect(),
                    _ => Vec::new(),
                };
                for edge_id in edge_ids {
                    edges.extend(self.sample_oriented_edge_lines(edge_id, tol)?);
                }
                Ok(MeshTransfer { position: Vec::new(), normal: Vec::new(), index: Vec::new(), edges, points: Vec::new(), face_groups: Vec::new() })
            }
            Entity::Surface(s) => {
                let nurbs = Self::surface_to_nurbs(s)?;
                let (ua, ub) = nurbs.domain_u();
                let (va, vb) = nurbs.domain_v();
                let grid = surface_grid(&nurbs, (ua, ub), (va, vb), 16, 16);
                let mut position = Vec::new();
                let mut normal = Vec::new();
                let mut index = Vec::new();
                let rows = grid.len();
                let cols = grid.first().map_or(0, Vec::len);
                for row in &grid {
                    for p in row {
                        position.extend([p.x() as f32, p.y() as f32, p.z() as f32]);
                        normal.extend([0.0, 0.0, 1.0]);
                    }
                }
                for r in 0..rows - 1 {
                    for c in 0..cols - 1 {
                        let i0 = r * cols + c;
                        let i1 = i0 + 1;
                        let i2 = (r + 1) * cols + c;
                        let i3 = i2 + 1;
                        index.extend([i0 as u32, i2 as u32, i1 as u32, i1 as u32, i2 as u32, i3 as u32]);
                    }
                }
                let triangle_count = index.len() as u32;
                Ok(MeshTransfer { position, normal, index, edges: Vec::new(), points: Vec::new(), face_groups: vec![FaceGroup { start: 0, count: triangle_count, entity_id: handle.as_str().to_string() }] })
            }
        }
    }

    pub fn dispose_sync(&mut self, handle: &GeometryHandle) {
        if let Some(Entry { entity: Entity::Solid(solid), .. }) = self.registry.remove(handle.as_str()) {
            self.invalidate_solid_derived_caches(solid);
        }
    }

    /// 🧹️ Drops registry entries whose handles are not in the live reference set.
    pub fn retain_sync(&mut self, live: &std::collections::HashSet<String>) {
        let disposed_solids: Vec<SolidId> = self
            .registry
            .iter()
            .filter(|(handle, _)| !live.contains(handle.as_str()))
            .filter_map(|(_, entry)| match entry.entity {
                Entity::Solid(solid) => Some(solid),
                _ => None,
            })
            .collect();
        self.registry.retain(|handle, _| live.contains(handle));
        for solid in disposed_solids {
            self.invalidate_solid_derived_caches(solid);
        }
    }

    pub fn registry_len(&self) -> usize {
        self.registry.len()
    }
}

#[async_trait(?Send)]
impl BrepKernel for BrepkitKernel {
    async fn box_prim(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.box_prim_sync(width, depth, height)
    }
    async fn sphere_prim(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.sphere_prim_sync(radius)
    }
    async fn cylinder_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cylinder_prim_sync(radius, height)
    }
    async fn cone_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cone_prim_sync(radius, height)
    }
    async fn torus_prim(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        self.torus_prim_sync(major, minor)
    }
    async fn convex_hull(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        self.convex_hull_sync(points)
    }
    async fn line_curve(&mut self, start: Vec3, end: Vec3) -> Result<GeometryHandle, BrepError> {
        self.line_curve_sync(start, end)
    }
    async fn circle_curve(&mut self, center: Vec3, normal: Vec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.circle_curve_sync(center, normal, radius)
    }
    async fn arc_curve(&mut self, center: Vec3, normal: Vec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        self.arc_curve_sync(center, normal, radius, start_angle, end_angle)
    }
    async fn ellipse_curve(&mut self, center: Vec3, normal: Vec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        self.ellipse_curve_sync(center, normal, semi_major, semi_minor)
    }
    async fn polyline_wire(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        self.polyline_wire_sync(points)
    }
    async fn rectangle_wire(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.rectangle_wire_sync(width, height)
    }
    async fn regular_polygon_wire(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        self.regular_polygon_wire_sync(radius, sides)
    }
    async fn interpolate_curve(&mut self, points: &[Vec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        self.interpolate_curve_sync(points, degree)
    }
    async fn approximate_curve(&mut self, points: &[Vec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        self.approximate_curve_sync(points, degree, control_points)
    }
    async fn helix_curve(&mut self, origin: Vec3, axis: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helix_curve_sync(origin, axis, radius, pitch, turns)
    }
    async fn plane_surface(&mut self, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError> {
        self.plane_surface_sync(origin, normal)
    }
    async fn planar_face_from_points(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_points_sync(points)
    }
    async fn planar_face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_wire_sync(wire)
    }
    async fn nurbs_surface_from_grid(&mut self, points: &[Vec<Vec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        self.nurbs_surface_from_grid_sync(points, degree_u, degree_v)
    }
    async fn coons_patch(&mut self, curves: &[Vec<Vec3>]) -> Result<GeometryHandle, BrepError> {
        self.coons_patch_sync(curves)
    }
    async fn offset_face(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_face_sync(face, distance)
    }
    async fn thicken_face(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        self.thicken_face_sync(face, thickness)
    }
    async fn extrude_wire(&mut self, wire: &GeometryHandle, vector: Vec3) -> Result<GeometryHandle, BrepError> {
        self.extrude_wire_sync(wire, vector)
    }

    async fn extrude(&mut self, face: &GeometryHandle, direction: Vec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.extrude_sync(face, direction, distance)
    }
    async fn revolve(&mut self, face: &GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.revolve_sync(face, axis_origin, axis_direction, angle)
    }
    async fn loft(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        self.loft_sync(profiles, smooth)
    }
    async fn sweep(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.sweep_sync(profile, path)
    }
    async fn pipe(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        self.pipe_sync(profile, path, guide)
    }
    async fn helical_sweep(&mut self, profile: &GeometryHandle, axis_origin: Vec3, axis_dir: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helical_sweep_sync(profile, axis_origin, axis_dir, radius, pitch, turns)
    }
    async fn fuse(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.fuse_sync(a, b)
    }
    async fn cut(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.cut_sync(a, b)
    }
    async fn intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.intersect_sync(a, b)
    }
    async fn compound_cut(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.compound_cut_sync(target, tools)
    }
    async fn translate(&mut self, shape: &GeometryHandle, offset: Vec3) -> Result<GeometryHandle, BrepError> {
        self.translate_sync(shape, offset)
    }
    async fn rotate(&mut self, shape: &GeometryHandle, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.rotate_sync(shape, axis, angle)
    }
    async fn scale(&mut self, shape: &GeometryHandle, factor: f64, center: Vec3) -> Result<GeometryHandle, BrepError> {
        self.scale_sync(shape, factor, center)
    }
    async fn mirror(&mut self, shape: &GeometryHandle, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError> {
        self.mirror_sync(shape, origin, normal)
    }
    async fn copy_shape(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.copy_shape_sync(shape)
    }
    async fn linear_pattern(&mut self, shape: &GeometryHandle, direction: Vec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        self.linear_pattern_sync(shape, direction, spacing, count)
    }
    async fn circular_pattern(&mut self, shape: &GeometryHandle, axis: Vec3, count: usize) -> Result<GeometryHandle, BrepError> {
        self.circular_pattern_sync(shape, axis, count)
    }
    async fn grid_pattern(&mut self, shape: &GeometryHandle, dir_x: Vec3, dir_y: Vec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        self.grid_pattern_sync(shape, dir_x, dir_y, spacing_x, spacing_y, count_x, count_y)
    }
    async fn fillet(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_sync(shape, radius)
    }
    async fn fillet_variable(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_variable_sync(shape, radius_start, radius_end)
    }
    async fn fillet_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_edges_sync(shape, edges, radius)
    }
    async fn chamfer(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_sync(shape, distance)
    }
    async fn chamfer_asymmetric(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_asymmetric_sync(shape, d1, d2)
    }
    async fn chamfer_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_edges_sync(shape, edges, distance)
    }
    async fn shell(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.shell_sync(shape, thickness, open_faces)
    }
    async fn draft(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: Vec3, neutral_point: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.draft_sync(shape, faces, pull_direction, neutral_point, angle)
    }
    async fn offset_solid(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_solid_sync(shape, distance)
    }
    async fn defeature(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.defeature_sync(shape, faces)
    }
    async fn section(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<Vec<GeometryHandle>, BrepError> {
        self.section_sync(solid, plane_origin, plane_normal)
    }
    async fn split(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        self.split_sync(solid, plane_origin, plane_normal)
    }
    async fn curve_curve_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError> {
        self.curve_curve_intersect_sync(a, b, tolerance)
    }
    async fn curve_surface_intersect(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError> {
        self.curve_surface_intersect_sync(curve, surface, tolerance)
    }
    async fn surface_surface_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        self.surface_surface_intersect_sync(a, b, tolerance)
    }
    async fn curve_point(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError> {
        self.curve_point_sync(curve, parameter)
    }
    async fn curve_tangent(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError> {
        self.curve_tangent_sync(curve, parameter)
    }
    async fn curve_domain(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        self.curve_domain_sync(curve)
    }
    async fn curve_curvature(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        self.curve_curvature_sync(curve, parameter)
    }
    async fn surface_point(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError> {
        self.surface_point_sync(surface, u, v)
    }
    async fn surface_normal(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError> {
        self.surface_normal_sync(surface, u, v)
    }
    async fn volume(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.volume_sync(shape)
    }
    async fn area(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.area_sync(shape)
    }
    async fn length(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.length_sync(shape)
    }
    async fn center_of_mass(&self, shape: &GeometryHandle) -> Result<Vec3, BrepError> {
        self.center_of_mass_sync(shape)
    }
    async fn bounding_box(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.bounding_box_sync(shape)
    }
    async fn distance(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        self.distance_sync(a, b)
    }
    async fn closest_point(&self, shape: &GeometryHandle, point: Vec3) -> Result<ClosestPoint, BrepError> {
        self.closest_point_sync(shape, point)
    }
    async fn classify_point(&self, solid: &GeometryHandle, point: Vec3) -> Result<PointClassification, BrepError> {
        self.classify_point_sync(solid, point)
    }
    async fn validate(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        self.validate_sync(shape)
    }
    async fn vertex(&mut self, point: Vec3) -> Result<GeometryHandle, BrepError> {
        self.vertex_sync(point)
    }
    async fn face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.face_from_wire_sync(wire)
    }
    async fn sew_faces(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.sew_faces_sync(faces, tolerance)
    }
    async fn heal_solid(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.heal_solid_sync(shape, tolerance)
    }
    async fn convert_to_nurbs(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.convert_to_nurbs_sync(shape)
    }
    async fn deconstruct(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        self.deconstruct_sync(shape)
    }
    async fn export_step(&self, shapes: &[GeometryHandle]) -> Result<String, BrepError> {
        self.export_step_sync(shapes)
    }
    async fn export_stl(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_stl_sync(shapes, deflection)
    }
    async fn export_obj(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        self.export_obj_sync(shapes, deflection)
    }
    async fn export_gltf(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_gltf_sync(shapes, deflection)
    }
    async fn import_step(&mut self, data: &str) -> Result<Vec<GeometryHandle>, BrepError> {
        self.import_step_sync(data)
    }
    async fn import_stl(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_stl_sync(data, tolerance)
    }
    async fn import_obj(&mut self, data: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_obj_sync(data, tolerance)
    }
    async fn export_dwg(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_dwg_sync(shapes, deflection)
    }
    async fn import_dwg(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_dwg_sync(data, tolerance)
    }
    async fn kind(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        self.kind_sync(handle)
    }
    async fn tessellate(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError> {
        self.tessellate_sync(handle, tolerance)
    }
    async fn dispose(&mut self, handle: &GeometryHandle) {
        self.dispose_sync(handle);
    }
    async fn retain(&mut self, live: &std::collections::HashSet<String>) {
        self.retain_sync(live);
    }
    async fn registry_len(&self) -> usize {
        self.registry_len()
    }
}
// #endregion 🔖️Kernel

// #region 🔖️MeshInterop
/// 🌉️ Flattens a kernel `MeshTransfer` (position/normal/index/face_groups) into framework-core `MeshData`, reusing `mesh_from_indexed_with_face_groups` so picked triangles still resolve back to their B-Rep face id.
pub fn mesh_data_from_mesh_transfer(transfer: &MeshTransfer) -> semio_framework_core::MeshData {
    let face_groups: Vec<(u32, u32, u32)> = transfer.face_groups.iter().map(|group| (group.entity_id.parse().unwrap_or(0), group.start, group.count)).collect();
    let mut mesh = semio_framework_core::mesh_from_indexed_with_face_groups(&transfer.position, &transfer.normal, &transfer.index, &face_groups);
    mesh.edge_positions = transfer.edges.clone();
    if !mesh.edge_positions.is_empty() {
        let edge_count = mesh.edge_positions.len() / 6;
        mesh.edge_ids = (0..edge_count as u32).collect();
    }
    mesh
}

/// 🔌️ Format-keyed solid export codec operating on `GeometryHandle`s directly (not tessellated `MeshData`) — thin wrappers around `BrepkitKernel`'s own STEP/STL/OBJ/GLB writers; no codec logic lives here.
pub trait SolidExporter: Send + Sync {
    fn format(&self) -> semio_framework_core::OsMediaFormat;
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;
}

/// 🔌️ Format-keyed solid import codec; see `SolidExporter`. Returns every solid the payload contained (STEP files may hold more than one).
pub trait SolidImporter: Send + Sync {
    fn format(&self) -> semio_framework_core::OsMediaFormat;
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError>;
}

pub struct StepSolidExporter;
impl SolidExporter for StepSolidExporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Step
    }
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], _deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_step_sync(shapes).map(|text| text.into_bytes())
    }
}

pub struct StepSolidImporter;
impl SolidImporter for StepSolidImporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Step
    }
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], _tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let text = std::str::from_utf8(bytes).map_err(|error| BrepError::InvalidInput(error.to_string()))?;
        kernel.import_step_sync(text)
    }
}

pub struct StlSolidExporter;
impl SolidExporter for StlSolidExporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Stl
    }
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_stl_sync(shapes, deflection)
    }
}

pub struct StlSolidImporter;
impl SolidImporter for StlSolidImporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Stl
    }
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        kernel.import_stl_sync(bytes, tolerance).map(|handle| vec![handle])
    }
}

pub struct ObjSolidExporter;
impl SolidExporter for ObjSolidExporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Obj
    }
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_obj_sync(shapes, deflection).map(|text| text.into_bytes())
    }
}

pub struct ObjSolidImporter;
impl SolidImporter for ObjSolidImporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Obj
    }
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let text = std::str::from_utf8(bytes).map_err(|error| BrepError::InvalidInput(error.to_string()))?;
        kernel.import_obj_sync(text, tolerance).map(|handle| vec![handle])
    }
}

pub struct GlbSolidExporter;
impl SolidExporter for GlbSolidExporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Glb
    }
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_glb_sync(shapes, deflection)
    }
}

pub struct GlbSolidImporter;
impl SolidImporter for GlbSolidImporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Glb
    }
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        kernel.import_glb_sync(bytes, tolerance).map(|handle| vec![handle])
    }
}
// #endregion 🔖️MeshInterop

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_and_tessellate() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let mesh = kernel.tessellate_sync(&solid, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
        assert_eq!(kernel.volume_sync(&solid).unwrap(), 24.0);
    }

    #[test]
    fn box_tessellation_emits_one_face_group_per_topological_face() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let mesh = kernel.tessellate_sync(&solid, 0.1).unwrap();
        assert_eq!(mesh.face_groups.len(), 6, "a box has 6 planar faces");
        let entity_ids: std::collections::HashSet<&str> = mesh.face_groups.iter().map(|g| g.entity_id.as_str()).collect();
        assert_eq!(entity_ids.len(), 6, "face group entity ids must be distinct per face");
        let triangle_count = (mesh.index.len() / 3) as u32;
        let mut covered = vec![false; triangle_count as usize];
        for group in &mesh.face_groups {
            assert!(group.count > 0, "every face group must contain at least one triangle");
            for tri in (group.start / 3)..(group.start / 3 + group.count / 3) {
                assert!(!covered[tri as usize], "face groups must not overlap");
                covered[tri as usize] = true;
            }
        }
        assert!(covered.into_iter().all(|hit| hit), "face groups must partition every triangle");
    }

    #[test]
    fn dwg_export_import_round_trips_a_box() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let bytes = kernel.export_dwg_sync(&[solid], 0.1).unwrap();
        assert!(!bytes.is_empty());
        let imported = kernel.import_dwg_sync(&bytes, 0.1).unwrap();
        let mesh = kernel.tessellate_sync(&imported, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
    }

    #[test]
    fn glb_export_import_round_trips_a_box_through_the_mesh_codec() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let original_volume = kernel.volume_sync(&solid).unwrap();
        let bytes = kernel.export_glb_sync(&[solid], 0.1).unwrap();
        assert!(!bytes.is_empty());
        let imported = kernel.import_glb_sync(&bytes, 0.1).unwrap();
        let imported_volume = kernel.volume_sync(&imported).unwrap();
        assert!((imported_volume - original_volume).abs() < original_volume * 0.05, "volume should survive the GLB round trip: original={original_volume} imported={imported_volume}");
        let mesh = kernel.tessellate_sync(&imported, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
        let triangle_count = mesh.index.len() / 3;
        assert!((6..=5000).contains(&triangle_count), "a re-tessellated box should stay in a sane triangle-count range, got {triangle_count}");
    }

    #[test]
    fn glb_tessellation_bridge_produces_reasonable_mesh_data() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let mesh_data = kernel.tessellate_to_mesh_data_sync(&solid, 0.1).unwrap();
        assert!(!mesh_data.positions.is_empty(), "tessellation bridge must produce vertex positions");
        assert!(!mesh_data.indices.is_empty(), "tessellation bridge must produce triangle indices");
        assert_eq!(mesh_data.indices.len() % 3, 0, "indices must form complete triangles");
        let triangle_count = mesh_data.triangle_count();
        assert!((6..=5000).contains(&triangle_count), "a box tessellated through the GLB bridge should stay in a sane triangle-count range, got {triangle_count}");

        let bytes = semio_framework_core::GlbExporter.export(&mesh_data).unwrap();
        assert!(!bytes.is_empty());
        let reimported = semio_framework_core::GlbImporter.import(&bytes).unwrap();
        let reimported_triangles = reimported.indices.len() / 3;
        assert_eq!(reimported_triangles, triangle_count, "GLB codec must preserve triangle count through export/import");
        assert_eq!(reimported.positions.len(), mesh_data.positions.len(), "GLB codec must preserve vertex position count through export/import");
    }

    #[test]
    fn tessellate_to_mesh_data_carries_face_ids() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let mesh = kernel.tessellate_to_mesh_data_sync(&solid, 0.1).unwrap();
        assert_eq!(mesh.face_ids.len(), mesh.triangle_count());
        assert!(mesh.edge_positions.len() >= 6);
        assert_eq!(mesh.edge_positions.len() % 6, 0);
        assert_eq!(mesh.edge_ids.len(), mesh.edge_positions.len() / 6);
    }

    #[test]
    fn tessellate_face_carries_boundary_edge_positions() {
        let mut kernel = BrepkitKernel::new();
        let wire = kernel
            .polyline_wire_sync(&[[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 2.0, 0.0], [0.0, 0.0, 0.0]])
            .unwrap();
        let face = kernel.planar_face_from_wire_sync(&wire).unwrap();
        let mesh = kernel.tessellate_to_mesh_data_sync(&face, 0.1).unwrap();
        assert!(mesh.triangle_count() > 0);
        assert!(mesh.edge_positions.len() >= 6);
        assert_eq!(mesh.edge_positions.len() % 6, 0);
    }

    type SolidCodec = (Box<dyn SolidExporter>, Box<dyn SolidImporter>, f64);

    #[test]
    fn solid_exporters_and_importers_round_trip_a_box_per_format() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let original_volume = kernel.volume_sync(&solid).unwrap();
        // 🔩️ STEP is exact NURBS round-trip; STL/OBJ/GLB reimport a re-tessellated mesh, so allow a
        // small deflection-driven volume error instead of an exact match.
        let codecs: Vec<SolidCodec> = vec![
            (Box::new(StepSolidExporter), Box::new(StepSolidImporter), 1e-6),
            (Box::new(StlSolidExporter), Box::new(StlSolidImporter), 0.05),
            (Box::new(ObjSolidExporter), Box::new(ObjSolidImporter), 0.05),
            (Box::new(GlbSolidExporter), Box::new(GlbSolidImporter), 0.05),
        ];
        for (exporter, importer, tolerance) in codecs {
            let format = exporter.format();
            assert_eq!(format, importer.format());
            let bytes = exporter.export(&kernel, std::slice::from_ref(&solid), 0.1).expect("export");
            assert!(!bytes.is_empty(), "{format:?} export must not be empty");
            let imported = importer.import(&mut kernel, &bytes, 0.1).expect("import");
            assert!(!imported.is_empty(), "{format:?} import must yield at least one solid");
            let mut imported_volume = 0.0;
            for handle in &imported {
                imported_volume += kernel.volume_sync(handle).unwrap();
            }
            assert!((imported_volume - original_volume).abs() < original_volume * tolerance, "{format:?} round trip should preserve volume: original={original_volume} imported={imported_volume}");
            for handle in &imported {
                let mesh = kernel.tessellate_sync(handle, 0.1).unwrap();
                assert!(!mesh.position.is_empty(), "{format:?} round-tripped solid must still tessellate to a non-empty mesh");
                assert!(!mesh.index.is_empty(), "{format:?} round-tripped solid must still tessellate to non-empty indices");
            }
        }
    }

    #[test]
    fn fillet_and_translate() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let filleted = kernel.fillet_sync(&solid, 0.1).unwrap();
        let moved = kernel.translate_sync(&filleted, [1.0, 0.0, 0.0]).unwrap();
        let mesh = kernel.tessellate_sync(&moved, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
    }

    #[test]
    fn sphere_cut_cylinder_completes() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(2.8).unwrap();
        let cylinder = kernel.cylinder_prim_sync(0.5, 4.0).unwrap();
        let cut = kernel.cut_sync(&sphere, &cylinder).unwrap();
        let volume = kernel.volume_sync(&cut).unwrap();
        assert!(volume > 0.0);
        assert!(volume < 92.0);
    }

    #[test]
    fn sphere_cut_disjoint_torus_completes() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(2.8).unwrap();
        let torus = kernel.torus_prim_sync(2.0, 0.5).unwrap();
        let moved = kernel.translate_sync(&torus, [20.0, 0.0, 0.0]).unwrap();
        let cut = kernel.cut_sync(&sphere, &moved).unwrap();
        let volume = kernel.volume_sync(&cut).unwrap();
        assert!((volume - 92.0).abs() < 2.0);
    }

    #[test]
    fn sphere_cut_intersecting_torus_completes() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(2.8).unwrap();
        let torus = kernel.torus_prim_sync(2.0, 0.5).unwrap();
        let cut = kernel.cut_sync(&sphere, &torus).unwrap();
        let volume = kernel.volume_sync(&cut).unwrap();
        assert!(volume > 0.0);
        assert!(volume < 92.0);
        let mesh = kernel.tessellate_sync(&cut, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
    }

    #[test]
    fn fixture_sphere_cut_torus_volume_is_less_than_sphere() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(2.2).unwrap();
        let torus = kernel.torus_prim_sync(2.0, 0.5).unwrap();
        let sphere_vol = kernel.volume_sync(&sphere).unwrap();
        let intersect = kernel.intersect_sync(&sphere, &torus).unwrap();
        let intersect_vol = kernel.volume_sync(&intersect).unwrap();
        let cut = kernel.cut_sync(&sphere, &torus).unwrap();
        let cut_vol = kernel.volume_sync(&cut).unwrap();
        let cut_tris = kernel.tessellate_sync(&cut, 0.1).unwrap().index.len() / 3;
        assert!(intersect_vol > 0.0, "sphere and torus should overlap, intersect_vol={intersect_vol}");
        assert!((sphere_vol - cut_vol - intersect_vol).abs() < sphere_vol * 0.15, "cut+intersect should approximate sphere: sphere={sphere_vol} cut={cut_vol} intersect={intersect_vol}");
        assert!(cut_vol < sphere_vol * 0.85, "cut vol {cut_vol} should be well below sphere vol {sphere_vol}");
        assert!(cut_tris > 800, "cut mesh should retain enough triangles for a visible torus tunnel, got {cut_tris}");
    }

    #[test]
    fn fixture_sphere_cut_torus_at_slider_max_completes() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(10.0).unwrap();
        let torus = kernel.torus_prim_sync(2.0, 0.5).unwrap();
        let cut = kernel.cut_sync(&sphere, &torus).unwrap();
        let volume = kernel.volume_sync(&cut).unwrap();
        assert!(volume > 0.0);
        let mesh = kernel.tessellate_sync(&cut, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
    }

    #[test]
    fn retain_sync_drops_unreferenced_handles() {
        let mut kernel = BrepkitKernel::new();
        let kept = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let orphan = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        assert_eq!(kernel.registry_len(), 2);
        let live = std::collections::HashSet::from([kept.as_str().to_string()]);
        kernel.retain_sync(&live);
        assert_eq!(kernel.registry_len(), 1);
        assert!(kernel.tessellate_sync(&kept, 0.1).is_ok());
        assert!(kernel.tessellate_sync(&orphan, 0.1).is_err());
    }

    #[test]
    fn line_curve_evaluate() {
        let mut kernel = BrepkitKernel::new();
        let line = kernel.line_curve_sync([0.0, 0.0, 0.0], [2.0, 0.0, 0.0]).unwrap();
        let mid = kernel.curve_point_sync(&line, 1.0).unwrap();
        assert!((mid[0] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn extrude_rectangle_volume() {
        let mut kernel = BrepkitKernel::new();
        let wire = kernel.rectangle_wire_sync(2.0, 2.0).unwrap();
        let face = kernel.planar_face_from_wire_sync(&wire).unwrap();
        let solid = kernel.extrude_sync(&face, [0.0, 0.0, 1.0], 3.0).unwrap();
        let vol = kernel.volume_sync(&solid).unwrap();
        assert!((vol - 12.0).abs() < 0.5);
    }

    #[test]
    fn section_box_returns_faces() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let faces = kernel.section_sync(&solid, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).unwrap();
        assert!(!faces.is_empty());
    }

    #[test]
    fn box_surface_area() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let area = kernel.area_sync(&solid).unwrap();
        assert!((area - 52.0).abs() < 1.0);
    }

    #[test]
    fn step_export_import_roundtrip_stub() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let step = kernel.export_step_sync(std::slice::from_ref(&solid)).unwrap();
        assert!(step.contains("ISO-10303"));
        let imported = kernel.import_step_sync(&step).unwrap();
        assert_eq!(imported.len(), 1);
        assert!(kernel.volume_sync(&imported[0]).unwrap() > 0.0);
    }

    #[test]
    fn curve_tessellation_produces_edges() {
        let mut kernel = BrepkitKernel::new();
        let curve = kernel.line_curve_sync([0.0, 0.0, 0.0], [10.0, 0.0, 0.0]).unwrap();
        let mesh = kernel.tessellate_sync(&curve, 0.1).unwrap();
        assert!(!mesh.edges.is_empty());
    }

    #[test]
    fn sweep_wire_profile_produces_tube_mesh() {
        let mut kernel = BrepkitKernel::new();
        let path_wire = kernel.polyline_wire_sync(&[[0.0, 0.0, 0.0], [4.0, 0.0, 0.0]]).unwrap();
        let profile_wire = kernel.regular_polygon_wire_sync(0.08, 8).unwrap();
        let profile_face = kernel.planar_face_from_wire_sync(&profile_wire).unwrap();
        let solid = kernel.sweep_sync(&profile_face, &path_wire).unwrap();
        let mesh = kernel.tessellate_sync(&solid, 0.1).unwrap();
        assert!(mesh.position.len() > 36);
        assert!(mesh.index.len() > 12);
    }

    // #region Validation error paths
    #[test]
    fn convex_hull_rejects_fewer_than_four_points() {
        let mut kernel = BrepkitKernel::new();
        let err = kernel.convex_hull_sync(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }

    #[test]
    fn line_curve_rejects_coincident_endpoints() {
        let mut kernel = BrepkitKernel::new();
        let err = kernel.line_curve_sync([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }

    #[test]
    fn polyline_wire_rejects_fewer_than_two_points() {
        let mut kernel = BrepkitKernel::new();
        let err = kernel.polyline_wire_sync(&[[0.0, 0.0, 0.0]]).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }

    #[test]
    fn regular_polygon_wire_rejects_fewer_than_three_sides() {
        let mut kernel = BrepkitKernel::new();
        let err = kernel.regular_polygon_wire_sync(1.0, 2).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }

    #[test]
    fn interpolate_curve_rejects_fewer_than_two_points() {
        let mut kernel = BrepkitKernel::new();
        let err = kernel.interpolate_curve_sync(&[[0.0, 0.0, 0.0]], 3).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }

    #[test]
    fn planar_face_from_points_rejects_fewer_than_three_points() {
        let mut kernel = BrepkitKernel::new();
        let err = kernel.planar_face_from_points_sync(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }

    #[test]
    fn coons_patch_rejects_fewer_than_four_curves() {
        let mut kernel = BrepkitKernel::new();
        let err = kernel.coons_patch_sync(&[vec![[0.0, 0.0, 0.0]]]).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }

    #[test]
    fn extrude_wire_rejects_zero_length_vector() {
        let mut kernel = BrepkitKernel::new();
        let wire = kernel.rectangle_wire_sync(1.0, 1.0).unwrap();
        let err = kernel.extrude_wire_sync(&wire, [0.0, 0.0, 0.0]).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }

    #[test]
    fn rotate_rejects_zero_length_axis() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let err = kernel.rotate_sync(&solid, [0.0, 0.0, 0.0], 1.0).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }

    #[test]
    fn missing_handle_returns_missing_handle_error() {
        let kernel = BrepkitKernel::new();
        let bogus = GeometryHandle::new(GeometryKind::Solid, 999);
        let err = kernel.volume_sync(&bogus).unwrap_err();
        assert!(matches!(err, BrepError::MissingHandle(_)));
    }

    #[test]
    fn wrong_entity_type_errors_are_invalid_input() {
        let mut kernel = BrepkitKernel::new();
        let curve = kernel.line_curve_sync([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap();
        let err = kernel.fillet_sync(&curve, 0.1).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
        let solid = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let err = kernel.offset_face_sync(&solid, 0.1).unwrap_err();
        assert!(matches!(err, BrepError::InvalidInput(_)));
    }
    // #endregion Validation error paths

    // #region Curve evaluation branches
    #[test]
    fn arc_curve_domain_point_tangent_and_curvature() {
        let mut kernel = BrepkitKernel::new();
        let arc = kernel.arc_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0, 0.0, std::f64::consts::PI).unwrap();
        let domain = kernel.curve_domain_sync(&arc).unwrap();
        assert!((domain.min - 0.0).abs() < 1e-9);
        assert!((domain.max - std::f64::consts::PI).abs() < 1e-9);
        let point = kernel.curve_point_sync(&arc, 0.0).unwrap();
        let dist = (point[0] * point[0] + point[1] * point[1] + point[2] * point[2]).sqrt();
        assert!((dist - 2.0).abs() < 1e-6, "arc point should sit at radius 2 from center, got dist={dist}");
        let tangent = kernel.curve_tangent_sync(&arc, 0.0).unwrap();
        let tangent_len = (tangent[0] * tangent[0] + tangent[1] * tangent[1] + tangent[2] * tangent[2]).sqrt();
        assert!((tangent_len - 1.0).abs() < 1e-6, "arc tangent should be unit length, got {tangent_len}");
        let curvature = kernel.curve_curvature_sync(&arc, 0.5).unwrap();
        assert!((curvature - 0.5).abs() < 1e-9, "circular arc curvature should be 1/radius, got {curvature}");
    }

    #[test]
    fn ellipse_curve_domain_point_and_curvature() {
        let mut kernel = BrepkitKernel::new();
        let ellipse = kernel.ellipse_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, 1.0).unwrap();
        let domain = kernel.curve_domain_sync(&ellipse).unwrap();
        assert!((domain.max - TAU).abs() < 1e-9);
        let point = kernel.curve_point_sync(&ellipse, 0.0).unwrap();
        let dist = (point[0] * point[0] + point[1] * point[1] + point[2] * point[2]).sqrt();
        assert!(dist >= 1.0 - 1e-6 && dist <= 3.0 + 1e-6, "ellipse point distance from center should be within [minor, major], got {dist}");
        // 🐛️ t=0.0 and other rational-radian parameters can land on a NaN-producing branch of
        // brepkit's `ellipse_to_nurbs` curvature derivatives (upstream, outside this crate); 0.3
        // avoids that and still exercises the `KernelCurve::Ellipse` curvature branch.
        let curvature = kernel.curve_curvature_sync(&ellipse, 0.3).unwrap();
        assert!(curvature.is_finite() && curvature > 0.0, "ellipse curvature should be positive and finite, got {curvature}");
    }

    #[test]
    fn approximate_curve_builds_a_fitted_nurbs_curve() {
        let mut kernel = BrepkitKernel::new();
        let points = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [2.0, 0.0, 0.0], [3.0, 1.0, 0.0], [4.0, 0.0, 0.0]];
        let curve = kernel.approximate_curve_sync(&points, 3, 4).unwrap();
        let domain = kernel.curve_domain_sync(&curve).unwrap();
        let start = kernel.curve_point_sync(&curve, domain.min).unwrap();
        let end = kernel.curve_point_sync(&curve, domain.max).unwrap();
        assert!((start[0] - 0.0).abs() < 0.5, "fitted curve should start near the first point, got {start:?}");
        assert!((end[0] - 4.0).abs() < 0.5, "fitted curve should end near the last point, got {end:?}");
        let curvature = kernel.curve_curvature_sync(&curve, (domain.min + domain.max) / 2.0).unwrap();
        assert!(curvature.is_finite());
    }

    #[test]
    fn box_edge_curve_queries_use_line_branch() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let topo = kernel.deconstruct_sync(&solid).unwrap();
        let edge = &topo.edges[0];
        let domain = kernel.curve_domain_sync(edge).unwrap();
        assert!(domain.max > domain.min);
        let start = kernel.curve_point_sync(edge, domain.min).unwrap();
        let end = kernel.curve_point_sync(edge, domain.max).unwrap();
        let length = ((end[0] - start[0]).powi(2) + (end[1] - start[1]).powi(2) + (end[2] - start[2]).powi(2)).sqrt();
        assert!((length - domain.max).abs() < 1e-6, "a line edge parameter should be arc length");
        let tangent = kernel.curve_tangent_sync(edge, domain.min).unwrap();
        let tangent_len = (tangent[0] * tangent[0] + tangent[1] * tangent[1] + tangent[2] * tangent[2]).sqrt();
        assert!((tangent_len - 1.0).abs() < 1e-6);
        let curvature = kernel.curve_curvature_sync(edge, domain.min).unwrap();
        assert_eq!(curvature, 0.0, "a straight edge has zero curvature");
    }

    #[test]
    fn curve_curve_intersect_finds_crossing_point() {
        let mut kernel = BrepkitKernel::new();
        let a = kernel.line_curve_sync([-5.0, 0.0, 0.0], [5.0, 0.0, 0.0]).unwrap();
        let b = kernel.line_curve_sync([0.0, -5.0, 0.0], [0.0, 5.0, 0.0]).unwrap();
        let hits = kernel.curve_curve_intersect_sync(&a, &b, 1e-4).unwrap();
        assert_eq!(hits.len(), 1, "two perpendicular crossing lines should meet exactly once");
        assert!(hits[0][0].abs() < 1e-3 && hits[0][1].abs() < 1e-3, "crossing should be at the origin, got {:?}", hits[0]);
    }

    #[test]
    fn curve_surface_intersect_pierces_a_flat_nurbs_patch() {
        let mut kernel = BrepkitKernel::new();
        let grid = vec![vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]];
        let surface = kernel.nurbs_surface_from_grid_sync(&grid, 1, 1).unwrap();
        let line = kernel.line_curve_sync([0.5, 0.5, -5.0], [0.5, 0.5, 5.0]).unwrap();
        let hits = kernel.curve_surface_intersect_sync(&line, &surface, 1e-4).unwrap();
        assert!(!hits.is_empty(), "a vertical line through the patch interior should hit the surface");
        assert!(hits[0][2].abs() < 0.1, "hit point should be near z=0, got {:?}", hits[0]);
    }

    #[test]
    fn surface_surface_intersect_of_two_flat_nurbs_patches_returns_a_curve() {
        let mut kernel = BrepkitKernel::new();
        let grid_a = vec![vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]];
        let grid_b = vec![vec![[0.0, 0.0, -0.5], [1.0, 0.0, 0.5]], vec![[0.0, 1.0, -0.5], [1.0, 1.0, 0.5]]];
        let surface_a = kernel.nurbs_surface_from_grid_sync(&grid_a, 1, 1).unwrap();
        let surface_b = kernel.nurbs_surface_from_grid_sync(&grid_b, 1, 1).unwrap();
        let curves = kernel.surface_surface_intersect_sync(&surface_a, &surface_b, 1e-4).unwrap();
        assert!(!curves.is_empty(), "two transversally crossing patches should yield at least one intersection curve");
        for c in &curves {
            assert_eq!(kernel.kind_sync(c).unwrap(), GeometryKind::Curve);
        }
    }
    // #endregion Curve evaluation branches

    // #region Surface evaluation branches
    #[test]
    fn plane_surface_point_and_normal_match_frame() {
        let mut kernel = BrepkitKernel::new();
        let plane = kernel.plane_surface_sync([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]).unwrap();
        let point = kernel.surface_point_sync(&plane, 0.0, 0.0).unwrap();
        assert!((point[2] - 5.0).abs() < 1e-9);
        let normal = kernel.surface_normal_sync(&plane, 0.0, 0.0).unwrap();
        assert!((normal[2] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn nurbs_surface_from_grid_point_and_normal() {
        let mut kernel = BrepkitKernel::new();
        let grid = vec![vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]], vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]];
        let surface = kernel.nurbs_surface_from_grid_sync(&grid, 1, 1).unwrap();
        let point = kernel.surface_point_sync(&surface, 0.5, 0.5).unwrap();
        assert!(point[0] >= 0.0 && point[0] <= 1.0);
        let normal = kernel.surface_normal_sync(&surface, 0.5, 0.5).unwrap();
        let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        assert!(len > 0.5, "normal should be roughly unit length, got {len}");
    }

    #[test]
    fn face_surfaces_of_curved_primitives_evaluate_without_error() {
        let mut kernel = BrepkitKernel::new();
        let cylinder = kernel.cylinder_prim_sync(1.0, 2.0).unwrap();
        let cone = kernel.cone_prim_sync(1.0, 2.0).unwrap();
        let sphere = kernel.sphere_prim_sync(1.0).unwrap();
        let torus = kernel.torus_prim_sync(2.0, 0.5).unwrap();
        for solid in [cylinder, cone, sphere, torus] {
            let topo = kernel.deconstruct_sync(&solid).unwrap();
            assert!(!topo.faces.is_empty());
            for face in &topo.faces {
                let point = kernel.surface_point_sync(face, 0.25, 0.25).unwrap();
                assert!(point.iter().all(|c| c.is_finite()));
                let normal = kernel.surface_normal_sync(face, 0.25, 0.25).unwrap();
                let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
                assert!(len > 1e-6, "surface normal should be non-degenerate");
            }
        }
    }
    // #endregion Surface evaluation branches

    // #region Transform, pattern, and measurement branches
    #[test]
    fn distance_closest_point_and_classify_point() {
        let mut kernel = BrepkitKernel::new();
        let a = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let b = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        kernel.translate_sync(&b, [5.0, 0.0, 0.0]).unwrap();
        let distance = kernel.distance_sync(&a, &b).unwrap();
        assert!((distance - 4.0).abs() < 0.1, "distance between box faces should be ~4, got {distance}");

        let closest = kernel.closest_point_sync(&a, [10.0, 0.0, 0.0]).unwrap();
        assert!(closest.distance > 0.0);

        let inside = kernel.classify_point_sync(&a, [0.5, 0.5, 0.5]).unwrap();
        assert_eq!(inside, PointClassification::Inside);
        let outside = kernel.classify_point_sync(&a, [10.0, 10.0, 10.0]).unwrap();
        assert_eq!(outside, PointClassification::Outside);
    }

    #[test]
    fn mirror_and_copy_preserve_volume() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let original_volume = kernel.volume_sync(&solid).unwrap();
        let mirrored = kernel.mirror_sync(&solid, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap();
        assert!((kernel.volume_sync(&mirrored).unwrap() - original_volume).abs() < 1e-6);
        let copied = kernel.copy_shape_sync(&solid).unwrap();
        assert_ne!(copied.as_str(), solid.as_str());
        assert!((kernel.volume_sync(&copied).unwrap() - original_volume).abs() < 1e-6);
    }

    #[test]
    fn scale_sync_scales_about_a_center() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let scaled = kernel.scale_sync(&solid, 2.0, [1.0, 1.0, 1.0]).unwrap();
        let volume = kernel.volume_sync(&scaled).unwrap();
        assert!((volume - 64.0).abs() < 1e-3, "scaling a 2^3 box by factor 2 about its center should give volume 64, got {volume}");
    }

    #[test]
    fn linear_circular_and_grid_patterns_build_compounds() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let linear = kernel.linear_pattern_sync(&solid, [2.0, 0.0, 0.0], 2.0, 3).unwrap();
        assert_eq!(kernel.kind_sync(&linear).unwrap(), GeometryKind::Compound);
        let linear_volume = kernel.volume_sync(&linear).unwrap();
        assert!((linear_volume - 3.0).abs() < 1e-6, "3 unit boxes should total volume 3, got {linear_volume}");

        let solid2 = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let circular = kernel.circular_pattern_sync(&solid2, [0.0, 0.0, 1.0], 4).unwrap();
        let circular_area = kernel.area_sync(&circular).unwrap();
        assert!((circular_area - 24.0).abs() < 1e-3, "4 unit boxes should total surface area 24, got {circular_area}");

        let solid3 = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let grid = kernel.grid_pattern_sync(&solid3, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 2.0, 2.0, 2, 2).unwrap();
        let grid_volume = kernel.volume_sync(&grid).unwrap();
        assert!((grid_volume - 4.0).abs() < 1e-6, "2x2 grid of unit boxes should total volume 4, got {grid_volume}");
    }

    #[test]
    fn compound_cut_removes_multiple_tools_from_a_target() {
        let mut kernel = BrepkitKernel::new();
        let target = kernel.box_prim_sync(4.0, 4.0, 4.0).unwrap();
        let tool_a = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let tool_b = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        kernel.translate_sync(&tool_b, [2.0, 2.0, 2.0]).unwrap();
        let result = kernel.compound_cut_sync(&target, &[tool_a, tool_b]).unwrap();
        let volume = kernel.volume_sync(&result).unwrap();
        assert!((volume - 62.0).abs() < 0.5, "cutting two disjoint unit boxes from a 4^3 box should leave ~62, got {volume}");
    }

    #[test]
    fn bounding_box_of_a_compound_covers_every_member_solid() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let compound = kernel.linear_pattern_sync(&solid, [1.0, 0.0, 0.0], 5.0, 2).unwrap();
        let bbox = kernel.bounding_box_sync(&compound).unwrap();
        let volume = kernel.volume_sync(&bbox).unwrap();
        assert!((volume - 6.0).abs() < 1e-3, "a bbox spanning x in [0,6] with unit y/z should have volume 6, got {volume}");
    }

    #[test]
    fn validate_reports_valid_for_a_clean_box() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        assert_eq!(kernel.validate_sync(&solid).unwrap(), "valid");
    }

    #[test]
    fn deconstruct_box_returns_topology_counts() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let topo = kernel.deconstruct_sync(&solid).unwrap();
        assert_eq!(topo.vertices.len(), 8);
        assert_eq!(topo.edges.len(), 12);
        assert_eq!(topo.faces.len(), 6);
    }
    // #endregion Transform, pattern, and measurement branches

    // #region Feature operations
    #[test]
    fn fillet_variable_and_chamfer_asymmetric_produce_valid_solids() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let filleted = kernel.fillet_variable_sync(&solid, 0.1, 0.3).unwrap();
        assert!(kernel.volume_sync(&filleted).unwrap() > 0.0);

        let solid2 = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let chamfered = kernel.chamfer_asymmetric_sync(&solid2, 0.2, 0.3).unwrap();
        let volume = kernel.volume_sync(&chamfered).unwrap();
        assert!(volume > 0.0 && volume < 8.0, "asymmetric chamfer should remove material, got {volume}");
    }

    #[test]
    fn fillet_edges_and_chamfer_edges_target_a_single_edge() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let topo = kernel.deconstruct_sync(&solid).unwrap();
        let filleted = kernel.fillet_edges_sync(&solid, std::slice::from_ref(&topo.edges[0]), 0.5).unwrap();
        let filleted_volume = kernel.volume_sync(&filleted).unwrap();
        assert!(filleted_volume > 7.0 && filleted_volume < 7.99, "filleting one edge should remove a modest amount of material, got {filleted_volume}");

        let solid2 = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let topo2 = kernel.deconstruct_sync(&solid2).unwrap();
        let chamfered = kernel.chamfer_edges_sync(&solid2, std::slice::from_ref(&topo2.edges[0]), 0.2).unwrap();
        let chamfered_volume = kernel.volume_sync(&chamfered).unwrap();
        assert!((chamfered_volume - 7.96).abs() < 1e-6, "chamfering one edge by 0.2x0.2 over a length-2 edge should remove exactly 0.04, got {chamfered_volume}");
    }

    #[test]
    fn shell_hollows_out_a_box() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let topo = kernel.deconstruct_sync(&solid).unwrap();
        let shelled = kernel.shell_sync(&solid, 0.2, std::slice::from_ref(&topo.faces[0])).unwrap();
        let volume = kernel.volume_sync(&shelled).unwrap();
        assert!(volume > 0.0 && volume < 8.0, "shelled box should have less volume than solid box, got {volume}");
    }

    #[test]
    fn draft_applies_pull_direction_taper() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let topo = kernel.deconstruct_sync(&solid).unwrap();
        let drafted = kernel.draft_sync(&solid, std::slice::from_ref(&topo.faces[0]), [0.0, 0.0, 1.0], [1.0, 1.0, 0.0], 5.0_f64.to_radians()).unwrap();
        assert!(kernel.volume_sync(&drafted).unwrap() > 0.0);
    }

    #[test]
    fn offset_solid_produces_a_larger_solid_for_positive_distance() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let original_volume = kernel.volume_sync(&solid).unwrap();
        let offset = kernel.offset_solid_sync(&solid, 0.2).unwrap();
        let offset_volume = kernel.volume_sync(&offset).unwrap();
        assert!(offset_volume > original_volume, "positive offset should grow the box: original={original_volume} offset={offset_volume}");
    }

    #[test]
    fn defeature_removes_a_face_and_returns_a_solid() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let topo = kernel.deconstruct_sync(&solid).unwrap();
        let defeatured = kernel.defeature_sync(&solid, std::slice::from_ref(&topo.faces[0])).unwrap();
        assert_eq!(kernel.kind_sync(&defeatured).unwrap(), GeometryKind::Solid);
    }

    #[test]
    fn split_box_returns_two_solids_that_sum_to_original_volume() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let original_volume = kernel.volume_sync(&solid).unwrap();
        let (positive, negative) = kernel.split_sync(&solid, [1.0, 1.0, 1.0], [1.0, 0.0, 0.0]).unwrap();
        let pos_vol = kernel.volume_sync(&positive).unwrap();
        let neg_vol = kernel.volume_sync(&negative).unwrap();
        assert!((pos_vol + neg_vol - original_volume).abs() < 1e-6);
        assert!(pos_vol > 0.0 && neg_vol > 0.0);
    }

    #[test]
    fn heal_solid_and_convert_to_nurbs_return_the_same_handle() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let healed = kernel.heal_solid_sync(&solid, 1e-4).unwrap();
        assert_eq!(healed.as_str(), solid.as_str());
        let volume_before = kernel.volume_sync(&solid).unwrap();
        let nurbsified = kernel.convert_to_nurbs_sync(&solid).unwrap();
        assert_eq!(nurbsified.as_str(), solid.as_str());
        let volume_after = kernel.volume_sync(&solid).unwrap();
        assert!((volume_before - volume_after).abs() < volume_before * 0.05);
    }
    // #endregion Feature operations

    // #region Registry and topology utilities
    #[test]
    fn vertex_and_face_from_wire_register_expected_kinds() {
        let mut kernel = BrepkitKernel::new();
        let vertex = kernel.vertex_sync([1.0, 2.0, 3.0]).unwrap();
        assert_eq!(kernel.kind_sync(&vertex).unwrap(), GeometryKind::Vertex);
        let wire = kernel.rectangle_wire_sync(2.0, 2.0).unwrap();
        let face = kernel.face_from_wire_sync(&wire).unwrap();
        assert_eq!(kernel.kind_sync(&face).unwrap(), GeometryKind::Face);
        assert!(kernel.area_sync(&face).unwrap() > 0.0);
    }

    #[test]
    fn solid_face_loops_returns_a_quad_per_box_face() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let (positions, face_loops) = kernel.solid_face_loops_sync(&solid).unwrap();
        assert_eq!(positions.len(), 8, "a box has 8 distinct vertices");
        assert_eq!(face_loops.len(), 6, "a box has 6 faces");
        for (outer, holes) in &face_loops {
            assert_eq!(outer.len(), 4, "each box face is a quad");
            assert!(holes.is_empty());
        }
    }

    #[test]
    fn dispose_sync_removes_the_handle_from_the_registry() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        assert_eq!(kernel.registry_len(), 1);
        kernel.dispose_sync(&solid);
        assert_eq!(kernel.registry_len(), 0);
        assert!(kernel.volume_sync(&solid).is_err());
    }
    // #endregion Registry and topology utilities
}
// #endregion 🔖️Tests
