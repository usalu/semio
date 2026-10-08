//! 🎯️ Middle-row law fixtures — every ordered-collection kind of the energy model that deletes, removes or moves a row is held to the
//! inverse laws on a collection of THREE rows, acting on the MIDDLE one. A last-row fixture cannot tell a position-exact inverse from
//! an append: the undo of a middle delete must put the row back at its original index (create/insert kinds carry an optional
//! `index`; absent appends), and the inverse diffs must sum to exactly the negative of the forward diff.

use crate::model::Model;
use crate::mutations as v;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

async fn holds(mut model: Model, setup: impl FnOnce(&mut Model), mutation: EnergyModelMutation) {
    setup(&mut model);
    let base: EnergyModelSnapshot = crate::mutations::fixtures::snapshot(model);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}

fn seed() -> Model {
    let mut model = Model::default();
    model.zones = vec![zone(1)];
    model.materials = vec![material(1)];
    model.constructions = vec![construction(1)];
    model.surfaces = vec![surface(1)];
    model.air_loops = vec![air_loop(1)];
    model.schedules.constants = vec![constant(1)];
    model
}

fn three<T>(row: fn(u32) -> T) -> Vec<T> {
    vec![row(1), row(2), row(3)]
}

fn id(value: u32) -> crate::model::EntityId {
    crate::model::EntityId(value)
}

fn schedule(value: u32) -> crate::model::ScheduleId {
    crate::model::ScheduleId(value)
}

fn zone(id: u32) -> crate::model::Zone {
    const LABEL: &str = "ZONE";
    crate::model::Zone {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        volume_m3: 1.0,
        multiplier: 1,
        conditioned: true,
        part_of_total_floor_area: true,
    }
}

fn space(id: u32) -> crate::model::Space {
    const LABEL: &str = "SPACE";
    crate::model::Space {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        zone_id: crate::model::EntityId(1),
        floor_area_m2: 1.0,
    }
}

fn surface(id: u32) -> crate::model::Surface {
    const LABEL: &str = "SURFACE";
    crate::model::Surface {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        zone_id: crate::model::EntityId(1),
        class: crate::model::SurfaceClass::ExteriorWall,
        vertices_m: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
        construction_id: crate::model::EntityId(1),
        outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir,
        sun_exposed: true,
        wind_exposed: true,
        multiplier: 1,
    }
}

fn fenestration(id: u32) -> crate::model::Fenestration {
    const LABEL: &str = "FENESTRATION";
    crate::model::Fenestration {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        surface_id: crate::model::EntityId(1),
        u_value_w_m2k: 1.0,
        shgc: 1.0,
        vlt: 1.0,
        area_m2: 1.0,
        height_m: 1.0,
        sill_height_m: 1.0,
        frame_conductance_w_k: 1.0,
        divider_conductance_w_k: 1.0,
        overhang_depth_m: 1.0,
        overhang_offset_m: 1.0,
        fin_depth_m: 1.0,
        fin_offset_m: 1.0,
        glazing_construction_id: None,
        vertices_m: Vec::new(),
    }
}

fn material(id: u32) -> crate::model::Material {
    const LABEL: &str = "MATERIAL";
    crate::model::Material {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        roughness: crate::model::SurfaceRoughness::VeryRough,
        thickness_m: 1.0,
        conductivity_w_m_k: 1.0,
        density_kg_m3: 1.0,
        specific_heat_j_kg_k: 1.0,
        thermal_absorptance: 0.5,
        solar_absorptance: 0.5,
        visible_absorptance: 0.5,
    }
}

fn construction(id: u32) -> crate::model::Construction {
    const LABEL: &str = "CONSTRUCTION";
    crate::model::Construction {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        layer_material_ids: Vec::new(),
    }
}

fn people(id: u32) -> crate::model::PeopleGain {
    const LABEL: &str = "PEOPLE";
    crate::model::PeopleGain {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        schedule_id: crate::model::ScheduleId(1),
        activity_schedule_id: crate::model::ScheduleId(1),
        people_per_area: 1.0,
        sensible_fraction: 0.5,
        latent_fraction: 0.5,
        radiant_fraction: 0.5,
    }
}

fn lighting(id: u32) -> crate::model::LightingGain {
    const LABEL: &str = "LIGHTING";
    crate::model::LightingGain {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        schedule_id: crate::model::ScheduleId(1),
        watts_per_area: 1.0,
        radiant_fraction: 0.5,
        visible_fraction: 0.5,
        return_air_fraction: 0.5,
    }
}

