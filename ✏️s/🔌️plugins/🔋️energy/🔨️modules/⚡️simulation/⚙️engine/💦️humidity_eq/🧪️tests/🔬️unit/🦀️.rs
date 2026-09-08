
use super::*;
use crate::units::P_STD;

#[test]
fn steam_humidifier_adds_moisture() {
    let hum = Humidifier::SteamElectric { capacity_kg_s: 0.01, efficiency: 0.95 };
    let inlet = HumidifierInlet { dry_bulb_c: 20.0, humidity_ratio: 0.005, mass_flow_kg_s: 0.5, target_humidity_ratio: 0.009, pressure_pa: P_STD };
    let out = humidifier_output_kg_s(&hum, &inlet);
    assert!(out.water_added_kg_s > 0.0);
    assert!(out.humidity_ratio > inlet.humidity_ratio);
    assert!(out.power_w > 0.0);
}

#[test]
fn refrigerant_dehumidifier_removes_moisture() {
    let dehum = Dehumidifier::Refrigerant { cop: 2.5, capacity_kg_s: 0.005 };
    let inlet = DehumidifierInlet { dry_bulb_c: 26.0, humidity_ratio: 0.014, mass_flow_kg_s: 0.6, target_humidity_ratio: 0.009, pressure_pa: P_STD };
    let out = dehumidifier_output_kg_s(&dehum, &inlet);
    assert!(out.moisture_removed_kg_s > 0.0);
    assert!(out.humidity_ratio < inlet.humidity_ratio);
}

#[test]
fn at_target_no_humidification() {
    let hum = Humidifier::SteamElectric { capacity_kg_s: 0.01, efficiency: 1.0 };
    let inlet = HumidifierInlet { dry_bulb_c: 22.0, humidity_ratio: 0.01, mass_flow_kg_s: 0.5, target_humidity_ratio: 0.009, pressure_pa: P_STD };
    let out = humidifier_output_kg_s(&hum, &inlet);
    assert_eq!(out.water_added_kg_s, 0.0);
}
