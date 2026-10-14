use super::*;
use crate::model::*;
use crate::schedule::ScheduleId;

#[test]
fn precompute_builds_surface_chains() {
    let model = crate::model::test_model_single_zone();
    let pre = PrecomputedModel::build(&model, 60, 60);
    assert!(pre.surfaces.get(&EntityId(30)).is_some_and(|surface| surface.chain.nodes() >= 2));
    assert!(pre.zone_geometry.contains_key(&EntityId(1)));
}

#[test]
fn thermostat_overrides_default_setpoints() {
    let mut model = crate::model::test_model_single_zone();
    model.thermostats.push(Thermostat { id: EntityId(50), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(1), heating_throttle_range_k: 3.0, cooling_throttle_range_k: 4.0 });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let sp = pre.default_setpoints.get(&EntityId(1)).unwrap();
    assert!((sp.heating_throttle_k - 3.0).abs() < 1e-9);
    assert!((sp.cooling_throttle_k - 4.0).abs() < 1e-9);
}
