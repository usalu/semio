//! ✂️ Surface/surface intersection emitting [`IntCurve`].
//!
//! Analytic fast paths for plane/plane, plane/cylinder, plane/sphere, sphere/sphere;
//! remaining pairs use a dense UV sampling fallback that emits a degree-1 NURBS through the hits.
//!
//! See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.

use crate::brep::curve::Curve3;
use crate::brep::error::IntersectError;
use crate::brep::mat::Frame3;
use crate::brep::surface::Surface;
use crate::brep::vec::{Pnt3, Vec3};

// #region 🔖️Api

/// ✂️ One surface/surface intersection branch (space curve; pcurves land later).
#[derive(Clone, Debug, PartialEq)]
pub struct IntCurve {
    pub curve3: Curve3,
}

/// ✂️ Intersect two parametric surfaces within `tol`.
pub fn intersect_surface_surface(
    a: &Surface,
    b: &Surface,
    tol: f64,
) -> Result<Vec<IntCurve>, IntersectError> {
    if !(tol.is_finite() && tol > 0.0) {
        return Err(IntersectError::Degenerate(
            "tolerance must be positive and finite".into(),
        ));
    }
    match (a, b) {
        (Surface::Plane { frame: fa }, Surface::Plane { frame: fb }) => {
            intersect_plane_plane(fa, fb, tol)
        }
        (Surface::Plane { frame }, Surface::Cylinder { frame: cf, radius }) => {
            intersect_plane_cylinder(frame, cf, *radius, tol)
        }
        (Surface::Cylinder { frame: cf, radius }, Surface::Plane { frame }) => {
            intersect_plane_cylinder(frame, cf, *radius, tol)
        }
        (Surface::Plane { frame }, Surface::Sphere { frame: sf, radius }) => {
            intersect_plane_sphere(frame, sf, *radius, tol)
        }
        (Surface::Sphere { frame: sf, radius }, Surface::Plane { frame }) => {
            intersect_plane_sphere(frame, sf, *radius, tol)
        }
        (Surface::Sphere { frame: fa, radius: ra }, Surface::Sphere { frame: fb, radius: rb }) => {
            intersect_sphere_sphere(fa, *ra, fb, *rb, tol)
        }
        _ => intersect_surfaces_sampled(a, b, tol),
    }
}


fn intersect_plane_sphere(plane: &Frame3, sphere: &Frame3, radius: f64, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    if !(radius.is_finite() && radius > tol) {
        return Err(IntersectError::Degenerate("sphere radius must be positive".into()));
    }
    let n = plane.z;
    let n_n = n.norm();
    if n_n <= tol {
        return Err(IntersectError::Degenerate("plane normal degenerate".into()));
    }
    let n_u = n * (1.0 / n_n);
    let dist = n_u.dot(sphere.origin - plane.origin);
    let abs_d = dist.abs();
    if abs_d > radius + tol {
        return Ok(Vec::new());
    }
    let h2 = radius * radius - dist * dist;
    let r = if h2 <= 0.0 { 0.0 } else { h2.sqrt() };
    let center = sphere.origin - n_u * dist;
    let x = plane.x - n_u * plane.x.dot(n_u);
    let x = x.normalized().unwrap_or(plane.y);
    let y = n_u.cross(x);
    Ok(vec![IntCurve {
        curve3: Curve3::Circle {
            frame: Frame3 { origin: center, x, y, z: n_u },
            radius: r.max(tol * 0.5),
        },
    }])
}

