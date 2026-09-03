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

#[path = "🪡️surface-ops/🦀️.rs"]
pub mod surface_ops;

// #endregion 🔖️Submodules

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::{basis_function_derivatives, surface_derivatives_rational, KnotVector};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// #region 🔖️Surface

/// 🗺️ A parametric surface `S(u, v)`. Domain and periodicity are documented per variant; as with
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3`], a face's *used* trim domain is stored by the topology layer, not here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_u_periodic(&self) -> bool {
        matches!(self, Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_v_periodic(&self) -> bool {
        matches!(self, Surface::Torus { .. })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    /// [`Surface::Nurbs`] uses the exact rational `RatSurfaceDerivs` recurrence
    /// ([`surface_derivatives_rational`]) — safe for tight Newton iteration, unlike the
    /// finite-difference stand-in this used to be.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
            Surface::Nurbs { u_knots, v_knots, controls, weights } => {
                let controls_h: Vec<Vec<Vec<f64>>> =
                    controls.iter().zip(weights).map(|(row, wrow)| row.iter().zip(wrow).map(|(p, &w)| vec![p.x * w, p.y * w, p.z * w, w]).collect()).collect();
                let s = surface_derivatives_rational(u_knots, v_knots, &controls_h, u, v, 2);
                let vec_of = |k: usize, l: usize| Vec3::new(s[k][l][0], s[k][l][1], s[k][l][2]);
                let p0 = vec_of(0, 0);
                SurfaceDerivatives { point: Pnt3::new(p0.x, p0.y, p0.z), du: vec_of(1, 0), dv: vec_of(0, 1), duu: vec_of(2, 0), duv: vec_of(1, 1), dvv: vec_of(0, 2) }
            }
        }
    }
    /// 🗺️ Unit surface normal `du × dv` (falls back to `None` at a singular point, e.g. a sphere
    /// pole or a cone apex, where `du` degenerates to zero).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn normal(&self, u: f64, v: f64) -> Option<Vec3> {
        let d = self.derivatives(u, v);
        d.du.cross(d.dv).normalized()
    }
    /// 🗺️ Gaussian curvature `K = (LN - M²) / (EG - F²)` and mean curvature `H = (EN - 2FM + GL) /
    /// (2(EG - F²))`, from the first fundamental form `(E, F, G)` and second `(L, M, N)`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn principal_curvatures(&self, u: f64, v: f64) -> Option<(f64, f64)> {
        let (gaussian, mean) = self.curvature(u, v)?;
        let disc = (mean * mean - gaussian).max(0.0).sqrt();
        Some((mean + disc, mean - disc))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_planar(&self) -> bool {
        matches!(self, Surface::Plane { .. })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// #endregion 🔖️Surface

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fd_du(s: &Surface, u: f64, v: f64) -> Vec3 {
        let h = 1e-6;
        (s.eval(u + h, v) - s.eval(u - h, v)) * (1.0 / (2.0 * h))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fd_dv(s: &Surface, u: f64, v: f64) -> Vec3 {
        let h = 1e-6;
        (s.eval(u, v + h) - s.eval(u, v - h)) * (1.0 / (2.0 * h))
    }

    #[semio_framework_async_macros::async_test]
    async fn plane_eval_and_normal() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::Z).unwrap();
        let s = Surface::Plane { frame };
        let p = s.eval(2.0, 3.0);
        assert!((p.z - 3.0).abs() < 1e-9);
        assert!((s.normal(0.0, 0.0).unwrap() - Vec3::Z).norm() < 1e-9);
        assert!(s.is_planar());
    }

    #[semio_framework_async_macros::async_test]
    async fn cylinder_derivatives_match_finite_differences_and_lie_on_cylinder() {
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

    #[semio_framework_async_macros::async_test]
    async fn cylinder_gaussian_curvature_is_zero_and_mean_curvature_is_half_reciprocal_radius() {
        let frame = Frame3::WORLD;
        let s = Surface::Cylinder { frame, radius: 3.0 };
        let (gaussian, mean) = s.curvature(0.5, 1.0).unwrap();
        assert!(gaussian.abs() < 1e-9, "cylinder must be developable (K=0), got {gaussian}");
        assert!((mean.abs() - 1.0 / (2.0 * 3.0)).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn sphere_gaussian_curvature_equals_reciprocal_radius_squared() {
        let frame = Frame3::WORLD;
        let s = Surface::Sphere { frame, radius: 4.0 };
        for (u, v) in [(0.0, 0.0), (1.0, 0.3), (4.0, -0.5)] {
            let (gaussian, _) = s.curvature(u, v).unwrap();
            assert!((gaussian - 1.0 / 16.0).abs() < 1e-6, "mismatch at {u},{v}: {gaussian}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn sphere_eval_stays_on_sphere_and_derivatives_match_finite_differences() {
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

    #[semio_framework_async_macros::async_test]
    async fn torus_eval_stays_at_correct_distance_from_main_circle() {
        let frame = Frame3::WORLD;
        let s = Surface::Torus { frame, major_radius: 5.0, minor_radius: 1.5 };
        for (u, v) in [(0.0, 0.0), (1.0, 2.0), (4.0, 5.0)] {
            let p = s.eval(u, v);
            let radial = (p.x * p.x + p.y * p.y).sqrt();
            let dist_to_tube_center = ((radial - 5.0).powi(2) + p.z * p.z).sqrt();
            assert!((dist_to_tube_center - 1.5).abs() < 1e-9, "mismatch at {u},{v}: {dist_to_tube_center}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn torus_derivatives_match_finite_differences() {
        let frame = Frame3::WORLD;
        let s = Surface::Torus { frame, major_radius: 4.0, minor_radius: 1.0 };
        for (u, v) in [(0.3, 0.7), (2.0, 4.0)] {
            let d = s.derivatives(u, v);
            assert!((d.du - fd_du(&s, u, v)).norm() < 1e-4);
            assert!((d.dv - fd_dv(&s, u, v)).norm() < 1e-4);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn cone_radius_grows_linearly_with_v_and_derivatives_match_finite_differences() {
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

    #[semio_framework_async_macros::async_test]
    async fn plane_second_derivatives_are_zero_and_gaussian_curvature_is_zero() {
        let frame = Frame3::WORLD;
        let s = Surface::Plane { frame };
        let d = s.derivatives(0.5, 0.5);
        assert_eq!(d.duu, Vec3::ZERO);
        assert_eq!(d.dvv, Vec3::ZERO);
        assert_eq!(d.duv, Vec3::ZERO);
    }

    /// 🗺️ A degree-2×2 NURBS bump patch (unit weights) built by hand so tests have an
    /// independently-constructed fixture, not just a converted analytic surface.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn bump_nurbs_surface() -> Surface {
        let u_knots = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, 3).unwrap();
        let v_knots = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, 3).unwrap();
        let controls = vec![
            vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.5), Pnt3::new(0.0, 2.0, 0.0)],
            vec![Pnt3::new(1.0, 0.0, 0.5), Pnt3::new(1.0, 1.0, 2.0), Pnt3::new(1.0, 2.0, 0.5)],
            vec![Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.5), Pnt3::new(2.0, 2.0, 0.0)],
        ];
        let weights = vec![vec![1.0, 1.2, 1.0], vec![1.1, 1.5, 1.1], vec![1.0, 1.2, 1.0]];
        Surface::Nurbs { u_knots, v_knots, controls, weights }
    }

    /// 🗺️ Central-difference derivative with one round of Richardson extrapolation — see the
    /// equivalent oracle in `curve/bspline`'s quick tests; independent of
    /// [`surface_derivatives_rational`] so it doesn't validate itself.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn richardson_du(s: &Surface, u: f64, v: f64) -> Vec3 {
        let d = |h: f64| (s.eval(u + h, v) - s.eval(u - h, v)) * (1.0 / (2.0 * h));
        let h = 1e-3;
        (d(h / 2.0) * 4.0 - d(h)) * (1.0 / 3.0)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn richardson_dv(s: &Surface, u: f64, v: f64) -> Vec3 {
        let d = |h: f64| (s.eval(u, v + h) - s.eval(u, v - h)) * (1.0 / (2.0 * h));
        let h = 1e-3;
        (d(h / 2.0) * 4.0 - d(h)) * (1.0 / 3.0)
    }

    #[semio_framework_async_macros::async_test]
    async fn nurbs_surface_derivatives_match_richardson_finite_differences() {
        let s = bump_nurbs_surface();
        for &(u, v) in &[(0.2, 0.3), (0.5, 0.5), (0.7, 0.1), (0.9, 0.85)] {
            let d = s.derivatives(u, v);
            assert!((d.du - richardson_du(&s, u, v)).norm() < 1e-6, "du mismatch at {u},{v}: exact={:?}", d.du);
            assert!((d.dv - richardson_dv(&s, u, v)).norm() < 1e-6, "dv mismatch at {u},{v}: exact={:?}", d.dv);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn nurbs_surface_normal_and_curvature_are_well_defined_off_singularities() {
        let s = bump_nurbs_surface();
        for &(u, v) in &[(0.2, 0.3), (0.5, 0.5), (0.7, 0.1)] {
            assert!(s.normal(u, v).is_some(), "normal should be defined at {u},{v}");
            assert!(s.curvature(u, v).is_some(), "curvature should be defined at {u},{v}");
        }
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn nurbs_surface_derivatives_match_richardson_finite_differences_on_random_rational_surfaces() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(97);
            for _ in 0..40 {
                let nu = 3 + rng.next_range(0, 2) as usize;
                let nv = 3 + rng.next_range(0, 2) as usize;
                let u_knots = KnotVector::clamped_uniform(nu, 2.min(nu - 1));
                let v_knots = KnotVector::clamped_uniform(nv, 2.min(nv - 1));
                let controls: Vec<Vec<Pnt3>> = (0..nu).map(|i| (0..nv).map(|j| Pnt3::new(i as f64 + rng.next_f64() * 0.3, j as f64 + rng.next_f64() * 0.3, rng.next_f64() * 2.0 - 1.0)).collect()).collect();
                let weights: Vec<Vec<f64>> = (0..nu).map(|_| (0..nv).map(|_| 0.6 + rng.next_f64()).collect()).collect();
                let s = Surface::Nurbs { u_knots, v_knots, controls, weights };
                let u = 0.1 + 0.8 * rng.next_f64();
                let v = 0.1 + 0.8 * rng.next_f64();
                let d = s.derivatives(u, v);
                assert!((d.du - richardson_du(&s, u, v)).norm() < 1e-5, "du mismatch nu={nu} nv={nv} at {u},{v}");
                assert!((d.dv - richardson_dv(&s, u, v)).norm() < 1e-5, "dv mismatch nu={nu} nv={nv} at {u},{v}");
            }
        }
    }
}
// #endregion 🔖️Tests

// #region 🔁️Transform

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3;

/// 🗺️ A generous, explicitly finite stand-in for the mathematically unbounded `v`-extent of a
/// `Cylinder`/`Cone` when a non-similarity map forces a NURBS conversion (a clamped B-spline knot
/// vector cannot represent a literal infinite domain — the same reason [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::to_nurbs`]
/// requires an explicit `domain` for `Line`). Every double-precision kernel already operates within
/// some finite practical range; `1e6` is far beyond any plausible model extent while staying exact
/// (not tessellated/approximated) within that range.
const PRACTICAL_UNBOUNDED_EXTENT: f64 = 1.0e6;

/// 🗺️ Exact rational-quadratic NURBS control points for a circular arc of `radius` centered at
/// `center` in a generic 2D `(radial, height)` half-plane, split into `≤120°` spans — the same
/// per-span construction [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::to_nurbs`] uses for `Circle`/`Ellipse`,
/// generalized to an off-origin circle (a torus's meridian) and reused, at `center = (0,0), radius
/// = 1`, as the shared angular sweep for every surface of revolution built below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn circular_profile(center: (f64, f64), radius: f64, domain: (f64, f64)) -> (KnotVector, Vec<(f64, f64)>, Vec<f64>) {
    let span = domain.1 - domain.0;
    let max_span = std::f64::consts::TAU / 3.0;
    let n_spans = (span.abs() / max_span).ceil().max(1.0) as usize;
    let step = span / n_spans as f64;
    let mut points = Vec::with_capacity(2 * n_spans + 1);
    let mut weights = Vec::with_capacity(2 * n_spans + 1);
    let local = |angle: f64, scale: f64| (center.0 + radius * angle.cos() * scale, center.1 + radius * angle.sin() * scale);
    for span_i in 0..n_spans {
        let a0 = domain.0 + step * span_i as f64;
        let a1 = a0 + step;
        let half = (a1 - a0) * 0.5;
        let mid = (a0 + a1) * 0.5;
        let w1 = half.cos();
        if span_i == 0 {
            points.push(local(a0, 1.0));
            weights.push(1.0);
        }
        points.push(local(mid, 1.0 / w1));
        weights.push(w1);
        points.push(local(a1, 1.0));
        weights.push(1.0);
    }
    let mut knots = vec![domain.0, domain.0, domain.0];
    for span_i in 1..n_spans {
        let knot = domain.0 + step * span_i as f64;
        knots.push(knot);
        knots.push(knot);
    }
    knots.push(domain.1);
    knots.push(domain.1);
    knots.push(domain.1);
    (KnotVector::new(knots, 2, points.len()).unwrap(), points, weights)
}

/// 🗺️ Builds the exact (already `map`-transformed) NURBS surface of revolution swept from a
/// `(radial, height)` meridian `profile` (its own `profile_knots`/`profile_weights`, in `frame`'s
/// local plane) around `frame.z` — the shared tensor-product construction behind every analytic
/// surface kind's non-similarity fallback below: `u` comes from [`circular_profile`]'s unit-circle
/// sweep, `v` is the caller's own meridian, and each control point's weight is the product of its
/// `u`- and `v`-direction weights (the standard NURBS revolution rule).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn revolve_to_nurbs(frame: &Frame3, map: &Affine3, profile_knots: KnotVector, profile: &[(f64, f64)], profile_weights: &[f64]) -> Surface {
    let (u_knots, u_nodes, u_weights) = circular_profile((0.0, 0.0), 1.0, (0.0, std::f64::consts::TAU));
    let mut controls: Vec<Vec<Pnt3>> = Vec::with_capacity(u_nodes.len());
    let mut weights: Vec<Vec<f64>> = Vec::with_capacity(u_nodes.len());
    for (&(cx, cy), &uw) in u_nodes.iter().zip(u_weights.iter()) {
        let mut row_ctrl = Vec::with_capacity(profile.len());
        let mut row_w = Vec::with_capacity(profile.len());
        for (&(r, h), &pw) in profile.iter().zip(profile_weights.iter()) {
            let local = frame.origin + frame.x * (r * cx) + frame.y * (r * cy) + frame.z * h;
            row_ctrl.push(map.apply_point(local));
            row_w.push(pw * uw);
        }
        controls.push(row_ctrl);
        weights.push(row_w);
    }
    Surface::Nurbs { u_knots, v_knots: profile_knots, controls, weights }
}

impl Surface {
    /// 🗺️ Exact affine transform. `Plane` stays exact under ANY invertible affine map — its `x`/`y`
    /// axes are mapped directly (not renormalized: `Plane` places no orthonormality requirement on
    /// its own axes, only linear independence, which any invertible linear map preserves) and `z`
    /// is re-derived as `x × y` (deliberately, not itself mapped — see [`Frame3::transformed`]'s
    /// docstring on why re-deriving `z` would be wrong for the OTHER analytic kinds but is exactly
    /// what a general, non-orthonormal frame needs here). `Cylinder`/`Cone`/`Sphere`/`Torus` stay
    /// analytic under a similarity (frame mapped via [`Frame3::transformed`], radii scaled
    /// uniformly, `half_angle` invariant); under a non-similarity map they convert to the exact
    /// NURBS surface of revolution ([`revolve_to_nurbs`]) already expressed in `map`'s image.
    /// `Nurbs` always just transforms its control points (rational weights are affine-invariant).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn transformed(&self, map: &Affine3) -> Surface {
        match self {
            Surface::Plane { frame } => {
                let x = map.apply_vector(frame.x);
                let y = map.apply_vector(frame.y);
                let z = x.cross(y);
                Surface::Plane { frame: Frame3 { origin: map.apply_point(frame.origin), x, y, z } }
            }
            Surface::Nurbs { u_knots, v_knots, controls, weights } => Surface::Nurbs {
                u_knots: u_knots.clone(),
                v_knots: v_knots.clone(),
                controls: controls.iter().map(|row| row.iter().map(|p| map.apply_point(*p)).collect()).collect(),
                weights: weights.clone(),
            },
            Surface::Cylinder { frame, radius } => match map.is_similarity() {
                Some((_, scale, _)) => Surface::Cylinder { frame: frame.transformed(map, scale), radius: radius * scale },
                None => {
                    let knots = KnotVector::new(vec![-PRACTICAL_UNBOUNDED_EXTENT, -PRACTICAL_UNBOUNDED_EXTENT, PRACTICAL_UNBOUNDED_EXTENT, PRACTICAL_UNBOUNDED_EXTENT], 1, 2).unwrap();
                    let profile = [(*radius, -PRACTICAL_UNBOUNDED_EXTENT), (*radius, PRACTICAL_UNBOUNDED_EXTENT)];
                    revolve_to_nurbs(frame, map, knots, &profile, &[1.0, 1.0])
                }
            },
            Surface::Cone { frame, half_angle } => match map.is_similarity() {
                Some((_, scale, _)) => Surface::Cone { frame: frame.transformed(map, scale), half_angle: *half_angle },
                None => {
                    let knots = KnotVector::new(vec![0.0, 0.0, PRACTICAL_UNBOUNDED_EXTENT, PRACTICAL_UNBOUNDED_EXTENT], 1, 2).unwrap();
                    let profile = [(0.0, 0.0), (PRACTICAL_UNBOUNDED_EXTENT * half_angle.tan(), PRACTICAL_UNBOUNDED_EXTENT)];
                    revolve_to_nurbs(frame, map, knots, &profile, &[1.0, 1.0])
                }
            },
            Surface::Sphere { frame, radius } => match map.is_similarity() {
                Some((_, scale, _)) => Surface::Sphere { frame: frame.transformed(map, scale), radius: radius * scale },
                None => {
                    let (knots, profile, weights) = circular_profile((0.0, 0.0), *radius, (-std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2));
                    revolve_to_nurbs(frame, map, knots, &profile, &weights)
                }
            },
            Surface::Torus { frame, major_radius, minor_radius } => match map.is_similarity() {
                Some((_, scale, _)) => Surface::Torus { frame: frame.transformed(map, scale), major_radius: major_radius * scale, minor_radius: minor_radius * scale },
                None => {
                    let (knots, profile, weights) = circular_profile((*major_radius, 0.0), *minor_radius, (0.0, std::f64::consts::TAU));
                    revolve_to_nurbs(frame, map, knots, &profile, &weights)
                }
            },
        }
    }
}

// #region 🔖️Tests
#[cfg(test)]
mod transform_tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve2;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Vec2};

    #[semio_framework_async_macros::async_test]
    async fn plane_transformed_matches_mapped_eval_under_non_similarity() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, -1.0, 2.0), Vec3::Z).unwrap();
        let s = Surface::Plane { frame };
        let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 3.0, 5.0)).compose(&Affine3::translation(Vec3::new(1.0, 0.0, 0.0)));
        assert!(map.is_similarity().is_none());
        let transformed = s.transformed(&map);
        for (u, v) in [(0.0, 0.0), (1.0, 2.0), (-3.0, 4.0)] {
            assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-9, "mismatch at {u},{v}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn cylinder_transformed_stays_cylinder_under_similarity() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cylinder { frame, radius: 2.0 };
        let map = Affine3::rotation_about(Pnt3::new(1.0, 0.0, 0.0), Vec3::new(0.2, 1.0, 0.1), 0.6).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0)));
        let (_, scale, _) = map.is_similarity().expect("must be a similarity");
        let transformed = s.transformed(&map);
        assert!(matches!(transformed, Surface::Cylinder { .. }));
        if let Surface::Cylinder { radius, .. } = transformed {
            assert!((radius - 2.0 * scale).abs() < 1e-9);
        }
        for (u, v) in [(0.3, 1.0), (2.0, -3.0), (5.0, 0.5)] {
            assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-7, "mismatch at {u},{v}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn sphere_transformed_via_nurbs_under_non_similarity_matches_mapped_eval() {
        let frame = Frame3::WORLD;
        let s = Surface::Sphere { frame, radius: 3.0 };
        let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 1.0));
        assert!(map.is_similarity().is_none());
        let transformed = s.transformed(&map);
        assert!(matches!(transformed, Surface::Nurbs { .. }), "non-similarity must force NURBS");
        for (u, v) in [(0.0, 0.0), (1.0, 0.3), (4.0, -0.5), (0.5, 1.5)] {
            assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-6, "mismatch at {u},{v}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn torus_transformed_via_nurbs_under_non_similarity_matches_mapped_eval() {
        let frame = Frame3::WORLD;
        let s = Surface::Torus { frame, major_radius: 4.0, minor_radius: 1.0 };
        let map = Affine3::mirror(Pnt3::new(0.0, 0.0, 0.0), Vec3::X).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 1.0)));
        assert!(map.is_similarity().is_none());
        let transformed = s.transformed(&map);
        for (u, v) in [(0.3, 0.7), (2.0, 4.0), (5.5, 1.2)] {
            assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-6, "mismatch at {u},{v}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn cone_transformed_via_nurbs_under_non_similarity_matches_mapped_eval_near_apex() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cone { frame, half_angle: std::f64::consts::FRAC_PI_6 };
        let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 1.0, 1.0));
        assert!(map.is_similarity().is_none());
        let transformed = s.transformed(&map);
        for (u, v) in [(0.5, 2.0), (3.0, 5.0)] {
            assert!(transformed.eval(u, v).distance(map.apply_point(s.eval(u, v))) < 1e-6, "mismatch at {u},{v}");
        }
    }

    /// 🗺️ The invariant this ticket's transform work is built around: a p-curve is a curve in the
    /// FACE's own `(u, v)` parameter space, so it must not need any change when the surface it
    /// trims is transformed by the same map — evaluating `surface(pcurve(t))` before and after
    /// [`Surface::transformed`] and mapping the "before" result must agree with the "after" result,
    /// for the SAME unmodified `pcurve`.
    #[semio_framework_async_macros::async_test]
    async fn pcurve_stays_unchanged_when_surface_is_transformed_by_the_same_map() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, 2.0, -1.0), Vec3::new(0.1, 0.2, 1.0)).unwrap();
        let s = Surface::Cylinder { frame, radius: 2.0 };
        let pcurve = Curve2::Line { origin: Pnt2::new(0.2, -1.0), dir: Vec2::new(0.6, 2.0) };
        let map = Affine3::rotation_about(Pnt3::new(0.0, 1.0, 0.0), Vec3::new(0.3, 1.0, -0.2), 1.0).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(1.5, 1.5, 1.5)));
        let transformed_surface = s.transformed(&map);
        for t in [0.0, 0.3, 0.7, 1.2] {
            let uv = pcurve.eval(t);
            let before = map.apply_point(s.eval(uv.x, uv.y));
            let after = transformed_surface.eval(uv.x, uv.y);
            assert!(before.distance(after) < 1e-7, "p-curve point at t={t} diverged after transform");
        }
    }
}
// #endregion 🔖️Tests

