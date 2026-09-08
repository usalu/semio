
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
        let point = Curve3::Nurbs { knots: nurbs.knots.clone(), controls: nurbs.controls.clone(), weights: nurbs.weights.clone() }.eval(t);
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
    let frame = crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3::WORLD;
    let surface = Surface::Cylinder { frame, radius: 2.0 };
    let p = surface.eval(0.7, 1.3);
    let (u, v) = exact_uv(&surface, p);
    assert!((u - 0.7).abs() < 1e-9);
    assert!((v - 1.3).abs() < 1e-9);
}
