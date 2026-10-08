use super::*;

#[test]
fn planar_helpers_follow_the_right_handed_convention() {
    let (x, y) = (Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0));
    assert_eq!(cross(x, y), 1.0);
    assert_eq!(perp(x), y);
    assert_eq!(unit(Vec2::new(0.0, 3.0), 1e-9), Some(y));
    assert_eq!(unit(Vec2::ZERO, 1e-9), None);
    assert!((angle_between(x, y) - std::f64::consts::FRAC_PI_2).abs() < 1e-15);
    assert!((angle_between(y, x) + std::f64::consts::FRAC_PI_2).abs() < 1e-15);
    assert_eq!(lerp(Point::new(0.0, 0.0), Point::new(2.0, 4.0), 0.5), Point::new(1.0, 2.0));
}

#[test]
fn spatial_helpers_obey_cross_product_identities() {
    let (a, b) = ([1.0, 2.0, 3.0], [-2.0, 0.5, 4.0]);
    let c = cross3(a, b);
    assert!(dot3(c, a).abs() < 1e-12 && dot3(c, b).abs() < 1e-12);
    assert!((length3(normalize3(a)) - 1.0).abs() < 1e-15);
    assert_eq!(normalize3([0.0; 3]), [0.0; 3]);
    assert_eq!(sub3(add3(a, b), b), a);
    assert_eq!(scale3(a, 2.0), [2.0, 4.0, 6.0]);
}
