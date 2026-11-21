//! ➰ Curve/curve intersection (analytic + Bézier clipping).
//!
//! Analytic fast paths cover lines, circles and ellipses.
//! Other combinations fall
//! through to a NURBS representation clipped with control-hull subdivision (the same convergence
//! guarantee a fat-line band gives, without the extra band-distance bookkeeping) and refined by
//! Newton, with tangential contact reported via `tangent`/`multiplicity` on the hit itself.
//!
//! See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`, upgraded in
//! `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave 2 (W2-A).

use super::LineIntersectionRefusal;
use semio_framework_3d_brep_curve::bezier::RationalBezier3;
use semio_framework_3d_brep_curve::{Curve3, NurbsCurve3};
use semio_framework_3d_brep_representation::error::IntersectError;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

#[path = "🎟️original/🦀️.rs"]
mod original;
pub use original::intersect_original_curve_curve;
pub(super) use original::conic_parameter;

// #region 🔖️Api

/// ➰ An isolated curve/curve intersection: world point, parameters on each operand, whether the
/// curves are tangent (rather than transversal) there, and how many raw samples the general path
/// collapsed into this one hit (always `1` for the analytic branches, which detect tangency
/// directly instead of via merge counting).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurveCurveHit {
    pub point: Pnt3,
    pub t_a: f64,
    pub t_b: f64,
    pub tangent: bool,
    pub multiplicity: u32,
}

/// ➰ Intersects two 3D curves. Supported original line and conic pairs use fixed analytic equations.
pub fn intersect_curve_curve(a: &Curve3, b: &Curve3, tol: f64) -> Result<Vec<CurveCurveHit>, IntersectError> {
    if let Some(hits) = intersect_original_curve_curve((a, None), (b, None), tol)? {
        return Ok(hits.into_iter().flatten().collect());
    }
    intersect_general(a, b, tol)
}

// #endregion 🔖️Api

// #region 🔖️Analytic

/// 📍️ Intersects two supporting lines without allocating a result list.
pub fn intersect_line_line_unique(o1: Pnt3, d1: Vec3, o2: Pnt3, d2: Vec3, tol: f64) -> Result<Option<CurveCurveHit>, LineIntersectionRefusal> {
    use LineIntersectionRefusal::{DegenerateDirection, Infinite, InvalidTolerance, NonFinite};
    if !(tol > 0.0 && tol.is_finite()) { return Err(InvalidTolerance); }
    let n1 = d1.norm();
    let n2 = d2.norm();
    if n1 <= tol || n2 <= tol { return Err(DegenerateDirection); }
    let w0 = o1 - o2;
    let a = d1.dot(d1);
    let b = d1.dot(d2);
    let c = d2.dot(d2);
    let d = d1.dot(w0);
    let e = d2.dot(w0);
    let denom = a * c - b * b;
    if ![n1, n2, d, e, denom].into_iter().all(f64::is_finite) { return Err(NonFinite); }
    if denom.abs() <= tol * tol * a * c {
        let distance = w0.cross(d1).norm() / n1;
        if !distance.is_finite() { return Err(NonFinite); }
        if distance <= tol { return Err(Infinite); }
        return Ok(None);
    }
    let t_a = (b * e - c * d) / denom;
    let t_b = (a * e - b * d) / denom;
    let p_a = o1 + d1 * t_a;
    let p_b = o2 + d2 * t_b;
    let distance = p_a.distance(p_b);
    if ![t_a, t_b, distance].into_iter().all(f64::is_finite) { return Err(NonFinite); }
    if distance > tol { return Ok(None); }
    let point = p_a + (p_b - p_a) * 0.5;
    Ok(Some(CurveCurveHit { point, t_a, t_b, tangent: false, multiplicity: 1 }))
}

fn pack_hit(point: Pnt3, t_line: f64, t_circle: f64, swap: bool, tangent: bool) -> CurveCurveHit {
    if swap {
        CurveCurveHit { point, t_a: t_circle, t_b: t_line, tangent, multiplicity: 1 }
    } else {
        CurveCurveHit { point, t_a: t_line, t_b: t_circle, tangent, multiplicity: 1 }
    }
}

// #endregion 🔖️Analytic

