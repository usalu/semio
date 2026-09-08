
use super::*;

#[test]
fn vertical_south_wall_noon_incidence() {
    let cos = beam_incidence_cosine([0.0, -1.0, 0.0], 60.0, 180.0);
    assert!(cos > 0.4);
}

#[test]
fn shading_reduces_with_overhang() {
    let unshaded = shading_factor(1.0, 0.0, 1.5, 45.0);
    let shaded = shading_factor(1.0, 0.8, 1.5, 45.0);
    assert!(shaded < unshaded);
}

#[test]
fn absorption_positive_at_noon() {
    let abs = surface_solar_absorption(800.0, 100.0, 0.8, 1.0, 0.6, 90.0);
    assert!(abs.total_w_m2 > 100.0);
}

#[test]
fn split_flux_allocates_floor_share() {
    let areas = vec![20.0, 10.0, 10.0];
    let dist = distribute_interior_solar(1000.0, InteriorSolarDistribution::SplitFlux, 20.0, &areas);
    assert!((dist[0] - 400.0).abs() < 1e-6);
}
