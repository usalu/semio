//! 🧳️ Typed immutable Energy authority separates path-local persisted fields from numerical runtime views.
use crate::model::*;
use crate::schedule::ScheduleId;
use semio_framework_value::shared_issuance::SealedShared;
use semio_framework_value_persistent::list::persistent::PersistentList;
use semio_framework_value_derive::ToValue;
#[path="🔺️diff/🦀️.rs"]pub mod diff;

/// 🪡️ Original Site fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateSite {
 pub latitude_deg: f64,
 pub longitude_deg: f64,
 pub elevation_m: f64,
 pub time_zone_hours: f64,
 pub north_axis_deg: f64,
}

/// 🪡️ Original Zone fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateZone {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub volume_m3: f64,
 pub multiplier: u32,
 pub conditioned: bool,
 pub part_of_total_floor_area: bool,
}

/// 🪡️ Original Space fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateSpace {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub zone_id: EntityId,
 pub floor_area_m2: f64,
}

/// 🪡️ Original Surface fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateSurface {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub zone_id: EntityId,
 pub class: SealedShared<SurfaceClass>,
 pub vertices_m: SealedShared<Vec<[f64; 3]>>,
 pub construction_id: EntityId,
 pub outside_boundary_condition: SealedShared<OutsideBoundary>,
 pub sun_exposed: bool,
 pub wind_exposed: bool,
 pub multiplier: u32,
}

/// 🪡️ Original Fenestration fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateFenestration {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub surface_id: EntityId,
 pub u_value_w_m2k: f64,
 pub shgc: f64,
 pub vlt: f64,
 pub area_m2: f64,
 pub height_m: f64,
 pub sill_height_m: f64,
 pub frame_conductance_w_k: f64,
 pub divider_conductance_w_k: f64,
 pub overhang_depth_m: f64,
 pub overhang_offset_m: f64,
 pub fin_depth_m: f64,
 pub fin_offset_m: f64,
 pub glazing_construction_id: SealedShared<Option<EntityId>>,
 pub vertices_m: SealedShared<Vec<[f64; 3]>>,
}

/// 🪡️ Original Material fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateMaterial {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub roughness: SealedShared<SurfaceRoughness>,
 pub thickness_m: f64,
 pub conductivity_w_m_k: f64,
 pub density_kg_m3: f64,
 pub specific_heat_j_kg_k: f64,
 pub thermal_absorptance: f64,
 pub solar_absorptance: f64,
 pub visible_absorptance: f64,
}

/// 🪡️ Original GlazingMaterial fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateGlazingMaterial {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub thickness_m: f64,
 pub conductivity_w_m_k: f64,
 pub solar_transmittance: f64,
 pub solar_reflectance_front: f64,
 pub solar_reflectance_back: f64,
 pub visible_transmittance: f64,
 pub visible_reflectance_front: f64,
 pub visible_reflectance_back: f64,
 pub infrared_transmittance: f64,
 pub infrared_emissivity_front: f64,
 pub infrared_emissivity_back: f64,
}

/// 🪡️ Original GasMaterial fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateGasMaterial {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub thickness_m: f64,
 pub gas: SealedShared<GasKind>,
}

/// 🪡️ Original Construction fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateConstruction {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub layer_material_ids: SealedShared<Vec<EntityId>>,
}

/// 🪡️ Original PeopleGain fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StatePeopleGain {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub schedule_id: ScheduleId,
 pub activity_schedule_id: ScheduleId,
 pub people_per_area: f64,
 pub sensible_fraction: f64,
 pub latent_fraction: f64,
 pub radiant_fraction: f64,
}

/// 🪡️ Original LightingGain fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateLightingGain {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub schedule_id: ScheduleId,
 pub watts_per_area: f64,
 pub radiant_fraction: f64,
 pub visible_fraction: f64,
 pub return_air_fraction: f64,
}

/// 🪡️ Original EquipmentGain fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateEquipmentGain {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub schedule_id: ScheduleId,
 pub watts_per_area: f64,
 pub radiant_fraction: f64,
 pub latent_fraction: f64,
}

/// 🪡️ Original Thermostat fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateThermostat {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub heating_setpoint_schedule_id: ScheduleId,
 pub cooling_setpoint_schedule_id: ScheduleId,
 pub heating_throttle_range_k: f64,
 pub cooling_throttle_range_k: f64,
}

/// 🪡️ Original IdealLoadsSystem fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateIdealLoadsSystem {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub max_heating_supply_air_temp_c: f64,
 pub min_cooling_supply_air_temp_c: f64,
 pub max_heating_capacity_w: SealedShared<Option<f64>>,
 pub max_cooling_capacity_w: SealedShared<Option<f64>>,
 pub outdoor_air_per_person_m3_s: f64,
 pub outdoor_air_per_area_m3_s_m2: f64,
}