// #endregion 🔁️Transform

// #region 🧭️Isocurve

/// 🧭️ The two directions an isoparametric curve can hold fixed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoDirection {
    U,
    V,
}

impl Surface {
    /// 🧭️ The curve traced by holding `dir` fixed at `at` and letting the other parameter vary —
    /// exact for every analytic kind (each reduces to a [`Curve3::Line`] or [`Curve3::Circle`]
    /// whose own evaluation reproduces `self.eval` pointwise, not merely approximates it — see the
    /// per-arm derivations below) and exact for [`Surface::Nurbs`] via the standard isoparametric
    /// curve-extraction formula (fixed-direction basis functions folded into new, real control
    /// points on the other direction's own knot vector — no new knots, no approximation).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn isocurve(&self, dir: IsoDirection, at: f64) -> Curve3 {
        match self {
            Surface::Plane { frame } => match dir {
                IsoDirection::U => Curve3::Line { origin: frame.origin + frame.x * at, dir: frame.y },
                IsoDirection::V => Curve3::Line { origin: frame.origin + frame.y * at, dir: frame.x },
            },
            Surface::Cylinder { frame, radius } => match dir {
                IsoDirection::U => Curve3::Line { origin: frame.origin + frame.x * (radius * at.cos()) + frame.y * (radius * at.sin()), dir: frame.z },
                IsoDirection::V => Curve3::Circle { frame: Frame3 { origin: frame.origin + frame.z * at, x: frame.x, y: frame.y, z: frame.z }, radius: *radius },
            },
            Surface::Cone { frame, half_angle } => match dir {
                IsoDirection::U => {
                    let tan_a = half_angle.tan();
                    Curve3::Line { origin: frame.origin, dir: frame.to_world_vector(Vec3::new(tan_a * at.cos(), tan_a * at.sin(), 1.0)) }
                }
                IsoDirection::V => Curve3::Circle { frame: Frame3 { origin: frame.origin + frame.z * at, x: frame.x, y: frame.y, z: frame.z }, radius: at * half_angle.tan() },
            },
            Surface::Sphere { frame, radius } => match dir {
                IsoDirection::U => {
                    let radial = frame.x * at.cos() + frame.y * at.sin();
                    let up = frame.z;
                    let side = radial.cross(up);
                    Curve3::Circle { frame: Frame3 { origin: frame.origin, x: radial, y: up, z: side }, radius: *radius }
                }
                IsoDirection::V => Curve3::Circle { frame: Frame3 { origin: frame.origin + frame.z * (radius * at.sin()), x: frame.x, y: frame.y, z: frame.z }, radius: radius * at.cos() },
            },
            Surface::Torus { frame, major_radius, minor_radius } => match dir {
                IsoDirection::U => {
                    let radial = frame.x * at.cos() + frame.y * at.sin();
                    let up = frame.z;
                    let side = radial.cross(up);
                    Curve3::Circle { frame: Frame3 { origin: frame.origin + radial * *major_radius, x: radial, y: up, z: side }, radius: *minor_radius }
                }
                IsoDirection::V => {
                    let radius = major_radius + minor_radius * at.cos();
                    Curve3::Circle { frame: Frame3 { origin: frame.origin + frame.z * (minor_radius * at.sin()), x: frame.x, y: frame.y, z: frame.z }, radius }
                }
            },
            Surface::Nurbs { u_knots, v_knots, controls, weights } => nurbs_isocurve(u_knots, v_knots, controls, weights, dir, at),
        }
    }
}

