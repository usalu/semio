//! 🤝 Numeric plumbing shared by [`super::curve_curve`], [`super::curve_surface`], and
//! [`super::surface_surface`]: a small linear solver, parameter-explicit global curve
//! interpolation (so a 3D fit and its paired 2D p-curve fits stay evaluable at the same `t`),
//! periodic-angle unwrapping, and the exact analytic `(u, v)` inverse for every non-NURBS
//! [`Surface`] kind. Kept as one file per the "one compute subdir, not a 1:1 file mapping"
//! precedent this whole `✂️intersect` directory already follows.
//!
//! See ticket `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave 2, worker W2-A.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bezier::RationalBezier3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::{basis_functions, insert_knot, KnotVector};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::{Curve2, Curve3, NurbsCurve3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::IntersectError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_uv;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec3};

// #region 🔖️LinearAlgebra

/// 🧮 Plain Gaussian elimination with partial pivoting for the small interpolation/Newton systems
/// built below (mirrors `curve_ops::solve_linear_system`, duplicated locally per the "keep
/// repeated code close together" rule — this file can't reach that module's private fn).
pub(super) fn gauss_elim(a: &[Vec<f64>], rhs: &[f64]) -> Vec<f64> {
    let n = rhs.len();
    let mut m: Vec<Vec<f64>> = a.to_vec();
    let mut b = rhs.to_vec();
    for col in 0..n {
        let pivot = (col..n).max_by(|&i, &j| m[i][col].abs().partial_cmp(&m[j][col].abs()).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        m.swap(col, pivot);
        b.swap(col, pivot);
        let diag = m[col][col];
        if diag.abs() <= 1e-300 {
            continue;
        }
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
        x[row] = if m[row][row].abs() > 1e-300 { (b[row] - sum) / m[row][row] } else { 0.0 };
    }
    x
}

// #endregion 🔖️LinearAlgebra

// #region 🔖️Fit

/// 🧮 Centripetal (square-root chord) parameterization of `points` onto `[0, 1]` — Lee's method,
/// the same one `curve_ops::interpolate_centripetal` uses, exposed here so a p-curve fit can share
/// the exact parameter array its 3D twin used.
pub(super) fn centripetal_params(points: &[Pnt3]) -> Vec<f64> {
    let n = points.len().max(1);
    let mut chord_sqrt = vec![0.0; n];
    for i in 1..n {
        chord_sqrt[i] = points[i].distance(points[i - 1]).sqrt();
    }
    let total: f64 = chord_sqrt.iter().sum();
    let mut params = vec![0.0; n];
    if total <= 0.0 {
        for (i, p) in params.iter_mut().enumerate() {
            *p = i as f64 / (n - 1).max(1) as f64;
        }
        return params;
    }
    let mut acc = 0.0;
    for i in 1..n {
        acc += chord_sqrt[i];
        params[i] = acc / total;
    }
    params[n - 1] = 1.0;
    params
}

/// 🧮 Global degree-≤3 curve interpolation at *explicit* parameter values (unlike
/// `curve_ops::interpolate_centripetal`, which computes its own) — the building block that keeps
/// a traced curve's 3D fit and its two p-curve fits sharing one `t`.
pub(super) fn interpolate_params_3d(points: &[Pnt3], params: &[f64]) -> Option<NurbsCurve3> {
    let n = points.len();
    if n < 2 || params.len() != n {
        return None;
    }
    let (kv, basis_rows) = fit_basis(params, n)?;
    let degree = kv.degree;
    let mut a = vec![vec![0.0; n]; n];
    for (row, (span, basis)) in basis_rows.iter().enumerate() {
        for (j, &b) in basis.iter().enumerate() {
            a[row][span - degree + j] = b;
        }
    }
    let xs = gauss_elim(&a, &points.iter().map(|p| p.x).collect::<Vec<_>>());
    let ys = gauss_elim(&a, &points.iter().map(|p| p.y).collect::<Vec<_>>());
    let zs = gauss_elim(&a, &points.iter().map(|p| p.z).collect::<Vec<_>>());
    let controls = (0..n).map(|i| Pnt3::new(xs[i], ys[i], zs[i])).collect();
    Some(NurbsCurve3 { knots: kv, controls, weights: vec![1.0; n] })
}

/// 🧮 The 2D twin of [`interpolate_params_3d`]: same knot placement rule, explicit `params`,
/// producing a [`Curve2::Nurbs`] p-curve.
pub(super) fn interpolate_params_2d(points: &[Pnt2], params: &[f64]) -> Option<Curve2> {
    let n = points.len();
    if n < 2 || params.len() != n {
        return None;
    }
    let (kv, basis_rows) = fit_basis(params, n)?;
    let degree = kv.degree;
    let mut a = vec![vec![0.0; n]; n];
    for (row, (span, basis)) in basis_rows.iter().enumerate() {
        for (j, &b) in basis.iter().enumerate() {
            a[row][span - degree + j] = b;
        }
    }
    let xs = gauss_elim(&a, &points.iter().map(|p| p.x).collect::<Vec<_>>());
    let ys = gauss_elim(&a, &points.iter().map(|p| p.y).collect::<Vec<_>>());
    let controls = (0..n).map(|i| Pnt2::new(xs[i], ys[i])).collect();
    Some(Curve2::Nurbs { knots: kv, controls, weights: vec![1.0; n] })
}

/// 🧮 Shared knot placement (averaging, degree ≤3) + per-sample basis evaluation for
/// [`interpolate_params_3d`]/[`interpolate_params_2d`].
fn fit_basis(params: &[f64], n: usize) -> Option<(KnotVector, Vec<(usize, Vec<f64>)>)> {
    let degree = (n - 1).min(3);
    let mut knots = vec![0.0; degree + 1];
    for j in 1..n - degree {
        let avg: f64 = params[j..j + degree].iter().sum::<f64>() / degree as f64;
        knots.push(avg);
    }
    knots.extend(std::iter::repeat_n(1.0, degree + 1));
    let kv = KnotVector::new(knots, degree, n)?;
    let rows = params
        .iter()
        .map(|&u| {
            let span = kv.find_span(u);
            (span, basis_functions(&kv, span, u))
        })
        .collect();
    Some((kv, rows))
}

// #endregion 🔖️Fit

// #region 🔖️Periodic

/// 🧮 Removes false `period`-multiple discontinuities from a sampled periodic sequence (e.g.
/// `atan2` angles) by shifting each sample by the multiple of `period` nearest its predecessor —
/// makes a traced/sampled periodic p-curve continuously interpolable instead of sawtoothed.
pub(super) fn unwrap_periodic(values: &mut [f64], period: f64) {
    for i in 1..values.len() {
        let mut d = values[i] - values[i - 1];
        while d > period * 0.5 {
            values[i] -= period;
            d -= period;
        }
        while d < -period * 0.5 {
            values[i] += period;
            d += period;
        }
    }
}

// #endregion 🔖️Periodic

// #region 🔖️Analytic

/// 🧮 The rotation axis a [`Surface`] variant is symmetric about, as `(point_on_axis, unit_dir)`
/// — `None` for [`Surface::Plane`]/[`Surface::Nurbs`], which carry no such symmetry here. The
/// coaxial-family and plane-perpendicular-section exact cases in `surface_surface` key off this.
pub(super) fn axis_of(surface: &Surface) -> Option<(Pnt3, Vec3)> {
    match surface {
        Surface::Cylinder { frame, .. } | Surface::Cone { frame, .. } | Surface::Sphere { frame, .. } | Surface::Torus { frame, .. } => Some((frame.origin, frame.z.normalized().unwrap_or(Vec3::Z))),
        Surface::Plane { .. } | Surface::Nurbs { .. } => None,
    }
}

/// 🧮 Exact `(u, v)` for a point already known to lie on `surface` — closed-form `atan2`/`asin`
/// inverses for the five analytic kinds; closest-point search for [`Surface::Nurbs`] (no closed
/// form exists there, so this is only as exact as that search).
pub(super) fn exact_uv(surface: &Surface, p: Pnt3) -> (f64, f64) {
    match surface {
        Surface::Plane { frame } => {
            let local = frame.to_local(p);
            (local.x, local.y)
        }
        Surface::Cylinder { frame, .. } | Surface::Cone { frame, .. } => {
            let local = frame.to_local(p);
            (local.y.atan2(local.x), local.z)
        }
        Surface::Sphere { frame, radius } => {
            let local = frame.to_local(p).to_vec();
            (local.y.atan2(local.x), (local.z / radius).clamp(-1.0, 1.0).asin())
        }
        Surface::Torus { frame, major_radius, .. } => {
            let local = frame.to_local(p);
            let u = local.y.atan2(local.x);
            let rho = (local.x * local.x + local.y * local.y).sqrt() - major_radius;
            (u, local.z.atan2(rho))
        }
        Surface::Nurbs { .. } => {
            let closest = closest_uv(surface, surface.domain(), p, 1e-9);
            let (u, v) = (closest.u, closest.v);
            (u, v)
        }
    }
}

// #endregion 🔖️Analytic

// #region 🔖️Bezier

/// 🧮 Extracts `curve`'s Bézier segments over `domain` (NURBS conversion + knot insertion up to
/// full multiplicity at every interior break) — the seed decomposition [`super::curve_surface`]'s
/// and [`super::curve_curve`]'s general paths both subdivide from.
pub(super) fn curve_to_bezier_segments(curve: &Curve3, domain: (f64, f64)) -> Result<Vec<(RationalBezier3, f64, f64)>, IntersectError> {
    if !(domain.0.is_finite() && domain.1.is_finite() && domain.1 > domain.0) {
        return Err(IntersectError::Degenerate("unable to form a finite NURBS domain".into()));
    }
    let nurbs = curve.to_nurbs(domain);
    let mut knots = nurbs.knots.clone();
    let mut hx: Vec<f64> = nurbs.controls.iter().zip(&nurbs.weights).map(|(p, w)| p.x * w).collect();
    let mut hy: Vec<f64> = nurbs.controls.iter().zip(&nurbs.weights).map(|(p, w)| p.y * w).collect();
    let mut hz: Vec<f64> = nurbs.controls.iter().zip(&nurbs.weights).map(|(p, w)| p.z * w).collect();
    let mut hw = nurbs.weights.clone();
    let p = knots.degree;
    let (d0, d1) = knots.domain();
    let mut unique: Vec<f64> = Vec::new();
    for &k in &knots.knots {
        if k > d0 + 1e-15 && k < d1 - 1e-15 && unique.last().map(|&u| (u - k).abs() > 1e-15).unwrap_or(true) {
            unique.push(k);
        }
    }
    for u in unique {
        while knots.multiplicity(u) < p {
            let (nk, nx) = insert_knot(&knots, &hx, u);
            let (_, ny) = insert_knot(&knots, &hy, u);
            let (_, nz) = insert_knot(&knots, &hz, u);
            let (_, nw) = insert_knot(&knots, &hw, u);
            knots = nk;
            hx = nx;
            hy = ny;
            hz = nz;
            hw = nw;
        }
    }
    let mut spans = Vec::new();
    let mut i = p;
    let last = knots.knots.len() - p - 1;
    while i < last {
        let u0 = knots.knots[i];
        let u1 = knots.knots[i + 1];
        if (u1 - u0).abs() > 1e-15 {
            let mut controls = Vec::with_capacity(p + 1);
            let mut weights = Vec::with_capacity(p + 1);
            for j in 0..=p {
                let idx = i - p + j;
                let w = hw[idx];
                if w.abs() <= 1e-300 {
                    return Err(IntersectError::Degenerate("zero weight in NURBS segment".into()));
                }
                controls.push(Pnt3::new(hx[idx] / w, hy[idx] / w, hz[idx] / w));
                weights.push(w);
            }
            spans.push((RationalBezier3::new(controls, weights), u0, u1));
        }
        i += 1;
        while i < last && (knots.knots[i + 1] - knots.knots[i]).abs() <= 1e-15 {
            i += 1;
        }
    }
    if spans.is_empty() {
        return Err(IntersectError::Unresolved("NURBS produced no Bézier spans".into()));
    }
    Ok(spans)
}

// #endregion 🔖️Bezier

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn gauss_elim_solves_small_system() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let rhs = vec![5.0, 10.0];
        let x = gauss_elim(&a, &rhs);
        assert!((x[0] - 1.0).abs() < 1e-9);
        assert!((x[1] - 3.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn interpolate_params_3d_passes_through_samples() {
        let points = vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(3.0, -1.0, 0.0)];
        let params = centripetal_params(&points);
        let nurbs = interpolate_params_3d(&points, &params).expect("interpolation");
        for (p, &t) in points.iter().zip(&params) {
            let point = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::Nurbs { knots: nurbs.knots.clone(), controls: nurbs.controls.clone(), weights: nurbs.weights.clone() }.eval(t);
            assert!(point.distance(*p) < 1e-8, "expected {p:?}, got {point:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn unwrap_periodic_removes_tau_jumps() {
        let mut values = vec![3.0, -3.1, -3.0, 3.05];
        unwrap_periodic(&mut values, std::f64::consts::TAU);
        for w in values.windows(2) {
            assert!((w[1] - w[0]).abs() < std::f64::consts::PI);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_uv_matches_cylinder_eval() {
        let frame = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3::WORLD;
        let surface = Surface::Cylinder { frame, radius: 2.0 };
        let p = surface.eval(0.7, 1.3);
        let (u, v) = exact_uv(&surface, p);
        assert!((u - 0.7).abs() < 1e-9);
        assert!((v - 1.3).abs() < 1e-9);
    }
}
// #endregion 🔖️Tests
