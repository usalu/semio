use super::*;

fn default_thermostat() -> ThermostatSpec {
    ThermostatSpec { heating_setpoint_c: 21.0, cooling_setpoint_c: 24.0, heating_throttle_range_k: 2.0, cooling_throttle_range_k: 2.0, min_heating_setpoint_c: 10.0, max_cooling_setpoint_c: 30.0 }
}

#[test]
fn heating_fraction_full_when_cold() {
    let out = evaluate_controls(&default_thermostat(), None, 18.0, 0.5);
    assert!((out.heating_fraction - 1.0).abs() < 1e-9);
    assert!((out.cooling_fraction).abs() < 1e-9);
}

#[test]
fn cooling_fraction_when_warm() {
    let out = evaluate_controls(&default_thermostat(), None, 26.0, 0.5);
    assert!((out.cooling_fraction - 1.0).abs() < 1e-9);
}

#[test]
fn predict_heating_load_when_negative_residual() {
    let out = evaluate_controls(&default_thermostat(), None, 19.0, 0.5);
    let load = predict_zone_load(-3000.0, 0.0, &out, 5000.0, 5000.0, 1000.0, 1000.0);
    assert!(load.heating_w > 0.0);
    assert!(load.heating_w <= 5000.0);
}

#[test]
fn equipment_priority_allocates_in_order() {
    let load = ZoneLoad { heating_w: 8000.0, ..Default::default() };
    let caps = [(ZoneEquipmentPriority(1), 3000.0), (ZoneEquipmentPriority(2), 5000.0)];
    let alloc = allocate_load_by_priority(load, &caps);
    assert_eq!(alloc.len(), 2);
    assert!((alloc[0].1.heating_w - 3000.0).abs() < 1e-6);
}
