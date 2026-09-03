//! ➿ Curve/surface intersection (analytic + Bézier-span subdivision + Newton).
//!
//! Analytic fast paths cover [`Curve3::Line`] against [`Surface::Plane`], [`Surface::Sphere`],
//! [`Surface::Cylinder`], [`Surface::Cone`] (all reduce to a quadratic), and [`Curve3::Circle`]
//! against [`Surface::Plane`]/[`Surface::Sphere`] (both reduce to `A cos t + B sin t + C = 0`).
//! Every other combination subdivides the curve into Bézier spans (`shared::curve_to_bezier_segments`)
//! and, at each span whose control hull can't be rejected against the surface, seeds a coupled
//! `C(t) − S(u, v) = 0` Newton solve via [`closest_uv`] — replacing the old fixed 32-sample scan.
//!
//! See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`, upgraded in
//! `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave 2 (W2-A).

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::IntersectError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_uv;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// #region 🔖️Api

/// ➿ An isolated curve/surface intersection: world point plus curve and surface parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurveSurfaceHit {
    pub point: Pnt3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
}

/// ➿ Intersect a 3D curve with a parametric surface within `tol`. Analytic for line/plane,
/// line/sphere, line/cylinder, line/cone, circle/plane, circle/sphere; general otherwise via
/// Bézier-span subdivision plus Newton refinement.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn intersect_curve_surface(curve: &Curve3, surface: &Surface, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    if !(tol.is_finite() && tol > 0.0) {
        return Err(IntersectError::Degenerate("tolerance must be positive and finite".into()));
    }
    match (curve, surface) {
        (Curve3::Line { origin, dir }, Surface::Plane { frame }) => intersect_line_plane(*origin, *dir, frame, tol),
        (Curve3::Line { origin, dir }, Surface::Sphere { frame, radius }) => intersect_line_sphere(*origin, *dir, frame, *radius, tol),
        (Curve3::Line { origin, dir }, Surface::Cylinder { frame, radius }) => intersect_line_cylinder(*origin, *dir, frame, *radius, tol),
        (Curve3::Line { origin, dir }, Surface::Cone { frame, half_angle }) => intersect_line_cone(*origin, *dir, frame, *half_angle, tol),
        (Curve3::Circle { frame, radius }, Surface::Plane { frame: pf }) => intersect_circle_plane(frame, *radius, pf, tol),
        (Curve3::Circle { frame, radius }, Surface::Sphere { frame: sf, radius: sr }) => intersect_circle_sphere(frame, *radius, sf, *sr, tol),
        _ => intersect_general(curve, surface, tol),
    }
}

// #endregion 🔖️Api

