
use super::*;

#[test]
fn analytical_temp_rises_with_gain() {
    let t = analytical_steady_temp_c(1000.0, 20.0, 100.0);
    assert!((t - 30.0).abs() < 1e-9);
}

#[test]
fn bdf3_rate_constant_history() {
    let h = [22.0, 22.0, 22.0, 22.0];
    assert!(bdf3_rate(h, 3600.0).abs() < 1e-9);
}

#[test]
fn floating_zone_uses_steady_analytical() {
    let state = ZoneAirState::new(20.0, 0.008);
    let balance = ZoneAirBalance {
        volume_m3: 100.0,
        conditioned: false,
        sensible_gain_w: 500.0,
        latent_gain_w: 0.0,
        infiltration_sensible_w: 0.0,
        infiltration_latent_w: 0.0,
        ventilation_sensible_w: 0.0,
        ventilation_latent_w: 0.0,
        system_sensible_w: 0.0,
        system_latent_w: 0.0,
        surface_convection_w: 0.0,
        mass_flow_in_kg_s: 0.0,
        supply_humidity_ratio: 0.008,
        outdoor_humidity_ratio: 0.008,
        heating_setpoint_c: None,
        cooling_setpoint_c: None,
        max_heating_w: None,
        max_cooling_w: None,
    };
    let result = advance_zone_air(&state, &balance, 3600.0, HumiditySolutionMethod::AnalyticalSteadyState, P_STD);
    assert!(result.temp_c > 20.0);
}

#[test]
fn conditioned_bdf3_warms_zone() {
    let state = ZoneAirState::new(20.0, 0.008);
    let balance = ZoneAirBalance {
        volume_m3: 200.0,
        conditioned: true,
        sensible_gain_w: 2000.0,
        latent_gain_w: 0.0,
        infiltration_sensible_w: -100.0,
        infiltration_latent_w: 0.0,
        ventilation_sensible_w: 0.0,
        ventilation_latent_w: 0.0,
        system_sensible_w: 0.0,
        system_latent_w: 0.0,
        surface_convection_w: 0.0,
        mass_flow_in_kg_s: 0.05,
        supply_humidity_ratio: 0.008,
        outdoor_humidity_ratio: 0.006,
        heating_setpoint_c: Some(21.0),
        cooling_setpoint_c: Some(26.0),
        max_heating_w: Some(5000.0),
        max_cooling_w: Some(5000.0),
    };
    let result = advance_zone_air(&state, &balance, 3600.0, HumiditySolutionMethod::ThirdOrderBackward, P_STD);
    assert!(result.temp_c > 20.0);
}

#[test]
fn humidity_analytical_increases_with_latent_gain() {
    let w = analytical_steady_humidity_ratio(200.0, 0.1, 0.008, 22.0);
    assert!(w > 0.008);
}