/// 🪡️ Original Humidistat fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateHumidistat {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub humidifying_setpoint_schedule_id: ScheduleId,
 pub dehumidifying_setpoint_schedule_id: ScheduleId,
 pub humidifying_throttle_range: f64,
 pub dehumidifying_throttle_range: f64,
}

/// 🪡️ Original SetpointManager fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateSetpointManager {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub kind: SealedShared<SetpointManagerKind>,
 pub schedule_id: SealedShared<Option<ScheduleId>>,
}

/// 🪡️ Original ZoneEquipmentAssignment fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateZoneEquipmentAssignment {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub equipment_type: SealedShared<ZoneEquipmentType>,
 pub priority: u8,
 pub heating_capacity_w: f64,
 pub cooling_capacity_w: f64,
}

/// 🪡️ Original ModelAirLoop fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateModelAirLoop {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub supply_node_id: u32,
 pub return_node_id: u32,
 pub design_supply_air_flow_m3_s: f64,
 pub terminal_zone_ids: SealedShared<Vec<EntityId>>,
}

/// 🪡️ Original PlantLoopConfig fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StatePlantLoopConfig {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub loop_type: SealedShared<PlantLoopType>,
 pub supply_temperature_c: f64,
 pub return_temperature_c: f64,
 pub design_flow_kg_s: f64,
 pub equipment_ids: SealedShared<Vec<EntityId>>,
}

/// 🪡️ Original OutdoorAirSystem fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateOutdoorAirSystem {
 pub id: EntityId,
 pub air_loop_id: EntityId,
 pub min_oa_flow_m3_s: f64,
 pub economizer_enabled: bool,
}

/// 🪡️ Original ShadingSurface fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateShadingSurface {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub vertices_m: SealedShared<Vec<[f64; 3]>>,
 pub transmittance_schedule_id: SealedShared<Option<ScheduleId>>,
}

/// 🪡️ Original SpaceList fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateSpaceList {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub space_ids: SealedShared<Vec<EntityId>>,
}

/// 🪡️ Original ThermalEnclosure fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateThermalEnclosure {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub zone_ids: SealedShared<Vec<EntityId>>,
}

/// 🪡️ Original AdjacencyPair fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateAdjacencyPair {
 pub surface_a_id: EntityId,
 pub surface_b_id: EntityId,
}

/// 🪡️ Original MechanicalVentilation fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateMechanicalVentilation {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub schedule_id: ScheduleId,
 pub design_flow_m3_s: f64,
 pub fan_total_efficiency: f64,
 pub fan_delta_pressure_pa: f64,
}

/// 🪡️ Original AirflowNetworkDefinition fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateAirflowNetworkDefinition {
 pub zone_node_ids: SealedShared<Vec<(EntityId, u32)>>,
 pub outdoor_node_id: u32,
 pub link_ids: SealedShared<Vec<u32>>,
}

/// 🪡️ Original ElectricalLoadCenter fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateElectricalLoadCenter {
 pub id: EntityId,
 pub name: SealedShared<String>,
 pub generator_ids: SealedShared<Vec<EntityId>>,
 pub pv_ids: SealedShared<Vec<EntityId>>,
 pub battery_ids: SealedShared<Vec<EntityId>>,
}

/// 🪡️ Original PvSystemAssignment fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StatePvSystemAssignment {
 pub id: EntityId,
 pub dc_capacity_w: f64,
 pub area_m2: f64,
 pub tilt_deg: f64,
 pub azimuth_deg: f64,
 pub module_efficiency: f64,
 pub inverter_efficiency: f64,
 pub temperature_coefficient: f64,
 pub faiman_heat_loss_constant_w_m2k: f64,
 pub faiman_heat_loss_wind_w_s_m3k: f64,
 pub mounting: SealedShared<PhotovoltaicMounting>,
}

/// 🪡️ Original BatteryAssignment fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateBatteryAssignment {
 pub id: EntityId,
 pub capacity_kwh: f64,
 pub max_charge_w: f64,
 pub max_discharge_w: f64,
 pub round_trip_efficiency: f64,
}

/// 🪡️ Original ShwSystemConfig fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateShwSystemConfig {
 pub id: EntityId,
 pub heater_capacity_w: f64,
 pub storage_volume_m3: f64,
 pub setpoint_c: f64,
 pub schedule_id: ScheduleId,
}

