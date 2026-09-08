
use super::*;

#[semio_framework_async_macros::async_test]
async fn tol_contains_checks_absolute_distance() {
    let t = Tol::new(0.01);
    assert!(t.contains(0.005));
    assert!(t.contains(-0.005));
    assert!(!t.contains(0.02));
}

#[semio_framework_async_macros::async_test]
async fn tol_tighter_and_looser_pick_correctly() {
    let a = Tol::new(0.1);
    let b = Tol::new(0.5);
    assert_eq!(a.tighter(b), a);
    assert_eq!(a.looser(b), b);
}

#[semio_framework_async_macros::async_test]
async fn negative_tolerance_clamps_to_zero() {
    assert_eq!(Tol::new(-1.0), Tol::new(0.0));
}

#[semio_framework_async_macros::async_test]
async fn check_containment_flags_violation() {
    // A vertex whose own tolerance ball (0.1) is larger than its incident edge's tube (0.01)
    // violates the containment hierarchy: the finer (vertex) must fit inside the coarser (edge).
    let vertex_tol = Tol::new(0.1);
    let edge_tol = Tol::new(0.01);
    let violation = check_containment("vertex-1", vertex_tol, "edge-1", edge_tol);
    assert!(violation.is_some());
    let ok = check_containment("vertex-1", vertex_tol, "edge-1", edge_tol.looser(vertex_tol));
    assert!(ok.is_none());
}

#[semio_framework_async_macros::async_test]
async fn interval_add_widens_conservatively() {
    let a = Iv::new(1.0, 2.0);
    let b = Iv::new(-1.0, 3.0);
    let sum = a.add(b);
    assert_eq!(sum, Iv::new(0.0, 5.0));
}

#[semio_framework_async_macros::async_test]
async fn interval_sign_is_none_when_straddling_zero() {
    let iv = Iv::new(-0.001, 0.001);
    assert_eq!(iv.sign(), None);
}

#[semio_framework_async_macros::async_test]
async fn interval_sign_certain_when_strictly_positive_or_negative() {
    assert_eq!(Iv::new(0.5, 1.0).sign(), Some(std::cmp::Ordering::Greater));
    assert_eq!(Iv::new(-1.0, -0.5).sign(), Some(std::cmp::Ordering::Less));
}

#[semio_framework_async_macros::async_test]
async fn interval_mul_contains_true_product_for_mixed_signs() {
    let a = Iv::new(-2.0, 3.0);
    let b = Iv::new(-1.0, 4.0);
    let product = a.mul(b);
    assert!(product.lo <= -8.0 && product.hi >= 12.0);
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn interval_arithmetic_always_contains_scalar_result() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(3);
        for _ in 0..500 {
            let a = rng.next_f64() * 20.0 - 10.0;
            let b = rng.next_f64() * 20.0 - 10.0;
            let ia = Iv::exact(a);
            let ib = Iv::exact(b);
            let sum = ia.add(ib);
            assert!(sum.lo <= a + b && sum.hi >= a + b);
            let prod = ia.mul(ib);
            assert!(prod.lo <= a * b + 1e-12 && prod.hi >= a * b - 1e-12);
        }
    }
}
