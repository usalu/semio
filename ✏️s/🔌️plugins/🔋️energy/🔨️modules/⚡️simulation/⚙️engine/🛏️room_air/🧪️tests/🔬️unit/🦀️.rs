
use super::*;

fn sample_input() -> RoomAirInput {
    RoomAirInput { zone_temp_c: 24.0, supply_temp_c: 18.0, outdoor_temp_c: 5.0, floor_area_m2: 50.0, ceiling_height_m: 3.0, supply_flow_m3_s: 0.2, internal_gain_w: 1500.0, surface_temps_c: [22.0, 23.0, 25.0, 24.0, 21.0, 26.0] }
}

#[test]
fn fully_mixed_uniform() {
    let out = RoomAirModel::FullyMixed.apply(&sample_input());
    assert!((out.occupied_temp_c - 24.0).abs() < 1e-9);
}

#[test]
fn displacement_cooler_at_occupancy() {
    let out = RoomAirModel::Displacement1Node { mixing_factor: 0.2 }.apply(&sample_input());
    assert!(out.occupied_temp_c < sample_input().zone_temp_c);
    assert!(out.occupied_temp_c > sample_input().supply_temp_c);
}

#[test]
fn vertical_gradient_stratifies() {
    let out = RoomAirModel::VerticalGradient { gradient_k_per_m: 1.0 }.apply(&sample_input());
    assert!(out.ceiling_temp_c > out.floor_temp_c);
}

#[test]
fn ufad_occupied_between_supply_and_zone() {
    let out = RoomAirModel::Ufad { diffuser_height_m: 0.3, throw_m: 1.5 }.apply(&sample_input());
    assert!(out.occupied_temp_c > sample_input().supply_temp_c);
    assert!(out.occupied_temp_c < sample_input().zone_temp_c);
}
