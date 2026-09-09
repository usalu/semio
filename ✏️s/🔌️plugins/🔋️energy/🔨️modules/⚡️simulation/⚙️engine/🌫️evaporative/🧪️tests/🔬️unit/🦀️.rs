use super::*;
use crate::units::P_STD;

#[test]
fn direct_cooling_lowers_dry_bulb() {
    let cooler = EvaporativeCooler::Direct { effectiveness: 0.8, pad_area_m2: 10.0 };
    let inlet = EvaporativeInlet { dry_bulb_c: 35.0, humidity_ratio: humidity_ratio_from_rh(35.0, 0.3, P_STD), mass_flow_kg_s: 1.0, pressure_pa: P_STD };
    let out = evaporative_cool(&cooler, &inlet, true);
    assert!(out.dry_bulb_c < inlet.dry_bulb_c);
    assert!(out.humidity_ratio > inlet.humidity_ratio);
    assert!(out.water_consumption_kg_s > 0.0);
}

#[test]
fn indirect_preserves_humidity_ratio() {
    let cooler = EvaporativeCooler::Indirect { sensible_effectiveness: 0.65, primary_flow_m3_s: 1.0, secondary_flow_m3_s: 1.0 };
    let inlet = EvaporativeInlet { dry_bulb_c: 32.0, humidity_ratio: 0.01, mass_flow_kg_s: 1.2, pressure_pa: P_STD };
    let out = evaporative_cool(&cooler, &inlet, true);
    assert!((out.humidity_ratio - inlet.humidity_ratio).abs() < 1e-9);
    assert!(out.sensible_cooling_w > 0.0);
}

#[test]
fn disabled_no_effect() {
    let cooler = EvaporativeCooler::Direct { effectiveness: 0.9, pad_area_m2: 5.0 };
    let inlet = EvaporativeInlet { dry_bulb_c: 30.0, humidity_ratio: 0.012, mass_flow_kg_s: 0.8, pressure_pa: P_STD };
    let out = evaporative_cool(&cooler, &inlet, false);
    assert!((out.dry_bulb_c - inlet.dry_bulb_c).abs() < 1e-9);
}