// #region 🔖️Analytic

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intersect_line_plane(origin: Pnt3, dir: Vec3, frame: &Frame3, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    let n = dir.norm();
    if n <= tol {
        return Err(IntersectError::Degenerate("zero-length line direction".into()));
    }
    let normal = frame.z;
    let denom = dir.dot(normal);
    let local = frame.to_local(origin);
    if denom.abs() <= tol * n {
        if local.z.abs() <= tol {
            return Err(IntersectError::Tangent);
        }
        return Ok(vec![]);
    }
    let t = -local.z / (frame.to_local_vector(dir).z);
    let point = origin + dir * t;
    let uv = frame.to_local(point);
    Ok(vec![CurveSurfaceHit { point, t, u: uv.x, v: uv.y }])
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intersect_line_sphere(origin: Pnt3, dir: Vec3, frame: &Frame3, radius: f64, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    if radius <= tol || dir.norm() <= tol {
        return Err(IntersectError::Degenerate("degenerate line or sphere".into()));
    }
    let o = frame.to_local(origin).to_vec();
    let d = frame.to_local_vector(dir);
    let a = d.dot(d);
    let b = 2.0 * o.dot(d);
    let c = o.dot(o) - radius * radius;
    let disc = b * b - 4.0 * a * c;
    if disc < -(tol * tol) * a * a {
        return Ok(vec![]);
    }
    let sqrt_disc = disc.max(0.0).sqrt();
    let mut hits = Vec::with_capacity(2);
    for sign in [-1.0, 1.0] {
        let t = (-b + sign * sqrt_disc) / (2.0 * a);
        let point = origin + dir * t;
        let local = frame.to_local(point).to_vec();
        let n = local.normalized().unwrap_or(Vec3::Z);
        let v = n.z.clamp(-1.0, 1.0).asin();
        let u = n.y.atan2(n.x).rem_euclid(std::f64::consts::TAU);
        let hit = CurveSurfaceHit { point, t, u, v };
        if hits.iter().all(|h: &CurveSurfaceHit| h.point.distance(hit.point) > tol) {
            hits.push(hit);
        }
    }
    if hits.len() == 1 && disc.abs() <= (tol * tol) * a * a * 4.0 {
        return Err(IntersectError::Tangent);
    }
    Ok(hits)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intersect_line_cylinder(origin: Pnt3, dir: Vec3, frame: &Frame3, radius: f64, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    if radius <= tol || dir.norm() <= tol {
        return Err(IntersectError::Degenerate("degenerate line or cylinder".into()));
    }
    let o = frame.to_local(origin);
    let d = frame.to_local_vector(dir);
    let a = d.x * d.x + d.y * d.y;
    let b = 2.0 * (o.x * d.x + o.y * d.y);
    let c = o.x * o.x + o.y * o.y - radius * radius;
    if a <= tol * tol {
        if c.abs() <= tol * (2.0 * radius).max(1.0) {
            return Err(IntersectError::Tangent);
        }
        return Ok(vec![]);
    }
    let disc = b * b - 4.0 * a * c;
    if disc < -(tol * tol) * a * a {
        return Ok(vec![]);
    }
    let sqrt_disc = disc.max(0.0).sqrt();
    let mut hits = Vec::with_capacity(2);
    for sign in [-1.0, 1.0] {
        let t = (-b + sign * sqrt_disc) / (2.0 * a);
        let point = origin + dir * t;
        let local = frame.to_local(point);
        let u = local.y.atan2(local.x).rem_euclid(std::f64::consts::TAU);
        let hit = CurveSurfaceHit { point, t, u, v: local.z };
        if hits.iter().all(|h: &CurveSurfaceHit| h.point.distance(hit.point) > tol) {
            hits.push(hit);
        }
    }
    if hits.len() == 1 && disc.abs() <= (tol * tol) * a * a * 4.0 {
        return Err(IntersectError::Tangent);
    }
    Ok(hits)
}

/// ➿ Line/cone: same quadratic-in-`t` reduction as line/cylinder, with the cone's radius itself
/// linear in the axial coordinate (`x² + y² = (z tanα)²`); roots landing on the mirror nappe
/// (`v < 0`, outside this single-nappe surface's own domain) are discarded rather than reported.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intersect_line_cone(origin: Pnt3, dir: Vec3, frame: &Frame3, half_angle: f64, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    if dir.norm() <= tol || half_angle.abs() <= tol || half_angle.abs() >= std::f64::consts::FRAC_PI_2 - tol {
        return Err(IntersectError::Degenerate("degenerate line or cone".into()));
    }
    let o = frame.to_local(origin);
    let d = frame.to_local_vector(dir);
    let k2 = half_angle.tan().powi(2);
    let a = d.x * d.x + d.y * d.y - k2 * d.z * d.z;
    let b = 2.0 * (o.x * d.x + o.y * d.y - k2 * o.z * d.z);
    let c = o.x * o.x + o.y * o.y - k2 * o.z * o.z;
    let mut roots = Vec::with_capacity(2);
    if a.abs() <= tol * tol {
        if b.abs() > tol {
            roots.push(-c / b);
        }
    } else {
        let disc = b * b - 4.0 * a * c;
        if disc >= 0.0 {
            let sqrt_disc = disc.sqrt();
            roots.push((-b + sqrt_disc) / (2.0 * a));
            if sqrt_disc > tol {
                roots.push((-b - sqrt_disc) / (2.0 * a));
            }
        }
    }
    let mut hits = Vec::new();
    for t in roots {
        let point = origin + dir * t;
        let local = frame.to_local(point);
        if local.z < -tol {
            continue;
        }
        let u = local.y.atan2(local.x).rem_euclid(std::f64::consts::TAU);
        let hit = CurveSurfaceHit { point, t, u, v: local.z.max(0.0) };
        if hits.iter().all(|h: &CurveSurfaceHit| h.point.distance(hit.point) > tol) {
            hits.push(hit);
        }
    }
    Ok(hits)
}

