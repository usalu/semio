
use super::*;

#[test]
fn co2_rises_with_occupancy_at_low_ventilation() {
    let balance = Co2Balance { zone_volume_m3: 200.0, occupancy: 10.0, co2_generation_per_person_mg_s: 7.0, outdoor_co2_ppm: 400.0, ventilation_flow_m3_s: 0.01, infiltration_flow_m3_s: 0.005 };
    let ppm = steady_state_co2_ppm(&balance);
    assert!(ppm > 400.0);
}

#[test]
fn contaminant_transient_approaches_steady_state() {
    let balance = ContaminantBalance { zone_volume_m3: 100.0, generation_rate_mg_s: 5.0, outdoor_concentration_ppm: 0.0, ventilation_flow_m3_s: 0.05, infiltration_flow_m3_s: 0.0, removal_rate_mg_s: 0.0, molecular_weight_g_mol: 44.01 };
    let ss = steady_state_concentration_ppm(&balance);
    let mut state = ContaminantState::new(0.0);
    for _ in 0..500 {
        let ppm = advance_contaminant(&state, &balance, 60.0);
        state.push(ppm);
    }
    assert!((state.concentration_ppm - ss).abs() / ss < 0.05);
}

#[test]
fn dcv_increases_flow_at_high_co2() {
    let ctrl = DcvControl { target_ppm: 1000.0, min_flow_per_person_m3_s: 0.00236, max_flow_per_person_m3_s: 0.01, outdoor_co2_ppm: 400.0 };
    let low = dcv_flow_per_person_m3_s(&ctrl, 5.0, 600.0);
    let high = dcv_flow_per_person_m3_s(&ctrl, 5.0, 1500.0);
    assert!(high > low);
}
