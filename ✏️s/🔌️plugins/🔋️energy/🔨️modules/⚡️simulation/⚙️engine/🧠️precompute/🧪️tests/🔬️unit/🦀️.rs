
use super::*;
use crate::model::*;

#[test]
fn precompute_builds_surface_ctf() {
    let model = crate::sim::test_model_single_zone();
    let pre = PrecomputedModel::build(&model, 60, 60);
    assert!(!pre.surfaces.is_empty());
    assert!(pre.zone_geometry.contains_key(&EntityId(1)));
}

#[test]
fn surface_incidence_is_zero_for_unknown_surface() {
    let model = crate::sim::test_model_single_zone();
    let pre = PrecomputedModel::build(&model, 60, 60);
    assert_eq!(pre.surface_incidence(EntityId(999), 45.0, 180.0), 0.0);
}

#[test]
fn surface_incidence_matches_known_surface_normal() {
    let model = crate::sim::test_model_single_zone();
    let pre = PrecomputedModel::build(&model, 60, 60);
    let incidence = pre.surface_incidence(EntityId(30), 45.0, 180.0);
    assert!((-1.0..=1.0).contains(&incidence));
}

#[test]
fn solar_at_returns_altitude_and_azimuth() {
    let model = crate::sim::test_model_single_zone();
    let pre = PrecomputedModel::build(&model, 60, 60);
    let (alt, az) = pre.solar_at(&model, 172, 12.0);
    assert!(alt > -90.0 && alt < 90.0);
    assert!((0.0..360.0).contains(&az));
}

#[test]
fn thermostat_overrides_default_setpoints() {
    let mut model = crate::sim::test_model_single_zone();
    model.thermostats.push(Thermostat { id: EntityId(50), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(1), heating_throttle_range_k: 3.0, cooling_throttle_range_k: 4.0 });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let sp = pre.default_setpoints.get(&EntityId(1)).unwrap();
    assert!((sp.heating_throttle_k - 3.0).abs() < 1e-9);
    assert!((sp.cooling_throttle_k - 4.0).abs() < 1e-9);
}

#[test]
fn fenestration_precompute_derives_from_host_surface() {
    let mut model = crate::sim::test_model_single_zone();
    model.fenestrations.push(Fenestration {
        id: EntityId(40),
        name: "Win".into(),
        surface_id: EntityId(30),
        u_value_w_m2k: 2.0,
        shgc: 0.4,
        vlt: 0.6,
        area_m2: 2.0,
        height_m: 1.0,
        sill_height_m: 0.8,
        frame_conductance_w_k: 0.0,
        divider_conductance_w_k: 0.0,
        overhang_depth_m: 0.0,
        overhang_offset_m: 0.0,
        fin_depth_m: 0.0,
        fin_offset_m: 0.0,
        glazing_construction_id: None,
    });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let fen = pre.fenestrations.get(&EntityId(40)).unwrap();
    assert_eq!(fen.surface_id, EntityId(30));
    assert!((fen.shgc - 0.4).abs() < 1e-9);
}