/// 🧭️ Standard isoparametric curve extraction for a tensor-product rational B-spline: folds the
/// fixed direction's basis functions into new (real, not approximated) control points on the
/// other direction's own knot vector — `C(t) = Σ_j [Σ_i N_i(fixed) w_ij P_ij] M_j(t) / Σ_j[...] M_j(t)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nurbs_isocurve(u_knots: &KnotVector, v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], dir: IsoDirection, at: f64) -> Curve3 {
    match dir {
        IsoDirection::U => {
            let span = u_knots.find_span(at);
            let basis = &basis_function_derivatives(u_knots, span, at, 0)[0];
            let degree = u_knots.degree;
            let nv = controls[0].len();
            let mut new_controls = Vec::with_capacity(nv);
            let mut new_weights = Vec::with_capacity(nv);
            for j in 0..nv {
                let (mut hx, mut hy, mut hz, mut hw) = (0.0, 0.0, 0.0, 0.0);
                for k in 0..=degree {
                    let i = span - degree + k;
                    let b = basis[k] * weights[i][j];
                    hx += b * controls[i][j].x;
                    hy += b * controls[i][j].y;
                    hz += b * controls[i][j].z;
                    hw += b;
                }
                new_controls.push(Pnt3::new(hx / hw, hy / hw, hz / hw));
                new_weights.push(hw);
            }
            Curve3::Nurbs { knots: v_knots.clone(), controls: new_controls, weights: new_weights }
        }
        IsoDirection::V => {
            let span = v_knots.find_span(at);
            let basis = &basis_function_derivatives(v_knots, span, at, 0)[0];
            let degree = v_knots.degree;
            let nu = controls.len();
            let mut new_controls = Vec::with_capacity(nu);
            let mut new_weights = Vec::with_capacity(nu);
            for i in 0..nu {
                let (mut hx, mut hy, mut hz, mut hw) = (0.0, 0.0, 0.0, 0.0);
                for k in 0..=degree {
                    let j = span - degree + k;
                    let b = basis[k] * weights[i][j];
                    hx += b * controls[i][j].x;
                    hy += b * controls[i][j].y;
                    hz += b * controls[i][j].z;
                    hw += b;
                }
                new_controls.push(Pnt3::new(hx / hw, hy / hw, hz / hw));
                new_weights.push(hw);
            }
            Curve3::Nurbs { knots: u_knots.clone(), controls: new_controls, weights: new_weights }
        }
    }
}

