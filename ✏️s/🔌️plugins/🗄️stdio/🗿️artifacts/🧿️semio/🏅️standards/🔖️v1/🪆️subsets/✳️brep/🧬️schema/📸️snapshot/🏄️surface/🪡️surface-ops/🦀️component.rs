//! 🧭️ Surface algorithms that operate *on* a [`super::Surface`]: closest-point
//! projection (with closed-form fast paths for the surfaces that admit one) and Coons-patch
//! transfinite interpolation from four boundary curves. Kept separate from `surface.rs` for the
//! same reason as [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops`] versus [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve`].
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🪡️surface-ops` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `🏄️surface` per that file's own pre-mounted-stub note.

use super::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// #region 🔖️Project

/// 🧭️ Closest point on `surface` (restricted to `domain`) to `target`. Uses an exact closed form
/// for [`Surface::Plane`] and [`Surface::Sphere`]; otherwise coarse-grid seeding followed by a 2D
/// Newton iteration on the first-order optimality conditions `(S(u,v)-P)·Su = 0`, `(S(u,v)-P)·Sv = 0`.
/// Returns `(u, v, distance)`.
pub async fn closest_point(surface: &Surface, domain: ((f64, f64), (f64, f64)), target: Pnt3, samples: usize) -> (f64, f64, f64) {
    match surface {
        Surface::Plane { frame } => {
            let local = frame.to_local(target);
            let p = surface.eval(local.x, local.y);
            (local.x, local.y, p.distance(target))
        }
        Surface::Sphere { frame, .. } => {
            let local = frame.to_local(target).to_vec();
            let n = local.normalized().unwrap_or(Vec3::Z);
            let v = n.z.clamp(-1.0, 1.0).asin();
            let u = n.y.atan2(n.x).rem_euclid(std::f64::consts::TAU);
            let p = surface.eval(u, v);
            (u, v, p.distance(target))
        }
        _ => closest_point_numeric(surface, domain, target, samples),
    }
}

/// 🧭️ Wraps into a periodic domain (mirrors [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops`]'s identical fix for closed
/// curves) rather than clamping — otherwise Newton can get trapped exactly at a domain boundary
/// when the true optimum sits just across the periodic seam.
async fn wrap_or_clamp(x: f64, lo: f64, hi: f64, periodic: bool) -> f64 {
    if periodic {
        let period = hi - lo;
        let mut w = (x - lo) % period;
        if w < 0.0 {
            w += period;
        }
        lo + w
    } else {
        x.clamp(lo, hi)
    }
}

async fn closest_point_numeric(surface: &Surface, domain: ((f64, f64), (f64, f64)), target: Pnt3, samples: usize) -> (f64, f64, f64) {
    let (u_dom, v_dom) = domain;
    let u_periodic = surface.is_u_periodic();
    let v_periodic = surface.is_v_periodic();
    let u_hi = if u_dom.1.is_finite() { u_dom.1 } else { u_dom.0 + std::f64::consts::TAU };
    let v_hi = if v_dom.1.is_finite() { v_dom.1 } else { v_dom.0 + 10.0 };
    let v_lo = if v_dom.0.is_finite() { v_dom.0 } else { v_hi - 20.0 };
    let mut best = (u_dom.0, v_lo, f64::INFINITY);
    for i in 0..=samples {
        for j in 0..=samples {
            let u = u_dom.0 + (u_hi - u_dom.0) * (i as f64 / samples as f64);
            let v = v_lo + (v_hi - v_lo) * (j as f64 / samples as f64);
            let d = surface.eval(u, v).distance_sq(target);
            if d < best.2 {
                best = (u, v, d);
            }
        }
    }
    best.2 = best.2.sqrt();
    let (mut u, mut v, _) = best;
    for _ in 0..30 {
        let d = surface.derivatives(u, v);
        let delta = d.point - target;
        let fu = delta.dot(d.du);
        let fv = delta.dot(d.dv);
        let fuu = d.du.dot(d.du) + delta.dot(d.duu);
        let fuv = d.du.dot(d.dv) + delta.dot(d.duv);
        let fvv = d.dv.dot(d.dv) + delta.dot(d.dvv);
        let det = fuu * fvv - fuv * fuv;
        if det.abs() <= 1e-300 {
            break;
        }
        let step_u = (fu * fvv - fv * fuv) / det;
        let step_v = (fv * fuu - fu * fuv) / det;
        let next_u = wrap_or_clamp(u - step_u, u_dom.0, u_hi, u_periodic);
        let next_v = wrap_or_clamp(v - step_v, v_lo, v_hi, v_periodic);
        if (next_u - u).abs() < 1e-13 && (next_v - v).abs() < 1e-13 {
            u = next_u;
            v = next_v;
            break;
        }
        u = next_u;
        v = next_v;
    }
    let refined_dist = surface.eval(u, v).distance(target);
    if refined_dist < best.2 {
        (u, v, refined_dist)
    } else {
        best
    }
}

// #endregion 🔖️Project

// #region 🔖️Coons

