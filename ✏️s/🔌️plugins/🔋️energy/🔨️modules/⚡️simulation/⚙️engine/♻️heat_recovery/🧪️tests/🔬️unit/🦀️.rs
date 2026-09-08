
use super::*;

fn erv() -> HeatRecoveryUnit {
    HeatRecoveryUnit { hx_type: HeatExchangerType::CounterFlow, sensible_effectiveness: 0.75, latent_effectiveness: 0.6, frost_control_temp_c: -5.0, defrost_power_w: 200.0 }
}

#[test]
fn winter_recovery_heats_supply() {
    let unit = erv();
    let supply = HxAirstream { temperature_c: 5.0, humidity_ratio: 0.004, mass_flow_kg_s: 0.3, pressure_pa: 101_325.0 };
    let exhaust = HxAirstream { temperature_c: 22.0, humidity_ratio: 0.009, mass_flow_kg_s: 0.3, pressure_pa: 101_325.0 };
    let out = heat_recovery_exchange_w(&unit, &supply, &exhaust);
    assert!(out.supply_out.temperature_c > supply.temperature_c);
    assert!(out.sensible_recovery_w > 0.0);
}

#[test]
fn effectiveness_ntu_counterflow() {
    let eps = effectiveness_from_ntu(3.0, 0.5, HeatExchangerType::CounterFlow);
    assert!(eps > 0.5 && eps < 1.0);
}

#[test]
fn frost_reduces_effectiveness() {
    let unit = erv();
    let supply = HxAirstream { temperature_c: -10.0, humidity_ratio: 0.002, mass_flow_kg_s: 0.2, pressure_pa: 101_325.0 };
    let exhaust = HxAirstream { temperature_c: 20.0, humidity_ratio: 0.008, mass_flow_kg_s: 0.2, pressure_pa: 101_325.0 };
    let out = heat_recovery_exchange_w(&unit, &supply, &exhaust);
    assert!(out.defrost_active);
    assert!(out.defrost_power_w > 0.0);
}