fn intersect_sphere_sphere(fa: &Frame3, ra: f64, fb: &Frame3, rb: f64, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    if !(ra.is_finite() && rb.is_finite() && ra > tol && rb > tol) {
        return Err(IntersectError::Degenerate("sphere radii must be positive".into()));
    }
    let d_vec = fb.origin - fa.origin;
    let d = d_vec.norm();
    if d <= tol {
        return if (ra - rb).abs() <= tol {
            Err(IntersectError::Unresolved("coincident spheres".into()))
        } else {
            Ok(Vec::new())
        };
    }
    if d > ra + rb + tol || d + tol < (ra - rb).abs() {
        return Ok(Vec::new());
    }
    let dir = d_vec * (1.0 / d);
    let a = (ra * ra - rb * rb + d * d) / (2.0 * d);
    let h2 = ra * ra - a * a;
    let h = if h2 <= 0.0 { 0.0 } else { h2.sqrt() };
    let center = fa.origin + dir * a;
    let x = dir
        .cross(Vec3::new(0.0, 0.0, 1.0))
        .normalized()
        .or_else(|| dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalized())
        .ok_or_else(|| IntersectError::Degenerate("sphere intersection frame".into()))?;
    let y = dir.cross(x);
    Ok(vec![IntCurve {
        curve3: Curve3::Circle {
            frame: Frame3 { origin: center, x, y, z: dir },
            radius: h.max(tol * 0.5),
        },
    }])
}

