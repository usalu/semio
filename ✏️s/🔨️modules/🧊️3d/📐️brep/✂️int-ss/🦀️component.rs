//! ✂️ Surface/surface intersection emitting [`IntCurve`].
//!
//! Analytic fast paths for plane/plane and plane/cylinder; general cases return
//! [`IntersectError::Unresolved`] until a later hardening pass.
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
        _ => Err(IntersectError::Unresolved(
            "surface pair has no analytic SSI path yet".into(),
        )),
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
