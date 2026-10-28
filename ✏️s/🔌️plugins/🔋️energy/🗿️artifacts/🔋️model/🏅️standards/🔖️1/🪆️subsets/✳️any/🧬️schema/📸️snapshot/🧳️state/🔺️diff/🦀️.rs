//! 🩹️ Sparse immutable Energy row fields use the shared original inverse and central header exchange.
use super::*;
macro_rules! state_patch{
 ($patch:ident,$owner:ident for $row:ty {$($field:ident:$kind:ty),+$(,)?})=>{
  #[derive(Default,Debug,PartialEq,ToValue,semio_framework_value_derive::NativeReceive,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree)]
  #[canonical_json(owner=semio_framework_pack_json)]
  pub struct $patch{$(#[value(skip_serializing_if="Option::is_none")]pub $field:Option<$kind>,)+}
  protocol::retained_row_patch!(pub $owner for $patch => $row {$($field:$kind),+});
 };
}
state_patch!(StateSitePatch,state_site_fields for StateSite {
 latitude_deg:f64,
 longitude_deg:f64,
 elevation_m:f64,
 time_zone_hours:f64,
 north_axis_deg:f64,
});
state_patch!(StateZonePatch,state_zone_fields for StateZone {
 name:SealedShared<String>,
 volume_m3:f64,
 multiplier:u32,
 conditioned:bool,
 part_of_total_floor_area:bool,
});
state_patch!(StateSpacePatch,state_space_fields for StateSpace {
 name:SealedShared<String>,
 zone_id:EntityId,
 floor_area_m2:f64,
});
state_patch!(StateSurfacePatch,state_surface_fields for StateSurface {
 name:SealedShared<String>,
 zone_id:EntityId,
 class:SealedShared<SurfaceClass>,
 vertices_m:SealedShared<Vec<[f64; 3]>>,
 construction_id:EntityId,
 outside_boundary_condition:SealedShared<OutsideBoundary>,
 sun_exposed:bool,
 wind_exposed:bool,
 multiplier:u32,
});
state_patch!(StateFenestrationPatch,state_fenestration_fields for StateFenestration {
 name:SealedShared<String>,
 surface_id:EntityId,
 u_value_w_m2k:f64,
 shgc:f64,
 vlt:f64,
 area_m2:f64,
 height_m:f64,
 sill_height_m:f64,
 frame_conductance_w_k:f64,
 divider_conductance_w_k:f64,
 overhang_depth_m:f64,
 overhang_offset_m:f64,
 fin_depth_m:f64,
 fin_offset_m:f64,
 glazing_construction_id:SealedShared<Option<EntityId>>,
 vertices_m:SealedShared<Vec<[f64; 3]>>,
});
state_patch!(StateMaterialPatch,state_material_fields for StateMaterial {
 name:SealedShared<String>,
 roughness:SealedShared<SurfaceRoughness>,
 thickness_m:f64,
 conductivity_w_m_k:f64,
 density_kg_m3:f64,
 specific_heat_j_kg_k:f64,
 thermal_absorptance:f64,
 solar_absorptance:f64,
 visible_absorptance:f64,
});
state_patch!(StateGlazingMaterialPatch,state_glazing_material_fields for StateGlazingMaterial {
 name:SealedShared<String>,
 thickness_m:f64,
 conductivity_w_m_k:f64,
 solar_transmittance:f64,
 solar_reflectance_front:f64,
 solar_reflectance_back:f64,
 visible_transmittance:f64,
 visible_reflectance_front:f64,
 visible_reflectance_back:f64,
 infrared_transmittance:f64,
 infrared_emissivity_front:f64,
 infrared_emissivity_back:f64,
});
state_patch!(StateGasMaterialPatch,state_gas_material_fields for StateGasMaterial {
 name:SealedShared<String>,
 thickness_m:f64,
 gas:SealedShared<GasKind>,
});
state_patch!(StateConstructionPatch,state_construction_fields for StateConstruction {
 name:SealedShared<String>,
 layer_material_ids:SealedShared<Vec<EntityId>>,
});
state_patch!(StatePeopleGainPatch,state_people_gain_fields for StatePeopleGain {
 zone_id:EntityId,
 schedule_id:ScheduleId,
 activity_schedule_id:ScheduleId,
 people_per_area:f64,
 sensible_fraction:f64,
 latent_fraction:f64,
 radiant_fraction:f64,
});
state_patch!(StateLightingGainPatch,state_lighting_gain_fields for StateLightingGain {
 zone_id:EntityId,
 schedule_id:ScheduleId,
 watts_per_area:f64,
 radiant_fraction:f64,
 visible_fraction:f64,
 return_air_fraction:f64,
});
state_patch!(StateEquipmentGainPatch,state_equipment_gain_fields for StateEquipmentGain {
 zone_id:EntityId,
 schedule_id:ScheduleId,
 watts_per_area:f64,
 radiant_fraction:f64,
 latent_fraction:f64,
});
state_patch!(StateThermostatPatch,state_thermostat_fields for StateThermostat {
 zone_id:EntityId,
 heating_setpoint_schedule_id:ScheduleId,
 cooling_setpoint_schedule_id:ScheduleId,
 heating_throttle_range_k:f64,
 cooling_throttle_range_k:f64,
});
state_patch!(StateIdealLoadsSystemPatch,state_ideal_loads_system_fields for StateIdealLoadsSystem {
 zone_id:EntityId,
 max_heating_supply_air_temp_c:f64,
 min_cooling_supply_air_temp_c:f64,
 max_heating_capacity_w:SealedShared<Option<f64>>,
 max_cooling_capacity_w:SealedShared<Option<f64>>,
 outdoor_air_per_person_m3_s:f64,
 outdoor_air_per_area_m3_s_m2:f64,
});
state_patch!(StateHumidistatPatch,state_humidistat_fields for StateHumidistat {
 zone_id:EntityId,
 humidifying_setpoint_schedule_id:ScheduleId,
 dehumidifying_setpoint_schedule_id:ScheduleId,
 humidifying_throttle_range:f64,
 dehumidifying_throttle_range:f64,
});
state_patch!(StateSetpointManagerPatch,state_setpoint_manager_fields for StateSetpointManager {
 name:SealedShared<String>,
 kind:SealedShared<SetpointManagerKind>,
 schedule_id:SealedShared<Option<ScheduleId>>,
});
state_patch!(StateZoneEquipmentAssignmentPatch,state_zone_equipment_assignment_fields for StateZoneEquipmentAssignment {
 zone_id:EntityId,
 equipment_type:SealedShared<ZoneEquipmentType>,
 priority:u8,
 heating_capacity_w:f64,
 cooling_capacity_w:f64,
});
state_patch!(StateModelAirLoopPatch,state_model_air_loop_fields for StateModelAirLoop {
 name:SealedShared<String>,
 supply_node_id:u32,
 return_node_id:u32,
 design_supply_air_flow_m3_s:f64,
 terminal_zone_ids:SealedShared<Vec<EntityId>>,
});
state_patch!(StatePlantLoopConfigPatch,state_plant_loop_config_fields for StatePlantLoopConfig {
 name:SealedShared<String>,
 loop_type:SealedShared<PlantLoopType>,
 supply_temperature_c:f64,
 return_temperature_c:f64,
 design_flow_kg_s:f64,
 equipment_ids:SealedShared<Vec<EntityId>>,
});
state_patch!(StateOutdoorAirSystemPatch,state_outdoor_air_system_fields for StateOutdoorAirSystem {
 air_loop_id:EntityId,
 min_oa_flow_m3_s:f64,
 economizer_enabled:bool,
});
state_patch!(StateShadingSurfacePatch,state_shading_surface_fields for StateShadingSurface {
 name:SealedShared<String>,
 vertices_m:SealedShared<Vec<[f64; 3]>>,
 transmittance_schedule_id:SealedShared<Option<ScheduleId>>,
});
state_patch!(StateSpaceListPatch,state_space_list_fields for StateSpaceList {
 name:SealedShared<String>,
 space_ids:SealedShared<Vec<EntityId>>,
});
state_patch!(StateThermalEnclosurePatch,state_thermal_enclosure_fields for StateThermalEnclosure {
 name:SealedShared<String>,
 zone_ids:SealedShared<Vec<EntityId>>,
});
state_patch!(StateMechanicalVentilationPatch,state_mechanical_ventilation_fields for StateMechanicalVentilation {
 zone_id:EntityId,
 schedule_id:ScheduleId,
 design_flow_m3_s:f64,
 fan_total_efficiency:f64,
 fan_delta_pressure_pa:f64,
});
state_patch!(StateAirflowNetworkDefinitionPatch,state_airflow_network_definition_fields for StateAirflowNetworkDefinition {
 zone_node_ids:SealedShared<Vec<(EntityId, u32)>>,
 outdoor_node_id:u32,
 link_ids:SealedShared<Vec<u32>>,
});
state_patch!(StateElectricalLoadCenterPatch,state_electrical_load_center_fields for StateElectricalLoadCenter {
 name:SealedShared<String>,
 generator_ids:SealedShared<Vec<EntityId>>,
 pv_ids:SealedShared<Vec<EntityId>>,
 battery_ids:SealedShared<Vec<EntityId>>,
});
state_patch!(StatePvSystemAssignmentPatch,state_pv_system_assignment_fields for StatePvSystemAssignment {
 dc_capacity_w:f64,
 area_m2:f64,
 tilt_deg:f64,
 azimuth_deg:f64,
 module_efficiency:f64,
 inverter_efficiency:f64,
 temperature_coefficient:f64,
 faiman_heat_loss_constant_w_m2k:f64,
 faiman_heat_loss_wind_w_s_m3k:f64,
 mounting:SealedShared<PhotovoltaicMounting>,
});
state_patch!(StateBatteryAssignmentPatch,state_battery_assignment_fields for StateBatteryAssignment {
 capacity_kwh:f64,
 max_charge_w:f64,
 max_discharge_w:f64,
 round_trip_efficiency:f64,
});
state_patch!(StateShwSystemConfigPatch,state_shw_system_config_fields for StateShwSystemConfig {
 heater_capacity_w:f64,
 storage_volume_m3:f64,
 setpoint_c:f64,
 schedule_id:ScheduleId,
});
state_patch!(StateSolarThermalConfigPatch,state_solar_thermal_config_fields for StateSolarThermalConfig {
 collector_area_m2:f64,
 efficiency:f64,
 storage_volume_m3:f64,
 tilt_deg:f64,
 azimuth_deg:f64,
});
state_patch!(StateRefrigerationConfigPatch,state_refrigeration_config_fields for StateRefrigerationConfig {
 case_count:u32,
 design_load_w:f64,
 defrost_schedule_id:ScheduleId,
});
state_patch!(StateWaterSystemConfigPatch,state_water_system_config_fields for StateWaterSystemConfig {
 fixture_count:u32,
 peak_flow_l_s:f64,
 schedule_id:ScheduleId,
});
state_patch!(StateFaultDefinitionPatch,state_fault_definition_fields for StateFaultDefinition {
 target_equipment_id:EntityId,
 fault_type:SealedShared<FaultType>,
 severity:f64,
 start_schedule_id:ScheduleId,
});
state_patch!(StateOutputVariableSpecPatch,state_output_variable_spec_fields for StateOutputVariableSpec {
 reporting_frequency:SealedShared<OutputReportFrequency>,
});
state_patch!(StateSizingObjectPatch,state_sizing_object_fields for StateSizingObject {
 zone_id:EntityId,
 sizing_type:SealedShared<SizingType>,
 design_day_type:SealedShared<DesignDayType>,
});
state_patch!(StateDaylightZoneConfigPatch,state_daylight_zone_config_fields for StateDaylightZoneConfig {
 zone_id:EntityId,
 illuminance_target_lux:f64,
 glare_limit:f64,
 window_transmittance:f64,
});
state_patch!(StateRoomAirModelAssignmentPatch,state_room_air_model_assignment_fields for StateRoomAirModelAssignment {
 model:SealedShared<RoomAirModelType>,
});
state_patch!(StateGroundTemperatureConfigPatch,state_ground_temperature_config_fields for StateGroundTemperatureConfig {
 building_surface_c:SealedShared<[f64; 12]>,
 shallow_c:SealedShared<[f64; 12]>,
 deep_c:f64,
});
state_patch!(StateInfiltrationPatch,state_infiltration_fields for StateInfiltration {
 zone_id:EntityId,
 schedule_id:ScheduleId,
 method:SealedShared<crate::air_exchange::InfiltrationMethod>,
 design_flow_ach:f64,
 flow_per_exterior_area_m3_s_m2:f64,
 effective_leakage_area_m2:f64,
 discharge_coefficient:f64,
 stack_height_m:f64,
 constant_term_coefficient:f64,
 temperature_term_coefficient:f64,
 velocity_term_coefficient:f64,
 velocity_squared_term_coefficient:f64,
});
state_patch!(StateScheduleLimitsPatch,state_schedule_limits_fields for StateScheduleLimits {
 min:f64,
 max:f64,
});
state_patch!(StateConstantSchedulePatch,state_constant_schedule_fields for StateConstantSchedule {
 value:f64,
});
state_patch!(StateDailySchedulePatch,state_daily_schedule_fields for StateDailySchedule {
 hourly_values:SealedShared<[f64; 24]>,
 interpolation:SealedShared<crate::schedule::ScheduleInterpolation>,
 limits:SealedShared<Option<crate::schedule::ScheduleLimits>>,
});
state_patch!(StateWeeklySchedulePatch,state_weekly_schedule_fields for StateWeeklySchedule {
 daily_schedule_ids:SealedShared<[ScheduleId; 7]>,
});
state_patch!(StateCompactScheduleRulePatch,state_compact_schedule_rule_fields for StateCompactScheduleRule {
 start_month:u8,
 start_day:u8,
 end_month:u8,
 end_day:u8,
 daily_schedule_id:ScheduleId,
});
state_patch!(StateAnnualSchedulePatch,state_annual_schedule_fields for StateAnnualSchedule {
 rules:SealedShared<Vec<crate::schedule::CompactScheduleRule>>,
 default_daily_schedule_id:ScheduleId,
 holiday_daily_schedule_id:SealedShared<Option<ScheduleId>>,
 holiday_dates:SealedShared<Vec<(u16, u8, u8)>>,
});
state_patch!(StateTimeSeriesSchedulePatch,state_time_series_schedule_fields for StateTimeSeriesSchedule {
 values:SealedShared<Vec<f64>>,
 timestep_seconds:u32,
});

#[path="🚚️apply/🦀️.rs"]mod apply;
pub use apply::StateZoneApply;

/// 🌳️ Prepared branch deltas retain the original immutable candidate produced by sparse path staging.
#[derive(Default,Debug,PartialEq,ToValue,semio_framework_value::RetireOwned)]
pub struct EnergyModelStatePatch{
 #[value(skip_serializing_if="Option::is_none")]pub identity:Option<SealedShared<ModelIdentityState>>,
 #[value(skip_serializing_if="Option::is_none")]pub geometry:Option<SealedShared<ModelGeometryState>>,
 #[value(skip_serializing_if="Option::is_none")]pub materials:Option<SealedShared<ModelMaterialState>>,
 #[value(skip_serializing_if="Option::is_none")]pub loads:Option<SealedShared<ModelLoadState>>,
 #[value(skip_serializing_if="Option::is_none")]pub controls:Option<SealedShared<ModelControlState>>,
 #[value(skip_serializing_if="Option::is_none")]pub systems:Option<SealedShared<ModelSystemState>>,
 #[value(skip_serializing_if="Option::is_none")]pub generation:Option<SealedShared<ModelGenerationState>>,
 #[value(skip_serializing_if="Option::is_none")]pub outputs:Option<SealedShared<ModelOutputState>>,
}
protocol::retained_row_patch!(pub energy_model_fields for EnergyModelStatePatch=>EnergyModelState{identity:SealedShared<ModelIdentityState>,geometry:SealedShared<ModelGeometryState>,materials:SealedShared<ModelMaterialState>,loads:SealedShared<ModelLoadState>,controls:SealedShared<ModelControlState>,systems:SealedShared<ModelSystemState>,generation:SealedShared<ModelGenerationState>,outputs:SealedShared<ModelOutputState>});

#[path="🌳️branches/🦀️.rs"]pub mod branches;

impl EnergyModelStatePatch{pub fn is_empty(&self)->bool{self.identity.is_none()&&self.geometry.is_none()&&self.materials.is_none()&&self.loads.is_none()&&self.controls.is_none()&&self.systems.is_none()&&self.generation.is_none()&&self.outputs.is_none()}}