fn equipment(id: u32) -> crate::model::EquipmentGain {
    const LABEL: &str = "EQUIPMENT";
    crate::model::EquipmentGain {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        schedule_id: crate::model::ScheduleId(1),
        watts_per_area: 1.0,
        radiant_fraction: 0.5,
        latent_fraction: 0.5,
    }
}

fn thermostat(id: u32) -> crate::model::Thermostat {
    const LABEL: &str = "THERMOSTAT";
    crate::model::Thermostat {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        heating_setpoint_schedule_id: crate::model::ScheduleId(1),
        cooling_setpoint_schedule_id: crate::model::ScheduleId(1),
        heating_throttle_range_k: 1.0,
        cooling_throttle_range_k: 1.0,
    }
}

fn humidistat(id: u32) -> crate::model::Humidistat {
    const LABEL: &str = "HUMIDISTAT";
    crate::model::Humidistat {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        humidifying_setpoint_schedule_id: crate::model::ScheduleId(1),
        dehumidifying_setpoint_schedule_id: crate::model::ScheduleId(1),
        humidifying_throttle_range: 1.0,
        dehumidifying_throttle_range: 1.0,
    }
}

fn setpoint_manager(id: u32) -> crate::model::SetpointManager {
    const LABEL: &str = "SETPOINT MANAGER";
    crate::model::SetpointManager {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        kind: crate::model::SetpointManagerKind::Scheduled,
        schedule_id: None,
    }
}

fn ideal_loads(id: u32) -> crate::model::IdealLoadsSystem {
    const LABEL: &str = "IDEAL LOADS";
    crate::model::IdealLoadsSystem {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        max_heating_supply_air_temp_c: 1.0,
        min_cooling_supply_air_temp_c: 1.0,
        max_heating_capacity_w: None,
        max_cooling_capacity_w: None,
        outdoor_air_per_person_m3_s: 1.0,
        outdoor_air_per_area_m3_s_m2: 1.0,
    }
}

fn zone_equipment(id: u32) -> crate::model::ZoneEquipmentAssignment {
    const LABEL: &str = "ZONE EQUIPMENT";
    crate::model::ZoneEquipmentAssignment {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        equipment_type: crate::model::ZoneEquipmentType::Baseboard,
        priority: 1,
        heating_capacity_w: 1.0,
        cooling_capacity_w: 1.0,
    }
}

fn air_loop(id: u32) -> crate::model::ModelAirLoop {
    const LABEL: &str = "AIR LOOP";
    crate::model::ModelAirLoop {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        supply_node_id: 1,
        return_node_id: 2,
        design_supply_air_flow_m3_s: 1.0,
        terminal_zone_ids: Vec::new(),
    }
}

fn plant_loop(id: u32) -> crate::model::PlantLoopConfig {
    const LABEL: &str = "PLANT LOOP";
    crate::model::PlantLoopConfig {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        loop_type: crate::model::PlantLoopType::Heating,
        supply_temperature_c: 1.0,
        return_temperature_c: 1.0,
        design_flow_kg_s: 1.0,
        equipment_ids: Vec::new(),
    }
}

fn outdoor_air(id: u32) -> crate::model::OutdoorAirSystem {
    const LABEL: &str = "OUTDOOR AIR";
    crate::model::OutdoorAirSystem {
        id: crate::model::EntityId(id),
        air_loop_id: crate::model::EntityId(1),
        min_oa_flow_m3_s: 1.0,
        economizer_enabled: true,
    }
}

fn infiltration(id: u32) -> crate::model::Infiltration {
    const LABEL: &str = "INFILTRATION";
    crate::model::Infiltration {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        schedule_id: crate::model::ScheduleId(1),
        method: crate::air_exchange::InfiltrationMethod::ScheduledAch,
        design_flow_ach: 1.0,
        flow_per_exterior_area_m3_s_m2: 1.0,
        effective_leakage_area_m2: 1.0,
        discharge_coefficient: 1.0,
        stack_height_m: 1.0,
        constant_term_coefficient: 1.0,
        temperature_term_coefficient: 1.0,
        velocity_term_coefficient: 1.0,
        velocity_squared_term_coefficient: 1.0,
    }
}

