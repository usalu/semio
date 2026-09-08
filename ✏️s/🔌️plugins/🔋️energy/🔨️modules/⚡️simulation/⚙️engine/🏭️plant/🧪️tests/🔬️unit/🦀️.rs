
use super::*;
use crate::curves::PerformanceCurve;

fn flat_curve() -> PerformanceCurve {
    PerformanceCurve::Constant(1.0)
}

#[test]
fn boiler_meets_partial_load() {
    let boiler = Boiler { rated_capacity_w: 100_000.0, combustion_efficiency: 0.9, part_load_curve: flat_curve(), standby_loss_w: 200.0, supply_temperature_c: 80.0 };
    let inlet = PlantStream::new(60.0, 2.0);
    let out = boiler.simulate(inlet, 50_000.0, true);
    assert!(out.thermal_power_w > 49_000.0);
    assert!(out.gas_power_w > out.thermal_power_w);
}

#[test]
fn chiller_eir_cooling() {
    let chiller = ChillerEir { rated_capacity_w: 200_000.0, reference_cop: 5.0, eir_curve: PerformanceCurve::Constant(0.2), eir_f_t_curve: PerformanceCurve::Constant(1.0), leaving_water_c: 7.0, entering_condenser_c: 29.0 };
    let inlet = PlantStream::new(12.0, 10.0);
    let out = chiller.simulate(inlet, 100_000.0, true);
    assert!(out.thermal_power_w < 0.0);
    assert!(out.electrical_power_w > 10_000.0);
    assert!(out.heat_rejection_w > 100_000.0);
}

#[test]
fn cooling_tower_rejects_heat() {
    let tower = CoolingTower { design_range_k: 5.0, design_approach_k: 3.0, design_flow_kg_s: 20.0, fan_power_at_design_w: 15_000.0, fan_curve: flat_curve() };
    let inlet = PlantStream::new(35.0, 20.0);
    let out = tower.simulate(inlet, 500_000.0, 22.0);
    assert!(out.outlet.temperature_c < inlet.temperature_c);
    assert!(out.electrical_power_w > 0.0);
}

#[test]
fn heat_exchanger_transfers_positive() {
    let hx = HeatExchanger { ua_w_per_k: 10_000.0, effectiveness: 0.8 };
    let hot = PlantStream::new(70.0, 5.0);
    let cold = PlantStream::new(10.0, 5.0);
    let (hot_out, cold_out) = hx.simulate(hot, cold);
    assert!(hot_out.thermal_power_w < 0.0);
    assert!(cold_out.thermal_power_w > 0.0);
    assert!(hot_out.outlet.temperature_c < hot.temperature_c);
    assert!(cold_out.outlet.temperature_c > cold.temperature_c);
}

#[test]
fn thermal_storage_changes_temperature() {
    let storage =
        ThermalStorage { volume_m3: 5.0, height_m: 2.0, loss_coefficient_w_per_k: 10.0, charge_efficiency: 0.95, discharge_efficiency: 0.95, state: ThermalStorageState { node_temperatures_c: vec![50.0, 45.0, 40.0], ambient_temperature_c: 20.0 } };
    let inlet = PlantStream::new(55.0, 1.0);
    let (out, state) = storage.simulate(inlet, 20_000.0, 3600.0);
    assert!(out.thermal_power_w > 0.0);
    assert!(state.node_temperatures_c[0] > 50.0);
}

#[test]
fn pump_power_scales_with_flow() {
    let pump = Pump { design_head_pa: 200_000.0, design_flow_kg_s: 10.0, motor_efficiency: 0.85, part_load_curve: flat_curve() };
    let inlet = PlantStream::new(20.0, 0.0);
    let low = pump.simulate(inlet, 2.0);
    let high = pump.simulate(inlet, 8.0);
    assert!(high.electrical_power_w > low.electrical_power_w);
}

#[test]
fn gshp_penalty_increases_with_ground_load() {
    let gshp = Gshp {
        heat_pump: HeatPump { rated_heating_w: 50_000.0, rated_cooling_w: 50_000.0, rated_cop_heating: 4.0, rated_cop_cooling: 4.5, heating_curve: flat_curve(), cooling_curve: flat_curve() },
        borehole_depth_m: 100.0,
        borehole_count: 4,
        grout_conductivity_w_m_k: 1.5,
        ground_temperature_c: 12.0,
    };
    let inlet = PlantStream::new(35.0, 2.0);
    let low = gshp.simulate(inlet, 30_000.0, HeatPumpMode::Heating, 0.0);
    let high = gshp.simulate(inlet, 30_000.0, HeatPumpMode::Heating, 5e9);
    assert!(high.electrical_power_w >= low.electrical_power_w);
}
