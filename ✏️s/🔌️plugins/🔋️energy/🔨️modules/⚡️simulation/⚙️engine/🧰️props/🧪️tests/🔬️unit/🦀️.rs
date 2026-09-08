
use super::*;

#[test]
fn saturation_at_zero_c() {
    let p = saturation_pressure_pa(0.0);
    assert!((p - 611.657).abs() < 1.0);
}

#[test]
fn rh_roundtrip() {
    let w = humidity_ratio_from_rh(25.0, 0.5, P_STD);
    let rh = rh_from_humidity_ratio(25.0, w, P_STD);
    assert!((rh - 0.5).abs() < 0.02);
}

#[test]
fn water_density_near_room_temp() {
    assert!((water_density(20.0) - 998.0).abs() < 5.0);
}