/// ➿ Circle/plane: substituting the circle's own parametrization into the plane's implicit
/// equation collapses to `A cos t + B sin t + C = 0`, solved via the amplitude/phase form.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intersect_circle_plane(cf: &Frame3, radius: f64, pf: &Frame3, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    if radius <= tol {
        return Err(IntersectError::Degenerate("non-positive circle radius".into()));
    }
    let n = pf.z;
    let d0 = n.dot(cf.origin - pf.origin);
    let a = radius * n.dot(cf.x);
    let b = radius * n.dot(cf.y);
    solve_trig_hits(a, b, d0, tol, |t| {
        let point = cf.origin + cf.x * (radius * t.cos()) + cf.y * (radius * t.sin());
        let uv = pf.to_local(point);
        CurveSurfaceHit { point, t, u: uv.x, v: uv.y }
    })
}

/// ➿ Circle/sphere: substituting the circle's parametrization into `|p − center|² = r²` collapses
/// to the same `A cos t + B sin t + C = 0` form as circle/plane (the circle's own orthonormal
/// frame makes the quadratic term in `cos²+sin²` a constant).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intersect_circle_sphere(cf: &Frame3, radius: f64, sf: &Frame3, sphere_radius: f64, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    if radius <= tol || sphere_radius <= tol {
        return Err(IntersectError::Degenerate("non-positive radius".into()));
    }
    let w = cf.origin - sf.origin;
    let a = 2.0 * radius * w.dot(cf.x);
    let b = 2.0 * radius * w.dot(cf.y);
    let c = w.norm_sq() + radius * radius - sphere_radius * sphere_radius;
    solve_trig_hits(a, b, c, tol, |t| {
        let point = cf.origin + cf.x * (radius * t.cos()) + cf.y * (radius * t.sin());
        let local = sf.to_local(point).to_vec();
        let n = local.normalized().unwrap_or(Vec3::Z);
        let v = n.z.clamp(-1.0, 1.0).asin();
        let u = n.y.atan2(n.x).rem_euclid(std::f64::consts::TAU);
        CurveSurfaceHit { point, t, u, v }
    })
}

/// ➿ Solves `a cos t + b sin t + c = 0` via the amplitude/phase substitution
/// `a cos t + b sin t = r cos(t − φ)`, `r = √(a²+b²)`, `φ = atan2(b, a)`. A discriminant within
/// `tol` of the tangency boundary reports [`IntersectError::Tangent`] (matching every other
/// quadratic analytic case's convention); otherwise up to two roots, deduplicated by 3D distance.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn solve_trig_hits(a: f64, b: f64, c: f64, tol: f64, build: impl Fn(f64) -> CurveSurfaceHit) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    let amp = (a * a + b * b).sqrt();
    if amp <= tol {
        if c.abs() <= tol {
            return Err(IntersectError::Tangent);
        }
        return Ok(vec![]);
    }
    let ratio = (-c / amp).clamp(-1.0, 1.0);
    if (ratio.abs() - 1.0).abs() * amp <= tol {
        let phase = b.atan2(a);
        let t = phase + ratio.acos();
        return Ok(vec![build(t)]);
    }
    if c.abs() > amp + tol {
        return Ok(vec![]);
    }
    let phase = b.atan2(a);
    let delta = ratio.acos();
    let mut hits = Vec::with_capacity(2);
    for t in [phase + delta, phase - delta] {
        let hit = build(t);
        if hits.iter().all(|h: &CurveSurfaceHit| h.point.distance(hit.point) > tol) {
            hits.push(hit);
        }
    }
    Ok(hits)
}

// #endregion 🔖️Analytic

// #region 🔖️General

/// ➿ Bézier-span subdivision (curve side) + [`closest_uv`] seeding + Newton refinement, replacing
/// the old fixed-32-sample scan.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intersect_general(curve: &Curve3, surface: &Surface, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    let domain_t = curve_sample_domain(curve, surface, tol)?;
    let surf_domain = super::shared::finite_surface_domain(surface);
    let segs = super::shared::curve_to_bezier_segments(curve, domain_t)?;
    let mut hits = Vec::new();
    for (bez, t0, t1) in &segs {
        subdivide_seed(bez, *t0, *t1, curve, surface, domain_t, surf_domain, tol, 0, &mut hits);
    }
    hits.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
    Ok(hits)
}

