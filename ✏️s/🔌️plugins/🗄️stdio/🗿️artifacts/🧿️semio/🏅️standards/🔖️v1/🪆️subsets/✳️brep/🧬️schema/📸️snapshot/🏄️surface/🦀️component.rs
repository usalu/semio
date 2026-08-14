//! 🗺️ Analytic and free-form parametric surfaces. Every variant supports position, first/second
//! partial derivatives, normal, and Gaussian/mean/principal curvature via the standard first- and
//! second-fundamental-form formulas — the common surface interface every face in the topology
//! layer evaluates through, regardless of whether it's a `Plane` or a full `Nurbs` patch.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🏄️surface` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, together with
//! its `🪡️surface-ops` sibling as a local child, mirroring the `⚙️engine` → `📄️step`/`📦️mesh-io`
//! local-mount pattern from wave PEEL3.

// #region 🔖️Submodules

#[path = "🪡️surface-ops/🦀️component.rs"]
pub mod surface_ops;

// #endregion 🔖️Submodules

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::{basis_function_derivatives, KnotVector};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// #region 🔖️Surface

/// 🗺️ A parametric surface `S(u, v)`. Domain and periodicity are documented per variant; as with
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3`], a face's *used* trim domain is stored by the topology layer, not here.
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
    /// NURBS surfaces use central finite differences (see [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve`]'s equivalent note —
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