fn mechanical(id: u32) -> crate::model::MechanicalVentilation {
    const LABEL: &str = "MECHANICAL";
    crate::model::MechanicalVentilation {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        schedule_id: crate::model::ScheduleId(1),
        design_flow_m3_s: 1.0,
        fan_total_efficiency: 0.5,
        fan_delta_pressure_pa: 1.0,
    }
}

fn shading(id: u32) -> crate::model::ShadingSurface {
    const LABEL: &str = "SHADING";
    crate::model::ShadingSurface {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        vertices_m: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
        transmittance_schedule_id: None,
    }
}

fn space_list(id: u32) -> crate::model::SpaceList {
    const LABEL: &str = "SPACE LIST";
    crate::model::SpaceList {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        space_ids: Vec::new(),
    }
}

fn thermal_enclosure(id: u32) -> crate::model::ThermalEnclosure {
    const LABEL: &str = "THERMAL ENCLOSURE";
    crate::model::ThermalEnclosure {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        zone_ids: Vec::new(),
    }
}

fn electrical_load(id: u32) -> crate::model::ElectricalLoadCenter {
    const LABEL: &str = "ELECTRICAL LOAD";
    crate::model::ElectricalLoadCenter {
        id: crate::model::EntityId(id),
        name: format!("{} {}", LABEL, id),
        generator_ids: Vec::new(),
        pv_ids: Vec::new(),
        battery_ids: Vec::new(),
    }
}

fn pv(id: u32) -> crate::model::PvSystemAssignment {
    const LABEL: &str = "PV";
    crate::model::PvSystemAssignment {
        id: crate::model::EntityId(id),
        dc_capacity_w: 1.0,
        area_m2: 1.0,
        tilt_deg: 1.0,
        azimuth_deg: 1.0,
        module_efficiency: 0.5,
        inverter_efficiency: 0.5,
    }
}

fn battery(id: u32) -> crate::model::BatteryAssignment {
    const LABEL: &str = "BATTERY";
    crate::model::BatteryAssignment {
        id: crate::model::EntityId(id),
        capacity_kwh: 1.0,
        max_charge_w: 1.0,
        max_discharge_w: 1.0,
        round_trip_efficiency: 0.5,
    }
}

fn shw(id: u32) -> crate::model::ShwSystemConfig {
    const LABEL: &str = "SHW";
    crate::model::ShwSystemConfig {
        id: crate::model::EntityId(id),
        heater_capacity_w: 1.0,
        storage_volume_m3: 1.0,
        setpoint_c: 1.0,
        schedule_id: crate::model::ScheduleId(1),
    }
}

fn solar_thermal(id: u32) -> crate::model::SolarThermalConfig {
    const LABEL: &str = "SOLAR THERMAL";
    crate::model::SolarThermalConfig {
        id: crate::model::EntityId(id),
        collector_area_m2: 1.0,
        efficiency: 0.5,
        storage_volume_m3: 1.0,
        tilt_deg: 1.0,
        azimuth_deg: 1.0,
    }
}

fn refrigeration(id: u32) -> crate::model::RefrigerationConfig {
    const LABEL: &str = "REFRIGERATION";
    crate::model::RefrigerationConfig {
        id: crate::model::EntityId(id),
        case_count: 1,
        design_load_w: 1.0,
        defrost_schedule_id: crate::model::ScheduleId(1),
    }
}

fn water(id: u32) -> crate::model::WaterSystemConfig {
    const LABEL: &str = "WATER";
    crate::model::WaterSystemConfig {
        id: crate::model::EntityId(id),
        fixture_count: 1,
        peak_flow_l_s: 1.0,
        schedule_id: crate::model::ScheduleId(1),
    }
}

fn fault(id: u32) -> crate::model::FaultDefinition {
    const LABEL: &str = "FAULT";
    crate::model::FaultDefinition {
        id: crate::model::EntityId(id),
        target_equipment_id: crate::model::EntityId(1),
        fault_type: crate::model::FaultType::SensorBias,
        severity: 1.0,
        start_schedule_id: crate::model::ScheduleId(1),
    }
}

fn sizing(id: u32) -> crate::model::SizingObject {
    const LABEL: &str = "SIZING";
    crate::model::SizingObject {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        sizing_type: crate::model::SizingType::Heating,
        design_day_type: crate::model::DesignDayType::Heating,
    }
}

