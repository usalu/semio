//! 📝️ Energy-model mutation text framing — the handcrafted `OpText`/`OpBinary` pair over the
//! `dsl::DslVariants` binding `#[derive(dsl::DslEnum)]` emits for `EnergyModelMutation`. P6: the
//! derive no longer emits these traits, so they are written once here for the whole aggregate and
//! never per kind (identical shape to `📸️remodel`'s own facet).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this mutation facet.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::mutations::EnergyModelMutation;

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for EnergyModelMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for EnergyModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use crate::standards::v1::subsets::any::schema::mutations::add_air_loop_terminal_zone::{add_air_loop_terminal_zone, AddAirLoopTerminalZone};
use crate::standards::v1::subsets::any::schema::mutations::add_annual_schedule_holiday::{add_annual_schedule_holiday, AddAnnualScheduleHoliday};
use crate::standards::v1::subsets::any::schema::mutations::add_construction_layer::{add_construction_layer, AddConstructionLayer};
use crate::standards::v1::subsets::any::schema::mutations::add_electrical_load_center_battery::{add_electrical_load_center_battery, AddElectricalLoadCenterBattery};
use crate::standards::v1::subsets::any::schema::mutations::add_electrical_load_center_pv::{add_electrical_load_center_pv, AddElectricalLoadCenterPv};
use crate::standards::v1::subsets::any::schema::mutations::add_output_variable::{add_output_variable, AddOutputVariable};
use crate::standards::v1::subsets::any::schema::mutations::add_plant_loop_equipment::{add_plant_loop_equipment, AddPlantLoopEquipment};
use crate::standards::v1::subsets::any::schema::mutations::add_space_list_member::{add_space_list_member, AddSpaceListMember};
use crate::standards::v1::subsets::any::schema::mutations::add_thermal_enclosure_zone::{add_thermal_enclosure_zone, AddThermalEnclosureZone};
use crate::standards::v1::subsets::any::schema::mutations::bind_fenestration_glazing_construction::{bind_fenestration_glazing_construction, BindFenestrationGlazingConstruction};
use crate::standards::v1::subsets::any::schema::mutations::bind_weather_file::{bind_weather_file, BindWeatherFile};
use crate::standards::v1::subsets::any::schema::mutations::change_air_loop_design_supply_air_flow::{change_air_loop_design_supply_air_flow, ChangeAirLoopDesignSupplyAirFlow};
use crate::standards::v1::subsets::any::schema::mutations::change_air_loop_return_node::{change_air_loop_return_node, ChangeAirLoopReturnNode};
use crate::standards::v1::subsets::any::schema::mutations::change_air_loop_supply_node::{change_air_loop_supply_node, ChangeAirLoopSupplyNode};
use crate::standards::v1::subsets::any::schema::mutations::change_annual_schedule_default_daily_schedule::{change_annual_schedule_default_daily_schedule, ChangeAnnualScheduleDefaultDailySchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_annual_schedule_holiday_daily_schedule::{change_annual_schedule_holiday_daily_schedule, ChangeAnnualScheduleHolidayDailySchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_battery_capacity::{change_battery_capacity, ChangeBatteryCapacity};
use crate::standards::v1::subsets::any::schema::mutations::change_battery_max_charge::{change_battery_max_charge, ChangeBatteryMaxCharge};
use crate::standards::v1::subsets::any::schema::mutations::change_battery_max_discharge::{change_battery_max_discharge, ChangeBatteryMaxDischarge};
use crate::standards::v1::subsets::any::schema::mutations::change_battery_round_trip_efficiency::{change_battery_round_trip_efficiency, ChangeBatteryRoundTripEfficiency};
use crate::standards::v1::subsets::any::schema::mutations::change_constant_schedule_value::{change_constant_schedule_value, ChangeConstantScheduleValue};
use crate::standards::v1::subsets::any::schema::mutations::change_daily_schedule_interpolation::{change_daily_schedule_interpolation, ChangeDailyScheduleInterpolation};
use crate::standards::v1::subsets::any::schema::mutations::change_daily_schedule_limits::{change_daily_schedule_limits, ChangeDailyScheduleLimits};
use crate::standards::v1::subsets::any::schema::mutations::change_daylight_zone_glare_limit::{change_daylight_zone_glare_limit, ChangeDaylightZoneGlareLimit};
use crate::standards::v1::subsets::any::schema::mutations::change_daylight_zone_illuminance_target::{change_daylight_zone_illuminance_target, ChangeDaylightZoneIlluminanceTarget};
use crate::standards::v1::subsets::any::schema::mutations::change_daylight_zone_window_transmittance::{change_daylight_zone_window_transmittance, ChangeDaylightZoneWindowTransmittance};
use crate::standards::v1::subsets::any::schema::mutations::change_daylight_zone_zone::{change_daylight_zone_zone, ChangeDaylightZoneZone};
use crate::standards::v1::subsets::any::schema::mutations::change_equipment_gain_latent_fraction::{change_equipment_gain_latent_fraction, ChangeEquipmentGainLatentFraction};
use crate::standards::v1::subsets::any::schema::mutations::change_equipment_gain_radiant_fraction::{change_equipment_gain_radiant_fraction, ChangeEquipmentGainRadiantFraction};
use crate::standards::v1::subsets::any::schema::mutations::change_equipment_gain_schedule::{change_equipment_gain_schedule, ChangeEquipmentGainSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_equipment_gain_watts_per_area::{change_equipment_gain_watts_per_area, ChangeEquipmentGainWattsPerArea};
use crate::standards::v1::subsets::any::schema::mutations::change_equipment_gain_zone::{change_equipment_gain_zone, ChangeEquipmentGainZone};
use crate::standards::v1::subsets::any::schema::mutations::change_fault_severity::{change_fault_severity, ChangeFaultSeverity};
use crate::standards::v1::subsets::any::schema::mutations::change_fault_start_schedule::{change_fault_start_schedule, ChangeFaultStartSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_fault_target_equipment::{change_fault_target_equipment, ChangeFaultTargetEquipment};
use crate::standards::v1::subsets::any::schema::mutations::change_fault_type::{change_fault_type, ChangeFaultType};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_area::{change_fenestration_area, ChangeFenestrationArea};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_divider_conductance::{change_fenestration_divider_conductance, ChangeFenestrationDividerConductance};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_fin_depth::{change_fenestration_fin_depth, ChangeFenestrationFinDepth};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_fin_offset::{change_fenestration_fin_offset, ChangeFenestrationFinOffset};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_frame_conductance::{change_fenestration_frame_conductance, ChangeFenestrationFrameConductance};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_height::{change_fenestration_height, ChangeFenestrationHeight};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_overhang_depth::{change_fenestration_overhang_depth, ChangeFenestrationOverhangDepth};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_overhang_offset::{change_fenestration_overhang_offset, ChangeFenestrationOverhangOffset};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_shgc::{change_fenestration_shgc, ChangeFenestrationShgc};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_sill_height::{change_fenestration_sill_height, ChangeFenestrationSillHeight};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_surface::{change_fenestration_surface, ChangeFenestrationSurface};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_u_value::{change_fenestration_u_value, ChangeFenestrationUValue};
use crate::standards::v1::subsets::any::schema::mutations::change_fenestration_vlt::{change_fenestration_vlt, ChangeFenestrationVlt};
use crate::standards::v1::subsets::any::schema::mutations::change_humidistat_dehumidifying_setpoint_schedule::{change_humidistat_dehumidifying_setpoint_schedule, ChangeHumidistatDehumidifyingSetpointSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_humidistat_dehumidifying_throttle_range::{change_humidistat_dehumidifying_throttle_range, ChangeHumidistatDehumidifyingThrottleRange};
use crate::standards::v1::subsets::any::schema::mutations::change_humidistat_humidifying_setpoint_schedule::{change_humidistat_humidifying_setpoint_schedule, ChangeHumidistatHumidifyingSetpointSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_humidistat_humidifying_throttle_range::{change_humidistat_humidifying_throttle_range, ChangeHumidistatHumidifyingThrottleRange};
use crate::standards::v1::subsets::any::schema::mutations::change_humidistat_zone::{change_humidistat_zone, ChangeHumidistatZone};
use crate::standards::v1::subsets::any::schema::mutations::change_ideal_loads_system_max_cooling_capacity::{change_ideal_loads_system_max_cooling_capacity, ChangeIdealLoadsSystemMaxCoolingCapacity};
use crate::standards::v1::subsets::any::schema::mutations::change_ideal_loads_system_max_heating_capacity::{change_ideal_loads_system_max_heating_capacity, ChangeIdealLoadsSystemMaxHeatingCapacity};
use crate::standards::v1::subsets::any::schema::mutations::change_ideal_loads_system_max_heating_supply_air_temp::{change_ideal_loads_system_max_heating_supply_air_temp, ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp};
use crate::standards::v1::subsets::any::schema::mutations::change_ideal_loads_system_min_cooling_supply_air_temp::{change_ideal_loads_system_min_cooling_supply_air_temp, ChangeIdealLoadsSystemMinCoolingSupplyAirTemp};
use crate::standards::v1::subsets::any::schema::mutations::change_ideal_loads_system_outdoor_air_per_area::{change_ideal_loads_system_outdoor_air_per_area, ChangeIdealLoadsSystemOutdoorAirPerArea};
use crate::standards::v1::subsets::any::schema::mutations::change_ideal_loads_system_outdoor_air_per_person::{change_ideal_loads_system_outdoor_air_per_person, ChangeIdealLoadsSystemOutdoorAirPerPerson};
use crate::standards::v1::subsets::any::schema::mutations::change_ideal_loads_system_zone::{change_ideal_loads_system_zone, ChangeIdealLoadsSystemZone};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_constant_term_coefficient::{change_infiltration_constant_term_coefficient, ChangeInfiltrationConstantTermCoefficient};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_design_flow_ach::{change_infiltration_design_flow_ach, ChangeInfiltrationDesignFlowAch};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_discharge_coefficient::{change_infiltration_discharge_coefficient, ChangeInfiltrationDischargeCoefficient};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_effective_leakage_area::{change_infiltration_effective_leakage_area, ChangeInfiltrationEffectiveLeakageArea};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_flow_per_exterior_area::{change_infiltration_flow_per_exterior_area, ChangeInfiltrationFlowPerExteriorArea};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_method::{change_infiltration_method, ChangeInfiltrationMethod};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_schedule::{change_infiltration_schedule, ChangeInfiltrationSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_stack_height::{change_infiltration_stack_height, ChangeInfiltrationStackHeight};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_temperature_term_coefficient::{change_infiltration_temperature_term_coefficient, ChangeInfiltrationTemperatureTermCoefficient};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_velocity_squared_term_coefficient::{change_infiltration_velocity_squared_term_coefficient, ChangeInfiltrationVelocitySquaredTermCoefficient};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_velocity_term_coefficient::{change_infiltration_velocity_term_coefficient, ChangeInfiltrationVelocityTermCoefficient};
use crate::standards::v1::subsets::any::schema::mutations::change_infiltration_zone::{change_infiltration_zone, ChangeInfiltrationZone};
use crate::standards::v1::subsets::any::schema::mutations::change_lighting_gain_radiant_fraction::{change_lighting_gain_radiant_fraction, ChangeLightingGainRadiantFraction};
use crate::standards::v1::subsets::any::schema::mutations::change_lighting_gain_return_air_fraction::{change_lighting_gain_return_air_fraction, ChangeLightingGainReturnAirFraction};
use crate::standards::v1::subsets::any::schema::mutations::change_lighting_gain_schedule::{change_lighting_gain_schedule, ChangeLightingGainSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_lighting_gain_visible_fraction::{change_lighting_gain_visible_fraction, ChangeLightingGainVisibleFraction};
use crate::standards::v1::subsets::any::schema::mutations::change_lighting_gain_watts_per_area::{change_lighting_gain_watts_per_area, ChangeLightingGainWattsPerArea};
use crate::standards::v1::subsets::any::schema::mutations::change_lighting_gain_zone::{change_lighting_gain_zone, ChangeLightingGainZone};
use crate::standards::v1::subsets::any::schema::mutations::change_material_conductivity::{change_material_conductivity, ChangeMaterialConductivity};
use crate::standards::v1::subsets::any::schema::mutations::change_material_density::{change_material_density, ChangeMaterialDensity};
use crate::standards::v1::subsets::any::schema::mutations::change_material_solar_absorptance::{change_material_solar_absorptance, ChangeMaterialSolarAbsorptance};
use crate::standards::v1::subsets::any::schema::mutations::change_material_specific_heat::{change_material_specific_heat, ChangeMaterialSpecificHeat};
use crate::standards::v1::subsets::any::schema::mutations::change_material_thermal_absorptance::{change_material_thermal_absorptance, ChangeMaterialThermalAbsorptance};
use crate::standards::v1::subsets::any::schema::mutations::change_material_thickness::{change_material_thickness, ChangeMaterialThickness};
use crate::standards::v1::subsets::any::schema::mutations::change_material_visible_absorptance::{change_material_visible_absorptance, ChangeMaterialVisibleAbsorptance};
use crate::standards::v1::subsets::any::schema::mutations::change_mechanical_ventilation_design_flow::{change_mechanical_ventilation_design_flow, ChangeMechanicalVentilationDesignFlow};
use crate::standards::v1::subsets::any::schema::mutations::change_mechanical_ventilation_fan_delta_pressure::{change_mechanical_ventilation_fan_delta_pressure, ChangeMechanicalVentilationFanDeltaPressure};
use crate::standards::v1::subsets::any::schema::mutations::change_mechanical_ventilation_fan_total_efficiency::{change_mechanical_ventilation_fan_total_efficiency, ChangeMechanicalVentilationFanTotalEfficiency};
use crate::standards::v1::subsets::any::schema::mutations::change_mechanical_ventilation_schedule::{change_mechanical_ventilation_schedule, ChangeMechanicalVentilationSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_mechanical_ventilation_zone::{change_mechanical_ventilation_zone, ChangeMechanicalVentilationZone};
use crate::standards::v1::subsets::any::schema::mutations::change_model_version::{change_model_version, ChangeModelVersion};
use crate::standards::v1::subsets::any::schema::mutations::change_outdoor_air_system_air_loop::{change_outdoor_air_system_air_loop, ChangeOutdoorAirSystemAirLoop};
use crate::standards::v1::subsets::any::schema::mutations::change_outdoor_air_system_economizer_enabled::{change_outdoor_air_system_economizer_enabled, ChangeOutdoorAirSystemEconomizerEnabled};
use crate::standards::v1::subsets::any::schema::mutations::change_outdoor_air_system_min_oa_flow::{change_outdoor_air_system_min_oa_flow, ChangeOutdoorAirSystemMinOaFlow};
use crate::standards::v1::subsets::any::schema::mutations::change_people_gain_activity_schedule::{change_people_gain_activity_schedule, ChangePeopleGainActivitySchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_people_gain_latent_fraction::{change_people_gain_latent_fraction, ChangePeopleGainLatentFraction};
use crate::standards::v1::subsets::any::schema::mutations::change_people_gain_people_per_area::{change_people_gain_people_per_area, ChangePeopleGainPeoplePerArea};
use crate::standards::v1::subsets::any::schema::mutations::change_people_gain_radiant_fraction::{change_people_gain_radiant_fraction, ChangePeopleGainRadiantFraction};
use crate::standards::v1::subsets::any::schema::mutations::change_people_gain_schedule::{change_people_gain_schedule, ChangePeopleGainSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_people_gain_sensible_fraction::{change_people_gain_sensible_fraction, ChangePeopleGainSensibleFraction};
use crate::standards::v1::subsets::any::schema::mutations::change_people_gain_zone::{change_people_gain_zone, ChangePeopleGainZone};
use crate::standards::v1::subsets::any::schema::mutations::change_plant_loop_design_flow::{change_plant_loop_design_flow, ChangePlantLoopDesignFlow};
use crate::standards::v1::subsets::any::schema::mutations::change_plant_loop_return_temperature::{change_plant_loop_return_temperature, ChangePlantLoopReturnTemperature};
use crate::standards::v1::subsets::any::schema::mutations::change_plant_loop_supply_temperature::{change_plant_loop_supply_temperature, ChangePlantLoopSupplyTemperature};
use crate::standards::v1::subsets::any::schema::mutations::change_plant_loop_type::{change_plant_loop_type, ChangePlantLoopType};
use crate::standards::v1::subsets::any::schema::mutations::change_pv_system_area::{change_pv_system_area, ChangePvSystemArea};
use crate::standards::v1::subsets::any::schema::mutations::change_pv_system_azimuth::{change_pv_system_azimuth, ChangePvSystemAzimuth};
use crate::standards::v1::subsets::any::schema::mutations::change_pv_system_dc_capacity::{change_pv_system_dc_capacity, ChangePvSystemDcCapacity};
use crate::standards::v1::subsets::any::schema::mutations::change_pv_system_inverter_efficiency::{change_pv_system_inverter_efficiency, ChangePvSystemInverterEfficiency};
use crate::standards::v1::subsets::any::schema::mutations::change_pv_system_module_efficiency::{change_pv_system_module_efficiency, ChangePvSystemModuleEfficiency};
use crate::standards::v1::subsets::any::schema::mutations::change_pv_system_tilt::{change_pv_system_tilt, ChangePvSystemTilt};
use crate::standards::v1::subsets::any::schema::mutations::change_refrigeration_system_case_count::{change_refrigeration_system_case_count, ChangeRefrigerationSystemCaseCount};
use crate::standards::v1::subsets::any::schema::mutations::change_refrigeration_system_defrost_schedule::{change_refrigeration_system_defrost_schedule, ChangeRefrigerationSystemDefrostSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_refrigeration_system_design_load::{change_refrigeration_system_design_load, ChangeRefrigerationSystemDesignLoad};
use crate::standards::v1::subsets::any::schema::mutations::change_room_air_model::{change_room_air_model, ChangeRoomAirModel};
use crate::standards::v1::subsets::any::schema::mutations::change_setpoint_manager_schedule::{change_setpoint_manager_schedule, ChangeSetpointManagerSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_shading_surface_transmittance_schedule::{change_shading_surface_transmittance_schedule, ChangeShadingSurfaceTransmittanceSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_shw_system_heater_capacity::{change_shw_system_heater_capacity, ChangeShwSystemHeaterCapacity};
use crate::standards::v1::subsets::any::schema::mutations::change_shw_system_schedule::{change_shw_system_schedule, ChangeShwSystemSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_shw_system_setpoint::{change_shw_system_setpoint, ChangeShwSystemSetpoint};
use crate::standards::v1::subsets::any::schema::mutations::change_shw_system_storage_volume::{change_shw_system_storage_volume, ChangeShwSystemStorageVolume};
use crate::standards::v1::subsets::any::schema::mutations::change_sizing_object_design_day_type::{change_sizing_object_design_day_type, ChangeSizingObjectDesignDayType};
use crate::standards::v1::subsets::any::schema::mutations::change_sizing_object_sizing_type::{change_sizing_object_sizing_type, ChangeSizingObjectSizingType};
use crate::standards::v1::subsets::any::schema::mutations::change_sizing_object_zone::{change_sizing_object_zone, ChangeSizingObjectZone};
use crate::standards::v1::subsets::any::schema::mutations::change_solar_thermal_system_azimuth::{change_solar_thermal_system_azimuth, ChangeSolarThermalSystemAzimuth};
use crate::standards::v1::subsets::any::schema::mutations::change_solar_thermal_system_collector_area::{change_solar_thermal_system_collector_area, ChangeSolarThermalSystemCollectorArea};
use crate::standards::v1::subsets::any::schema::mutations::change_solar_thermal_system_efficiency::{change_solar_thermal_system_efficiency, ChangeSolarThermalSystemEfficiency};
use crate::standards::v1::subsets::any::schema::mutations::change_solar_thermal_system_storage_volume::{change_solar_thermal_system_storage_volume, ChangeSolarThermalSystemStorageVolume};
use crate::standards::v1::subsets::any::schema::mutations::change_solar_thermal_system_tilt::{change_solar_thermal_system_tilt, ChangeSolarThermalSystemTilt};
use crate::standards::v1::subsets::any::schema::mutations::change_space_floor_area::{change_space_floor_area, ChangeSpaceFloorArea};
use crate::standards::v1::subsets::any::schema::mutations::change_space_zone::{change_space_zone, ChangeSpaceZone};
use crate::standards::v1::subsets::any::schema::mutations::change_surface_boundary_condition::{change_surface_boundary_condition, ChangeSurfaceBoundaryCondition};
use crate::standards::v1::subsets::any::schema::mutations::change_surface_class::{change_surface_class, ChangeSurfaceClass};
use crate::standards::v1::subsets::any::schema::mutations::change_surface_construction::{change_surface_construction, ChangeSurfaceConstruction};
use crate::standards::v1::subsets::any::schema::mutations::change_surface_multiplier::{change_surface_multiplier, ChangeSurfaceMultiplier};
use crate::standards::v1::subsets::any::schema::mutations::change_surface_sun_exposed::{change_surface_sun_exposed, ChangeSurfaceSunExposed};
use crate::standards::v1::subsets::any::schema::mutations::change_surface_wind_exposed::{change_surface_wind_exposed, ChangeSurfaceWindExposed};
use crate::standards::v1::subsets::any::schema::mutations::change_surface_zone::{change_surface_zone, ChangeSurfaceZone};
use crate::standards::v1::subsets::any::schema::mutations::change_thermostat_cooling_setpoint_schedule::{change_thermostat_cooling_setpoint_schedule, ChangeThermostatCoolingSetpointSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_thermostat_cooling_throttle_range::{change_thermostat_cooling_throttle_range, ChangeThermostatCoolingThrottleRange};
use crate::standards::v1::subsets::any::schema::mutations::change_thermostat_heating_setpoint_schedule::{change_thermostat_heating_setpoint_schedule, ChangeThermostatHeatingSetpointSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_thermostat_heating_throttle_range::{change_thermostat_heating_throttle_range, ChangeThermostatHeatingThrottleRange};
use crate::standards::v1::subsets::any::schema::mutations::change_thermostat_zone::{change_thermostat_zone, ChangeThermostatZone};
use crate::standards::v1::subsets::any::schema::mutations::change_time_series_schedule_timestep::{change_time_series_schedule_timestep, ChangeTimeSeriesScheduleTimestep};
use crate::standards::v1::subsets::any::schema::mutations::change_water_system_fixture_count::{change_water_system_fixture_count, ChangeWaterSystemFixtureCount};
use crate::standards::v1::subsets::any::schema::mutations::change_water_system_peak_flow::{change_water_system_peak_flow, ChangeWaterSystemPeakFlow};
use crate::standards::v1::subsets::any::schema::mutations::change_water_system_schedule::{change_water_system_schedule, ChangeWaterSystemSchedule};
use crate::standards::v1::subsets::any::schema::mutations::change_weekly_schedule_day::{change_weekly_schedule_day, ChangeWeeklyScheduleDay};
use crate::standards::v1::subsets::any::schema::mutations::change_zone_conditioned::{change_zone_conditioned, ChangeZoneConditioned};
use crate::standards::v1::subsets::any::schema::mutations::change_zone_equipment_cooling_capacity::{change_zone_equipment_cooling_capacity, ChangeZoneEquipmentCoolingCapacity};
use crate::standards::v1::subsets::any::schema::mutations::change_zone_equipment_heating_capacity::{change_zone_equipment_heating_capacity, ChangeZoneEquipmentHeatingCapacity};
use crate::standards::v1::subsets::any::schema::mutations::change_zone_equipment_priority::{change_zone_equipment_priority, ChangeZoneEquipmentPriority};
use crate::standards::v1::subsets::any::schema::mutations::change_zone_equipment_type::{change_zone_equipment_type, ChangeZoneEquipmentType};
use crate::standards::v1::subsets::any::schema::mutations::change_zone_equipment_zone::{change_zone_equipment_zone, ChangeZoneEquipmentZone};
use crate::standards::v1::subsets::any::schema::mutations::change_zone_floor_area_participation::{change_zone_floor_area_participation, ChangeZoneFloorAreaParticipation};
use crate::standards::v1::subsets::any::schema::mutations::change_zone_multiplier::{change_zone_multiplier, ChangeZoneMultiplier};
use crate::standards::v1::subsets::any::schema::mutations::change_zone_volume::{change_zone_volume, ChangeZoneVolume};
use crate::standards::v1::subsets::any::schema::mutations::clear_fenestration_glazing_construction::{clear_fenestration_glazing_construction, ClearFenestrationGlazingConstruction};
use crate::standards::v1::subsets::any::schema::mutations::connect_referenced_model::{connect_referenced_model, ConnectReferencedModel};
use crate::standards::v1::subsets::any::schema::mutations::connect_surfaces::{connect_surfaces, ConnectSurfaces};
use crate::standards::v1::subsets::any::schema::mutations::create_air_loop::{create_air_loop, CreateAirLoop};
use crate::standards::v1::subsets::any::schema::mutations::create_annual_schedule::{create_annual_schedule, CreateAnnualSchedule};
use crate::standards::v1::subsets::any::schema::mutations::create_battery::{create_battery, CreateBattery};
use crate::standards::v1::subsets::any::schema::mutations::create_constant_schedule::{create_constant_schedule, CreateConstantSchedule};
use crate::standards::v1::subsets::any::schema::mutations::create_construction::{create_construction, CreateConstruction};
use crate::standards::v1::subsets::any::schema::mutations::create_daily_schedule::{create_daily_schedule, CreateDailySchedule};
use crate::standards::v1::subsets::any::schema::mutations::create_daylight_zone::{create_daylight_zone, CreateDaylightZone};
use crate::standards::v1::subsets::any::schema::mutations::create_electrical_load_center::{create_electrical_load_center, CreateElectricalLoadCenter};
use crate::standards::v1::subsets::any::schema::mutations::create_equipment_gain::{create_equipment_gain, CreateEquipmentGain};
use crate::standards::v1::subsets::any::schema::mutations::create_fault::{create_fault, CreateFault};
use crate::standards::v1::subsets::any::schema::mutations::create_fenestration::{create_fenestration, CreateFenestration};
use crate::standards::v1::subsets::any::schema::mutations::create_humidistat::{create_humidistat, CreateHumidistat};
use crate::standards::v1::subsets::any::schema::mutations::create_ideal_loads_system::{create_ideal_loads_system, CreateIdealLoadsSystem};
use crate::standards::v1::subsets::any::schema::mutations::create_infiltration::{create_infiltration, CreateInfiltration};
use crate::standards::v1::subsets::any::schema::mutations::create_lighting_gain::{create_lighting_gain, CreateLightingGain};
use crate::standards::v1::subsets::any::schema::mutations::create_material::{create_material, CreateMaterial};
use crate::standards::v1::subsets::any::schema::mutations::create_mechanical_ventilation::{create_mechanical_ventilation, CreateMechanicalVentilation};
use crate::standards::v1::subsets::any::schema::mutations::create_outdoor_air_system::{create_outdoor_air_system, CreateOutdoorAirSystem};
use crate::standards::v1::subsets::any::schema::mutations::create_people_gain::{create_people_gain, CreatePeopleGain};
use crate::standards::v1::subsets::any::schema::mutations::create_plant_loop::{create_plant_loop, CreatePlantLoop};
use crate::standards::v1::subsets::any::schema::mutations::create_pv_system::{create_pv_system, CreatePvSystem};
use crate::standards::v1::subsets::any::schema::mutations::create_refrigeration_system::{create_refrigeration_system, CreateRefrigerationSystem};
use crate::standards::v1::subsets::any::schema::mutations::create_room_air_model_assignment::{create_room_air_model_assignment, CreateRoomAirModelAssignment};
use crate::standards::v1::subsets::any::schema::mutations::create_setpoint_manager::{create_setpoint_manager, CreateSetpointManager};
use crate::standards::v1::subsets::any::schema::mutations::create_shading_surface::{create_shading_surface, CreateShadingSurface};
use crate::standards::v1::subsets::any::schema::mutations::create_shw_system::{create_shw_system, CreateShwSystem};
use crate::standards::v1::subsets::any::schema::mutations::create_sizing_object::{create_sizing_object, CreateSizingObject};
use crate::standards::v1::subsets::any::schema::mutations::create_solar_thermal_system::{create_solar_thermal_system, CreateSolarThermalSystem};
use crate::standards::v1::subsets::any::schema::mutations::create_space::{create_space, CreateSpace};
use crate::standards::v1::subsets::any::schema::mutations::create_space_list::{create_space_list, CreateSpaceList};
use crate::standards::v1::subsets::any::schema::mutations::create_surface::{create_surface, CreateSurface};
use crate::standards::v1::subsets::any::schema::mutations::create_thermal_enclosure::{create_thermal_enclosure, CreateThermalEnclosure};
use crate::standards::v1::subsets::any::schema::mutations::create_thermostat::{create_thermostat, CreateThermostat};
use crate::standards::v1::subsets::any::schema::mutations::create_time_series_schedule::{create_time_series_schedule, CreateTimeSeriesSchedule};
use crate::standards::v1::subsets::any::schema::mutations::create_water_system::{create_water_system, CreateWaterSystem};
use crate::standards::v1::subsets::any::schema::mutations::create_weekly_schedule::{create_weekly_schedule, CreateWeeklySchedule};
use crate::standards::v1::subsets::any::schema::mutations::create_zone::{create_zone, CreateZone};
use crate::standards::v1::subsets::any::schema::mutations::create_zone_equipment::{create_zone_equipment, CreateZoneEquipment};
use crate::standards::v1::subsets::any::schema::mutations::delete_air_loop::{delete_air_loop, DeleteAirLoop};
use crate::standards::v1::subsets::any::schema::mutations::delete_annual_schedule::{delete_annual_schedule, DeleteAnnualSchedule};
use crate::standards::v1::subsets::any::schema::mutations::delete_battery::{delete_battery, DeleteBattery};
use crate::standards::v1::subsets::any::schema::mutations::delete_constant_schedule::{delete_constant_schedule, DeleteConstantSchedule};
use crate::standards::v1::subsets::any::schema::mutations::delete_construction::{delete_construction, DeleteConstruction};
use crate::standards::v1::subsets::any::schema::mutations::delete_daily_schedule::{delete_daily_schedule, DeleteDailySchedule};
use crate::standards::v1::subsets::any::schema::mutations::delete_daylight_zone::{delete_daylight_zone, DeleteDaylightZone};
use crate::standards::v1::subsets::any::schema::mutations::delete_electrical_load_center::{delete_electrical_load_center, DeleteElectricalLoadCenter};
use crate::standards::v1::subsets::any::schema::mutations::delete_equipment_gain::{delete_equipment_gain, DeleteEquipmentGain};
use crate::standards::v1::subsets::any::schema::mutations::delete_fault::{delete_fault, DeleteFault};
use crate::standards::v1::subsets::any::schema::mutations::delete_fenestration::{delete_fenestration, DeleteFenestration};
use crate::standards::v1::subsets::any::schema::mutations::delete_humidistat::{delete_humidistat, DeleteHumidistat};
use crate::standards::v1::subsets::any::schema::mutations::delete_ideal_loads_system::{delete_ideal_loads_system, DeleteIdealLoadsSystem};
use crate::standards::v1::subsets::any::schema::mutations::delete_infiltration::{delete_infiltration, DeleteInfiltration};
use crate::standards::v1::subsets::any::schema::mutations::delete_lighting_gain::{delete_lighting_gain, DeleteLightingGain};
use crate::standards::v1::subsets::any::schema::mutations::delete_material::{delete_material, DeleteMaterial};
use crate::standards::v1::subsets::any::schema::mutations::delete_mechanical_ventilation::{delete_mechanical_ventilation, DeleteMechanicalVentilation};
use crate::standards::v1::subsets::any::schema::mutations::delete_outdoor_air_system::{delete_outdoor_air_system, DeleteOutdoorAirSystem};
use crate::standards::v1::subsets::any::schema::mutations::delete_people_gain::{delete_people_gain, DeletePeopleGain};
use crate::standards::v1::subsets::any::schema::mutations::delete_plant_loop::{delete_plant_loop, DeletePlantLoop};
use crate::standards::v1::subsets::any::schema::mutations::delete_pv_system::{delete_pv_system, DeletePvSystem};
use crate::standards::v1::subsets::any::schema::mutations::delete_refrigeration_system::{delete_refrigeration_system, DeleteRefrigerationSystem};
use crate::standards::v1::subsets::any::schema::mutations::delete_room_air_model_assignment::{delete_room_air_model_assignment, DeleteRoomAirModelAssignment};
use crate::standards::v1::subsets::any::schema::mutations::delete_setpoint_manager::{delete_setpoint_manager, DeleteSetpointManager};
use crate::standards::v1::subsets::any::schema::mutations::delete_shading_surface::{delete_shading_surface, DeleteShadingSurface};
use crate::standards::v1::subsets::any::schema::mutations::delete_shw_system::{delete_shw_system, DeleteShwSystem};
use crate::standards::v1::subsets::any::schema::mutations::delete_sizing_object::{delete_sizing_object, DeleteSizingObject};
use crate::standards::v1::subsets::any::schema::mutations::delete_solar_thermal_system::{delete_solar_thermal_system, DeleteSolarThermalSystem};
use crate::standards::v1::subsets::any::schema::mutations::delete_space::{delete_space, DeleteSpace};
use crate::standards::v1::subsets::any::schema::mutations::delete_space_list::{delete_space_list, DeleteSpaceList};
use crate::standards::v1::subsets::any::schema::mutations::delete_surface::{delete_surface, DeleteSurface};
use crate::standards::v1::subsets::any::schema::mutations::delete_thermal_enclosure::{delete_thermal_enclosure, DeleteThermalEnclosure};
use crate::standards::v1::subsets::any::schema::mutations::delete_thermostat::{delete_thermostat, DeleteThermostat};
use crate::standards::v1::subsets::any::schema::mutations::delete_time_series_schedule::{delete_time_series_schedule, DeleteTimeSeriesSchedule};
use crate::standards::v1::subsets::any::schema::mutations::delete_water_system::{delete_water_system, DeleteWaterSystem};
use crate::standards::v1::subsets::any::schema::mutations::delete_weekly_schedule::{delete_weekly_schedule, DeleteWeeklySchedule};
use crate::standards::v1::subsets::any::schema::mutations::delete_zone::{delete_zone, DeleteZone};
use crate::standards::v1::subsets::any::schema::mutations::delete_zone_equipment::{delete_zone_equipment, DeleteZoneEquipment};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_referenced_model::{disconnect_referenced_model, DisconnectReferencedModel};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_surfaces::{disconnect_surfaces, DisconnectSurfaces};
use crate::standards::v1::subsets::any::schema::mutations::insert_annual_schedule_rule::{insert_annual_schedule_rule, InsertAnnualScheduleRule};
use crate::standards::v1::subsets::any::schema::mutations::remove_air_loop_terminal_zone::{remove_air_loop_terminal_zone, RemoveAirLoopTerminalZone};
use crate::standards::v1::subsets::any::schema::mutations::remove_annual_schedule_holiday::{remove_annual_schedule_holiday, RemoveAnnualScheduleHoliday};
use crate::standards::v1::subsets::any::schema::mutations::remove_annual_schedule_rule::{remove_annual_schedule_rule, RemoveAnnualScheduleRule};
use crate::standards::v1::subsets::any::schema::mutations::remove_construction_layer::{remove_construction_layer, RemoveConstructionLayer};
use crate::standards::v1::subsets::any::schema::mutations::remove_electrical_load_center_battery::{remove_electrical_load_center_battery, RemoveElectricalLoadCenterBattery};
use crate::standards::v1::subsets::any::schema::mutations::remove_electrical_load_center_pv::{remove_electrical_load_center_pv, RemoveElectricalLoadCenterPv};
use crate::standards::v1::subsets::any::schema::mutations::remove_output_variable::{remove_output_variable, RemoveOutputVariable};
use crate::standards::v1::subsets::any::schema::mutations::remove_plant_loop_equipment::{remove_plant_loop_equipment, RemovePlantLoopEquipment};
use crate::standards::v1::subsets::any::schema::mutations::remove_space_list_member::{remove_space_list_member, RemoveSpaceListMember};
use crate::standards::v1::subsets::any::schema::mutations::remove_thermal_enclosure_zone::{remove_thermal_enclosure_zone, RemoveThermalEnclosureZone};
use crate::standards::v1::subsets::any::schema::mutations::rename_air_loop::{rename_air_loop, RenameAirLoop};
use crate::standards::v1::subsets::any::schema::mutations::rename_construction::{rename_construction, RenameConstruction};
use crate::standards::v1::subsets::any::schema::mutations::rename_electrical_load_center::{rename_electrical_load_center, RenameElectricalLoadCenter};
use crate::standards::v1::subsets::any::schema::mutations::rename_fenestration::{rename_fenestration, RenameFenestration};
use crate::standards::v1::subsets::any::schema::mutations::rename_material::{rename_material, RenameMaterial};
use crate::standards::v1::subsets::any::schema::mutations::rename_model::{rename_model, RenameModel};
use crate::standards::v1::subsets::any::schema::mutations::rename_plant_loop::{rename_plant_loop, RenamePlantLoop};
use crate::standards::v1::subsets::any::schema::mutations::rename_setpoint_manager::{rename_setpoint_manager, RenameSetpointManager};
use crate::standards::v1::subsets::any::schema::mutations::rename_shading_surface::{rename_shading_surface, RenameShadingSurface};
use crate::standards::v1::subsets::any::schema::mutations::rename_space::{rename_space, RenameSpace};
use crate::standards::v1::subsets::any::schema::mutations::rename_space_list::{rename_space_list, RenameSpaceList};
use crate::standards::v1::subsets::any::schema::mutations::rename_surface::{rename_surface, RenameSurface};
use crate::standards::v1::subsets::any::schema::mutations::rename_thermal_enclosure::{rename_thermal_enclosure, RenameThermalEnclosure};
use crate::standards::v1::subsets::any::schema::mutations::rename_zone::{rename_zone, RenameZone};
use crate::standards::v1::subsets::any::schema::mutations::reorder_annual_schedule_rules::{reorder_annual_schedule_rules, ReorderAnnualScheduleRules};
use crate::standards::v1::subsets::any::schema::mutations::reorder_construction_layers::{reorder_construction_layers, ReorderConstructionLayers};
use crate::standards::v1::subsets::any::schema::mutations::replace_airflow_network::{replace_airflow_network, ReplaceAirflowNetwork};
use crate::standards::v1::subsets::any::schema::mutations::replace_daily_schedule_hourly_values::{replace_daily_schedule_hourly_values, ReplaceDailyScheduleHourlyValues};
use crate::standards::v1::subsets::any::schema::mutations::replace_setpoint_manager_kind::{replace_setpoint_manager_kind, ReplaceSetpointManagerKind};
use crate::standards::v1::subsets::any::schema::mutations::replace_shading_surface_vertices::{replace_shading_surface_vertices, ReplaceShadingSurfaceVertices};
use crate::standards::v1::subsets::any::schema::mutations::replace_surface_vertices::{replace_surface_vertices, ReplaceSurfaceVertices};
use crate::standards::v1::subsets::any::schema::mutations::replace_time_series_schedule_values::{replace_time_series_schedule_values, ReplaceTimeSeriesScheduleValues};
use crate::standards::v1::subsets::any::schema::mutations::unbind_weather_file::{unbind_weather_file, UnbindWeatherFile};
use crate::standards::v1::subsets::any::schema::mutations::change_site_latitude::{change_site_latitude, ChangeSiteLatitude};
use crate::standards::v1::subsets::any::schema::mutations::change_site_longitude::{change_site_longitude, ChangeSiteLongitude};
use crate::standards::v1::subsets::any::schema::mutations::change_site_elevation::{change_site_elevation, ChangeSiteElevation};
use crate::standards::v1::subsets::any::schema::mutations::change_site_time_zone::{change_site_time_zone, ChangeSiteTimeZone};
use crate::standards::v1::subsets::any::schema::mutations::change_site_north_axis::{change_site_north_axis, ChangeSiteNorthAxis};
use crate::standards::v1::subsets::any::schema::mutations::change_ground_building::{change_ground_building, ChangeGroundBuilding};
use crate::standards::v1::subsets::any::schema::mutations::change_ground_shallow::{change_ground_shallow, ChangeGroundShallow};
use crate::standards::v1::subsets::any::schema::mutations::change_ground_deep::{change_ground_deep, ChangeGroundDeep};
use crate::standards::v1::subsets::any::schema::mutations::change_run_start_month::{change_run_start_month, ChangeRunStartMonth};
use crate::standards::v1::subsets::any::schema::mutations::change_run_start_day::{change_run_start_day, ChangeRunStartDay};
use crate::standards::v1::subsets::any::schema::mutations::change_run_end_month::{change_run_end_month, ChangeRunEndMonth};
use crate::standards::v1::subsets::any::schema::mutations::change_run_end_day::{change_run_end_day, ChangeRunEndDay};
use crate::standards::v1::subsets::any::schema::mutations::change_run_year::{change_run_year, ChangeRunYear};
use crate::standards::v1::subsets::any::schema::mutations::replace_fenestration_vertices::{replace_fenestration_vertices, ReplaceFenestrationVertices};
use crate::standards::v1::subsets::any::schema::mutations::change_glazing_material_thickness::{change_glazing_material_thickness, ChangeGlazingMaterialThickness};
use crate::standards::v1::subsets::any::schema::mutations::change_glazing_material_conductivity::{change_glazing_material_conductivity, ChangeGlazingMaterialConductivity};
use crate::standards::v1::subsets::any::schema::mutations::change_glazing_material_solar_transmittance::{change_glazing_material_solar_transmittance, ChangeGlazingMaterialSolarTransmittance};
use crate::standards::v1::subsets::any::schema::mutations::change_glazing_material_visible_transmittance::{change_glazing_material_visible_transmittance, ChangeGlazingMaterialVisibleTransmittance};
use crate::standards::v1::subsets::any::schema::mutations::change_glazing_material_infrared_emissivity::{change_glazing_material_infrared_emissivity, ChangeGlazingMaterialInfraredEmissivity};
use crate::standards::v1::subsets::any::schema::mutations::rename_glazing_material::{rename_glazing_material, RenameGlazingMaterial};
use crate::standards::v1::subsets::any::schema::mutations::change_gas_material_thickness::{change_gas_material_thickness, ChangeGasMaterialThickness};
use crate::standards::v1::subsets::any::schema::mutations::change_gas_material_gas::{change_gas_material_gas, ChangeGasMaterialGas};
use crate::standards::v1::subsets::any::schema::mutations::change_material_roughness::{change_material_roughness, ChangeMaterialRoughness};
use crate::standards::v1::subsets::any::schema::mutations::rename_gas_material::{rename_gas_material, RenameGasMaterial};

/// 🔮️ Reports the forward and inverse behavior of one committed language-neutral vector.
pub fn energy_model_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    use semio_framework_value::ToValue;
    let decode_snapshot = |text: &str| -> Result<EnergyModelSnapshot, String> { semio_framework_pack_json::from_json_str::<EnergyModelSnapshot>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    // ↩️ Reversed, as the store replays an inverse (`ArtifactStore::replay_mutations`).
    for step in inverse.iter().rev() {
        let outcome = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let messages_json = semio_framework_pack_json::from_dsl_value(&forward.messages().to_value());
    let inverse_messages_json = semio_framework_pack_json::from_dsl_value(&inverse_messages.to_value());
    let report = semio_framework_pack_json::object([
        ("base".to_string(), semio_framework_pack_json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), semio_framework_pack_json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), messages_json),
        ("inverseSteps".to_string(), semio_framework_pack_json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), inverse_messages_json),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
}
pub use mutations_codec::*;
