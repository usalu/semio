//! 🧭️ 3×3/4×4 matrices, unit quaternions, rigid+uniform-scale transforms and orthonormal frames —
//! plain `f64` arrays, no external linear-algebra crate. `Trsf` (not a raw `Mat4`) is the type
//! every kernel operation accepts for placement, so a transform can never silently carry
//! non-uniform shear that would break analytic-surface recognition downstream.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🔢️matrix` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `➡️vector` (its sole dependency) since no target stub was pre-mounted for it.

use super::{Pnt3, Vec3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::polynomial::solve_cubic;

// #region 🔖️Mat

/// 🧭️ A 3×3 matrix in row-major order, used for rotations and normal-transform cofactors.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Mat3 {
    pub rows: [[f64; 3]; 3],
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Mat3 { rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_rows(rows: [[f64; 3]; 3]) -> Self {
        Mat3 { rows }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_columns(c0: Vec3, c1: Vec3, c2: Vec3) -> Self {
        Mat3 { rows: [[c0.x, c1.x, c2.x], [c0.y, c1.y, c2.y], [c0.z, c1.z, c2.z]] }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn column(&self, i: usize) -> Vec3 {
        Vec3::new(self.rows[0][i], self.rows[1][i], self.rows[2][i])
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn transform(&self, v: Vec3) -> Vec3 {
        Vec3::new(self.rows[0][0] * v.x + self.rows[0][1] * v.y + self.rows[0][2] * v.z, self.rows[1][0] * v.x + self.rows[1][1] * v.y + self.rows[1][2] * v.z, self.rows[2][0] * v.x + self.rows[2][1] * v.y + self.rows[2][2] * v.z)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn transpose(&self) -> Mat3 {
        Mat3::from_rows([[self.rows[0][0], self.rows[1][0], self.rows[2][0]], [self.rows[0][1], self.rows[1][1], self.rows[2][1]], [self.rows[0][2], self.rows[1][2], self.rows[2][2]]])
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn mul(&self, o: &Mat3) -> Mat3 {
        let mut out = [[0.0; 3]; 3];
        for (r, out_row) in out.iter_mut().enumerate() {
            for (c, out_cell) in out_row.iter_mut().enumerate() {
                *out_cell = (0..3).map(|k| self.rows[r][k] * o.rows[k][c]).sum();
            }
        }
        Mat3::from_rows(out)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn determinant(&self) -> f64 {
        let m = &self.rows;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }
    /// 🧭️ From an axis-angle rotation (Rodrigues' formula). `axis` need not be normalized.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_axis_angle(axis: Vec3, angle: f64) -> Mat3 {
        let a = axis.normalized().unwrap_or(Vec3::Z);
        let (s, c) = angle.sin_cos();
        let t = 1.0 - c;
        Mat3::from_rows([[t * a.x * a.x + c, t * a.x * a.y - s * a.z, t * a.x * a.z + s * a.y], [t * a.x * a.y + s * a.z, t * a.y * a.y + c, t * a.y * a.z - s * a.x], [t * a.x * a.z - s * a.y, t * a.y * a.z + s * a.x, t * a.z * a.z + c]])
    }
    /// 🧭️ The diagonal matrix `diag(d.x, d.y, d.z)` — an axis-aligned (possibly non-uniform) scale.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_diagonal(d: Vec3) -> Mat3 {
        Mat3::from_rows([[d.x, 0.0, 0.0], [0.0, d.y, 0.0], [0.0, 0.0, d.z]])
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn scaled(&self, s: f64) -> Mat3 {
        let mut out = self.rows;
        for row in out.iter_mut() {
            for v in row.iter_mut() {
                *v *= s;
            }
        }
        Mat3::from_rows(out)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sub(&self, o: &Mat3) -> Mat3 {
        let mut out = [[0.0; 3]; 3];
        for r in 0..3 {
            for c in 0..3 {
                out[r][c] = self.rows[r][c] - o.rows[r][c];
            }
        }
        Mat3::from_rows(out)
    }
    /// 🧭️ The cofactor matrix `C` with `C[i][j] = (-1)^(i+j) · minor(i, j)`; `Cᵀ / det` is `self`'s
    /// inverse, and `C` alone (unscaled by `det`) is the standard robust normal-transform matrix
    /// `(M⁻¹)ᵀ · det` — used by [`Affine3::apply_normal`] so a normal transform never divides by a
    /// near-zero determinant.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cofactor(&self) -> Mat3 {
        let m = &self.rows;
        let minor = |r0: usize, r1: usize, c0: usize, c1: usize| m[r0][c0] * m[r1][c1] - m[r0][c1] * m[r1][c0];
        Mat3::from_rows([
            [minor(1, 2, 1, 2), -minor(1, 2, 0, 2), minor(1, 2, 0, 1)],
            [-minor(0, 2, 1, 2), minor(0, 2, 0, 2), -minor(0, 2, 0, 1)],
            [minor(0, 1, 1, 2), -minor(0, 1, 0, 2), minor(0, 1, 0, 1)],
        ])
    }
}

// #endregion 🔖️Mat

// #region 🔖️Quat

/// 🧭️ A unit quaternion `(w, x, y, z)` representing a pure rotation.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Quat {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quat {
    pub const IDENTITY: Quat = Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_axis_angle(axis: Vec3, angle: f64) -> Self {
        let a = axis.normalized().unwrap_or(Vec3::Z);
        let half = angle * 0.5;
        let (s, c) = half.sin_cos();
        Quat { w: c, x: a.x * s, y: a.y * s, z: a.z * s }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn norm(self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn normalized(self) -> Quat {
        let n = self.norm();
        if n <= f64::EPSILON {
            Quat::IDENTITY
        } else {
            Quat { w: self.w / n, x: self.x / n, y: self.y / n, z: self.z / n }
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn conjugate(self) -> Quat {
        Quat { w: self.w, x: -self.x, y: -self.y, z: -self.z }
    }
    #[allow(clippy::should_implement_trait)]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn mul(self, o: Quat) -> Quat {
        Quat {
            w: self.w * o.w - self.x * o.x - self.y * o.y - self.z * o.z,
            x: self.w * o.x + self.x * o.w + self.y * o.z - self.z * o.y,
            y: self.w * o.y - self.x * o.z + self.y * o.w + self.z * o.x,
            z: self.w * o.z + self.x * o.y - self.y * o.x + self.z * o.w,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rotate(self, v: Vec3) -> Vec3 {
        let qv = Quat { w: 0.0, x: v.x, y: v.y, z: v.z };
        let r = self.mul(qv).mul(self.conjugate());
        Vec3::new(r.x, r.y, r.z)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_mat3(self) -> Mat3 {
        let q = self.normalized();
        let (w, x, y, z) = (q.w, q.x, q.y, q.z);
        Mat3::from_rows([
            [1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y - z * w), 2.0 * (x * z + y * w)],
            [2.0 * (x * y + z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z - x * w)],
            [2.0 * (x * z - y * w), 2.0 * (y * z + x * w), 1.0 - 2.0 * (x * x + y * y)],
        ])
    }
    /// 🧭️ Recovers the unit quaternion for a proper rotation matrix (`det ≈ +1`), via Shepperd's
    /// branch-on-largest-diagonal method for numerical stability near every rotation angle.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_mat3(m: &Mat3) -> Quat {
        let r = &m.rows;
        let trace = r[0][0] + r[1][1] + r[2][2];
        let q = if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            Quat { w: 0.25 * s, x: (r[2][1] - r[1][2]) / s, y: (r[0][2] - r[2][0]) / s, z: (r[1][0] - r[0][1]) / s }
        } else if r[0][0] > r[1][1] && r[0][0] > r[2][2] {
            let s = (1.0 + r[0][0] - r[1][1] - r[2][2]).sqrt() * 2.0;
            Quat { w: (r[2][1] - r[1][2]) / s, x: 0.25 * s, y: (r[0][1] + r[1][0]) / s, z: (r[0][2] + r[2][0]) / s }
        } else if r[1][1] > r[2][2] {
            let s = (1.0 + r[1][1] - r[0][0] - r[2][2]).sqrt() * 2.0;
            Quat { w: (r[0][2] - r[2][0]) / s, x: (r[0][1] + r[1][0]) / s, y: 0.25 * s, z: (r[1][2] + r[2][1]) / s }
        } else {
            let s = (1.0 + r[2][2] - r[0][0] - r[1][1]).sqrt() * 2.0;
            Quat { w: (r[1][0] - r[0][1]) / s, x: (r[0][2] + r[2][0]) / s, y: (r[1][2] + r[2][1]) / s, z: 0.25 * s }
        };
        q.normalized()
    }
    /// 🧭️ Spherical linear interpolation, taking the short arc between `self` and `o`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Trsf {
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: f64,
}

impl Trsf {
    pub const IDENTITY: Trsf = Trsf { rotation: Quat::IDENTITY, translation: Vec3::ZERO, scale: 1.0 };

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn translation(t: Vec3) -> Self {
        Trsf { translation: t, ..Trsf::IDENTITY }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rotation(q: Quat) -> Self {
        Trsf { rotation: q, ..Trsf::IDENTITY }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn uniform_scale(s: f64) -> Self {
        Trsf { scale: s, ..Trsf::IDENTITY }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply_point(&self, p: Pnt3) -> Pnt3 {
        let scaled = p.to_vec() * self.scale;
        Pnt3::from_array(self.rotation.rotate(scaled).to_array()) + self.translation
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply_vector(&self, v: Vec3) -> Vec3 {
        self.rotation.rotate(v * self.scale)
    }
    /// 🧭️ Transforms a surface normal correctly under non-uniform-free `Trsf` (uniform scale
    /// leaves direction unchanged, so this is just the rotation — kept as its own method so
    /// callers never reach for `apply_vector` on a normal by habit).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply_normal(&self, n: Vec3) -> Vec3 {
        self.rotation.rotate(n)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn semio_compose_rs(&self, inner: &Trsf) -> Trsf {
        Trsf { rotation: self.rotation.mul(inner.rotation), translation: self.apply_vector(inner.translation) + self.translation, scale: self.scale * inner.scale }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self) -> Trsf {
        let inv_rot = self.rotation.conjugate();
        let inv_scale = 1.0 / self.scale;
        let inv_translation = inv_rot.rotate(-self.translation * inv_scale);
        Trsf { rotation: inv_rot, translation: inv_translation, scale: inv_scale }
    }
}

// #endregion 🔖️Trsf

// #region 🔖️Affine

/// 🧭️ The full affine group `p ↦ linear·p + translation` — mirror (`det < 0`) and non-uniform
/// scale/shear included, unlike [`Trsf`] (kept for callers that only ever need rigid+uniform-scale
/// and rely on that restriction to keep analytic surfaces analytic by construction). Every exact
/// B-Rep transform (`transform_solid` and its `Curve3`/`Surface::transformed` building blocks)
/// accepts `Affine3` instead: [`Self::is_similarity`] is the single gate a caller consults to
/// decide whether a curve/surface can stay in its own analytic representation or must convert to
/// NURBS to stay exact.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Affine3 {
    pub linear: Mat3,
    pub translation: Vec3,
}

impl Affine3 {
    pub const IDENTITY: Affine3 = Affine3 { linear: Mat3::IDENTITY, translation: Vec3::ZERO };

    /// 🧭️ Lifts a rigid+uniform-scale [`Trsf`] into the affine group.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_trsf(t: &Trsf) -> Self {
        Affine3 { linear: t.rotation.to_mat3().scaled(t.scale), translation: t.translation }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn translation(t: Vec3) -> Self {
        Affine3 { translation: t, ..Affine3::IDENTITY }
    }
    /// 🧭️ A pure rotation about the world origin — compose with [`Self::translation`] (or use
    /// [`Self::rotation_about`] directly) for rotation about an arbitrary point.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rotation_axis_angle(axis: Vec3, angle: f64) -> Self {
        Affine3 { linear: Mat3::from_axis_angle(axis, angle), translation: Vec3::ZERO }
    }
    /// 🧭️ A rotation about an explicit `origin` — the origin is the map's one fixed point.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rotation_about(origin: Pnt3, axis: Vec3, angle: f64) -> Self {
        let linear = Mat3::from_axis_angle(axis, angle);
        let t = origin.to_vec() - linear.transform(origin.to_vec());
        Affine3 { linear, translation: t }
    }
    /// 🧭️ Per-axis (possibly non-uniform) scale by `factors` about `center` — `center` is the
    /// map's one fixed point. `factors` uses the world axes; compose with a rotation for scale
    /// along arbitrary axes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn scaling(center: Pnt3, factors: Vec3) -> Self {
        let linear = Mat3::from_diagonal(factors);
        let t = center.to_vec() - linear.transform(center.to_vec());
        Affine3 { linear, translation: t }
    }
    /// 🧭️ Reflection across the plane through `origin` with unit `normal` (Householder matrix
    /// `I - 2·n·nᵀ`); `origin` is a fixed point of the map.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn mirror(origin: Pnt3, normal: Vec3) -> Self {
        let n = normal.normalized().unwrap_or(Vec3::Z);
        let outer = Mat3::from_rows([[n.x * n.x, n.x * n.y, n.x * n.z], [n.y * n.x, n.y * n.y, n.y * n.z], [n.z * n.x, n.z * n.y, n.z * n.z]]);
        let linear = Mat3::IDENTITY.sub(&outer.scaled(2.0));
        let t = origin.to_vec() - linear.transform(origin.to_vec());
        Affine3 { linear, translation: t }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply_point(&self, p: Pnt3) -> Pnt3 {
        Pnt3::from_array(self.linear.transform(p.to_vec()).to_array()) + self.translation
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply_vector(&self, v: Vec3) -> Vec3 {
        self.linear.transform(v)
    }
    /// 🧭️ Transforms a surface/vertex normal by the cofactor matrix (`(linear⁻¹)ᵀ` scaled by
    /// `det`, robust to a near-singular `linear` since it never divides by `det`) — the standard
    /// normal-transform law under a general (non-conformal) linear map; direction-only, so callers
    /// normalize the result. A `det < 0` (mirror/reflection) flips the returned direction, which
    /// is why every caller that needs a consistently-outward face normal also flips `Face::flipped`
    /// when `self.determinant() < 0`, rather than expecting this method to compensate.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply_normal(&self, n: Vec3) -> Vec3 {
        self.linear.cofactor().transform(n)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn determinant(&self) -> f64 {
        self.linear.determinant()
    }
    /// 🧭️ `self ∘ inner`: applying the result to a point matches applying `inner` then `self`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(&self, inner: &Affine3) -> Affine3 {
        Affine3 { linear: self.linear.mul(&inner.linear), translation: self.apply_vector(inner.translation) + self.translation }
    }
    /// 🧭️ `None` iff `linear` is (numerically) singular.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(&self) -> Option<Affine3> {
        let det = self.linear.determinant();
        if det.abs() <= 1e-300 {
            return None;
        }
        let inv_linear = self.linear.cofactor().transpose().scaled(1.0 / det);
        let inv_translation = -inv_linear.transform(self.translation);
        Some(Affine3 { linear: inv_linear, translation: inv_translation })
    }
    /// 🧭️ `Some((rotation, uniform_scale, is_reflection))` when `linear` is `uniform_scale ·
    /// rotation` (optionally composed with a fixed reflection, iff `is_reflection`) — i.e. exactly
    /// the maps that keep an analytic curve/surface's own kind analytic after [`Curve3::transformed`]
    /// / [`Surface::transformed`] ([`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3`] /
    /// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface`]). `None` for shear or non-uniform scale, which
    /// force a NURBS conversion instead. Decomposition: `uniform_scale = |det|^(1/3)`;
    /// `linear/uniform_scale` must be orthogonal (checked numerically) to qualify at all; a
    /// negative `det` means that orthogonal matrix is an improper rotation (`det = -1`), factored
    /// as `rotation · diag(1,1,-1)` so `rotation` itself is always a proper (`det = +1`) quaternion.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_similarity(&self) -> Option<(Quat, f64, bool)> {
        let det = self.linear.determinant();
        if det.abs() <= 1e-12 {
            return None;
        }
        let scale = det.abs().cbrt();
        if scale <= f64::EPSILON {
            return None;
        }
        let normalized = self.linear.scaled(1.0 / scale);
        let (c0, c1, c2) = (normalized.column(0), normalized.column(1), normalized.column(2));
        let tol = 1e-7;
        let unit_ok = (c0.norm_sq() - 1.0).abs() < tol && (c1.norm_sq() - 1.0).abs() < tol && (c2.norm_sq() - 1.0).abs() < tol;
        let perp_ok = c0.dot(c1).abs() < tol && c0.dot(c2).abs() < tol && c1.dot(c2).abs() < tol;
        if !unit_ok || !perp_ok {
            return None;
        }
        let is_reflection = det < 0.0;
        let rotation_matrix = if is_reflection {
            let flip = Mat3::from_rows([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]]);
            normalized.mul(&flip)
        } else {
            normalized
        };
        Some((Quat::from_mat3(&rotation_matrix), scale, is_reflection))
    }
    /// 🧭️ The largest singular value of `linear` — how much the map can stretch a unit vector in
    /// its worst direction, the correct factor to scale a tolerance radius by after transforming
    /// the geometry it bounds. Exact and cheap (`= uniform_scale`) for a similarity; otherwise the
    /// largest root of `MᵀM`'s cubic characteristic polynomial (`M = linear`, symmetric positive
    /// semi-definite, so every root is real and ≥ 0), square-rooted.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn max_singular_value(&self) -> f64 {
        if let Some((_, scale, _)) = self.is_similarity() {
            return scale;
        }
        let mtm = self.linear.transpose().mul(&self.linear);
        let m = &mtm.rows;
        let trace = m[0][0] + m[1][1] + m[2][2];
        let minor = |i: usize, j: usize| m[i][i] * m[j][j] - m[i][j] * m[j][i];
        let sum_minors = minor(0, 1) + minor(0, 2) + minor(1, 2);
        let det = mtm.determinant();
        let roots = solve_cubic(1.0, -trace, sum_minors, -det);
        roots.into_iter().fold(0.0_f64, f64::max).max(0.0).sqrt()
    }
}

// #endregion 🔖️Affine

// #region 🔖️Frame

/// 🧭️ A right-handed orthonormal frame: origin plus three unit axes with `z = x × y`.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_normal(origin: Pnt3, normal: Vec3) -> Option<Frame3> {
        let z = normal.normalized()?;
        let x = z.any_orthogonal();
        let y = z.cross(x);
        Some(Frame3 { origin, x, y, z })
    }
    /// 🧭️ Builds a frame from an origin, a preferred `x` direction and a `z` normal — `x` is
    /// Gram-Schmidt orthogonalized against `z`, `y` completes the right-handed triad.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_x_z(origin: Pnt3, x_hint: Vec3, z_hint: Vec3) -> Option<Frame3> {
        let z = z_hint.normalized()?;
        let x_proj = x_hint - z * x_hint.dot(z);
        let x = x_proj.normalized()?;
        let y = z.cross(x);
        Some(Frame3 { origin, x, y, z })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_world(&self, local: Pnt3) -> Pnt3 {
        self.origin + self.x * local.x + self.y * local.y + self.z * local.z
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_world_vector(&self, local: Vec3) -> Vec3 {
        self.x * local.x + self.y * local.y + self.z * local.z
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_local(&self, world: Pnt3) -> Pnt3 {
        let v = world - self.origin;
        Pnt3::new(v.dot(self.x), v.dot(self.y), v.dot(self.z))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_local_vector(&self, world: Vec3) -> Vec3 {
        Vec3::new(world.dot(self.x), world.dot(self.y), world.dot(self.z))
    }
    /// 🧭️ Maps every axis of `self` directly through `map`'s linear part under a similarity of
    /// uniform factor `scale` (dividing the mapped axis by `scale` so it stays unit), rather than
    /// re-deriving `z` from `x × y` — a mirror flips handedness (`z` no longer equals `x × y`),
    /// which is exactly the correct image of the original frame's axes and is deliberately
    /// preserved rather than corrected, since every `Curve3`/`Surface` variant that carries a
    /// `Frame3` uses `frame.x`/`frame.y`/`frame.z` directly in its `eval`/`derivatives`, never
    /// re-derives `z`. Only valid for a similarity map — call under [`Affine3::is_similarity`]'s
    /// `Some` arm, passing back its own `scale`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn transformed(&self, map: &Affine3, scale: f64) -> Frame3 {
        let inv_scale = 1.0 / scale;
        Frame3 { origin: map.apply_point(self.origin), x: map.apply_vector(self.x) * inv_scale, y: map.apply_vector(self.y) * inv_scale, z: map.apply_vector(self.z) * inv_scale }
    }
}

// #endregion 🔖️Frame

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn quat_from_axis_angle_rotates_correctly() {
        let q = Quat::from_axis_angle(Vec3::Z, std::f64::consts::FRAC_PI_2);
        let r = q.rotate(Vec3::X);
        assert!((r - Vec3::Y).norm() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn quat_conjugate_is_inverse_rotation() {
        let q = Quat::from_axis_angle(Vec3::new(1.0, 2.0, 3.0), 0.7);
        let v = Vec3::new(4.0, -1.0, 2.0);
        let round_trip = q.conjugate().rotate(q.rotate(v));
        assert!((round_trip - v).norm() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn quat_to_mat3_matches_direct_rotation() {
        let q = Quat::from_axis_angle(Vec3::new(0.3, 0.7, -0.2), 1.1);
        let v = Vec3::new(1.0, 0.0, 0.0);
        let via_quat = q.rotate(v);
        let via_mat = q.to_mat3().transform(v);
        assert!((via_quat - via_mat).norm() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn slerp_endpoints_match_inputs() {
        let a = Quat::from_axis_angle(Vec3::Z, 0.0);
        let b = Quat::from_axis_angle(Vec3::Z, 1.5);
        let s0 = a.slerp(b, 0.0);
        let s1 = a.slerp(b, 1.0);
        assert!((s0.rotate(Vec3::X) - a.rotate(Vec3::X)).norm() < 1e-9);
        assert!((s1.rotate(Vec3::X) - b.rotate(Vec3::X)).norm() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn trsf_inverse_round_trips_a_point() {
        let t = Trsf { rotation: Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0), 0.9), translation: Vec3::new(5.0, -2.0, 3.0), scale: 2.5 };
        let p = Pnt3::new(1.0, 2.0, 3.0);
        let round_trip = t.inverse().apply_point(t.apply_point(p));
        assert!(round_trip.distance(p) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn trsf_compose_matches_sequential_application() {
        let a = Trsf { rotation: Quat::from_axis_angle(Vec3::Z, 0.4), translation: Vec3::new(1.0, 0.0, 0.0), scale: 1.0 };
        let b = Trsf { rotation: Quat::from_axis_angle(Vec3::X, 0.9), translation: Vec3::new(0.0, 2.0, 0.0), scale: 1.5 };
        let p = Pnt3::new(3.0, -1.0, 2.0);
        let composed = a.semio_compose_rs(&b).apply_point(p);
        let sequential = a.apply_point(b.apply_point(p));
        assert!(composed.distance(sequential) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn frame_from_normal_round_trips_local_world() {
        let f = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 5.0)).unwrap();
        let local = Pnt3::new(2.0, -1.0, 0.5);
        let round_trip = f.to_local(f.to_world(local));
        assert!(round_trip.distance(local) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn frame_from_normal_is_right_handed() {
        let f = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        assert!((f.x.cross(f.y) - f.z).norm() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn frame_from_normal_deterministic_for_axis_aligned_normals() {
        let f1 = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::X).unwrap();
        let f2 = Frame3::from_normal(Pnt3::new(5.0, 5.0, 5.0), Vec3::X).unwrap();
        assert_eq!(f1.x, f2.x);
        assert_eq!(f1.y, f2.y);
    }

    #[semio_framework_async_macros::async_test]
    async fn mat3_determinant_of_identity_is_one() {
        assert!((Mat3::IDENTITY.determinant() - 1.0).abs() < 1e-12);
    }

    #[semio_framework_async_macros::async_test]
    async fn mat3_from_axis_angle_matches_quat_rotation() {
        let axis = Vec3::new(0.2, -0.4, 0.9);
        let angle = 1.3;
        let m = Mat3::from_axis_angle(axis, angle);
        let q = Quat::from_axis_angle(axis, angle);
        let v = Vec3::new(1.0, 2.0, -3.0);
        assert!((m.transform(v) - q.rotate(v)).norm() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_translation_round_trips_via_inverse() {
        let a = Affine3::translation(Vec3::new(3.0, -2.0, 5.0));
        let p = Pnt3::new(1.0, 2.0, 3.0);
        let round_trip = a.inverse().unwrap().apply_point(a.apply_point(p));
        assert!(round_trip.distance(p) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_from_trsf_matches_trsf_apply_point() {
        let t = Trsf { rotation: Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0), 0.9), translation: Vec3::new(5.0, -2.0, 3.0), scale: 2.5 };
        let a = Affine3::from_trsf(&t);
        let p = Pnt3::new(1.0, 2.0, 3.0);
        assert!(a.apply_point(p).distance(t.apply_point(p)) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_rotation_about_fixes_its_own_origin() {
        let origin = Pnt3::new(4.0, -1.0, 2.0);
        let a = Affine3::rotation_about(origin, Vec3::Z, 1.234);
        assert!(a.apply_point(origin).distance(origin) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_scaling_fixes_center_and_scales_offset() {
        let center = Pnt3::new(1.0, 1.0, 1.0);
        let a = Affine3::scaling(center, Vec3::new(2.0, 3.0, 4.0));
        assert!(a.apply_point(center).distance(center) < 1e-9);
        let p = center + Vec3::new(1.0, 1.0, 1.0);
        let mapped = a.apply_point(p);
        assert!(mapped.distance(center + Vec3::new(2.0, 3.0, 4.0)) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_mirror_fixes_plane_and_flips_normal_side() {
        let origin = Pnt3::new(0.0, 0.0, 0.0);
        let a = Affine3::mirror(origin, Vec3::Z);
        assert!(a.apply_point(origin).distance(origin) < 1e-9);
        assert!((a.determinant() + 1.0).abs() < 1e-9);
        let above = Pnt3::new(1.0, 2.0, 3.0);
        let mapped = a.apply_point(above);
        assert!((mapped.z + 3.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_compose_matches_sequential_application() {
        let a = Affine3::rotation_axis_angle(Vec3::Z, 0.4).compose(&Affine3::translation(Vec3::new(1.0, 0.0, 0.0)));
        let b = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(1.5, 1.5, 1.5)).compose(&Affine3::translation(Vec3::new(0.0, 2.0, 0.0)));
        let p = Pnt3::new(3.0, -1.0, 2.0);
        let composed = a.compose(&b).apply_point(p);
        let sequential = a.apply_point(b.apply_point(p));
        assert!(composed.distance(sequential) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_is_similarity_recognizes_rotation_translation_uniform_scale() {
        let t = Trsf { rotation: Quat::from_axis_angle(Vec3::new(0.2, 0.7, -0.3), 1.1), translation: Vec3::new(1.0, 2.0, 3.0), scale: 3.5 };
        let a = Affine3::from_trsf(&t);
        let (rotation, scale, is_reflection) = a.is_similarity().expect("rigid+uniform-scale must be recognized as a similarity");
        assert!((scale - 3.5).abs() < 1e-9);
        assert!(!is_reflection);
        let v = Vec3::new(1.0, 0.0, 0.0);
        assert!((rotation.rotate(v) - t.rotation.rotate(v)).norm() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_is_similarity_recognizes_reflection() {
        let a = Affine3::mirror(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z);
        let (_, scale, is_reflection) = a.is_similarity().expect("a mirror must be recognized as a similarity with reflection");
        assert!((scale - 1.0).abs() < 1e-9);
        assert!(is_reflection);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_is_similarity_rejects_non_uniform_scale() {
        let a = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0));
        assert!(a.is_similarity().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_apply_normal_matches_rotation_for_similarity() {
        let a = Affine3::rotation_axis_angle(Vec3::new(0.3, 0.6, 0.1), 0.8);
        let n = Vec3::new(0.0, 0.0, 1.0).normalized().unwrap();
        let via_normal = a.apply_normal(n).normalized().unwrap();
        let via_vector = a.apply_vector(n).normalized().unwrap();
        assert!((via_normal - via_vector).norm() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_max_singular_value_matches_scale_for_similarity() {
        let a = Affine3::from_trsf(&Trsf { rotation: Quat::from_axis_angle(Vec3::new(0.1, 1.0, 0.2), 0.5), translation: Vec3::ZERO, scale: 4.2 });
        assert!((a.max_singular_value() - 4.2).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn affine_max_singular_value_matches_largest_scale_factor_for_diagonal_scale() {
        let a = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 5.0, 3.0));
        assert!((a.max_singular_value() - 5.0).abs() < 1e-9);
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn trsf_inverse_round_trips_random_points() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(7);
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