fn daylight(id: u32) -> crate::model::DaylightZoneConfig {
    const LABEL: &str = "DAYLIGHT";
    crate::model::DaylightZoneConfig {
        id: crate::model::EntityId(id),
        zone_id: crate::model::EntityId(1),
        illuminance_target_lux: 1.0,
        glare_limit: 0.5,
        window_transmittance: 0.5,
    }
}

fn room_air(id: u32) -> crate::model::RoomAirModelAssignment {
    const LABEL: &str = "ROOM AIR";
    crate::model::RoomAirModelAssignment {
        zone_id: crate::model::EntityId(id),
        model: crate::model::RoomAirModelType::WellMixed,
    }
}

fn constant(id: u32) -> crate::schedule::ConstantSchedule {
    const LABEL: &str = "CONSTANT";
    crate::schedule::ConstantSchedule {
        id: crate::model::ScheduleId(id),
        value: 1.0,
    }
}

fn daily(id: u32) -> crate::schedule::DailySchedule {
    const LABEL: &str = "DAILY";
    crate::schedule::DailySchedule {
        id: crate::model::ScheduleId(id),
        hourly_values: [0.0; 24],
        interpolation: crate::schedule::ScheduleInterpolation::Continuous,
        limits: None,
    }
}

fn weekly(id: u32) -> crate::schedule::WeeklySchedule {
    const LABEL: &str = "WEEKLY";
    crate::schedule::WeeklySchedule {
        id: crate::model::ScheduleId(id),
        daily_schedule_ids: [crate::model::ScheduleId(1); 7],
    }
}

fn annual(id: u32) -> crate::schedule::AnnualSchedule {
    const LABEL: &str = "ANNUAL";
    crate::schedule::AnnualSchedule {
        id: crate::model::ScheduleId(id),
        rules: Vec::new(),
        default_daily_schedule_id: crate::model::ScheduleId(1),
        holiday_daily_schedule_id: None,
        holiday_dates: Vec::new(),
    }
}

fn time_series(id: u32) -> crate::schedule::TimeSeriesSchedule {
    const LABEL: &str = "TIME SERIES";
    crate::schedule::TimeSeriesSchedule {
        id: crate::model::ScheduleId(id),
        values: vec![1.0],
        timestep_seconds: 3600,
    }
}