/// 🪡️ Original SolarThermalConfig fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateSolarThermalConfig {
 pub id: EntityId,
 pub collector_area_m2: f64,
 pub efficiency: f64,
 pub storage_volume_m3: f64,
 pub tilt_deg: f64,
 pub azimuth_deg: f64,
}

/// 🪡️ Original RefrigerationConfig fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateRefrigerationConfig {
 pub id: EntityId,
 pub case_count: u32,
 pub design_load_w: f64,
 pub defrost_schedule_id: ScheduleId,
}

/// 🪡️ Original WaterSystemConfig fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateWaterSystemConfig {
 pub id: EntityId,
 pub fixture_count: u32,
 pub peak_flow_l_s: f64,
 pub schedule_id: ScheduleId,
}

/// 🪡️ Original FaultDefinition fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateFaultDefinition {
 pub id: EntityId,
 pub target_equipment_id: EntityId,
 pub fault_type: SealedShared<FaultType>,
 pub severity: f64,
 pub start_schedule_id: ScheduleId,
}

/// 🪡️ Original OutputVariableSpec fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateOutputVariableSpec {
 pub name: SealedShared<String>,
 pub key: SealedShared<String>,
 pub reporting_frequency: SealedShared<OutputReportFrequency>,
}

/// 🪡️ Original SizingObject fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateSizingObject {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub sizing_type: SealedShared<SizingType>,
 pub design_day_type: SealedShared<DesignDayType>,
}

/// 🪡️ Original DaylightZoneConfig fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateDaylightZoneConfig {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub illuminance_target_lux: f64,
 pub glare_limit: f64,
 pub window_transmittance: f64,
}

/// 🪡️ Original RoomAirModelAssignment fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateRoomAirModelAssignment {
 pub zone_id: EntityId,
 pub model: SealedShared<RoomAirModelType>,
}

/// 🪡️ Original GroundTemperatureConfig fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateGroundTemperatureConfig {
 pub building_surface_c: SealedShared<[f64; 12]>,
 pub shallow_c: SealedShared<[f64; 12]>,
 pub deep_c: f64,
}

/// 🪡️ Original Infiltration fields retain independent immutable payload authorities.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateInfiltration {
 pub id: EntityId,
 pub zone_id: EntityId,
 pub schedule_id: ScheduleId,
 pub method: SealedShared<crate::air_exchange::InfiltrationMethod>,
 pub design_flow_ach: f64,
 pub flow_per_exterior_area_m3_s_m2: f64,
 pub effective_leakage_area_m2: f64,
 pub discharge_coefficient: f64,
 pub stack_height_m: f64,
 pub constant_term_coefficient: f64,
 pub temperature_term_coefficient: f64,
 pub velocity_term_coefficient: f64,
 pub velocity_squared_term_coefficient: f64,
}

/// 📅️ Original ScheduleLimits schedule fields remain separately shared.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateScheduleLimits {
 pub min: f64,
 pub max: f64,
}

/// 📅️ Original ConstantSchedule schedule fields remain separately shared.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateConstantSchedule {
 pub id: ScheduleId,
 pub value: f64,
}

/// 📅️ Original DailySchedule schedule fields remain separately shared.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateDailySchedule {
 pub id: ScheduleId,
 pub hourly_values: SealedShared<[f64; 24]>,
 pub interpolation: SealedShared<crate::schedule::ScheduleInterpolation>,
 pub limits: SealedShared<Option<crate::schedule::ScheduleLimits>>,
}

/// 📅️ Original WeeklySchedule schedule fields remain separately shared.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateWeeklySchedule {
 pub id: ScheduleId,
 pub daily_schedule_ids: SealedShared<[ScheduleId; 7]>,
}

/// 📅️ Original CompactScheduleRule schedule fields remain separately shared.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateCompactScheduleRule {
 pub start_month: u8,
 pub start_day: u8,
 pub end_month: u8,
 pub end_day: u8,
 pub daily_schedule_id: ScheduleId,
}

/// 📅️ Original AnnualSchedule schedule fields remain separately shared.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateAnnualSchedule {
 pub id: ScheduleId,
 pub rules: SealedShared<Vec<crate::schedule::CompactScheduleRule>>,
 pub default_daily_schedule_id: ScheduleId,
 pub holiday_daily_schedule_id: SealedShared<Option<ScheduleId>>,
 pub holiday_dates: SealedShared<Vec<(u16, u8, u8)>>,
}

/// 📅️ Original TimeSeriesSchedule schedule fields remain separately shared.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateTimeSeriesSchedule {
 pub id: ScheduleId,
 pub values: SealedShared<Vec<f64>>,
 pub timestep_seconds: u32,
}