/// 🧭️ Bilinear Coons-patch transfinite interpolation from four boundary curves parametrized on
/// `[0, 1]`: `c0`/`c1` are the `v=0`/`v=1` boundaries (functions of `u`), `d0`/`d1` are the `u=0`/
/// `u=1` boundaries (functions of `v`). Requires the four curves to agree at shared corners
/// (`c0(0)==d0(0)`, `c0(1)==d1(0)`, `c1(0)==d0(1)`, `c1(1)==d1(1)`) — the caller is responsible for
/// that consistency; this function does not check it.
pub async fn coons_patch_eval(c0: &dyn Fn(f64) -> Pnt3, c1: &dyn Fn(f64) -> Pnt3, d0: &dyn Fn(f64) -> Pnt3, d1: &dyn Fn(f64) -> Pnt3, u: f64, v: f64) -> Pnt3 {
    let p00 = c0(0.0);
    let p10 = c0(1.0);
    let p01 = c1(0.0);
    let p11 = c1(1.0);
    let ruled_uv = c0(u).to_vec() * (1.0 - v) + c1(u).to_vec() * v;
    let ruled_vu = d0(v).to_vec() * (1.0 - u) + d1(v).to_vec() * u;
    let bilinear_corners = (p00.to_vec() * (1.0 - u) * (1.0 - v) + p10.to_vec() * u * (1.0 - v) + p01.to_vec() * (1.0 - u) * v + p11.to_vec() * u * v).to_array();
    Pnt3::from_array((ruled_uv + ruled_vu).to_array()) - Vec3::from_array(bilinear_corners)
}

// #endregion 🔖️Coons

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;

    #[test]
    async fn closest_point_on_plane_matches_orthogonal_projection() {
        let frame = Frame3::WORLD;
        let s = Surface::Plane { frame };
        let target = Pnt3::new(2.0, 3.0, 5.0);
        let (u, v, d) = closest_point(&s, s.domain(), target, 10);
        assert!((u - 2.0).abs() < 1e-9);
        assert!((v - 3.0).abs() < 1e-9);
        assert!((d - 5.0).abs() < 1e-9);
    }

    #[test]
    async fn closest_point_on_sphere_matches_radial_projection() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, 1.0, 1.0), Vec3::Z).unwrap();
        let s = Surface::Sphere { frame, radius: 3.0 };
        let target = Pnt3::new(1.0, 1.0, 21.0); // 20 units above the sphere along its axis
        let (_, _, d) = closest_point(&s, s.domain(), target, 10);
        assert!((d - 17.0).abs() < 1e-6);
    }

    #[test]
    async fn closest_point_on_cylinder_matches_expected_geometry() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cylinder { frame, radius: 2.0 };
        let target = Pnt3::new(10.0, 0.0, 5.0);
        let (u, v, d) = closest_point(&s, ((0.0, std::f64::consts::TAU), (0.0, 10.0)), target, 32);
        assert!((d - 8.0).abs() < 1e-5, "distance mismatch: {d}");
        let p = s.eval(u, v);
        assert!(p.distance(Pnt3::new(2.0, 0.0, 5.0)) < 1e-5);
    }

    #[test]
    async fn coons_patch_reproduces_boundary_curves_exactly() {
        let c0 = |u: f64| Pnt3::new(u, 0.0, 0.0);
        let c1 = |u: f64| Pnt3::new(u, 1.0, u * u);
        let d0 = |v: f64| Pnt3::new(0.0, v, 0.0);
        let d1 = |v: f64| Pnt3::new(1.0, v, v);
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, t, 0.0).distance(c0(t)) < 1e-9, "v=0 boundary mismatch at u={t}");
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, t, 1.0).distance(c1(t)) < 1e-9, "v=1 boundary mismatch at u={t}");
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, 0.0, t).distance(d0(t)) < 1e-9, "u=0 boundary mismatch at v={t}");
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, 1.0, t).distance(d1(t)) < 1e-9, "u=1 boundary mismatch at v={t}");
        }
    }

    #[test]
    async fn coons_patch_of_planar_boundaries_is_the_bilinear_plane() {
        let c0 = |u: f64| Pnt3::new(u, 0.0, 0.0);
        let c1 = |u: f64| Pnt3::new(u, 1.0, 0.0);
        let d0 = |v: f64| Pnt3::new(0.0, v, 0.0);
        let d1 = |v: f64| Pnt3::new(1.0, v, 0.0);
        let p = coons_patch_eval(&c0, &c1, &d0, &d1, 0.3, 0.7);
        assert!(p.distance(Pnt3::new(0.3, 0.7, 0.0)) < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        async fn closest_point_on_cylinder_matches_brute_force_grid_oracle() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(71);
            for _ in 0..50 {
                let frame =
                    Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z))
                        .unwrap();
                let radius = 0.5 + rng.next_f64() * 5.0;
                let s = Surface::Cylinder { frame, radius };
                let target = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let domain = ((0.0, std::f64::consts::TAU), (-15.0, 15.0));
                let (_, _, dist) = closest_point(&s, domain, target, 24);
                let mut oracle = f64::INFINITY;
                for i in 0..2000 {
                    let u = std::f64::consts::TAU * i as f64 / 2000.0;
                    for j in 0..200 {
                        let v = -15.0 + 30.0 * j as f64 / 200.0;
                        oracle = oracle.min(s.eval(u, v).distance(target));
                    }
                }
                assert!(dist <= oracle + 1e-3, "newton found {dist} worse than oracle {oracle}");
            }
        }
    }
}
// #endregion 🔖️Tests