fn rule(first: u8, last: u8) -> crate::schedule::CompactScheduleRule {
    crate::schedule::CompactScheduleRule { start_month: first, start_day: 1, end_month: last, end_day: 28, daily_schedule_id: schedule(1) }
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_zone() {
    holds(seed(), |m| m.zones = three(zone), v::delete_zone(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_space() {
    holds(seed(), |m| m.spaces = three(space), v::delete_space(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_surface_cascades_its_middle_window() {
    holds(seed(), |m| {
        m.surfaces = three(surface);
        m.fenestrations = three(fenestration);
        m.fenestrations[0].surface_id = id(1);
        m.fenestrations[1].surface_id = id(2);
        m.fenestrations[2].surface_id = id(3);
    }, v::delete_surface(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_fenestration() {
    holds(seed(), |m| m.fenestrations = three(fenestration), v::delete_fenestration(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_material() {
    holds(seed(), |m| m.materials = three(material), v::delete_material(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_construction() {
    holds(seed(), |m| m.constructions = three(construction), v::delete_construction(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_people_gain() {
    holds(seed(), |m| m.people = three(people), v::delete_people_gain(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_lighting_gain() {
    holds(seed(), |m| m.lighting = three(lighting), v::delete_lighting_gain(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_equipment_gain() {
    holds(seed(), |m| m.equipment = three(equipment), v::delete_equipment_gain(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_thermostat() {
    holds(seed(), |m| m.thermostats = three(thermostat), v::delete_thermostat(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_humidistat() {
    holds(seed(), |m| m.humidistats = three(humidistat), v::delete_humidistat(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_setpoint_manager() {
    holds(seed(), |m| m.setpoint_managers = three(setpoint_manager), v::delete_setpoint_manager(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_ideal_loads_system() {
    holds(seed(), |m| m.ideal_loads = three(ideal_loads), v::delete_ideal_loads_system(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_zone_equipment() {
    holds(seed(), |m| m.zone_equipment = three(zone_equipment), v::delete_zone_equipment(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_air_loop() {
    holds(seed(), |m| m.air_loops = three(air_loop), v::delete_air_loop(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_plant_loop() {
    holds(seed(), |m| m.plant_loops = three(plant_loop), v::delete_plant_loop(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_outdoor_air_system() {
    holds(seed(), |m| m.outdoor_air_systems = three(outdoor_air), v::delete_outdoor_air_system(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_infiltration() {
    holds(seed(), |m| m.infiltrations = three(infiltration), v::delete_infiltration(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_mechanical_ventilation() {
    holds(seed(), |m| m.mechanical_ventilations = three(mechanical), v::delete_mechanical_ventilation(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_shading_surface() {
    holds(seed(), |m| m.shading_surfaces = three(shading), v::delete_shading_surface(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_space_list() {
    holds(seed(), |m| m.space_lists = three(space_list), v::delete_space_list(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_thermal_enclosure() {
    holds(seed(), |m| m.thermal_enclosures = three(thermal_enclosure), v::delete_thermal_enclosure(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_electrical_load_center() {
    holds(seed(), |m| m.electrical_load_centers = three(electrical_load), v::delete_electrical_load_center(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_pv_system() {
    holds(seed(), |m| m.pv_systems = three(pv), v::delete_pv_system(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_battery() {
    holds(seed(), |m| m.battery_storage = three(battery), v::delete_battery(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_shw_system() {
    holds(seed(), |m| m.shw_systems = three(shw), v::delete_shw_system(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_solar_thermal_system() {
    holds(seed(), |m| m.solar_thermal_systems = three(solar_thermal), v::delete_solar_thermal_system(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_refrigeration_system() {
    holds(seed(), |m| m.refrigeration_systems = three(refrigeration), v::delete_refrigeration_system(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_water_system() {
    holds(seed(), |m| m.water_systems = three(water), v::delete_water_system(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_fault() {
    holds(seed(), |m| m.faults = three(fault), v::delete_fault(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_sizing_object() {
    holds(seed(), |m| m.sizing_objects = three(sizing), v::delete_sizing_object(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_daylight_zone() {
    holds(seed(), |m| m.daylight_zones = three(daylight), v::delete_daylight_zone(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_room_air_model_assignment() {
    holds(seed(), |m| {
        m.zones = three(zone);
        m.room_air_models = three(room_air);
    }, v::delete_room_air_model_assignment(id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_constant_schedule() {
    holds(seed(), |m| m.schedules.constants = three(constant), v::delete_constant_schedule(schedule(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_daily_schedule() {
    holds(seed(), |m| m.schedules.daily = vec![daily(4), daily(5), daily(6)], v::delete_daily_schedule(schedule(5))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_weekly_schedule() {
    holds(seed(), |m| m.schedules.weekly = vec![weekly(4), weekly(5), weekly(6)], v::delete_weekly_schedule(schedule(5))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_annual_schedule() {
    holds(seed(), |m| m.schedules.annual = vec![annual(4), annual(5), annual(6)], v::delete_annual_schedule(schedule(5))).await;
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_middle_time_series_schedule() {
    holds(seed(), |m| m.schedules.time_series = vec![time_series(4), time_series(5), time_series(6)], v::delete_time_series_schedule(schedule(5))).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_output_variable() {
    holds(seed(), |m| m.output_variables = (1..=3).map(|n| crate::model::OutputVariableSpec { name: format!("VARIABLE {n}"), key: "KEY".to_string(), reporting_frequency: crate::model::OutputReportFrequency::Hourly }).collect(), v::remove_output_variable("VARIABLE 2".to_string(), "KEY".to_string())).await;
}

#[semio_framework_async_macros::async_test]
async fn disconnecting_the_middle_adjacency_pair() {
    holds(seed(), |m| {
        m.surfaces = three(surface);
        m.adjacency_pairs = vec![(1, 2), (1, 3), (2, 3)].into_iter().map(|(a, b)| crate::model::AdjacencyPair { surface_a_id: id(a), surface_b_id: id(b) }).collect();
    }, v::disconnect_surfaces(id(1), id(3))).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_construction_layer() {
    holds(seed(), |m| {
        m.materials = three(material);
        m.constructions[0].layer_material_ids = vec![id(1), id(2), id(3)];
    }, v::remove_construction_layer(id(1), 1)).await;
}

#[semio_framework_async_macros::async_test]
async fn reordering_the_middle_construction_layer() {
    holds(seed(), |m| {
        m.materials = three(material);
        m.constructions[0].layer_material_ids = vec![id(1), id(2), id(3)];
    }, v::reorder_construction_layers(id(1), vec![id(1), id(3), id(2)])).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_space_list_member() {
    holds(seed(), |m| {
        m.spaces = three(space);
        m.space_lists = vec![crate::model::SpaceList { space_ids: vec![id(1), id(2), id(3)], ..space_list(1) }];
    }, v::remove_space_list_member(id(1), id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_thermal_enclosure_zone() {
    holds(seed(), |m| {
        m.zones = three(zone);
        m.thermal_enclosures = vec![crate::model::ThermalEnclosure { zone_ids: vec![id(1), id(2), id(3)], ..thermal_enclosure(1) }];
    }, v::remove_thermal_enclosure_zone(id(1), id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_air_loop_terminal_zone() {
    holds(seed(), |m| {
        m.zones = three(zone);
        m.air_loops[0].terminal_zone_ids = vec![id(1), id(2), id(3)];
    }, v::remove_air_loop_terminal_zone(id(1), id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_plant_loop_equipment() {
    holds(seed(), |m| m.plant_loops = vec![crate::model::PlantLoopConfig { equipment_ids: vec![id(10), id(20), id(30)], ..plant_loop(1) }], v::remove_plant_loop_equipment(id(1), id(20))).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_electrical_load_center_pv() {
    holds(seed(), |m| {
        m.pv_systems = three(pv);
        m.electrical_load_centers = vec![crate::model::ElectricalLoadCenter { pv_ids: vec![id(1), id(2), id(3)], ..electrical_load(1) }];
    }, v::remove_electrical_load_center_pv(id(1), id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_electrical_load_center_battery() {
    holds(seed(), |m| {
        m.battery_storage = three(battery);
        m.electrical_load_centers = vec![crate::model::ElectricalLoadCenter { battery_ids: vec![id(1), id(2), id(3)], ..electrical_load(1) }];
    }, v::remove_electrical_load_center_battery(id(1), id(2))).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_annual_schedule_rule() {
    holds(seed(), |m| m.schedules.annual = vec![crate::schedule::AnnualSchedule { rules: vec![rule(1, 3), rule(4, 6), rule(7, 9)], ..annual(4) }], v::remove_annual_schedule_rule(schedule(4), 1)).await;
}

#[semio_framework_async_macros::async_test]
async fn reordering_the_middle_annual_schedule_rule() {
    holds(seed(), |m| m.schedules.annual = vec![crate::schedule::AnnualSchedule { rules: vec![rule(1, 3), rule(4, 6), rule(7, 9)], ..annual(4) }], v::reorder_annual_schedule_rules(schedule(4), 1, 0)).await;
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_annual_schedule_holiday() {
    holds(seed(), |m| m.schedules.annual = vec![crate::schedule::AnnualSchedule { holiday_dates: vec![(2026, 1, 1), (2026, 5, 1), (2026, 12, 25)], ..annual(4) }], v::remove_annual_schedule_holiday(schedule(4), 2026, 5, 1)).await;
}

#[semio_framework_async_macros::async_test]
async fn creating_a_zone_at_a_middle_index_is_undone_by_its_delete() {
    holds(seed(), |m| m.zones = three(zone), v::create_zone(id(9), "ZONE NINE".to_string(), 1.0, 1, true, true, Some(1))).await;
}

#[semio_framework_async_macros::async_test]
async fn creating_a_thermostat_at_a_middle_index_is_undone_by_its_delete() {
    holds(seed(), |m| m.thermostats = three(thermostat), v::create_thermostat(id(9), id(1), schedule(1), schedule(1), 1.0, 1.0, Some(1))).await;
}

#[semio_framework_async_macros::async_test]
async fn adding_an_output_variable_at_a_middle_index_is_undone_by_its_removal() {
    holds(seed(), |m| m.output_variables = Vec::new(), v::add_output_variable("VARIABLE".to_string(), "KEY".to_string(), crate::model::OutputReportFrequency::Hourly, Some(0))).await;
}

#[semio_framework_async_macros::async_test]
async fn adding_plant_loop_equipment_at_a_middle_index_is_undone_by_its_removal() {
    holds(seed(), |m| m.plant_loops = vec![crate::model::PlantLoopConfig { equipment_ids: vec![id(10), id(30)], ..plant_loop(1) }], v::add_plant_loop_equipment(id(1), id(20), Some(1))).await;
}
