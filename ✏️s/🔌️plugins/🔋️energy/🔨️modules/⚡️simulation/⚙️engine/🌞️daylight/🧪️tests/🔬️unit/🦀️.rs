
use super::*;

fn sample_zone() -> DaylightZone {
    DaylightZone {
        zone_id: 1,
        floor_area_m2: 25.0,
        window_transmittance: 0.6,
        reference_points: vec![ReferencePoint { x_m: 2.0, y_m: 2.0, z_m: 0.8, fraction: 0.5 }, ReferencePoint { x_m: 4.0, y_m: 2.0, z_m: 0.8, fraction: 0.5 }],
        illuminance_target_lux: 500.0,
        glare_limit: 0.4,
    }
}

#[test]
fn illuminance_increases_with_sun() {
    let e = reference_point_illuminance_lux(10_000.0, 50_000.0, 0.5, 0.6, 0.05, 1.0);
    assert!(e > 500.0);
}

#[test]
fn dimming_reduces_at_high_daylight() {
    let frac = lighting_dimming_fraction(600.0, 500.0, 0.1);
    assert!((frac - 0.1).abs() < 1e-6);
}

#[test]
fn dimming_full_when_dark() {
    let frac = lighting_dimming_fraction(50.0, 500.0, 0.1);
    assert!(frac > 0.8);
}

#[test]
fn glare_high_for_bright_window() {
    let gi = simplified_glare_index(5000.0, 0.2, 300.0);
    assert!(gi > 0.1);
}

#[test]
fn zone_average_illuminance() {
    let zone = sample_zone();
    let lux = vec![400.0, 600.0];
    let avg = zone_daylight_illuminance(&zone, &lux);
    assert!((avg - 500.0).abs() < 1e-6);
}
