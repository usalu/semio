//! ✂️ Curve/surface intersection (analytic + Newton).
//!
//! Analytic fast paths cover [`Curve3::Line`] against [`Surface::Plane`], [`Surface::Sphere`], and
//! [`Surface::Cylinder`]. Every other combination falls through to sample seeding plus Newton on
//! the coupled 3×3 system `C(t) − S(u, v) = 0`.
//!
//! See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.

use crate::brep::curve::Curve3;
use crate::brep::error::IntersectError;
use crate::brep::mat::Frame3;
use crate::brep::surface::Surface;
use crate::brep::surface_ops::closest_point;
use crate::brep::vec::{Pnt3, Vec3};

// #region 🔖️Api

/// ✂️ An isolated curve/surface intersection: world point plus curve and surface parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurveSurfaceHit {
    pub point: Pnt3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
}

/// ✂️ Intersect a 3D curve with a parametric surface within `tol`. Analytic for line/plane,
/// line/sphere, and line/cylinder; general otherwise via sample seeds plus Newton refinement.
pub fn intersect_curve_surface(
    curve: &Curve3,
    surface: &Surface,
    tol: f64,
) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    if !(tol.is_finite() && tol > 0.0) {
        return Err(IntersectError::Degenerate("tolerance must be positive and finite".into()));
    }
    match (curve, surface) {
        (Curve3::Line { origin, dir }, Surface::Plane { frame }) => {
            intersect_line_plane(*origin, *dir, frame, tol)
        }
        (Curve3::Line { origin, dir }, Surface::Sphere { frame, radius }) => {
            intersect_line_sphere(*origin, *dir, frame, *radius, tol)
        }
        (Curve3::Line { origin, dir }, Surface::Cylinder { frame, radius }) => {
            intersect_line_cylinder(*origin, *dir, frame, *radius, tol)
        }
        _ => intersect_general(curve, surface, tol),
    }
}

// #endregion 🔖️Api

// #region 🔖️Analytic

fn intersect_line_plane(
    origin: Pnt3,
    dir: Vec3,
    frame: &Frame3,
    tol: f64,
) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
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
    Ok(vec![CurveSurfaceHit {
        point,
        t,
        u: uv.x,
        v: uv.y,
    }])
}

fn intersect_line_sphere(
    origin: Pnt3,
    dir: Vec3,
    frame: &Frame3,
    radius: f64,
    tol: f64,
) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
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

fn intersect_line_cylinder(
    origin: Pnt3,
    dir: Vec3,
    frame: &Frame3,
    radius: f64,
    tol: f64,
) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
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
        let hit = CurveSurfaceHit {
            point,
            t,
            u,
            v: local.z,
        };
        if hits.iter().all(|h: &CurveSurfaceHit| h.point.distance(hit.point) > tol) {
            hits.push(hit);
        }
    }
    if hits.len() == 1 && disc.abs() <= (tol * tol) * a * a * 4.0 {
        return Err(IntersectError::Tangent);
    }
    Ok(hits)
}

// #endregion 🔖️Analytic

// #region 🔖️General

fn intersect_general(
    curve: &Curve3,
    surface: &Surface,
    tol: f64,
) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
    let domain_t = curve_sample_domain(curve, surface, tol)?;
    let surf_domain = finite_surface_domain(surface);
    let n_samples = 32usize;
    let mut hits = Vec::new();
    for i in 0..=n_samples {
        let t = domain_t.0 + (domain_t.1 - domain_t.0) * (i as f64 / n_samples as f64);
        let pt = curve.eval(t);
        let (u, v, dist) = closest_point(surface, surf_domain, pt, 8);
        if dist <= tol * 50.0 {
            if let Some(hit) = newton_refine(curve, surface, t, u, v, domain_t, surf_domain, tol) {
                push_unique(&mut hits, hit, tol);
            }
        }
    }
    hits.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
    Ok(hits)
}

