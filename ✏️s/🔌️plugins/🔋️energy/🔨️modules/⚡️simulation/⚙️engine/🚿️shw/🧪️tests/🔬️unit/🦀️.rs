use super::*;

fn electric_tank() -> MixedWaterHeater {
    MixedWaterHeater { volume_l: 300.0, ua_standby_w_per_k: 5.0, heating_capacity_w: 4500.0, setpoint_c: 55.0, ambient_c: 20.0, recovery_efficiency: 0.98, fuel: WaterHeaterFuel::Electric }
}

#[test]
fn mixed_tank_recovers_after_draw() {
    let tank = electric_tank();
    let state = WaterHeaterState { average_temperature_c: 55.0, top_temperature_c: 55.0, bottom_temperature_c: 55.0, volume_m3: 0.3 };
    let (out, new_state) = tank.simulate(&state, 0.05, 10.0, 3600.0);
    assert!(out.delivered_flow_kg_s > 0.0);
    assert!(new_state.average_temperature_c < state.average_temperature_c);
}

#[test]
fn stratified_tank_top_hotter_than_bottom() {
    let tank = StratifiedWaterHeater { volume_l: 400.0, node_count: 4, ua_standby_w_per_k: 4.0, setpoint_c: 60.0, ambient_c: 20.0, heating_capacity_w: 6000.0, heater_position: 3 };
    let initial = vec![55.0, 50.0, 45.0, 40.0];
    let (_out, temps) = tank.simulate(&initial, 0.0, 10.0, 3600.0);
    assert!(temps[3] > temps[0]);
    assert!(temps[3] > 40.0);
}

#[test]
fn hpwh_cop_reduces_electrical() {
    let hpwh = HeatPumpWaterHeater { tank: electric_tank(), rated_cop: 3.0, min_ambient_c: -5.0 };
    let state = WaterHeaterState { average_temperature_c: 45.0, top_temperature_c: 45.0, bottom_temperature_c: 45.0, volume_m3: 0.3 };
    let (out, _) = hpwh.simulate(&state, 0.0, 10.0, 20.0, 3600.0);
    if out.heating_power_w > 100.0 {
        assert!(out.electrical_power_w < out.heating_power_w);
    }
}

#[test]
fn fixture_demand_positive() {
    let fixture = HotWaterFixture { name: "Shower".into(), peak_flow_l_s: 0.15, target_temperature_c: 40.0, schedule_factor: 0.5 };
    assert!(fixture.demand_w(10.0, 55.0) > 0.0);
}

#[test]
fn drain_recovery_preheats_mains() {
    let dwhr = DrainWaterHeatRecovery { effectiveness: 0.6, ua_w_per_k: 500.0 };
    let preheated = dwhr.preheated_mains_c(0.05, 35.0, 0.05, 10.0);
    assert!(preheated > 10.0);
    assert!(preheated < 35.0);
}

#[test]
fn standby_loss_increases_with_temperature() {
    let standby = StandbyLoss { ua_w_per_k: 8.0, ambient_temperature_c: 20.0, circulation_pump_w: 30.0 };
    assert!(standby.loss_w(55.0) > standby.loss_w(40.0));
}