/// ➿ Recursively subdivides one Bézier span of the curve; at each leaf (or once the surface's
/// certified closest point is close enough), seeds a joint Newton refinement.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
#[allow(clippy::too_many_arguments)]
fn subdivide_seed(bez: &crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bezier::RationalBezier3, t0: f64, t1: f64, curve: &Curve3, surface: &Surface, domain_t: (f64, f64), surf_domain: ((f64, f64), (f64, f64)), tol: f64, depth: u32, hits: &mut Vec<CurveSurfaceHit>) {
    let (lo, hi) = bez.control_hull_box();
    let size = (hi.x - lo.x).max(hi.y - lo.y).max(hi.z - lo.z);
    let mid_t = 0.5 * (t0 + t1);
    if size <= tol * 4.0 || depth >= 24 || (t1 - t0).abs() <= tol {
        let seed = bez.eval(0.5);
        let closest = closest_uv(surface, surf_domain, seed, tol);
        if closest.distance <= tol * 50.0 + size {
            if let Some(hit) = newton_refine(curve, surface, mid_t, closest.u, closest.v, domain_t, surf_domain, tol) {
                push_unique(hits, hit, tol);
            }
        }
        return;
    }
    let center = Pnt3::new((lo.x + hi.x) * 0.5, (lo.y + hi.y) * 0.5, (lo.z + hi.z) * 0.5);
    let reject = closest_uv(surface, surf_domain, center, tol);
    if reject.distance > size + tol * 10.0 {
        return;
    }
    let (left, right) = bez.subdivide(0.5);
    subdivide_seed(&left, t0, mid_t, curve, surface, domain_t, surf_domain, tol, depth + 1, hits);
    subdivide_seed(&right, mid_t, t1, curve, surface, domain_t, surf_domain, tol, depth + 1, hits);
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn curve_sample_domain(curve: &Curve3, surface: &Surface, tol: f64) -> Result<(f64, f64), IntersectError> {
    match curve {
        Curve3::Line { origin, dir } => super::shared::line_domain_against_surface(origin, dir, surface, tol),
        Curve3::Circle { .. } | Curve3::Ellipse { .. } => Ok(curve.domain()),
        Curve3::Nurbs { knots, .. } => {
            let d = knots.domain();
            if !d.0.is_finite() || !d.1.is_finite() || d.1 <= d.0 {
                return Err(IntersectError::Degenerate("unable to form a finite curve domain".into()));
            }
            Ok(d)
        }
    }
}


// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn wrap_or_clamp(x: f64, lo: f64, hi: f64, periodic: bool) -> f64 {
    if periodic {
        let period = hi - lo;
        if period.abs() <= f64::EPSILON {
            return lo;
        }
        let mut w = (x - lo) % period;
        if w < 0.0 {
            w += period;
        }
        lo + w
    } else if lo.is_finite() && hi.is_finite() {
        x.clamp(lo, hi)
    } else {
        x
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn newton_refine(curve: &Curve3, surface: &Surface, mut t: f64, mut u: f64, mut v: f64, domain_t: (f64, f64), surf_domain: ((f64, f64), (f64, f64)), tol: f64) -> Option<CurveSurfaceHit> {
    let ((u_lo, u_hi), (v_lo, v_hi)) = surf_domain;
    let u_periodic = surface.is_u_periodic();
    let v_periodic = surface.is_v_periodic();
    let t_periodic = curve.is_periodic();
    for _ in 0..16 {
        let c_pt = curve.eval(t);
        let d = surface.derivatives(u, v);
        let residual = c_pt - d.point;
        if residual.norm() <= tol {
            return Some(CurveSurfaceHit { point: c_pt, t, u, v });
        }
        let ct = curve.d1(t);
        let col0 = ct;
        let col1 = -d.du;
        let col2 = -d.dv;
        let det = col0.x * (col1.y * col2.z - col1.z * col2.y) - col1.x * (col0.y * col2.z - col0.z * col2.y) + col2.x * (col0.y * col1.z - col0.z * col1.y);
        let (dt, du, dv) = if det.abs() < 1e-30 {
            let lambda = 1e-6;
            let jtj = [[col0.dot(col0) + lambda, col0.dot(col1), col0.dot(col2)], [col1.dot(col0), col1.dot(col1) + lambda, col1.dot(col2)], [col2.dot(col0), col2.dot(col1), col2.dot(col2) + lambda]];
            let jtr = [col0.dot(residual), col1.dot(residual), col2.dot(residual)];
            solve_3x3(&jtj, &jtr)?
        } else {
            let inv_det = 1.0 / det;
            let neg_r = -residual;
            let dt = inv_det * (neg_r.x * (col1.y * col2.z - col1.z * col2.y) - col1.x * (neg_r.y * col2.z - neg_r.z * col2.y) + col2.x * (neg_r.y * col1.z - neg_r.z * col1.y));
            let du = inv_det * (col0.x * (neg_r.y * col2.z - neg_r.z * col2.y) - neg_r.x * (col0.y * col2.z - col0.z * col2.y) + col2.x * (col0.y * neg_r.z - col0.z * neg_r.y));
            let dv = inv_det * (col0.x * (col1.y * neg_r.z - col1.z * neg_r.y) - col1.x * (col0.y * neg_r.z - col0.z * neg_r.y) + neg_r.x * (col0.y * col1.z - col0.z * col1.y));
            (dt, du, dv)
        };
        t = if t_periodic {
            wrap_or_clamp(t + dt, domain_t.0, domain_t.1, true)
        } else if domain_t.0.is_finite() && domain_t.1.is_finite() {
            (t + dt).clamp(domain_t.0, domain_t.1)
        } else {
            t + dt
        };
        u = wrap_or_clamp(u + du, u_lo, u_hi, u_periodic);
        v = wrap_or_clamp(v + dv, v_lo, v_hi, v_periodic);
    }
    let c_pt = curve.eval(t);
    let s_pt = surface.eval(u, v);
    if c_pt.distance(s_pt) <= tol * 10.0 {
        Some(CurveSurfaceHit { point: c_pt, t, u, v })
    } else {
        None
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn solve_3x3(a: &[[f64; 3]; 3], b: &[f64; 3]) -> Option<(f64, f64, f64)> {
    let det = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1]) - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0]) + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
    if det.abs() < 1e-30 {
        return None;
    }
    let inv = 1.0 / det;
    let x = inv * (b[0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1]) - a[0][1] * (b[1] * a[2][2] - a[1][2] * b[2]) + a[0][2] * (b[1] * a[2][1] - a[1][1] * b[2]));
    let y = inv * (a[0][0] * (b[1] * a[2][2] - a[1][2] * b[2]) - b[0] * (a[1][0] * a[2][2] - a[1][2] * a[2][0]) + a[0][2] * (a[1][0] * b[2] - b[1] * a[2][0]));
    let z = inv * (a[0][0] * (a[1][1] * b[2] - b[1] * a[2][1]) - a[0][1] * (a[1][0] * b[2] - b[1] * a[2][0]) + b[0] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]));
    Some((x, y, z))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn push_unique(hits: &mut Vec<CurveSurfaceHit>, hit: CurveSurfaceHit, tol: f64) {
    let dedup = tol.max(1e-6) * 10.0;
    if hits.iter().all(|h| h.point.distance(hit.point) > dedup) {
        hits.push(hit);
    }
}