fn curve_sample_domain(
    curve: &Curve3,
    surface: &Surface,
    tol: f64,
) -> Result<(f64, f64), IntersectError> {
    match curve {
        Curve3::Line { origin, dir } => line_domain_against_surface(origin, dir, surface, tol),
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

fn line_domain_against_surface(
    origin: &Pnt3,
    dir: &Vec3,
    surface: &Surface,
    tol: f64,
) -> Result<(f64, f64), IntersectError> {
    let n = dir.norm();
    if n <= tol {
        return Err(IntersectError::Degenerate("zero-length line direction".into()));
    }
    let unit = *dir * (1.0 / n);
    let ((u0, u1), (v0, v1)) = finite_surface_domain(surface);
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for i in 0..=8 {
        for j in 0..=8 {
            let u = u0 + (u1 - u0) * (i as f64 / 8.0);
            let v = v0 + (v1 - v0) * (j as f64 / 8.0);
            let p = surface.eval(u, v);
            let s = (p - *origin).dot(unit);
            lo = lo.min(s);
            hi = hi.max(s);
        }
    }
    if !lo.is_finite() || !hi.is_finite() {
        return Err(IntersectError::Degenerate("unable to bound line against surface".into()));
    }
    let pad = ((hi - lo).abs() + 1.0).max(1.0);
    Ok(((lo - pad) / n, (hi + pad) / n))
}

fn finite_surface_domain(surface: &Surface) -> ((f64, f64), (f64, f64)) {
    let ((u0, u1), (v0, v1)) = surface.domain();
    let u_hi = if u1.is_finite() { u1 } else { u0 + std::f64::consts::TAU };
    let u_lo = if u0.is_finite() { u0 } else { u_hi - std::f64::consts::TAU };
    let v_hi = if v1.is_finite() { v1 } else { 10.0 };
    let v_lo = if v0.is_finite() { v0 } else { -10.0 };
    ((u_lo, u_hi), (v_lo, v_hi))
}

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

fn newton_refine(
    curve: &Curve3,
    surface: &Surface,
    mut t: f64,
    mut u: f64,
    mut v: f64,
    domain_t: (f64, f64),
    surf_domain: ((f64, f64), (f64, f64)),
    tol: f64,
) -> Option<CurveSurfaceHit> {
    let ((u_lo, u_hi), (v_lo, v_hi)) = surf_domain;
    let u_periodic = surface.is_u_periodic();
    let v_periodic = surface.is_v_periodic();
    let t_periodic = curve.is_periodic();
    for _ in 0..16 {
        let c_pt = curve.eval(t);
        let d = surface.derivatives(u, v);
        let residual = c_pt - d.point;
        if residual.norm() <= tol {
            return Some(CurveSurfaceHit {
                point: c_pt,
                t,
                u,
                v,
            });
        }
        let ct = curve.d1(t);
        let col0 = ct;
        let col1 = -d.du;
        let col2 = -d.dv;
        let det = col0.x * (col1.y * col2.z - col1.z * col2.y)
            - col1.x * (col0.y * col2.z - col0.z * col2.y)
            + col2.x * (col0.y * col1.z - col0.z * col1.y);
        let (dt, du, dv) = if det.abs() < 1e-30 {
            let lambda = 1e-6;
            let jtj = [
                [col0.dot(col0) + lambda, col0.dot(col1), col0.dot(col2)],
                [col1.dot(col0), col1.dot(col1) + lambda, col1.dot(col2)],
                [col2.dot(col0), col2.dot(col1), col2.dot(col2) + lambda],
            ];
            let jtr = [col0.dot(residual), col1.dot(residual), col2.dot(residual)];
            solve_3x3(&jtj, &jtr)?
        } else {
            let inv_det = 1.0 / det;
            let neg_r = -residual;
            let dt = inv_det
                * (neg_r.x * (col1.y * col2.z - col1.z * col2.y)
                    - col1.x * (neg_r.y * col2.z - neg_r.z * col2.y)
                    + col2.x * (neg_r.y * col1.z - neg_r.z * col1.y));
            let du = inv_det
                * (col0.x * (neg_r.y * col2.z - neg_r.z * col2.y)
                    - neg_r.x * (col0.y * col2.z - col0.z * col2.y)
                    + col2.x * (col0.y * neg_r.z - col0.z * neg_r.y));
            let dv = inv_det
                * (col0.x * (col1.y * neg_r.z - col1.z * neg_r.y)
                    - col1.x * (col0.y * neg_r.z - col0.z * neg_r.y)
                    + neg_r.x * (col0.y * col1.z - col0.z * col1.y));
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
        Some(CurveSurfaceHit {
            point: c_pt,
            t,
            u,
            v,
        })
    } else {
        None
    }
}

fn solve_3x3(a: &[[f64; 3]; 3], b: &[f64; 3]) -> Option<(f64, f64, f64)> {
    let det = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
    if det.abs() < 1e-30 {
        return None;
    }
    let inv = 1.0 / det;
    let x = inv
        * (b[0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
            - a[0][1] * (b[1] * a[2][2] - a[1][2] * b[2])
            + a[0][2] * (b[1] * a[2][1] - a[1][1] * b[2]));
    let y = inv
        * (a[0][0] * (b[1] * a[2][2] - a[1][2] * b[2])
            - b[0] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
            + a[0][2] * (a[1][0] * b[2] - b[1] * a[2][0]));
    let z = inv
        * (a[0][0] * (a[1][1] * b[2] - b[1] * a[2][1])
            - a[0][1] * (a[1][0] * b[2] - b[1] * a[2][0])
            + b[0] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]));
    Some((x, y, z))
}

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

    #[test]
    fn line_pierces_plane_z0() {
        let curve = Curve3::Line {
            origin: Pnt3::new(0.0, 0.0, -1.0),
            dir: Vec3::new(0.0, 0.0, 1.0),
        };
        let surface = Surface::Plane {
            frame: Frame3::WORLD,
        };
        let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
        assert_eq!(hits.len(), 1);
        assert!(hits[0].point.distance(Pnt3::new(0.0, 0.0, 0.0)) < 1e-9);
        assert!((hits[0].t - 1.0).abs() < 1e-9);
        assert!(hits[0].u.abs() < 1e-9);
        assert!(hits[0].v.abs() < 1e-9);
    }

    #[test]
    fn line_through_sphere() {
        let curve = Curve3::Line {
            origin: Pnt3::new(-2.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        let surface = Surface::Sphere {
            frame: Frame3::WORLD,
            radius: 1.0,
        };
        let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
        assert_eq!(hits.len(), 2);
        let mut xs: Vec<f64> = hits.iter().map(|h| h.point.x).collect();
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!((xs[0] + 1.0).abs() < 1e-9);
        assert!((xs[1] - 1.0).abs() < 1e-9);
        for h in &hits {
            assert!(h.point.y.abs() < 1e-9);
            assert!(h.point.z.abs() < 1e-9);
            assert!((h.point.to_vec().norm() - 1.0).abs() < 1e-9);
            let on_curve = curve.eval(h.t);
            let on_surf = surface.eval(h.u, h.v);
            assert!(on_curve.distance(h.point) < 1e-8);
            assert!(on_surf.distance(h.point) < 1e-8);
        }
    }

    #[test]
    fn line_through_cylinder() {
        let curve = Curve3::Line {
            origin: Pnt3::new(-2.0, 0.0, 1.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        let surface = Surface::Cylinder {
            frame: Frame3::WORLD,
            radius: 1.0,
        };
        let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
        assert_eq!(hits.len(), 2);
        for h in &hits {
            assert!((h.point.x * h.point.x + h.point.y * h.point.y - 1.0).abs() < 1e-9);
            assert!((h.point.z - 1.0).abs() < 1e-9);
            assert!((h.v - 1.0).abs() < 1e-9);
        }
    }

    mod quick {
        use super::*;

        #[test]
        fn parallel_line_misses_plane() {
            let curve = Curve3::Line {
                origin: Pnt3::new(0.0, 0.0, 1.0),
                dir: Vec3::X,
            };
            let surface = Surface::Plane {
                frame: Frame3::WORLD,
            };
            let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
            assert!(hits.is_empty());
        }

        #[test]
        fn circle_plane_equator() {
            let curve = Curve3::Circle {
                frame: Frame3::WORLD,
                radius: 2.0,
            };
            let surface = Surface::Plane {
                frame: Frame3::WORLD,
            };
            let hits = intersect_curve_surface(&curve, &surface, 1e-6).unwrap();
            assert!(!hits.is_empty());
            for h in &hits {
                assert!(h.point.z.abs() < 1e-5);
                assert!((h.point.x * h.point.x + h.point.y * h.point.y - 4.0).abs() < 1e-4);
            }
        }
    }
}

// #endregion 🔖️Tests