/// 📚️ Persisted schedule collections retain original immutable ranked rows.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct StateScheduleSet {
 pub constants:PersistentList<StateConstantSchedule>,
 pub daily:PersistentList<StateDailySchedule>,
 pub weekly:PersistentList<StateWeeklySchedule>,
 pub annual:PersistentList<StateAnnualSchedule>,
 pub time_series:PersistentList<StateTimeSeriesSchedule>,
}

macro_rules! state_group {
 ($name:ident {$($field:ident:$kind:ty),*$(,)?})=>{
  #[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
  #[canonical_json(owner=semio_framework_pack_json)]
  pub struct $name{$(pub $field:$kind,)*}
 };
}
state_group!(ModelIdentityState{
 name:SealedShared<String>,version:SealedShared<String>,site:SealedShared<StateSite>,
 ground_temperature:SealedShared<StateGroundTemperatureConfig>,run_period:SealedShared<crate::calendar::RunPeriod>,
});
state_group!(ModelGeometryState{
 zones:PersistentList<StateZone>,spaces:PersistentList<StateSpace>,surfaces:PersistentList<StateSurface>,
 fenestrations:PersistentList<StateFenestration>,shading_surfaces:PersistentList<StateShadingSurface>,
 space_lists:PersistentList<StateSpaceList>,thermal_enclosures:PersistentList<StateThermalEnclosure>,adjacency_pairs:PersistentList<StateAdjacencyPair>,
});
state_group!(ModelMaterialState{
 materials:PersistentList<StateMaterial>,glazing_materials:PersistentList<StateGlazingMaterial>,
 gas_materials:PersistentList<StateGasMaterial>,constructions:PersistentList<StateConstruction>,
});
state_group!(ModelLoadState{
 people:PersistentList<StatePeopleGain>,lighting:PersistentList<StateLightingGain>,equipment:PersistentList<StateEquipmentGain>,
 infiltrations:PersistentList<StateInfiltration>,mechanical_ventilations:PersistentList<StateMechanicalVentilation>,
 daylight_zones:PersistentList<StateDaylightZoneConfig>,room_air_models:PersistentList<StateRoomAirModelAssignment>,
});
state_group!(ModelControlState{
 thermostats:PersistentList<StateThermostat>,humidistats:PersistentList<StateHumidistat>,setpoint_managers:PersistentList<StateSetpointManager>,
 ideal_loads:PersistentList<StateIdealLoadsSystem>,zone_equipment:PersistentList<StateZoneEquipmentAssignment>,
});
state_group!(ModelSystemState{
 air_loops:PersistentList<StateModelAirLoop>,plant_loops:PersistentList<StatePlantLoopConfig>,outdoor_air_systems:PersistentList<StateOutdoorAirSystem>,
 airflow_network:Option<SealedShared<StateAirflowNetworkDefinition>>,electrical_load_centers:PersistentList<StateElectricalLoadCenter>,
});
state_group!(ModelGenerationState{
 pv_systems:PersistentList<StatePvSystemAssignment>,battery_storage:PersistentList<StateBatteryAssignment>,shw_systems:PersistentList<StateShwSystemConfig>,
 solar_thermal_systems:PersistentList<StateSolarThermalConfig>,refrigeration_systems:PersistentList<StateRefrigerationConfig>,
 water_systems:PersistentList<StateWaterSystemConfig>,faults:PersistentList<StateFaultDefinition>,
});
state_group!(ModelOutputState{
 output_variables:PersistentList<StateOutputVariableSpec>,sizing_objects:PersistentList<StateSizingObject>,schedules:SealedShared<StateScheduleSet>,
});
/// 🏢️ The sole logical Model record has fixed shared branches and no numerical or child mirrors.
#[derive(Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
pub struct EnergyModelState{
 #[value(flatten)]pub identity:SealedShared<ModelIdentityState>,
 #[value(flatten)]pub geometry:SealedShared<ModelGeometryState>,
 #[value(flatten)]pub materials:SealedShared<ModelMaterialState>,
 #[value(flatten)]pub loads:SealedShared<ModelLoadState>,
 #[value(flatten)]pub controls:SealedShared<ModelControlState>,
 #[value(flatten)]pub systems:SealedShared<ModelSystemState>,
 #[value(flatten)]pub generation:SealedShared<ModelGenerationState>,
 #[value(flatten)]pub outputs:SealedShared<ModelOutputState>,
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]pub(crate) mod tests;

#[path="👁️read/🦀️.rs"]pub mod read;
pub use read::EnergyModelView;

#[path="🪡️fields/🦀️.rs"]pub mod fields;

pub use crate::calendar::RunPeriod;
