
use super::*;

#[test]
fn lerp_endpoints() {
    assert!((lerp(0.5, 0.0, 1.0, 0.0, 10.0) - 5.0).abs() < 1e-9);
}

#[test]
fn newton_finds_sqrt() {
    let r = newton_raphson(2.0, |x| x * x - 2.0, |x| 2.0 * x, 20, 1e-10).unwrap();
    assert!((r - std::f64::consts::SQRT_2).abs() < 1e-8);
}

#[test]
fn simpson_integrates_x_squared() {
    let integral = simpson_integrate(|x| x * x, 0.0, 1.0, 100);
    assert!((integral - 1.0 / 3.0).abs() < 1e-6);
}
