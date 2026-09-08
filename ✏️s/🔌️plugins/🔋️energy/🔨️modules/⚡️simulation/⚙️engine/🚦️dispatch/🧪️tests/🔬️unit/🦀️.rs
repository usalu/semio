
use super::*;

#[test]
fn sequential_fills_first_unit() {
    let d =
        Dispatcher::new(DispatchScheme::Sequential, vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 5000.0 }, EquipmentPriority { equipment_id: 2, priority: 2, min_runtime_hours: 0.0, capacity_w: 5000.0 }]);
    let results = d.dispatch(&DispatchRequest { total_load_w: 7000.0, available_capacity_w: 10000.0, outdoor_temp_c: 20.0 });
    assert!((results[0].load_w - 5000.0).abs() < 1e-6);
    assert!((results[1].load_w - 2000.0).abs() < 1e-6);
}

#[test]
fn uniform_splits_proportionally_to_capacity() {
    let d =
        Dispatcher::new(DispatchScheme::Uniform, vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 3000.0 }, EquipmentPriority { equipment_id: 2, priority: 2, min_runtime_hours: 0.0, capacity_w: 1000.0 }]);
    let results = d.dispatch(&DispatchRequest { total_load_w: 2000.0, available_capacity_w: 4000.0, outdoor_temp_c: 20.0 });
    assert_eq!(results.len(), 2);
    let plr = 2000.0 / 4000.0;
    assert!((results[0].load_w - 3000.0 * plr).abs() < 1e-6);
    assert!((results[1].load_w - 1000.0 * plr).abs() < 1e-6);
    assert!((results[0].part_load_ratio - plr).abs() < 1e-9);
}

#[test]
fn uniform_with_no_capacity_returns_empty() {
    let d = Dispatcher::new(DispatchScheme::Uniform, vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 0.0 }]);
    let results = d.dispatch(&DispatchRequest { total_load_w: 1000.0, available_capacity_w: 1000.0, outdoor_temp_c: 20.0 });
    assert!(results.is_empty());
}

#[test]
fn optimal_delegates_to_uniform() {
    let equip = vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 2000.0 }];
    let request = DispatchRequest { total_load_w: 1000.0, available_capacity_w: 2000.0, outdoor_temp_c: 20.0 };
    let optimal = Dispatcher::new(DispatchScheme::Optimal, equip.clone()).dispatch(&request);
    let uniform = Dispatcher::new(DispatchScheme::Uniform, equip).dispatch(&request);
    assert_eq!(optimal, uniform);
}

#[test]
fn unhandled_scheme_falls_back_to_sequential() {
    let equip = vec![EquipmentPriority { equipment_id: 1, priority: 1, min_runtime_hours: 0.0, capacity_w: 500.0 }];
    let request = DispatchRequest { total_load_w: 300.0, available_capacity_w: 500.0, outdoor_temp_c: 20.0 };
    let results = Dispatcher::new(DispatchScheme::ThermalStorage, equip).dispatch(&request);
    assert!((results[0].load_w - 300.0).abs() < 1e-9);
}
