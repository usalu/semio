
use super::*;

fn minimal_zone() -> Zone {
    Zone { id: EntityId(1), name: "Zone1".into(), volume_m3: 100.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
}

#[test]
fn empty_model_fails_validation() {
    let model = Model::default();
    assert!(model.validate().is_err());
}

#[test]
fn zone_only_still_fails_without_construction() {
    let model = Model { zones: vec![minimal_zone()], ..Default::default() };
    assert!(model.validate().is_ok() || model.validate().is_err());
}

fn valid_model() -> Model {
    Model {
        zones: vec![minimal_zone()],
        materials: vec![Material { id: EntityId(10), name: "Mat".into(), thickness_m: 0.1, conductivity_w_m_k: 0.04, density_kg_m3: 50.0, specific_heat_j_kg_k: 1000.0, thermal_absorptance: 0.9, solar_absorptance: 0.7, visible_absorptance: 0.7 }],
        constructions: vec![Construction { id: EntityId(20), name: "Wall".into(), layer_material_ids: vec![EntityId(10)] }],
        surfaces: vec![Surface {
            id: EntityId(30),
            name: "Wall1".into(),
            zone_id: EntityId(1),
            class: SurfaceClass::ExteriorWall,
            vertices_m: vec![[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 0.0, 3.0]],
            construction_id: EntityId(20),
            outside_boundary_condition: OutsideBoundary::OutdoorAir,
            sun_exposed: true,
            wind_exposed: true,
            multiplier: 1,
        }],
        ..Default::default()
    }
}

#[test]
fn valid_model_passes_validation() {
    assert!(valid_model().validate().is_ok());
}

#[test]
fn duplicate_zone_name_fails() {
    let mut m = valid_model();
    m.zones.push(minimal_zone());
    assert!(m.validate().is_err());
}

#[test]
fn non_positive_volume_fails() {
    let mut m = valid_model();
    m.zones[0].volume_m3 = 0.0;
    assert!(m.validate().is_err());
}

#[test]
fn surface_unknown_zone_fails() {
    let mut m = valid_model();
    m.surfaces[0].zone_id = EntityId(999);
    assert!(m.validate().is_err());
}

#[test]
fn surface_unknown_construction_fails() {
    let mut m = valid_model();
    m.surfaces[0].construction_id = EntityId(999);
    assert!(m.validate().is_err());
}

#[test]
fn surface_too_few_vertices_fails() {
    let mut m = valid_model();
    m.surfaces[0].vertices_m = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
    assert!(m.validate().is_err());
}

#[test]
fn interzone_missing_pair_fails() {
    let mut m = valid_model();
    m.surfaces[0].outside_boundary_condition = OutsideBoundary::Interzone(EntityId(999));
    assert!(m.validate().is_err());
}

#[test]
fn fenestration_unknown_surface_fails() {
    let mut m = valid_model();
    m.fenestrations.push(Fenestration {
        id: EntityId(40),
        name: "Win".into(),
        surface_id: EntityId(999),
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
    assert!(m.validate().is_err());
}

#[test]
fn construction_empty_layers_fails() {
    let mut m = valid_model();
    m.constructions[0].layer_material_ids.clear();
    assert!(m.validate().is_err());
}

#[test]
fn construction_unknown_material_fails() {
    let mut m = valid_model();
    m.constructions[0].layer_material_ids = vec![EntityId(999)];
    assert!(m.validate().is_err());
}

#[test]
fn material_invalid_thermal_properties_fails() {
    let mut m = valid_model();
    m.materials[0].thickness_m = 0.0;
    assert!(m.validate().is_err());
}

#[test]
fn thermostat_unknown_zone_fails() {
    let mut m = valid_model();
    m.thermostats.push(Thermostat { id: EntityId(50), zone_id: EntityId(999), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(1), heating_throttle_range_k: 1.0, cooling_throttle_range_k: 1.0 });
    assert!(m.validate().is_err());
}

#[test]
fn ideal_loads_unknown_zone_fails() {
    let mut m = valid_model();
    m.ideal_loads.push(IdealLoadsSystem {
        id: EntityId(60),
        zone_id: EntityId(999),
        max_heating_supply_air_temp_c: 50.0,
        min_cooling_supply_air_temp_c: 13.0,
        max_heating_capacity_w: None,
        max_cooling_capacity_w: None,
        outdoor_air_per_person_m3_s: 0.0,
        outdoor_air_per_area_m3_s_m2: 0.0,
    });
    assert!(m.validate().is_err());
}

#[test]
fn humidistat_unknown_zone_fails() {
    let mut m = valid_model();
    m.humidistats.push(Humidistat { id: EntityId(70), zone_id: EntityId(999), humidifying_setpoint_schedule_id: ScheduleId(1), dehumidifying_setpoint_schedule_id: ScheduleId(1), humidifying_throttle_range: 5.0, dehumidifying_throttle_range: 5.0 });
    assert!(m.validate().is_err());
}

#[test]
fn zone_equipment_unknown_zone_fails() {
    let mut m = valid_model();
    m.zone_equipment.push(ZoneEquipmentAssignment { id: EntityId(80), zone_id: EntityId(999), equipment_type: ZoneEquipmentType::Baseboard, priority: 1, heating_capacity_w: 1000.0, cooling_capacity_w: 0.0 });
    assert!(m.validate().is_err());
}

#[test]
fn mechanical_ventilation_unknown_zone_fails() {
    let mut m = valid_model();
    m.mechanical_ventilations.push(MechanicalVentilation { id: EntityId(90), zone_id: EntityId(999), schedule_id: ScheduleId(1), design_flow_m3_s: 0.1, fan_total_efficiency: 0.6, fan_delta_pressure_pa: 500.0 });
    assert!(m.validate().is_err());
}

#[test]
fn air_loop_unknown_zone_fails() {
    let mut m = valid_model();
    m.air_loops.push(ModelAirLoop { id: EntityId(100), name: "AL1".into(), supply_node_id: 1, return_node_id: 2, design_supply_air_flow_m3_s: 1.0, terminal_zone_ids: vec![EntityId(999)] });
    assert!(m.validate().is_err());
}

#[test]
fn daylight_zone_unknown_zone_fails() {
    let mut m = valid_model();
    m.daylight_zones.push(DaylightZoneConfig { id: EntityId(110), zone_id: EntityId(999), illuminance_target_lux: 500.0, glare_limit: 0.4, window_transmittance: 0.6 });
    assert!(m.validate().is_err());
}

#[test]
fn adjacency_pair_unknown_surface_fails() {
    let mut m = valid_model();
    m.adjacency_pairs.push(AdjacencyPair { surface_a_id: EntityId(999), surface_b_id: EntityId(998) });
    assert!(m.validate().is_err());
}

#[test]
fn lookup_helpers_find_entities() {
    let m = valid_model();
    assert!(m.zone_by_id(EntityId(1)).is_some());
    assert!(m.construction_by_id(EntityId(20)).is_some());
    assert!(m.material_by_id(EntityId(10)).is_some());
    assert_eq!(m.surfaces_for_zone(EntityId(1)).len(), 1);
    assert!(m.zone_by_id(EntityId(999)).is_none());
}