/// ✂️ Dense UV sampling fallback: keep samples of `a` near `b`, then emit a polyline.
fn intersect_surfaces_sampled(a: &Surface, b: &Surface, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    let nu = 24usize;
    let nv = 24usize;
    let mut pts = Vec::new();
    for iu in 0..=nu {
        let u = (iu as f64) / (nu as f64) * std::f64::consts::TAU;
        for iv in 0..=nv {
            let v = ((iv as f64) / (nv as f64) - 0.5) * 4.0;
            let p = a.eval(u, v);
            if let Some(q) = project_point_to_surface(b, p, tol) {
                if (q - p).norm() <= tol * 4.0 {
                    pts.push(p);
                }
            }
        }
    }
    if pts.len() < 2 {
        return Ok(Vec::new());
    }
    let mut ordered = vec![pts.remove(0)];
    while !pts.is_empty() {
        let last = *ordered.last().unwrap();
        let (idx, _) = pts
            .iter()
            .enumerate()
            .min_by(|(_, aa), (_, bb)| {
                (last - **aa)
                    .norm()
                    .partial_cmp(&(last - **bb).norm())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap();
        ordered.push(pts.swap_remove(idx));
    }
    // Dedup near-duplicates
    let mut controls = Vec::<Pnt3>::new();
    for p in ordered {
        if controls.last().map(|q| (*q - p).norm() > tol).unwrap_or(true) {
            controls.push(p);
        }
    }
    if controls.len() < 2 {
        return Ok(Vec::new());
    }
    if controls.len() == 2 {
        let origin = controls[0];
        let dir = controls[1] - origin;
        if dir.norm() <= tol {
            return Ok(Vec::new());
        }
        return Ok(vec![IntCurve {
            curve3: Curve3::Line { origin, dir },
        }]);
    }
    let n = controls.len();
    let knots = crate::brep::bspline::KnotVector::clamped_uniform(n, 1);
    let weights = vec![1.0; n];
    Ok(vec![IntCurve {
        curve3: Curve3::Nurbs {
            knots,
            controls,
            weights,
        },
    }])
}

fn project_point_to_surface(surface: &Surface, point: Pnt3, tol: f64) -> Option<Pnt3> {
    match surface {
        Surface::Plane { frame } => {
            let n = frame.z;
            let n_n = n.norm();
            if n_n <= tol {
                return None;
            }
            let n_u = n * (1.0 / n_n);
            Some(point - n_u * n_u.dot(point - frame.origin))
        }
        Surface::Sphere { frame, radius } => {
            let v = point - frame.origin;
            let n = v.norm();
            if n <= tol {
                return None;
            }
            Some(frame.origin + v * (*radius / n))
        }
        Surface::Cylinder { frame, radius } => {
            let w = point - frame.origin;
            let axis = frame.z;
            let axial = axis * axis.dot(w);
            let radial = w - axial;
            let rn = radial.norm();
            if rn <= tol {
                return None;
            }
            Some(frame.origin + axial + radial * (*radius / rn))
        }
        _ => {
            let mut u = 0.0;
            let mut v = 0.0;
            for _ in 0..8 {
                let p = surface.eval(u, v);
                let r = point - p;
                if r.norm() <= tol {
                    return Some(p);
                }
                let pu = surface.eval(u + 1e-3, v) - p;
                let pv = surface.eval(u, v + 1e-3) - p;
                let gu = pu.dot(r);
                let gv = pv.dot(r);
                let du = pu.dot(pu);
                let dv = pv.dot(pv);
                if du > tol * tol {
                    u += gu / du;
                }
                if dv > tol * tol {
                    v += gv / dv;
                }
            }
            Some(surface.eval(u, v))
        }
    }
}

// #endregion 🔖️Api

// #region 🔖️Analytic

fn intersect_plane_plane(fa: &Frame3, fb: &Frame3, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
    let n1 = fa.z;
    let n2 = fb.z;
    let dir = n1.cross(n2);
    let dir_n = dir.norm();
    if dir_n <= tol {
        let dist = n1.dot(fb.origin - fa.origin).abs();
        if dist <= tol {
            return Err(IntersectError::Tangent);
        }
        return Ok(vec![]);
    }
    let d1 = n1.dot(fa.origin.to_vec());
    let d2 = n2.dot(fb.origin.to_vec());
    let point = (n2.cross(dir) * d1 + dir.cross(n1) * d2) * (1.0 / (dir_n * dir_n));
    let origin = Pnt3::new(point.x, point.y, point.z);
    let unit = dir * (1.0 / dir_n);
    Ok(vec![IntCurve {
        curve3: Curve3::Line { origin, dir: unit },
    }])
}

fn intersect_plane_cylinder(
    plane: &Frame3,
    cyl: &Frame3,
    radius: f64,
    tol: f64,
) -> Result<Vec<IntCurve>, IntersectError> {
    if radius <= tol {
        return Err(IntersectError::Degenerate(
            "non-positive cylinder radius".into(),
        ));
    }
    let n = plane.z.normalized().unwrap_or(Vec3::Z);
    let axis = cyl.z.normalized().unwrap_or(Vec3::Z);
    let cos_theta = n.dot(axis).abs();
    if cos_theta <= tol {
        return plane_cylinder_parallel(plane, cyl, radius, n, axis, tol);
    }
    let n_dot_axis = n.dot(axis);
    let t = n.dot(plane.origin - cyl.origin) / n_dot_axis;
    let center = cyl.origin + axis * t;
    if (1.0 - cos_theta) <= tol {
        let frame = Frame3::from_normal(center, axis).ok_or_else(|| {
            IntersectError::Degenerate("degenerate circle frame on cylinder".into())
        })?;
        return Ok(vec![IntCurve {
            curve3: Curve3::Circle { frame, radius },
        }]);
    }
    let minor = radius;
    let major = radius / cos_theta;
    let major_dir = (axis - n * axis.dot(n))
        .normalized()
        .unwrap_or_else(|| n.any_orthogonal());
    let frame = Frame3::from_x_z(center, major_dir, n).ok_or_else(|| {
        IntersectError::Degenerate("degenerate ellipse frame on cylinder".into())
    })?;
    Ok(vec![IntCurve {
        curve3: Curve3::Ellipse {
            frame,
            major_radius: major,
            minor_radius: minor,
        },
    }])
}

fn plane_cylinder_parallel(
    plane: &Frame3,
    cyl: &Frame3,
    radius: f64,
    n: Vec3,
    axis: Vec3,
    tol: f64,
) -> Result<Vec<IntCurve>, IntersectError> {
    let signed = n.dot(cyl.origin - plane.origin);
    let dist = signed.abs();
    if dist > radius + tol {
        return Ok(vec![]);
    }
    let h_sq = radius * radius - dist * dist;
    if h_sq < -(tol * tol) {
        return Ok(vec![]);
    }
    let h = h_sq.max(0.0).sqrt();
    let foot = cyl.origin - n * signed;
    let perp = n
        .cross(axis)
        .normalized()
        .unwrap_or_else(|| axis.any_orthogonal());
    if h <= tol {
        return Err(IntersectError::Tangent);
    }
    Ok(vec![
        IntCurve {
            curve3: Curve3::Line {
                origin: foot + perp * (-h),
                dir: axis,
            },
        },
        IntCurve {
            curve3: Curve3::Line {
                origin: foot + perp * h,
                dir: axis,
            },
        },
    ])
}

// #endregion 🔖️Analytic

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthogonal_planes_intersect_in_line() {
        let xy = Surface::Plane {
            frame: Frame3::WORLD,
        };
        let xz = Surface::Plane {
            frame: Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Y).unwrap(),
        };
        let curves = intersect_surface_surface(&xy, &xz, 1e-8).expect("planes intersect");
        assert_eq!(curves.len(), 1);
        match &curves[0].curve3 {
            Curve3::Line { origin, dir } => {
                assert!(origin.y.abs() < 1e-8 && origin.z.abs() < 1e-8);
                let u = dir.normalized().unwrap();
                assert!((u.x.abs() - 1.0).abs() < 1e-8);
                assert!(u.y.abs() < 1e-8 && u.z.abs() < 1e-8);
            }
            other => panic!("expected line, got {other:?}"),
        }
        for t in [-2.0_f64, -0.5, 0.0, 1.5, 3.0] {
            let p = curves[0].curve3.eval(t);
            assert!(p.y.abs() < 1e-8 && p.z.abs() < 1e-8);
        }
    }

    #[test]
    fn parallel_planes_empty_or_tangent() {
        let a = Surface::Plane {
            frame: Frame3::WORLD,
        };
        let b = Surface::Plane {
            frame: Frame3 {
                origin: Pnt3::new(0.0, 0.0, 2.0),
                ..Frame3::WORLD
            },
        };
        assert!(intersect_surface_surface(&a, &b, 1e-8).unwrap().is_empty());
        let c = Surface::Plane {
            frame: Frame3 {
                origin: Pnt3::new(1.0, 2.0, 0.0),
                ..Frame3::WORLD
            },
        };
        assert!(matches!(
            intersect_surface_surface(&a, &c, 1e-8),
            Err(IntersectError::Tangent)
        ));
    }

    #[test]
    fn plane_cylinder_perpendicular_is_circle() {
        let plane = Surface::Plane {
            frame: Frame3 {
                origin: Pnt3::new(0.0, 0.0, 3.0),
                ..Frame3::WORLD
            },
        };
        let cyl = Surface::Cylinder {
            frame: Frame3::WORLD,
            radius: 2.0,
        };
        let curves = intersect_surface_surface(&plane, &cyl, 1e-8).expect("plane/cyl");
        assert_eq!(curves.len(), 1);
        match &curves[0].curve3 {
            Curve3::Circle { frame, radius } => {
                assert!((radius - 2.0).abs() < 1e-8);
                assert!(frame.origin.distance(Pnt3::new(0.0, 0.0, 3.0)) < 1e-8);
            }
            other => panic!("expected circle, got {other:?}"),
        }
        for i in 0..16 {
            let t = i as f64 * std::f64::consts::TAU / 16.0;
            let p = curves[0].curve3.eval(t);
            let r = (p.x * p.x + p.y * p.y).sqrt();
            assert!((r - 2.0).abs() < 1e-6);
            assert!((p.z - 3.0).abs() < 1e-6);
        }
    }

    #[test]
    fn plane_cylinder_parallel_two_lines() {
        let plane = Surface::Plane {
            frame: Frame3::from_x_z(Pnt3::new(0.0, 0.0, 0.0), Vec3::Y, Vec3::X).unwrap(),
        };
        let cyl = Surface::Cylinder {
            frame: Frame3::WORLD,
            radius: 2.0,
        };
        let curves = intersect_surface_surface(&plane, &cyl, 1e-8).expect("parallel plane/cyl");
        assert_eq!(curves.len(), 2);
        for c in &curves {
            match &c.curve3 {
                Curve3::Line { origin, dir } => {
                    assert!(origin.x.abs() < 1e-6);
                    assert!((origin.y.abs() - 2.0).abs() < 1e-6);
                    assert!(dir.normalized().unwrap().z.abs() > 0.99);
                }
                other => panic!("expected line, got {other:?}"),
            }
        }
    }
}

// #endregion 🔖️Tests