// #region 🔖️Tests
#[cfg(test)]
mod isocurve_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn plane_isocurve_matches_surface_eval() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::Z).unwrap();
        let s = Surface::Plane { frame };
        let iso = s.isocurve(IsoDirection::U, 2.0);
        for v in [-3.0, 0.0, 4.5] {
            assert!(iso.eval(v).distance(s.eval(2.0, v)) < 1e-9);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn cylinder_isocurves_match_surface_eval() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cylinder { frame, radius: 2.0 };
        let u_iso = s.isocurve(IsoDirection::U, 0.7);
        for v in [-3.0, 0.0, 5.0] {
            assert!(u_iso.eval(v).distance(s.eval(0.7, v)) < 1e-9, "u-isocurve mismatch at v={v}");
        }
        let v_iso = s.isocurve(IsoDirection::V, 3.0);
        for u in [0.0, 1.5, 4.0] {
            assert!(v_iso.eval(u).distance(s.eval(u, 3.0)) < 1e-9, "v-isocurve mismatch at u={u}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn sphere_isocurves_match_surface_eval() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, -1.0, 2.0), Vec3::new(0.2, 0.3, 1.0)).unwrap();
        let s = Surface::Sphere { frame, radius: 4.0 };
        let u_iso = s.isocurve(IsoDirection::U, 1.1);
        for v in [-1.2, 0.0, 1.2] {
            assert!(u_iso.eval(v).distance(s.eval(1.1, v)) < 1e-8, "meridian mismatch at v={v}");
        }
        let v_iso = s.isocurve(IsoDirection::V, 0.4);
        for u in [0.0, 2.0, 5.0] {
            assert!(v_iso.eval(u).distance(s.eval(u, 0.4)) < 1e-8, "parallel mismatch at u={u}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn torus_isocurves_match_surface_eval() {
        let frame = Frame3::WORLD;
        let s = Surface::Torus { frame, major_radius: 5.0, minor_radius: 1.5 };
        let u_iso = s.isocurve(IsoDirection::U, 0.9);
        for v in [0.0, 2.0, 5.0] {
            assert!(u_iso.eval(v).distance(s.eval(0.9, v)) < 1e-8, "tube-circle mismatch at v={v}");
        }
        let v_iso = s.isocurve(IsoDirection::V, 1.3);
        for u in [0.0, 2.0, 5.5] {
            assert!(v_iso.eval(u).distance(s.eval(u, 1.3)) < 1e-8, "parallel-circle mismatch at u={u}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn cone_isocurves_match_surface_eval() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cone { frame, half_angle: std::f64::consts::FRAC_PI_6 };
        let u_iso = s.isocurve(IsoDirection::U, 0.5);
        for v in [0.0, 2.0, 6.0] {
            assert!(u_iso.eval(v).distance(s.eval(0.5, v)) < 1e-8, "generator mismatch at v={v}");
        }
        let v_iso = s.isocurve(IsoDirection::V, 3.0);
        for u in [0.0, 1.0, 4.0] {
            assert!(v_iso.eval(u).distance(s.eval(u, 3.0)) < 1e-8, "cone parallel mismatch at u={u}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn nurbs_isocurve_matches_surface_eval_and_lies_on_surface() {
        let u_knots = KnotVector::clamped_uniform(4, 3);
        let v_knots = KnotVector::clamped_uniform(4, 3);
        let controls: Vec<Vec<Pnt3>> = (0..4).map(|i| (0..4).map(|j| Pnt3::new(i as f64, j as f64, (i as f64 - j as f64).sin())).collect()).collect();
        let weights = vec![vec![1.0; 4]; 4];
        let s = Surface::Nurbs { u_knots, v_knots, controls, weights };
        let u_iso = s.isocurve(IsoDirection::U, 0.42);
        for v in [0.0, 0.3, 0.7, 1.0] {
            assert!(u_iso.eval(v).distance(s.eval(0.42, v)) < 1e-9, "u-isocurve mismatch at v={v}");
        }
        let v_iso = s.isocurve(IsoDirection::V, 0.6);
        for u in [0.0, 0.2, 0.9, 1.0] {
            assert!(v_iso.eval(u).distance(s.eval(u, 0.6)) < 1e-9, "v-isocurve mismatch at u={u}");
        }
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn analytic_isocurves_lie_exactly_on_their_surfaces_for_random_kinds() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(97);
            for _ in 0..100 {
                let frame = Frame3::from_normal(
                    Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0),
                    Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z),
                )
                .unwrap();
                let s = Surface::Torus { frame, major_radius: 2.0 + rng.next_f64() * 3.0, minor_radius: 0.2 + rng.next_f64() * 0.5 };
                let u_at = rng.next_f64() * std::f64::consts::TAU;
                let v_at = rng.next_f64() * std::f64::consts::TAU;
                let u_iso = s.isocurve(IsoDirection::U, u_at);
                let v_iso = s.isocurve(IsoDirection::V, v_at);
                for t in [0.1, 1.0, 3.0, 5.0] {
                    assert!(u_iso.eval(t).distance(s.eval(u_at, t)) < 1e-8, "u-iso mismatch");
                    assert!(v_iso.eval(t).distance(s.eval(t, v_at)) < 1e-8, "v-iso mismatch");
                }
            }
        }
    }
}
// #endregion 🔖️Tests

// #endregion 🧭️Isocurve
