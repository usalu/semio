
use super::*;
use crate::curves::PerformanceCurve;

#[test]
fn display_case_cooling_positive() {
    let case = DisplayCase { length_m: 3.0, design_cooling_w: 2000.0, fan_power_w: 150.0, lighting_power_w: 200.0, anti_sweat_heater_w: 100.0, defrost_power_w: 500.0, defrost_fraction: 0.05, evaporating_temperature_c: -8.0 };
    let out = case.simulate(-2.0, 22.0, 1.0);
    assert!(out.cooling_power_w > 0.0);
    assert!(out.compressor_power_w > out.cooling_power_w * 0.3);
}

#[test]
fn walk_in_freezer_higher_compressor_ratio() {
    let cooler = WalkIn { floor_area_m2: 20.0, wall_area_m2: 60.0, ua_w_per_k: 30.0, internal_gain_w: 500.0, design_box_temperature_c: 2.0, evaporating_temperature_c: -5.0, door_opening_fraction: 0.1 };
    let freezer = WalkIn { design_box_temperature_c: -20.0, evaporating_temperature_c: -28.0, ..cooler };
    let cool_out = cooler.simulate(25.0, 1.0);
    let freeze_out = freezer.simulate(25.0, 1.0);
    assert!(freeze_out.compressor_power_w / freeze_out.cooling_power_w > cool_out.compressor_power_w / cool_out.cooling_power_w);
}

#[test]
fn compressor_rack_part_load() {
    let rack = CompressorRack { rated_capacity_w: 100_000.0, compressor_count: 2, eir_curve: PerformanceCurve::Constant(0.35), eir_f_t_curve: PerformanceCurve::Constant(1.0), min_evaporating_c: -15.0, max_condensing_c: 45.0 };
    let out = rack.simulate(80_000.0, -8.0, 35.0);
    assert!(out.cooling_power_w > 70_000.0);
    assert!(out.condenser_heat_w > out.cooling_power_w);
}

#[test]
fn condenser_approach_temperature() {
    let cond = RefrigerationCondenser { ua_w_per_k: 5000.0, fan_power_w: 3000.0, design_approach_k: 8.0, evaporative: false };
    let (t_cond, fan_w) = cond.simulate(150_000.0, 30.0, 22.0);
    assert!(t_cond > 30.0);
    assert!(fan_w > 0.0);
}

#[test]
fn secondary_loop_return_rises_with_load() {
    let loop_sys = SecondaryLoop { pump_power_w: 800.0, pipe_ua_w_per_k: 20.0, supply_temperature_c: -5.0, return_temperature_c: -3.0, mass_flow_kg_s: 5.0, fluid_cp: 3500.0 };
    let (_, return_t, _) = loop_sys.simulate(30_000.0, 20.0);
    assert!(return_t > loop_sys.supply_temperature_c);
}

#[test]
fn saturation_pressure_increases_with_temperature() {
    let low = evaporating_pressure_pa(-10.0);
    let high = evaporating_pressure_pa(5.0);
    assert!(high > low);
}
