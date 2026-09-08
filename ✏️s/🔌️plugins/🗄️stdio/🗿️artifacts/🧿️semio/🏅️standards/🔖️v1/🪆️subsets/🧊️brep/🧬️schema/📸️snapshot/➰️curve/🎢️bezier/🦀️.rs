//! 🎀️ Rational Bézier curve segments in 2D and 3D: de Casteljau evaluation/splitting, degree
//! elevation, a convex-hull-derived bounding box, and the Bézier-clipping primitive that
//! [`crate::int_cc`]/[`crate::int_cs`] build their NURBS intersectors on. Weighted (rational)
//! control points are the uniform representation — an unweighted Bézier is just every weight `1`.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🎢️bezier` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `➰️curve` per that file's own pre-mounted-stub note.

use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3};

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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(controls: Vec<Pnt3>, weights: Vec<f64>) -> Self {
        debug_assert_eq!(controls.len(), weights.len());
        RationalBezier3 { controls, weights }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn unweighted(controls: Vec<Pnt3>) -> Self {
        let weights = vec![1.0; controls.len()];
        RationalBezier3::new(controls, weights)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn degree(&self) -> usize {
        self.controls.len().saturating_sub(1)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_rational(&self) -> bool {
        self.weights.iter().any(|w| (w - 1.0).abs() > 1e-12)
    }
    /// 🎀️ De Casteljau evaluation via homogeneous (weighted) coordinates, so a single algorithm
    /// covers both the polynomial and rational cases.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn elevate(&self) -> RationalBezier3 {
        debug_assert!(!self.is_rational(), "degree elevation is only implemented for polynomial (unweighted) Beziers");
        let n = self.degree();
        let m = n + 1;
        let mut controls = Vec::with_capacity(m + 1);
        for i in 0..=m {
            let a = i as f64 / m as f64;
            let b = (m - i) as f64 / m as f64;
            let left = if i > 0 { self.controls[i - 1].to_vec() * a } else { crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::ZERO };
            let right = if i <= n { self.controls[i].to_vec() * b } else { crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::ZERO };
            controls.push(Pnt3::from_array((left + right).to_array()));
        }
        RationalBezier3::unweighted(controls)
    }
}

impl RationalBezier2 {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(controls: Vec<Pnt2>, weights: Vec<f64>) -> Self {
        debug_assert_eq!(controls.len(), weights.len());
        RationalBezier2 { controls, weights }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn unweighted(controls: Vec<Pnt2>) -> Self {
        let weights = vec![1.0; controls.len()];
        RationalBezier2::new(controls, weights)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn degree(&self) -> usize {
        self.controls.len().saturating_sub(1)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn subdivide_until_flat(b: &RationalBezier2, tol: f64, max_depth: u32) -> Vec<RationalBezier2> {
    let mut leaves = Vec::new();
    subdivide_recursive(b.clone(), tol, max_depth, &mut leaves);
    leaves
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn boxes_overlap2(a: (Pnt2, Pnt2), b: (Pnt2, Pnt2), tol: f64) -> bool {
    a.0.x - tol <= b.1.x && b.0.x - tol <= a.1.x && a.0.y - tol <= b.1.y && b.0.y - tol <= a.1.y
}

// #endregion 🔖️Clip

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