// #endregion 🔖️General

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn line_pierces_plane_z0() {
        let curve = Curve3::Line { origin: Pnt3::new(0.0, 0.0, -1.0), dir: Vec3::new(0.0, 0.0, 1.0) };
        let surface = Surface::Plane { frame: Frame3::WORLD };
        let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
        assert_eq!(hits.len(), 1);
        assert!(hits[0].point.distance(Pnt3::new(0.0, 0.0, 0.0)) < 1e-9);
        assert!((hits[0].t - 1.0).abs() < 1e-9);
        assert!(hits[0].u.abs() < 1e-9);
        assert!(hits[0].v.abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn line_through_sphere() {
        let curve = Curve3::Line { origin: Pnt3::new(-2.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
        let surface = Surface::Sphere { frame: Frame3::WORLD, radius: 1.0 };
        let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
        assert_eq!(hits.len(), 2);
        for h in &hits {
            assert!((h.point.to_vec().norm() - 1.0).abs() < 1e-9);
            let on_curve = curve.eval(h.t);
            let on_surf = surface.eval(h.u, h.v);
            assert!(on_curve.distance(h.point) < 1e-8);
            assert!(on_surf.distance(h.point) < 1e-8);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn line_through_cylinder() {
        let curve = Curve3::Line { origin: Pnt3::new(-2.0, 0.0, 1.0), dir: Vec3::new(1.0, 0.0, 0.0) };
        let surface = Surface::Cylinder { frame: Frame3::WORLD, radius: 1.0 };
        let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
        assert_eq!(hits.len(), 2);
        for h in &hits {
            assert!((h.point.x * h.point.x + h.point.y * h.point.y - 1.0).abs() < 1e-9);
            assert!((h.point.z - 1.0).abs() < 1e-9);
            assert!((h.v - 1.0).abs() < 1e-9);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn line_through_cone() {
        let curve = Curve3::Line { origin: Pnt3::new(-5.0, 0.0, 2.0), dir: Vec3::X };
        let surface = Surface::Cone { frame: Frame3::WORLD, half_angle: std::f64::consts::FRAC_PI_4 };
        let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
        assert_eq!(hits.len(), 2);
        for h in &hits {
            let on_surf = surface.eval(h.u, h.v);
            assert!(on_surf.distance(h.point) < 1e-8);
            assert!((h.point.z - 2.0).abs() < 1e-9);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_plane_off_axis_is_exact() {
        let circle = Curve3::Circle { frame: Frame3::WORLD, radius: 2.0 };
        let plane = Surface::Plane { frame: Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 1.0)).unwrap() };
        let hits = intersect_curve_surface(&circle, &plane, 1e-9).unwrap();
        assert_eq!(hits.len(), 2);
        for h in &hits {
            let on_curve = circle.eval(h.t);
            assert!(on_curve.distance(h.point) < 1e-8);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_sphere_exact() {
        let circle = Curve3::Circle { frame: Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap(), radius: 1.0 };
        let sphere = Surface::Sphere { frame: Frame3 { origin: Pnt3::new(0.5, 0.0, 0.0), ..Frame3::WORLD }, radius: 1.0 };
        let hits = intersect_curve_surface(&circle, &sphere, 1e-9).unwrap();
        assert!(!hits.is_empty());
        for h in &hits {
            let on_curve = circle.eval(h.t);
            let on_surf = sphere.eval(h.u, h.v);
            assert!(on_curve.distance(h.point) < 1e-8);
            assert!(on_surf.distance(h.point) < 1e-7);
        }
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn parallel_line_misses_plane() {
            let curve = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 1.0), dir: Vec3::X };
            let surface = Surface::Plane { frame: Frame3::WORLD };
            let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
            assert!(hits.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn circle_coincident_with_plane_is_tangent_degenerate() {
            // The circle lies entirely *in* the plane (every point is an "intersection") — the
            // same degenerate convention `intersect_line_plane` already uses for an in-plane line.
            let curve = Curve3::Circle { frame: Frame3::WORLD, radius: 2.0 };
            let surface = Surface::Plane { frame: Frame3::WORLD };
            assert!(matches!(intersect_curve_surface(&curve, &surface, 1e-6), Err(IntersectError::Tangent)));
        }

        #[semio_framework_async_macros::async_test]
        async fn circle_plane_transversal_two_hits() {
            let curve = Curve3::Circle { frame: Frame3::WORLD, radius: 2.0 };
            let surface = Surface::Plane { frame: Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.3, 1.0)).unwrap() };
            let hits = intersect_curve_surface(&curve, &surface, 1e-6).unwrap();
            assert_eq!(hits.len(), 2);
            for h in &hits {
                let on_curve = curve.eval(h.t);
                assert!(on_curve.distance(h.point) < 1e-8);
                assert!(surface.eval(h.u, h.v).distance(h.point) < 1e-6);
            }
        }
    }
}

// #endregion 🔖️Tests
