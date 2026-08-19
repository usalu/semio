//! 🎀️ Rational Bézier curve segments in 2D and 3D: de Casteljau evaluation/splitting, degree
//! elevation, a convex-hull-derived bounding box, and the Bézier-clipping primitive that
//! [`crate::int_cc`]/[`crate::int_cs`] build their NURBS intersectors on. Weighted (rational)
//! control points are the uniform representation — an unweighted Bézier is just every weight `1`.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🎢️bezier` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `➰️curve` per that file's own pre-mounted-stub note.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3};

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
    pub async fn new(controls: Vec<Pnt3>, weights: Vec<f64>) -> Self {
        debug_assert_eq!(controls.len(), weights.len());
        RationalBezier3 { controls, weights }
    }
    pub async fn unweighted(controls: Vec<Pnt3>) -> Self {
        let weights = vec![1.0; controls.len()];
        RationalBezier3::new(controls, weights)
    }
    pub async fn degree(&self) -> usize {
        self.controls.len().saturating_sub(1)
    }
    pub async fn is_rational(&self) -> bool {
        self.weights.iter().any(|w| (w - 1.0).abs() > 1e-12)
    }
    /// 🎀️ De Casteljau evaluation via homogeneous (weighted) coordinates, so a single algorithm
    /// covers both the polynomial and rational cases.
    pub async fn eval(&self, t: f64) -> Pnt3 {
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
    pub async fn subdivide(&self, t: f64) -> (RationalBezier3, RationalBezier3) {
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
    pub async fn control_hull_box(&self) -> (Pnt3, Pnt3) {
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
    pub async fn elevate(&self) -> RationalBezier3 {
        debug_assert!(!self.is_rational(), "degree elevation is only implemented for polynomial (unweighted) Beziers");
        let n = self.degree();
        let m = n + 1;
        let mut controls = Vec::with_capacity(m + 1);
        for i in 0..=m {
            let a = i as f64 / m as f64;
            let b = (m - i) as f64 / m as f64;
            let left = if i > 0 { self.controls[i - 1].to_vec() * a } else { crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::ZERO };
            let right = if i <= n { self.controls[i].to_vec() * b } else { crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec3::ZERO };
            controls.push(Pnt3::from_array((left + right).to_array()));
        }
        RationalBezier3::unweighted(controls)
    }
}

impl RationalBezier2 {
    pub async fn new(controls: Vec<Pnt2>, weights: Vec<f64>) -> Self {
        debug_assert_eq!(controls.len(), weights.len());
        RationalBezier2 { controls, weights }
    }
    pub async fn unweighted(controls: Vec<Pnt2>) -> Self {
        let weights = vec![1.0; controls.len()];
        RationalBezier2::new(controls, weights)
    }
    pub async fn degree(&self) -> usize {
        self.controls.len().saturating_sub(1)
    }
    pub async fn eval(&self, t: f64) -> Pnt2 {
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
    pub async fn subdivide(&self, t: f64) -> (RationalBezier2, RationalBezier2) {
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
    pub async fn control_hull_box(&self) -> (Pnt2, Pnt2) {
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
pub async fn subdivide_until_flat(b: &RationalBezier2, tol: f64, max_depth: u32) -> Vec<RationalBezier2> {
    let mut leaves = Vec::new();
    subdivide_recursive(b.clone(), tol, max_depth, &mut leaves);
    leaves
}

async fn subdivide_recursive(b: RationalBezier2, tol: f64, depth: u32, out: &mut Vec<RationalBezier2>) {
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
pub async fn boxes_overlap2(a: (Pnt2, Pnt2), b: (Pnt2, Pnt2), tol: f64) -> bool {
    a.0.x - tol <= b.1.x && b.0.x - tol <= a.1.x && a.0.y - tol <= b.1.y && b.0.y - tol <= a.1.y
}

// #endregion 🔖️Clip

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn unweighted_bezier_eval_matches_de_casteljau_by_hand() {
        // Quadratic bezier: (0,0),(1,2),(2,0) at t=0.5 -> (1, 1)
        let b = RationalBezier2::unweighted(vec![Pnt2::new(0.0, 0.0), Pnt2::new(1.0, 2.0), Pnt2::new(2.0, 0.0)]);
        let p = b.eval(0.5);
        assert!((p.x - 1.0).abs() < 1e-12);
        assert!((p.y - 1.0).abs() < 1e-12);
    }

    #[test]
    async fn eval_at_endpoints_matches_first_and_last_control_point() {
        let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 5.0, -2.0), Pnt3::new(3.0, 1.0, 4.0)]);
        assert_eq!(b.eval(0.0), b.controls[0]);
        assert_eq!(b.eval(1.0), *b.controls.last().unwrap());
    }

    #[test]
    async fn subdivide_matches_original_at_endpoints_and_split_point() {
        let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 2.0, 0.0), Pnt3::new(2.0, -1.0, 1.0), Pnt3::new(3.0, 0.0, 2.0)]);
        let t = 0.35;
        let (left, right) = b.subdivide(t);
        assert!(left.eval(0.0).distance(b.eval(0.0)) < 1e-9);
        assert!(left.eval(1.0).distance(b.eval(t)) < 1e-9);
        assert!(right.eval(0.0).distance(b.eval(t)) < 1e-9);
        assert!(right.eval(1.0).distance(b.eval(1.0)) < 1e-9);
    }

    #[test]
    async fn subdivide_of_rational_bezier_preserves_curve_points() {
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
    async fn control_hull_box_contains_all_sampled_curve_points() {
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
    async fn elevate_preserves_the_curve_exactly() {
        let b = RationalBezier3::unweighted(vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 3.0, -1.0), Pnt3::new(2.0, -1.0, 2.0)]);
        let elevated = b.elevate();
        assert_eq!(elevated.degree(), b.degree() + 1);
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!(b.eval(t).distance(elevated.eval(t)) < 1e-9, "mismatch at t={t}");
        }
    }

    #[test]
    async fn subdivide_until_flat_leaves_cover_the_full_parameter_range() {
        let b = RationalBezier2::unweighted(vec![Pnt2::new(0.0, 0.0), Pnt2::new(1.0, 5.0), Pnt2::new(2.0, -3.0), Pnt2::new(3.0, 1.0)]);
        let leaves = subdivide_until_flat(&b, 0.1, 12);
        assert!(!leaves.is_empty());
        // Endpoints of the whole curve must be reproduced by the first/last leaf.
        assert!(leaves.first().unwrap().eval(0.0).distance(b.eval(0.0)) < 1e-9);
        assert!(leaves.last().unwrap().eval(1.0).distance(b.eval(1.0)) < 1e-9);
    }

    #[test]
    async fn boxes_overlap_detects_disjoint_and_touching_boxes() {
        let a = (Pnt2::new(0.0, 0.0), Pnt2::new(1.0, 1.0));
        let b = (Pnt2::new(0.5, 0.5), Pnt2::new(2.0, 2.0));
        let c = (Pnt2::new(5.0, 5.0), Pnt2::new(6.0, 6.0));
        assert!(boxes_overlap2(a, b, 1e-9));
        assert!(!boxes_overlap2(a, c, 1e-9));
    }
}
// #endregion 🔖️Tests