// #region 🔖️General

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intersect_general(a: &Curve3, b: &Curve3, tol: f64) -> Result<Vec<CurveCurveHit>, IntersectError> {
    let (dom_a, nurbs_a) = curve_as_nurbs(a, b, tol)?;
    let (dom_b, nurbs_b) = curve_as_nurbs(b, a, tol)?;
    let segs_a = nurbs_to_bezier_segments(&nurbs_a)?;
    let segs_b = nurbs_to_bezier_segments(&nurbs_b)?;
    let mut hits = Vec::new();
    for (bez_a, a0, a1) in &segs_a {
        for (bez_b, b0, b1) in &segs_b {
            clip_pair((bez_a, *a0, *a1), (bez_b, *b0, *b1), a, b, tol, 0, &mut hits)?;
        }
    }
    if hits.is_empty() {
        sample_newton(a, dom_a, b, dom_b, tol, &mut hits);
    }
    merge_hits(&mut hits, tol);
    Ok(hits)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn curve_as_nurbs(curve: &Curve3, other: &Curve3, tol: f64) -> Result<((f64, f64), NurbsCurve3), IntersectError> {
    let domain = match curve {
        Curve3::Line { origin, dir } => line_domain_against(origin, dir, other, tol)?,
        Curve3::Circle { .. } | Curve3::Ellipse { .. } => curve.domain(),
        Curve3::Nurbs { knots, .. } => knots.domain(),
    };
    if !domain.0.is_finite() || !domain.1.is_finite() || domain.1 <= domain.0 {
        return Err(IntersectError::Degenerate("unable to form a finite NURBS domain".into()));
    }
    Ok((domain, curve.to_nurbs(domain)))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn line_domain_against(origin: &Pnt3, dir: &Vec3, other: &Curve3, tol: f64) -> Result<(f64, f64), IntersectError> {
    let n = dir.norm();
    if n <= tol {
        return Err(IntersectError::Degenerate("zero-length line direction".into()));
    }
    let unit = *dir * (1.0 / n);
    let (t0, t1) = other.domain();
    let samples = if t0.is_finite() && t1.is_finite() {
        let mut ts = Vec::new();
        for i in 0..=16 {
            ts.push(t0 + (t1 - t0) * (i as f64 / 16.0));
        }
        ts
    } else {
        vec![0.0]
    };
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for t in samples {
        let p = other.eval(t);
        let s = (p - *origin).dot(unit);
        lo = lo.min(s);
        hi = hi.max(s);
    }
    let pad = ((hi - lo).abs() + 1.0).max(1.0);
    lo -= pad;
    hi += pad;
    Ok((lo / n, hi / n))
}

/// ➰ Re-extracts Bézier spans for a NURBS already computed via [`curve_as_nurbs`] (kept local
/// rather than routed through `intersection_numerics::curve_to_bezier_segments`, which starts from a [`Curve3`]
/// + domain rather than an already-converted [`NurbsCurve3`] — this file needs the latter because
///   `curve_as_nurbs`'s domain resolution against the *other* curve has no `intersection_numerics` equivalent).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn nurbs_to_bezier_segments(nurbs: &NurbsCurve3) -> Result<Vec<(RationalBezier3, f64, f64)>, IntersectError> {
    let mut knots = nurbs.knots.clone();
    let mut hx: Vec<f64> = nurbs.controls.iter().zip(&nurbs.weights).map(|(p, w)| p.x * w).collect();
    let mut hy: Vec<f64> = nurbs.controls.iter().zip(&nurbs.weights).map(|(p, w)| p.y * w).collect();
    let mut hz: Vec<f64> = nurbs.controls.iter().zip(&nurbs.weights).map(|(p, w)| p.z * w).collect();
    let mut hw = nurbs.weights.clone();
    let p = knots.degree;
    let (d0, d1) = knots.domain();
    let mut unique: Vec<f64> = Vec::new();
    for &k in &knots.knots {
        if k > d0 + 1e-15 && k < d1 - 1e-15 && unique.last().is_none_or(|&u| (u - k).abs() > 1e-15) {
            unique.push(k);
        }
    }
    for u in unique {
        while knots.multiplicity(u) < p {
            let (nk, nx) = semio_framework_3d_brep_curve::bspline::insert_knot(&knots, &hx, u);
            let (_, ny) = semio_framework_3d_brep_curve::bspline::insert_knot(&knots, &hy, u);
            let (_, nz) = semio_framework_3d_brep_curve::bspline::insert_knot(&knots, &hz, u);
            let (_, nw) = semio_framework_3d_brep_curve::bspline::insert_knot(&knots, &hw, u);
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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn boxes_overlap3(a: (Pnt3, Pnt3), b: (Pnt3, Pnt3), tol: f64) -> bool {
    a.0.x - tol <= b.1.x && b.0.x - tol <= a.1.x && a.0.y - tol <= b.1.y && b.0.y - tol <= a.1.y && a.0.z - tol <= b.1.z && b.0.z - tol <= a.1.z
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn clip_pair((bez_a, a0, a1): (&RationalBezier3, f64, f64), (bez_b, b0, b1): (&RationalBezier3, f64, f64), curve_a: &Curve3, curve_b: &Curve3, tol: f64, depth: u32, hits: &mut Vec<CurveCurveHit>) -> Result<(), IntersectError> {
    if !boxes_overlap3(bez_a.control_hull_box(), bez_b.control_hull_box(), tol) {
        return Ok(());
    }
    let span_a = (a1 - a0).abs();
    let span_b = (b1 - b0).abs();
    let (lo_a, hi_a) = bez_a.control_hull_box();
    let (lo_b, hi_b) = bez_b.control_hull_box();
    let size_a = (hi_a.x - lo_a.x).max(hi_a.y - lo_a.y).max(hi_a.z - lo_a.z);
    let size_b = (hi_b.x - lo_b.x).max(hi_b.y - lo_b.y).max(hi_b.z - lo_b.z);
    if (size_a <= tol && size_b <= tol) || depth >= 32 || (span_a <= tol && span_b <= tol) {
        let t_a = 0.5 * (a0 + a1);
        let t_b = 0.5 * (b0 + b1);
        if let Some(hit) = newton_refine(curve_a, curve_b, t_a, t_b, tol) {
            hits.push(hit);
        }
        return Ok(());
    }
    if span_a >= span_b {
        let (left, right) = bez_a.subdivide(0.5);
        let mid = 0.5 * (a0 + a1);
        clip_pair((&left, a0, mid), (bez_b, b0, b1), curve_a, curve_b, tol, depth + 1, hits)?;
        clip_pair((&right, mid, a1), (bez_b, b0, b1), curve_a, curve_b, tol, depth + 1, hits)?;
    } else {
        let (left, right) = bez_b.subdivide(0.5);
        let mid = 0.5 * (b0 + b1);
        clip_pair((bez_a, a0, a1), (&left, b0, mid), curve_a, curve_b, tol, depth + 1, hits)?;
        clip_pair((bez_a, a0, a1), (&right, mid, b1), curve_a, curve_b, tol, depth + 1, hits)?;
    }
    Ok(())
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_newton(a: &Curve3, dom_a: (f64, f64), b: &Curve3, dom_b: (f64, f64), tol: f64, hits: &mut Vec<CurveCurveHit>) {
    const N: usize = 24;
    for i in 0..=N {
        let t_a = dom_a.0 + (dom_a.1 - dom_a.0) * (i as f64 / N as f64);
        let pa = a.eval(t_a);
        let mut best_t = dom_b.0;
        let mut best_d = f64::INFINITY;
        for j in 0..=N {
            let t_b = dom_b.0 + (dom_b.1 - dom_b.0) * (j as f64 / N as f64);
            let d = pa.distance(b.eval(t_b));
            if d < best_d {
                best_d = d;
                best_t = t_b;
            }
        }
        if best_d <= tol * 50.0 {
            if let Some(hit) = newton_refine(a, b, t_a, best_t, tol) {
                if hits.iter().all(|h| h.point.distance(hit.point) > tol) {
                    hits.push(hit);
                }
            }
        }
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn newton_refine(a: &Curve3, b: &Curve3, mut t_a: f64, mut t_b: f64, tol: f64) -> Option<CurveCurveHit> {
    for _ in 0..12 {
        let pa = a.eval(t_a);
        let pb = b.eval(t_b);
        let f = pa - pb;
        if f.norm() <= tol {
            let tangent = a.d1(t_a).cross(b.d1(t_b)).norm() <= tol.max(1e-12) * (a.d1(t_a).norm() * b.d1(t_b).norm()).max(1.0);
            return Some(CurveCurveHit { point: pa, t_a, t_b, tangent, multiplicity: 1 });
        }
        let da = a.d1(t_a);
        let db = b.d1(t_b);
        let j11 = da.dot(da);
        let j12 = -da.dot(db);
        let j22 = db.dot(db);
        let r1 = -da.dot(f);
        let r2 = db.dot(f);
        let det = j11 * j22 - j12 * j12;
        if det.abs() <= 1e-30 {
            return None;
        }
        let du1 = (j22 * r1 - j12 * r2) / det;
        let du2 = (j11 * r2 - j12 * r1) / det;
        t_a += du1;
        t_b += du2;
    }
    let pa = a.eval(t_a);
    let pb = b.eval(t_b);
    if pa.distance(pb) <= tol * 10.0 {
        let tangent = a.d1(t_a).cross(b.d1(t_b)).norm() <= tol.max(1e-12) * (a.d1(t_a).norm() * b.d1(t_b).norm()).max(1.0);
        Some(CurveCurveHit { point: Pnt3::new((pa.x + pb.x) * 0.5, (pa.y + pb.y) * 0.5, (pa.z + pb.z) * 0.5), t_a, t_b, tangent, multiplicity: 1 })
    } else {
        None
    }
}

/// ➰ Collapses near-duplicate raw hits (adjacent Bézier leaves, or clip + sample-Newton both
/// finding the same root) into one, accumulating a `multiplicity` count and OR-ing `tangent`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn merge_hits(hits: &mut Vec<CurveCurveHit>, tol: f64) {
    hits.sort_by(|x, y| x.t_a.partial_cmp(&y.t_a).unwrap_or(std::cmp::Ordering::Equal));
    let mut out: Vec<CurveCurveHit> = Vec::new();
    for hit in hits.drain(..) {
        if let Some(existing) = out.iter_mut().find(|h: &&mut CurveCurveHit| h.point.distance(hit.point) <= tol) {
            existing.multiplicity += 1;
            existing.tangent = existing.tangent || hit.tangent;
        } else {
            out.push(hit);
        }
    }
    *hits = out;
}

// #endregion 🔖️General

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
