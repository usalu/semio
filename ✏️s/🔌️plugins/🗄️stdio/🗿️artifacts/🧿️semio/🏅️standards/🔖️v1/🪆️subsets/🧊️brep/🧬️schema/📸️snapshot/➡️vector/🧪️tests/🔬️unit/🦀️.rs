use super::*;

#[semio_framework_async_macros::async_test]
async fn cross_product_is_orthogonal_to_both_operands() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(-2.0, 0.5, 4.0);
    let c = a.cross(b);
    assert!(c.dot(a).abs() < 1e-12);
    assert!(c.dot(b).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn normalized_returns_none_for_zero_vector() {
    assert_eq!(Vec3::ZERO.normalized(), None);
    assert_eq!(Vec2::ZERO.normalized(), None);
}

#[semio_framework_async_macros::async_test]
async fn normalized_has_unit_length() {
    let v = Vec3::new(3.0, 4.0, 0.0).normalized().unwrap();
    assert!((v.norm() - 1.0).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn any_orthogonal_is_perpendicular_for_all_axis_aligned_inputs() {
    for v in [Vec3::X, Vec3::Y, Vec3::Z, -Vec3::X, -Vec3::Y, -Vec3::Z, Vec3::new(1.0, 1.0, 1.0)] {
        let o = v.any_orthogonal();
        assert!(v.dot(o).abs() < 1e-9, "not orthogonal for {v:?}: dot={}", v.dot(o));
        assert!((o.norm() - 1.0).abs() < 1e-9);
    }
}

#[semio_framework_async_macros::async_test]
async fn vec2_cross_sign_matches_turn_direction() {
    let a = Vec2::new(1.0, 0.0);
    let b = Vec2::new(0.0, 1.0);
    assert!(a.cross(b) > 0.0);
    assert!(b.cross(a) < 0.0);
}

#[semio_framework_async_macros::async_test]
async fn point_minus_point_is_vector_and_point_plus_vector_is_point() {
    let p = Pnt3::new(1.0, 2.0, 3.0);
    let q = Pnt3::new(4.0, 6.0, 8.0);
    let v: Vec3 = q - p;
    assert_eq!(v, Vec3::new(3.0, 4.0, 5.0));
    assert_eq!(p + v, q);
}

#[semio_framework_async_macros::async_test]
async fn normalize_angle_wraps_into_0_tau() {
    assert!((normalize_angle(-0.1) - (std::f64::consts::TAU - 0.1)).abs() < 1e-12);
    assert!((normalize_angle(std::f64::consts::TAU + 0.5) - 0.5).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn angle_diff_wraps_into_minus_pi_pi() {
    let d = angle_diff(0.1, std::f64::consts::TAU - 0.1);
    assert!((d - (-0.2)).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn lerp_at_endpoints_returns_endpoints() {
    let a = Pnt3::new(0.0, 0.0, 0.0);
    let b = Pnt3::new(10.0, 20.0, 30.0);
    assert_eq!(a.lerp(b, 0.0), a);
    assert_eq!(a.lerp(b, 1.0), b);
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn cross_product_antisymmetric_on_random_vectors() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(1);
        for _ in 0..200 {
            let a = Vec3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let b = Vec3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let ab = a.cross(b);
            let ba = b.cross(a);
            assert!((ab + ba).norm() < 1e-9);
        }
    }
}
