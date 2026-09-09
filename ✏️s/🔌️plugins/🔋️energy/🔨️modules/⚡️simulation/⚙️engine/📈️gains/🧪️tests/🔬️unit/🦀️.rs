use super::*;

#[test]
fn people_gain_scales_with_count() {
    let g1 = compute_people_gain_w(1.0, ActivityLevel::OfficeWork, 1.0, 0.3);
    let g10 = compute_people_gain_w(10.0, ActivityLevel::OfficeWork, 1.0, 0.3);
    assert!((g10.total_w - 10.0 * g1.total_w).abs() < 1e-6);
}

#[test]
fn lighting_return_air_reduces_convective() {
    let g = compute_lighting_gain_w(10.0, 100.0, 1.0, 0.2, 0.5);
    assert!((g.total_w - 1000.0).abs() < 1e-6);
    assert!((g.return_air_w - 500.0).abs() < 1e-6);
    assert!((g.radiant_w - 200.0).abs() < 1e-6);
}

#[test]
fn equipment_latent_reduces_sensible() {
    let g = compute_equipment_gain_w(5.0, 200.0, 1.0, 0.5, 0.1);
    assert!((g.latent_w - 100.0).abs() < 1e-6);
    assert!((g.sensible_w - 900.0).abs() < 1e-6);
}

#[test]
fn datacenter_total_matches_it_load() {
    let g = compute_datacenter_gain_w(50_000.0, 0.8, 0.7, 12.0);
    assert!((g.total_w - 40_000.0).abs() < 1e-6);
}
