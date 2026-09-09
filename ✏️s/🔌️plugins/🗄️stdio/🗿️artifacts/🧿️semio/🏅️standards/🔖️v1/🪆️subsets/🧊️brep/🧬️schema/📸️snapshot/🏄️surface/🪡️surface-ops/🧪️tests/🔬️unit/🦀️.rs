use super::*;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;

#[semio_framework_async_macros::async_test]
async fn closest_point_on_plane_matches_orthogonal_projection() {
    let frame = Frame3::WORLD;
    let s = Surface::Plane { frame };
    let target = Pnt3::new(2.0, 3.0, 5.0);
    let cp = closest_uv(&s, s.domain(), target, 1e-9);
    assert!(cp.certified);
    assert!((cp.u - 2.0).abs() < 1e-9);
    assert!((cp.v - 3.0).abs() < 1e-9);
    assert!((cp.distance - 5.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn closest_point_on_sphere_matches_radial_projection() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, 1.0, 1.0), Vec3::Z).unwrap();
    let s = Surface::Sphere { frame, radius: 3.0 };
    let target = Pnt3::new(1.0, 1.0, 21.0); // 20 units above the sphere along its axis
    let cp = closest_uv(&s, s.domain(), target, 1e-9);
    assert!((cp.distance - 17.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn closest_point_on_sphere_pole_is_certified_and_exact() {
    let frame = Frame3::WORLD;
    let s = Surface::Sphere { frame, radius: 2.0 };
    let target = Pnt3::new(0.0, 0.0, 10.0); // directly above the north pole
    let cp = closest_uv(&s, s.domain(), target, 1e-9);
    assert!(cp.certified);
    assert!((cp.distance - 8.0).abs() < 1e-6);
    assert!(cp.point.distance(Pnt3::new(0.0, 0.0, 2.0)) < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn closest_point_on_cylinder_matches_expected_geometry() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cylinder { frame, radius: 2.0 };
    let target = Pnt3::new(10.0, 0.0, 5.0);
    let cp = closest_uv(&s, ((0.0, std::f64::consts::TAU), (0.0, 10.0)), target, 1e-9);
    assert!((cp.distance - 8.0).abs() < 1e-5, "distance mismatch: {}", cp.distance);
    assert!(cp.point.distance(Pnt3::new(2.0, 0.0, 5.0)) < 1e-5);
}

#[semio_framework_async_macros::async_test]
async fn closest_point_on_cone_apex_when_unconstrained_minimum_is_negative() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cone { frame, half_angle: std::f64::consts::FRAC_PI_6 };
    // Straight down from the apex: the unconstrained v* is negative, so the true closest
    // point on the (v >= 0) cone is the apex itself.
    let target = Pnt3::new(0.0, 0.0, -5.0);
    let cp = closest_uv(&s, s.domain(), target, 1e-9);
    assert!(cp.certified);
    assert!((cp.distance - 5.0).abs() < 1e-6);
    assert!(cp.point.distance(Pnt3::new(0.0, 0.0, 0.0)) < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn closest_point_on_torus_matches_meridional_projection() {
    let frame = Frame3::WORLD;
    let s = Surface::Torus { frame, major_radius: 5.0, minor_radius: 1.0 };
    let target = Pnt3::new(5.0, 0.0, 5.0); // straight up from the tube center at u=0
    let cp = closest_uv(&s, s.domain(), target, 1e-9);
    assert!(cp.certified);
    assert!((cp.distance - 4.0).abs() < 1e-6, "distance mismatch: {}", cp.distance);
    assert!(cp.point.distance(Pnt3::new(5.0, 0.0, 1.0)) < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn closest_point_on_cylinder_seam_wraps_correctly() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cylinder { frame, radius: 1.0 };
    let angle = -0.01_f64;
    // 🐛 Placed via `frame.to_world` (the frame's OWN local axes), not raw world `cos`/`sin` —
    // `Frame3::from_normal`'s `x`/`y` come from `Vec3::any_orthogonal` (for `normal = Z` that's
    // `x = (0,1,0), y = (-1,0,0)`, a 90°-rotated basis, not world `X`/`Y`), so a world-angle-`(-0.01)`
    // point sits at LOCAL angle `-0.01 - π/2`, nowhere near this cylinder's own `u = 0`/`TAU` seam —
    // the test was inadvertently probing the middle of the domain instead of the seam it names.
    let target = frame.to_world(Pnt3::new(2.0 * angle.cos(), 2.0 * angle.sin(), 0.0));
    let cp = closest_uv(&s, ((0.0, std::f64::consts::TAU), (-5.0, 5.0)), target, 1e-9);
    let expected = std::f64::consts::TAU + angle;
    assert!((cp.u - expected).abs() < 1e-6, "seam wrap failed: u={}, expected near {expected}", cp.u);
}

#[semio_framework_async_macros::async_test]
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

#[semio_framework_async_macros::async_test]
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

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_cylinder_matches_brute_force_grid_oracle() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(71);
        for _ in 0..50 {
            let frame =
                Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z)).unwrap();
            let radius = 0.5 + rng.next_f64() * 5.0;
            let s = Surface::Cylinder { frame, radius };
            let target = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let domain = ((0.0, std::f64::consts::TAU), (-15.0, 15.0));
            let cp = closest_uv(&s, domain, target, 1e-9);
            let mut oracle = f64::INFINITY;
            for i in 0..2000 {
                let u = std::f64::consts::TAU * i as f64 / 2000.0;
                for j in 0..200 {
                    let v = -15.0 + 30.0 * j as f64 / 200.0;
                    oracle = oracle.min(s.eval(u, v).distance(target));
                }
            }
            assert!(cp.distance <= oracle + 1e-3, "closed-form found {} worse than oracle {oracle}", cp.distance);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_nurbs_patch_matches_dense_sampling_oracle() {
        let frame = Frame3::WORLD;
        let cyl = Surface::Cylinder { frame, radius: 2.5 };
        // A NURBS surface built by densely sampling a cylinder patch is an independent,
        // easily oracled shape for the patch-subdivision + Newton path.
        let u_knots = KnotVector::clamped_uniform(5, 3);
        let v_knots = KnotVector::clamped_uniform(4, 3);
        let controls: Vec<Vec<Pnt3>> = (0..5)
            .map(|i| {
                let u = std::f64::consts::PI * i as f64 / 4.0;
                (0..4).map(|j| cyl.eval(u, j as f64 * 2.0)).collect()
            })
            .collect();
        let weights = vec![vec![1.0; 4]; 5];
        let nurbs = Surface::Nurbs { u_knots, v_knots, controls, weights };
        let mut rng = semio_framework_geometry::random::Rng::from_seed(83);
        for _ in 0..20 {
            let target = Pnt3::new(rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 6.0);
            let cp = closest_uv(&nurbs, nurbs.domain(), target, 1e-9);
            let (u_dom, v_dom) = nurbs.domain();
            let mut oracle = f64::INFINITY;
            for i in 0..400 {
                let u = u_dom.0 + (u_dom.1 - u_dom.0) * i as f64 / 400.0;
                for j in 0..400 {
                    let v = v_dom.0 + (v_dom.1 - v_dom.0) * j as f64 / 400.0;
                    oracle = oracle.min(nurbs.eval(u, v).distance(target));
                }
            }
            assert!(cp.distance <= oracle + 1e-2, "certified={} worse than oracle={oracle}", cp.distance);
        }
    }
}
