//! 🧬️ Transparent energy-model semantic mutation aggregate. Every variant is a single-field tuple
//! wrapping a handcrafted `protocol::MutationKind` payload (the `🧬️mutations/<slug>/` leaves);
//! `#[derive(dsl::Mutations)]` generates the `protocol::Mutation`/`protocol::SemanticMutation`
//! dispatch and `#[derive(dsl::DslEnum)]` generates the `dsl::DslVariants` binding that
//! `📝️text/🦀️.rs`'s op codecs are written against — so a new kind costs one enum variant and one
//! re-export line here, never a hand-written match arm, an opcode constant or a tag registry
//! (ticket 26/09/06/ENERGY-PLUGIN-END-TO-END, adopting `📸️remodel`'s architecture).
//!
//! There is deliberately NO whole-document replace in this enum: `📓️derivation-rules.md` rule 6
//! routes file-open / import / load-example through `store::ArtifactStore::reset`, outside history.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️LinkRoles
/// 🌦️ The `weather_link` slot's role name — one constant shared by the bind/unbind pair.
pub const WEATHER_LINK_ROLE: &str = "weather";
/// 🪢️ The `referenced_model` slot's role name — one constant shared by the connect/disconnect pair.
pub const REFERENCED_MODEL_LINK_ROLE: &str = "model";
//#endregion 🔖️LinkRoles

//#region 🔖️Reexports
pub use super::rename_model::{rename_model, RenameModel};
pub use super::change_model_version::{change_model_version, ChangeModelVersion};
pub use super::update_site::{update_site, UpdateSite};
pub use super::update_ground_temperature::{update_ground_temperature, UpdateGroundTemperature};
pub use super::update_run_period::{update_run_period, UpdateRunPeriod};
pub use super::replace_airflow_network::{replace_airflow_network, ReplaceAirflowNetwork};
pub use super::add_output_variable::{add_output_variable, AddOutputVariable};
pub use super::remove_output_variable::{remove_output_variable, RemoveOutputVariable};
pub use super::bind_weather_file::{bind_weather_file, BindWeatherFile};
pub use super::unbind_weather_file::{unbind_weather_file, UnbindWeatherFile};
pub use super::connect_referenced_model::{connect_referenced_model, ConnectReferencedModel};
pub use super::disconnect_referenced_model::{disconnect_referenced_model, DisconnectReferencedModel};
pub use super::rename_zone::{rename_zone, RenameZone};
pub use super::change_zone_volume::{change_zone_volume, ChangeZoneVolume};
pub use super::change_zone_multiplier::{change_zone_multiplier, ChangeZoneMultiplier};
pub use super::change_zone_conditioned::{change_zone_conditioned, ChangeZoneConditioned};
pub use super::change_zone_floor_area_participation::{change_zone_floor_area_participation, ChangeZoneFloorAreaParticipation};
pub use super::create_zone::{create_zone, CreateZone};
pub use super::delete_zone::{delete_zone, DeleteZone};
pub use super::create_space::{create_space, CreateSpace};
pub use super::delete_space::{delete_space, DeleteSpace};
pub use super::rename_space::{rename_space, RenameSpace};
pub use super::change_space_floor_area::{change_space_floor_area, ChangeSpaceFloorArea};
pub use super::change_space_zone::{change_space_zone, ChangeSpaceZone};
pub use super::create_surface::{create_surface, CreateSurface};
pub use super::delete_surface::{delete_surface, DeleteSurface};
pub use super::rename_surface::{rename_surface, RenameSurface};
pub use super::change_surface_zone::{change_surface_zone, ChangeSurfaceZone};
pub use super::change_surface_class::{change_surface_class, ChangeSurfaceClass};
pub use super::replace_surface_vertices::{replace_surface_vertices, ReplaceSurfaceVertices};
pub use super::change_surface_construction::{change_surface_construction, ChangeSurfaceConstruction};
pub use super::change_surface_boundary_condition::{change_surface_boundary_condition, ChangeSurfaceBoundaryCondition};
pub use super::change_surface_sun_exposed::{change_surface_sun_exposed, ChangeSurfaceSunExposed};
pub use super::change_surface_wind_exposed::{change_surface_wind_exposed, ChangeSurfaceWindExposed};
pub use super::change_surface_multiplier::{change_surface_multiplier, ChangeSurfaceMultiplier};
pub use super::create_fenestration::{create_fenestration, CreateFenestration};
pub use super::delete_fenestration::{delete_fenestration, DeleteFenestration};
pub use super::rename_fenestration::{rename_fenestration, RenameFenestration};
pub use super::change_fenestration_surface::{change_fenestration_surface, ChangeFenestrationSurface};
pub use super::change_fenestration_u_value::{change_fenestration_u_value, ChangeFenestrationUValue};
pub use super::change_fenestration_shgc::{change_fenestration_shgc, ChangeFenestrationShgc};
pub use super::change_fenestration_vlt::{change_fenestration_vlt, ChangeFenestrationVlt};
pub use super::change_fenestration_area::{change_fenestration_area, ChangeFenestrationArea};
pub use super::change_fenestration_frame_conductance::{change_fenestration_frame_conductance, ChangeFenestrationFrameConductance};
pub use super::change_fenestration_divider_conductance::{change_fenestration_divider_conductance, ChangeFenestrationDividerConductance};
pub use super::create_shading_surface::{create_shading_surface, CreateShadingSurface};
pub use super::delete_shading_surface::{delete_shading_surface, DeleteShadingSurface};
pub use super::rename_shading_surface::{rename_shading_surface, RenameShadingSurface};
pub use super::replace_shading_surface_vertices::{replace_shading_surface_vertices, ReplaceShadingSurfaceVertices};
pub use super::change_shading_surface_transmittance_schedule::{change_shading_surface_transmittance_schedule, ChangeShadingSurfaceTransmittanceSchedule};
pub use super::connect_surfaces::{connect_surfaces, ConnectSurfaces};
pub use super::disconnect_surfaces::{disconnect_surfaces, DisconnectSurfaces};
pub use super::bind_fenestration_glazing_construction::{bind_fenestration_glazing_construction, BindFenestrationGlazingConstruction};
pub use super::clear_fenestration_glazing_construction::{clear_fenestration_glazing_construction, ClearFenestrationGlazingConstruction};
pub use super::change_fenestration_height::{change_fenestration_height, ChangeFenestrationHeight};
pub use super::change_fenestration_sill_height::{change_fenestration_sill_height, ChangeFenestrationSillHeight};
pub use super::change_fenestration_overhang_depth::{change_fenestration_overhang_depth, ChangeFenestrationOverhangDepth};
pub use super::change_fenestration_overhang_offset::{change_fenestration_overhang_offset, ChangeFenestrationOverhangOffset};
pub use super::change_fenestration_fin_depth::{change_fenestration_fin_depth, ChangeFenestrationFinDepth};
pub use super::change_fenestration_fin_offset::{change_fenestration_fin_offset, ChangeFenestrationFinOffset};
pub use super::create_material::{create_material, CreateMaterial};
pub use super::delete_material::{delete_material, DeleteMaterial};
pub use super::rename_material::{rename_material, RenameMaterial};
pub use super::change_material_thickness::{change_material_thickness, ChangeMaterialThickness};
pub use super::change_material_conductivity::{change_material_conductivity, ChangeMaterialConductivity};
pub use super::change_material_density::{change_material_density, ChangeMaterialDensity};
pub use super::change_material_specific_heat::{change_material_specific_heat, ChangeMaterialSpecificHeat};
pub use super::change_material_thermal_absorptance::{change_material_thermal_absorptance, ChangeMaterialThermalAbsorptance};
pub use super::change_material_solar_absorptance::{change_material_solar_absorptance, ChangeMaterialSolarAbsorptance};
pub use super::change_material_visible_absorptance::{change_material_visible_absorptance, ChangeMaterialVisibleAbsorptance};
pub use super::create_construction::{create_construction, CreateConstruction};
pub use super::delete_construction::{delete_construction, DeleteConstruction};
pub use super::rename_construction::{rename_construction, RenameConstruction};
pub use super::add_construction_layer::{add_construction_layer, AddConstructionLayer};
pub use super::remove_construction_layer::{remove_construction_layer, RemoveConstructionLayer};
pub use super::reorder_construction_layers::{reorder_construction_layers, ReorderConstructionLayers};
pub use super::create_people_gain::{create_people_gain, CreatePeopleGain};
pub use super::delete_people_gain::{delete_people_gain, DeletePeopleGain};
pub use super::change_people_gain_zone::{change_people_gain_zone, ChangePeopleGainZone};
pub use super::change_people_gain_schedule::{change_people_gain_schedule, ChangePeopleGainSchedule};
pub use super::change_people_gain_activity_schedule::{change_people_gain_activity_schedule, ChangePeopleGainActivitySchedule};
pub use super::change_people_gain_people_per_area::{change_people_gain_people_per_area, ChangePeopleGainPeoplePerArea};
pub use super::change_people_gain_sensible_fraction::{change_people_gain_sensible_fraction, ChangePeopleGainSensibleFraction};
pub use super::change_people_gain_latent_fraction::{change_people_gain_latent_fraction, ChangePeopleGainLatentFraction};
pub use super::change_people_gain_radiant_fraction::{change_people_gain_radiant_fraction, ChangePeopleGainRadiantFraction};
pub use super::create_lighting_gain::{create_lighting_gain, CreateLightingGain};
pub use super::delete_lighting_gain::{delete_lighting_gain, DeleteLightingGain};
pub use super::change_lighting_gain_zone::{change_lighting_gain_zone, ChangeLightingGainZone};
pub use super::change_lighting_gain_schedule::{change_lighting_gain_schedule, ChangeLightingGainSchedule};
pub use super::change_lighting_gain_watts_per_area::{change_lighting_gain_watts_per_area, ChangeLightingGainWattsPerArea};
pub use super::change_lighting_gain_radiant_fraction::{change_lighting_gain_radiant_fraction, ChangeLightingGainRadiantFraction};
pub use super::change_lighting_gain_visible_fraction::{change_lighting_gain_visible_fraction, ChangeLightingGainVisibleFraction};
pub use super::change_lighting_gain_return_air_fraction::{change_lighting_gain_return_air_fraction, ChangeLightingGainReturnAirFraction};
pub use super::create_equipment_gain::{create_equipment_gain, CreateEquipmentGain};
pub use super::delete_equipment_gain::{delete_equipment_gain, DeleteEquipmentGain};
pub use super::change_equipment_gain_zone::{change_equipment_gain_zone, ChangeEquipmentGainZone};
pub use super::change_equipment_gain_schedule::{change_equipment_gain_schedule, ChangeEquipmentGainSchedule};
pub use super::change_equipment_gain_watts_per_area::{change_equipment_gain_watts_per_area, ChangeEquipmentGainWattsPerArea};
pub use super::change_equipment_gain_radiant_fraction::{change_equipment_gain_radiant_fraction, ChangeEquipmentGainRadiantFraction};
pub use super::change_equipment_gain_latent_fraction::{change_equipment_gain_latent_fraction, ChangeEquipmentGainLatentFraction};
pub use super::create_infiltration::{create_infiltration, CreateInfiltration};
pub use super::delete_infiltration::{delete_infiltration, DeleteInfiltration};
pub use super::change_infiltration_zone::{change_infiltration_zone, ChangeInfiltrationZone};
pub use super::change_infiltration_schedule::{change_infiltration_schedule, ChangeInfiltrationSchedule};
pub use super::change_infiltration_flow_per_exterior_area::{change_infiltration_flow_per_exterior_area, ChangeInfiltrationFlowPerExteriorArea};
pub use super::change_infiltration_constant_term_coefficient::{change_infiltration_constant_term_coefficient, ChangeInfiltrationConstantTermCoefficient};
pub use super::change_infiltration_temperature_term_coefficient::{change_infiltration_temperature_term_coefficient, ChangeInfiltrationTemperatureTermCoefficient};
pub use super::change_infiltration_velocity_term_coefficient::{change_infiltration_velocity_term_coefficient, ChangeInfiltrationVelocityTermCoefficient};
pub use super::change_infiltration_velocity_squared_term_coefficient::{change_infiltration_velocity_squared_term_coefficient, ChangeInfiltrationVelocitySquaredTermCoefficient};
pub use super::create_mechanical_ventilation::{create_mechanical_ventilation, CreateMechanicalVentilation};
pub use super::delete_mechanical_ventilation::{delete_mechanical_ventilation, DeleteMechanicalVentilation};
pub use super::change_mechanical_ventilation_zone::{change_mechanical_ventilation_zone, ChangeMechanicalVentilationZone};
pub use super::change_mechanical_ventilation_schedule::{change_mechanical_ventilation_schedule, ChangeMechanicalVentilationSchedule};
pub use super::change_mechanical_ventilation_design_flow::{change_mechanical_ventilation_design_flow, ChangeMechanicalVentilationDesignFlow};
pub use super::change_mechanical_ventilation_fan_total_efficiency::{change_mechanical_ventilation_fan_total_efficiency, ChangeMechanicalVentilationFanTotalEfficiency};
pub use super::change_mechanical_ventilation_fan_delta_pressure::{change_mechanical_ventilation_fan_delta_pressure, ChangeMechanicalVentilationFanDeltaPressure};
pub use super::change_infiltration_method::{change_infiltration_method, ChangeInfiltrationMethod};
pub use super::change_infiltration_design_flow_ach::{change_infiltration_design_flow_ach, ChangeInfiltrationDesignFlowAch};
pub use super::change_infiltration_effective_leakage_area::{change_infiltration_effective_leakage_area, ChangeInfiltrationEffectiveLeakageArea};
pub use super::change_infiltration_discharge_coefficient::{change_infiltration_discharge_coefficient, ChangeInfiltrationDischargeCoefficient};
pub use super::change_infiltration_stack_height::{change_infiltration_stack_height, ChangeInfiltrationStackHeight};
pub use super::create_thermostat::{create_thermostat, CreateThermostat};
pub use super::delete_thermostat::{delete_thermostat, DeleteThermostat};
pub use super::change_thermostat_zone::{change_thermostat_zone, ChangeThermostatZone};
pub use super::change_thermostat_heating_setpoint_schedule::{change_thermostat_heating_setpoint_schedule, ChangeThermostatHeatingSetpointSchedule};
pub use super::change_thermostat_cooling_setpoint_schedule::{change_thermostat_cooling_setpoint_schedule, ChangeThermostatCoolingSetpointSchedule};
pub use super::change_thermostat_heating_throttle_range::{change_thermostat_heating_throttle_range, ChangeThermostatHeatingThrottleRange};
pub use super::change_thermostat_cooling_throttle_range::{change_thermostat_cooling_throttle_range, ChangeThermostatCoolingThrottleRange};
pub use super::create_humidistat::{create_humidistat, CreateHumidistat};
pub use super::delete_humidistat::{delete_humidistat, DeleteHumidistat};
pub use super::change_humidistat_zone::{change_humidistat_zone, ChangeHumidistatZone};
pub use super::change_humidistat_humidifying_setpoint_schedule::{change_humidistat_humidifying_setpoint_schedule, ChangeHumidistatHumidifyingSetpointSchedule};
pub use super::change_humidistat_dehumidifying_setpoint_schedule::{change_humidistat_dehumidifying_setpoint_schedule, ChangeHumidistatDehumidifyingSetpointSchedule};
pub use super::change_humidistat_humidifying_throttle_range::{change_humidistat_humidifying_throttle_range, ChangeHumidistatHumidifyingThrottleRange};
pub use super::change_humidistat_dehumidifying_throttle_range::{change_humidistat_dehumidifying_throttle_range, ChangeHumidistatDehumidifyingThrottleRange};
pub use super::create_ideal_loads_system::{create_ideal_loads_system, CreateIdealLoadsSystem};
pub use super::delete_ideal_loads_system::{delete_ideal_loads_system, DeleteIdealLoadsSystem};
pub use super::change_ideal_loads_system_zone::{change_ideal_loads_system_zone, ChangeIdealLoadsSystemZone};
pub use super::change_ideal_loads_system_max_heating_supply_air_temp::{change_ideal_loads_system_max_heating_supply_air_temp, ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp};
pub use super::change_ideal_loads_system_min_cooling_supply_air_temp::{change_ideal_loads_system_min_cooling_supply_air_temp, ChangeIdealLoadsSystemMinCoolingSupplyAirTemp};
pub use super::change_ideal_loads_system_max_heating_capacity::{change_ideal_loads_system_max_heating_capacity, ChangeIdealLoadsSystemMaxHeatingCapacity};
pub use super::change_ideal_loads_system_max_cooling_capacity::{change_ideal_loads_system_max_cooling_capacity, ChangeIdealLoadsSystemMaxCoolingCapacity};
pub use super::change_ideal_loads_system_outdoor_air_per_person::{change_ideal_loads_system_outdoor_air_per_person, ChangeIdealLoadsSystemOutdoorAirPerPerson};
pub use super::change_ideal_loads_system_outdoor_air_per_area::{change_ideal_loads_system_outdoor_air_per_area, ChangeIdealLoadsSystemOutdoorAirPerArea};
pub use super::create_zone_equipment::{create_zone_equipment, CreateZoneEquipment};
pub use super::delete_zone_equipment::{delete_zone_equipment, DeleteZoneEquipment};
pub use super::change_zone_equipment_zone::{change_zone_equipment_zone, ChangeZoneEquipmentZone};
pub use super::change_zone_equipment_type::{change_zone_equipment_type, ChangeZoneEquipmentType};
pub use super::change_zone_equipment_priority::{change_zone_equipment_priority, ChangeZoneEquipmentPriority};
pub use super::change_zone_equipment_heating_capacity::{change_zone_equipment_heating_capacity, ChangeZoneEquipmentHeatingCapacity};
pub use super::change_zone_equipment_cooling_capacity::{change_zone_equipment_cooling_capacity, ChangeZoneEquipmentCoolingCapacity};
pub use super::create_daylight_zone::{create_daylight_zone, CreateDaylightZone};
pub use super::delete_daylight_zone::{delete_daylight_zone, DeleteDaylightZone};
pub use super::change_daylight_zone_zone::{change_daylight_zone_zone, ChangeDaylightZoneZone};
pub use super::change_daylight_zone_illuminance_target::{change_daylight_zone_illuminance_target, ChangeDaylightZoneIlluminanceTarget};
pub use super::change_daylight_zone_glare_limit::{change_daylight_zone_glare_limit, ChangeDaylightZoneGlareLimit};
pub use super::change_daylight_zone_window_transmittance::{change_daylight_zone_window_transmittance, ChangeDaylightZoneWindowTransmittance};
pub use super::create_sizing_object::{create_sizing_object, CreateSizingObject};
pub use super::delete_sizing_object::{delete_sizing_object, DeleteSizingObject};
pub use super::change_sizing_object_zone::{change_sizing_object_zone, ChangeSizingObjectZone};
pub use super::change_sizing_object_sizing_type::{change_sizing_object_sizing_type, ChangeSizingObjectSizingType};
pub use super::change_sizing_object_design_day_type::{change_sizing_object_design_day_type, ChangeSizingObjectDesignDayType};
pub use super::create_room_air_model_assignment::{create_room_air_model_assignment, CreateRoomAirModelAssignment};
pub use super::delete_room_air_model_assignment::{delete_room_air_model_assignment, DeleteRoomAirModelAssignment};
pub use super::change_room_air_model::{change_room_air_model, ChangeRoomAirModel};
pub use super::create_setpoint_manager::{create_setpoint_manager, CreateSetpointManager};
pub use super::delete_setpoint_manager::{delete_setpoint_manager, DeleteSetpointManager};
pub use super::rename_setpoint_manager::{rename_setpoint_manager, RenameSetpointManager};
pub use super::replace_setpoint_manager_kind::{replace_setpoint_manager_kind, ReplaceSetpointManagerKind};
pub use super::change_setpoint_manager_schedule::{change_setpoint_manager_schedule, ChangeSetpointManagerSchedule};
pub use super::create_air_loop::{create_air_loop, CreateAirLoop};
pub use super::delete_air_loop::{delete_air_loop, DeleteAirLoop};
pub use super::rename_air_loop::{rename_air_loop, RenameAirLoop};
pub use super::change_air_loop_supply_node::{change_air_loop_supply_node, ChangeAirLoopSupplyNode};
pub use super::change_air_loop_return_node::{change_air_loop_return_node, ChangeAirLoopReturnNode};
pub use super::change_air_loop_design_supply_air_flow::{change_air_loop_design_supply_air_flow, ChangeAirLoopDesignSupplyAirFlow};
pub use super::add_air_loop_terminal_zone::{add_air_loop_terminal_zone, AddAirLoopTerminalZone};
pub use super::remove_air_loop_terminal_zone::{remove_air_loop_terminal_zone, RemoveAirLoopTerminalZone};
pub use super::create_plant_loop::{create_plant_loop, CreatePlantLoop};
pub use super::delete_plant_loop::{delete_plant_loop, DeletePlantLoop};
pub use super::rename_plant_loop::{rename_plant_loop, RenamePlantLoop};
pub use super::change_plant_loop_type::{change_plant_loop_type, ChangePlantLoopType};
pub use super::change_plant_loop_supply_temperature::{change_plant_loop_supply_temperature, ChangePlantLoopSupplyTemperature};
pub use super::change_plant_loop_return_temperature::{change_plant_loop_return_temperature, ChangePlantLoopReturnTemperature};
pub use super::change_plant_loop_design_flow::{change_plant_loop_design_flow, ChangePlantLoopDesignFlow};
pub use super::add_plant_loop_equipment::{add_plant_loop_equipment, AddPlantLoopEquipment};
pub use super::remove_plant_loop_equipment::{remove_plant_loop_equipment, RemovePlantLoopEquipment};
pub use super::create_outdoor_air_system::{create_outdoor_air_system, CreateOutdoorAirSystem};
pub use super::delete_outdoor_air_system::{delete_outdoor_air_system, DeleteOutdoorAirSystem};
pub use super::change_outdoor_air_system_air_loop::{change_outdoor_air_system_air_loop, ChangeOutdoorAirSystemAirLoop};
pub use super::change_outdoor_air_system_min_oa_flow::{change_outdoor_air_system_min_oa_flow, ChangeOutdoorAirSystemMinOaFlow};
pub use super::change_outdoor_air_system_economizer_enabled::{change_outdoor_air_system_economizer_enabled, ChangeOutdoorAirSystemEconomizerEnabled};
pub use super::create_electrical_load_center::{create_electrical_load_center, CreateElectricalLoadCenter};
pub use super::delete_electrical_load_center::{delete_electrical_load_center, DeleteElectricalLoadCenter};
pub use super::rename_electrical_load_center::{rename_electrical_load_center, RenameElectricalLoadCenter};
pub use super::add_electrical_load_center_pv::{add_electrical_load_center_pv, AddElectricalLoadCenterPv};
pub use super::remove_electrical_load_center_pv::{remove_electrical_load_center_pv, RemoveElectricalLoadCenterPv};
pub use super::add_electrical_load_center_battery::{add_electrical_load_center_battery, AddElectricalLoadCenterBattery};
pub use super::remove_electrical_load_center_battery::{remove_electrical_load_center_battery, RemoveElectricalLoadCenterBattery};
pub use super::create_pv_system::{create_pv_system, CreatePvSystem};
pub use super::delete_pv_system::{delete_pv_system, DeletePvSystem};
pub use super::change_pv_system_dc_capacity::{change_pv_system_dc_capacity, ChangePvSystemDcCapacity};
pub use super::change_pv_system_area::{change_pv_system_area, ChangePvSystemArea};
pub use super::change_pv_system_tilt::{change_pv_system_tilt, ChangePvSystemTilt};
pub use super::change_pv_system_azimuth::{change_pv_system_azimuth, ChangePvSystemAzimuth};
pub use super::change_pv_system_module_efficiency::{change_pv_system_module_efficiency, ChangePvSystemModuleEfficiency};
pub use super::change_pv_system_inverter_efficiency::{change_pv_system_inverter_efficiency, ChangePvSystemInverterEfficiency};
pub use super::create_battery::{create_battery, CreateBattery};
pub use super::delete_battery::{delete_battery, DeleteBattery};
pub use super::change_battery_capacity::{change_battery_capacity, ChangeBatteryCapacity};
pub use super::change_battery_max_charge::{change_battery_max_charge, ChangeBatteryMaxCharge};
pub use super::change_battery_max_discharge::{change_battery_max_discharge, ChangeBatteryMaxDischarge};
pub use super::change_battery_round_trip_efficiency::{change_battery_round_trip_efficiency, ChangeBatteryRoundTripEfficiency};
pub use super::create_shw_system::{create_shw_system, CreateShwSystem};
pub use super::delete_shw_system::{delete_shw_system, DeleteShwSystem};
pub use super::change_shw_system_heater_capacity::{change_shw_system_heater_capacity, ChangeShwSystemHeaterCapacity};
pub use super::change_shw_system_storage_volume::{change_shw_system_storage_volume, ChangeShwSystemStorageVolume};
pub use super::change_shw_system_setpoint::{change_shw_system_setpoint, ChangeShwSystemSetpoint};
pub use super::change_shw_system_schedule::{change_shw_system_schedule, ChangeShwSystemSchedule};
pub use super::create_solar_thermal_system::{create_solar_thermal_system, CreateSolarThermalSystem};
pub use super::delete_solar_thermal_system::{delete_solar_thermal_system, DeleteSolarThermalSystem};
pub use super::change_solar_thermal_system_collector_area::{change_solar_thermal_system_collector_area, ChangeSolarThermalSystemCollectorArea};
pub use super::change_solar_thermal_system_efficiency::{change_solar_thermal_system_efficiency, ChangeSolarThermalSystemEfficiency};
pub use super::change_solar_thermal_system_storage_volume::{change_solar_thermal_system_storage_volume, ChangeSolarThermalSystemStorageVolume};
pub use super::change_solar_thermal_system_tilt::{change_solar_thermal_system_tilt, ChangeSolarThermalSystemTilt};
pub use super::change_solar_thermal_system_azimuth::{change_solar_thermal_system_azimuth, ChangeSolarThermalSystemAzimuth};
pub use super::create_refrigeration_system::{create_refrigeration_system, CreateRefrigerationSystem};
pub use super::delete_refrigeration_system::{delete_refrigeration_system, DeleteRefrigerationSystem};
pub use super::change_refrigeration_system_case_count::{change_refrigeration_system_case_count, ChangeRefrigerationSystemCaseCount};
pub use super::change_refrigeration_system_design_load::{change_refrigeration_system_design_load, ChangeRefrigerationSystemDesignLoad};
pub use super::change_refrigeration_system_defrost_schedule::{change_refrigeration_system_defrost_schedule, ChangeRefrigerationSystemDefrostSchedule};
pub use super::create_water_system::{create_water_system, CreateWaterSystem};
pub use super::delete_water_system::{delete_water_system, DeleteWaterSystem};
pub use super::change_water_system_fixture_count::{change_water_system_fixture_count, ChangeWaterSystemFixtureCount};
pub use super::change_water_system_peak_flow::{change_water_system_peak_flow, ChangeWaterSystemPeakFlow};
pub use super::change_water_system_schedule::{change_water_system_schedule, ChangeWaterSystemSchedule};
pub use super::create_fault::{create_fault, CreateFault};
pub use super::delete_fault::{delete_fault, DeleteFault};
pub use super::change_fault_target_equipment::{change_fault_target_equipment, ChangeFaultTargetEquipment};
pub use super::change_fault_type::{change_fault_type, ChangeFaultType};
pub use super::change_fault_severity::{change_fault_severity, ChangeFaultSeverity};
pub use super::change_fault_start_schedule::{change_fault_start_schedule, ChangeFaultStartSchedule};
pub use super::create_space_list::{create_space_list, CreateSpaceList};
pub use super::delete_space_list::{delete_space_list, DeleteSpaceList};
pub use super::rename_space_list::{rename_space_list, RenameSpaceList};
pub use super::add_space_list_member::{add_space_list_member, AddSpaceListMember};
pub use super::remove_space_list_member::{remove_space_list_member, RemoveSpaceListMember};
pub use super::create_thermal_enclosure::{create_thermal_enclosure, CreateThermalEnclosure};
pub use super::delete_thermal_enclosure::{delete_thermal_enclosure, DeleteThermalEnclosure};
pub use super::rename_thermal_enclosure::{rename_thermal_enclosure, RenameThermalEnclosure};
pub use super::add_thermal_enclosure_zone::{add_thermal_enclosure_zone, AddThermalEnclosureZone};
pub use super::remove_thermal_enclosure_zone::{remove_thermal_enclosure_zone, RemoveThermalEnclosureZone};
pub use super::create_constant_schedule::{create_constant_schedule, CreateConstantSchedule};
pub use super::delete_constant_schedule::{delete_constant_schedule, DeleteConstantSchedule};
pub use super::change_constant_schedule_value::{change_constant_schedule_value, ChangeConstantScheduleValue};
pub use super::create_daily_schedule::{create_daily_schedule, CreateDailySchedule};
pub use super::delete_daily_schedule::{delete_daily_schedule, DeleteDailySchedule};
pub use super::replace_daily_schedule_hourly_values::{replace_daily_schedule_hourly_values, ReplaceDailyScheduleHourlyValues};
pub use super::change_daily_schedule_interpolation::{change_daily_schedule_interpolation, ChangeDailyScheduleInterpolation};
pub use super::change_daily_schedule_limits::{change_daily_schedule_limits, ChangeDailyScheduleLimits};
pub use super::create_weekly_schedule::{create_weekly_schedule, CreateWeeklySchedule};
pub use super::delete_weekly_schedule::{delete_weekly_schedule, DeleteWeeklySchedule};
pub use super::change_weekly_schedule_day::{change_weekly_schedule_day, ChangeWeeklyScheduleDay};
pub use super::create_annual_schedule::{create_annual_schedule, CreateAnnualSchedule};
pub use super::delete_annual_schedule::{delete_annual_schedule, DeleteAnnualSchedule};
pub use super::insert_annual_schedule_rule::{insert_annual_schedule_rule, InsertAnnualScheduleRule};
pub use super::remove_annual_schedule_rule::{remove_annual_schedule_rule, RemoveAnnualScheduleRule};
pub use super::reorder_annual_schedule_rules::{reorder_annual_schedule_rules, ReorderAnnualScheduleRules};
pub use super::change_annual_schedule_default_daily_schedule::{change_annual_schedule_default_daily_schedule, ChangeAnnualScheduleDefaultDailySchedule};
pub use super::change_annual_schedule_holiday_daily_schedule::{change_annual_schedule_holiday_daily_schedule, ChangeAnnualScheduleHolidayDailySchedule};
pub use super::add_annual_schedule_holiday::{add_annual_schedule_holiday, AddAnnualScheduleHoliday};
pub use super::remove_annual_schedule_holiday::{remove_annual_schedule_holiday, RemoveAnnualScheduleHoliday};
pub use super::create_time_series_schedule::{create_time_series_schedule, CreateTimeSeriesSchedule};
pub use super::delete_time_series_schedule::{delete_time_series_schedule, DeleteTimeSeriesSchedule};
pub use super::replace_time_series_schedule_values::{replace_time_series_schedule_values, ReplaceTimeSeriesScheduleValues};
pub use super::change_time_series_schedule_timestep::{change_time_series_schedule_timestep, ChangeTimeSeriesScheduleTimestep};
//#endregion 🔖️Reexports

//#region 🔖️Aggregate
/// 🧬️ Closed semantic mutation vocabulary for an energy model.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = EnergyModelSnapshot, diff = EnergyModelDiff, schema = "energy.model")]
pub enum EnergyModelMutation {
    RenameModel(RenameModel),
    ChangeModelVersion(ChangeModelVersion),
    UpdateSite(UpdateSite),
    UpdateGroundTemperature(UpdateGroundTemperature),
    UpdateRunPeriod(UpdateRunPeriod),
    ReplaceAirflowNetwork(ReplaceAirflowNetwork),
    AddOutputVariable(AddOutputVariable),
    RemoveOutputVariable(RemoveOutputVariable),
    BindWeatherFile(BindWeatherFile),
    UnbindWeatherFile(UnbindWeatherFile),
    ConnectReferencedModel(ConnectReferencedModel),
    DisconnectReferencedModel(DisconnectReferencedModel),
    RenameZone(RenameZone),
    ChangeZoneVolume(ChangeZoneVolume),
    ChangeZoneMultiplier(ChangeZoneMultiplier),
    ChangeZoneConditioned(ChangeZoneConditioned),
    ChangeZoneFloorAreaParticipation(ChangeZoneFloorAreaParticipation),
    CreateZone(CreateZone),
    DeleteZone(DeleteZone),
    CreateSpace(CreateSpace),
    DeleteSpace(DeleteSpace),
    RenameSpace(RenameSpace),
    ChangeSpaceFloorArea(ChangeSpaceFloorArea),
    ChangeSpaceZone(ChangeSpaceZone),
    CreateSurface(CreateSurface),
    DeleteSurface(DeleteSurface),
    RenameSurface(RenameSurface),
    ChangeSurfaceZone(ChangeSurfaceZone),
    ChangeSurfaceClass(ChangeSurfaceClass),
    ReplaceSurfaceVertices(ReplaceSurfaceVertices),
    ChangeSurfaceConstruction(ChangeSurfaceConstruction),
    ChangeSurfaceBoundaryCondition(ChangeSurfaceBoundaryCondition),
    ChangeSurfaceSunExposed(ChangeSurfaceSunExposed),
    ChangeSurfaceWindExposed(ChangeSurfaceWindExposed),
    ChangeSurfaceMultiplier(ChangeSurfaceMultiplier),
    CreateFenestration(CreateFenestration),
    DeleteFenestration(DeleteFenestration),
    RenameFenestration(RenameFenestration),
    ChangeFenestrationSurface(ChangeFenestrationSurface),
    ChangeFenestrationUValue(ChangeFenestrationUValue),
    ChangeFenestrationShgc(ChangeFenestrationShgc),
    ChangeFenestrationVlt(ChangeFenestrationVlt),
    ChangeFenestrationArea(ChangeFenestrationArea),
    ChangeFenestrationFrameConductance(ChangeFenestrationFrameConductance),
    ChangeFenestrationDividerConductance(ChangeFenestrationDividerConductance),
    CreateShadingSurface(CreateShadingSurface),
    DeleteShadingSurface(DeleteShadingSurface),
    RenameShadingSurface(RenameShadingSurface),
    ReplaceShadingSurfaceVertices(ReplaceShadingSurfaceVertices),
    ChangeShadingSurfaceTransmittanceSchedule(ChangeShadingSurfaceTransmittanceSchedule),
    ConnectSurfaces(ConnectSurfaces),
    DisconnectSurfaces(DisconnectSurfaces),
    BindFenestrationGlazingConstruction(BindFenestrationGlazingConstruction),
    ClearFenestrationGlazingConstruction(ClearFenestrationGlazingConstruction),
    ChangeFenestrationHeight(ChangeFenestrationHeight),
    ChangeFenestrationSillHeight(ChangeFenestrationSillHeight),
    ChangeFenestrationOverhangDepth(ChangeFenestrationOverhangDepth),
    ChangeFenestrationOverhangOffset(ChangeFenestrationOverhangOffset),
    ChangeFenestrationFinDepth(ChangeFenestrationFinDepth),
    ChangeFenestrationFinOffset(ChangeFenestrationFinOffset),
    CreateMaterial(CreateMaterial),
    DeleteMaterial(DeleteMaterial),
    RenameMaterial(RenameMaterial),
    ChangeMaterialThickness(ChangeMaterialThickness),
    ChangeMaterialConductivity(ChangeMaterialConductivity),
    ChangeMaterialDensity(ChangeMaterialDensity),
    ChangeMaterialSpecificHeat(ChangeMaterialSpecificHeat),
    ChangeMaterialThermalAbsorptance(ChangeMaterialThermalAbsorptance),
    ChangeMaterialSolarAbsorptance(ChangeMaterialSolarAbsorptance),
    ChangeMaterialVisibleAbsorptance(ChangeMaterialVisibleAbsorptance),
    CreateConstruction(CreateConstruction),
    DeleteConstruction(DeleteConstruction),
    RenameConstruction(RenameConstruction),
    AddConstructionLayer(AddConstructionLayer),
    RemoveConstructionLayer(RemoveConstructionLayer),
    ReorderConstructionLayers(ReorderConstructionLayers),
    CreatePeopleGain(CreatePeopleGain),
    DeletePeopleGain(DeletePeopleGain),
    ChangePeopleGainZone(ChangePeopleGainZone),
    ChangePeopleGainSchedule(ChangePeopleGainSchedule),
    ChangePeopleGainActivitySchedule(ChangePeopleGainActivitySchedule),
    ChangePeopleGainPeoplePerArea(ChangePeopleGainPeoplePerArea),
    ChangePeopleGainSensibleFraction(ChangePeopleGainSensibleFraction),
    ChangePeopleGainLatentFraction(ChangePeopleGainLatentFraction),
    ChangePeopleGainRadiantFraction(ChangePeopleGainRadiantFraction),
    CreateLightingGain(CreateLightingGain),
    DeleteLightingGain(DeleteLightingGain),
    ChangeLightingGainZone(ChangeLightingGainZone),
    ChangeLightingGainSchedule(ChangeLightingGainSchedule),
    ChangeLightingGainWattsPerArea(ChangeLightingGainWattsPerArea),
    ChangeLightingGainRadiantFraction(ChangeLightingGainRadiantFraction),
    ChangeLightingGainVisibleFraction(ChangeLightingGainVisibleFraction),
    ChangeLightingGainReturnAirFraction(ChangeLightingGainReturnAirFraction),
    CreateEquipmentGain(CreateEquipmentGain),
    DeleteEquipmentGain(DeleteEquipmentGain),
    ChangeEquipmentGainZone(ChangeEquipmentGainZone),
    ChangeEquipmentGainSchedule(ChangeEquipmentGainSchedule),
    ChangeEquipmentGainWattsPerArea(ChangeEquipmentGainWattsPerArea),
    ChangeEquipmentGainRadiantFraction(ChangeEquipmentGainRadiantFraction),
    ChangeEquipmentGainLatentFraction(ChangeEquipmentGainLatentFraction),
    CreateInfiltration(CreateInfiltration),
    DeleteInfiltration(DeleteInfiltration),
    ChangeInfiltrationZone(ChangeInfiltrationZone),
    ChangeInfiltrationSchedule(ChangeInfiltrationSchedule),
    ChangeInfiltrationFlowPerExteriorArea(ChangeInfiltrationFlowPerExteriorArea),
    ChangeInfiltrationConstantTermCoefficient(ChangeInfiltrationConstantTermCoefficient),
    ChangeInfiltrationTemperatureTermCoefficient(ChangeInfiltrationTemperatureTermCoefficient),
    ChangeInfiltrationVelocityTermCoefficient(ChangeInfiltrationVelocityTermCoefficient),
    ChangeInfiltrationVelocitySquaredTermCoefficient(ChangeInfiltrationVelocitySquaredTermCoefficient),
    CreateMechanicalVentilation(CreateMechanicalVentilation),
    DeleteMechanicalVentilation(DeleteMechanicalVentilation),
    ChangeMechanicalVentilationZone(ChangeMechanicalVentilationZone),
    ChangeMechanicalVentilationSchedule(ChangeMechanicalVentilationSchedule),
    ChangeMechanicalVentilationDesignFlow(ChangeMechanicalVentilationDesignFlow),
    ChangeMechanicalVentilationFanTotalEfficiency(ChangeMechanicalVentilationFanTotalEfficiency),
    ChangeMechanicalVentilationFanDeltaPressure(ChangeMechanicalVentilationFanDeltaPressure),
    ChangeInfiltrationMethod(ChangeInfiltrationMethod),
    ChangeInfiltrationDesignFlowAch(ChangeInfiltrationDesignFlowAch),
    ChangeInfiltrationEffectiveLeakageArea(ChangeInfiltrationEffectiveLeakageArea),
    ChangeInfiltrationDischargeCoefficient(ChangeInfiltrationDischargeCoefficient),
    ChangeInfiltrationStackHeight(ChangeInfiltrationStackHeight),
    CreateThermostat(CreateThermostat),
    DeleteThermostat(DeleteThermostat),
    ChangeThermostatZone(ChangeThermostatZone),
    ChangeThermostatHeatingSetpointSchedule(ChangeThermostatHeatingSetpointSchedule),
    ChangeThermostatCoolingSetpointSchedule(ChangeThermostatCoolingSetpointSchedule),
    ChangeThermostatHeatingThrottleRange(ChangeThermostatHeatingThrottleRange),
    ChangeThermostatCoolingThrottleRange(ChangeThermostatCoolingThrottleRange),
    CreateHumidistat(CreateHumidistat),
    DeleteHumidistat(DeleteHumidistat),
    ChangeHumidistatZone(ChangeHumidistatZone),
    ChangeHumidistatHumidifyingSetpointSchedule(ChangeHumidistatHumidifyingSetpointSchedule),
    ChangeHumidistatDehumidifyingSetpointSchedule(ChangeHumidistatDehumidifyingSetpointSchedule),
    ChangeHumidistatHumidifyingThrottleRange(ChangeHumidistatHumidifyingThrottleRange),
    ChangeHumidistatDehumidifyingThrottleRange(ChangeHumidistatDehumidifyingThrottleRange),
    CreateIdealLoadsSystem(CreateIdealLoadsSystem),
    DeleteIdealLoadsSystem(DeleteIdealLoadsSystem),
    ChangeIdealLoadsSystemZone(ChangeIdealLoadsSystemZone),
    ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp(ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp),
    ChangeIdealLoadsSystemMinCoolingSupplyAirTemp(ChangeIdealLoadsSystemMinCoolingSupplyAirTemp),
    ChangeIdealLoadsSystemMaxHeatingCapacity(ChangeIdealLoadsSystemMaxHeatingCapacity),
    ChangeIdealLoadsSystemMaxCoolingCapacity(ChangeIdealLoadsSystemMaxCoolingCapacity),
    ChangeIdealLoadsSystemOutdoorAirPerPerson(ChangeIdealLoadsSystemOutdoorAirPerPerson),
    ChangeIdealLoadsSystemOutdoorAirPerArea(ChangeIdealLoadsSystemOutdoorAirPerArea),
    CreateZoneEquipment(CreateZoneEquipment),
    DeleteZoneEquipment(DeleteZoneEquipment),
    ChangeZoneEquipmentZone(ChangeZoneEquipmentZone),
    ChangeZoneEquipmentType(ChangeZoneEquipmentType),
    ChangeZoneEquipmentPriority(ChangeZoneEquipmentPriority),
    ChangeZoneEquipmentHeatingCapacity(ChangeZoneEquipmentHeatingCapacity),
    ChangeZoneEquipmentCoolingCapacity(ChangeZoneEquipmentCoolingCapacity),
    CreateDaylightZone(CreateDaylightZone),
    DeleteDaylightZone(DeleteDaylightZone),
    ChangeDaylightZoneZone(ChangeDaylightZoneZone),
    ChangeDaylightZoneIlluminanceTarget(ChangeDaylightZoneIlluminanceTarget),
    ChangeDaylightZoneGlareLimit(ChangeDaylightZoneGlareLimit),
    ChangeDaylightZoneWindowTransmittance(ChangeDaylightZoneWindowTransmittance),
    CreateSizingObject(CreateSizingObject),
    DeleteSizingObject(DeleteSizingObject),
    ChangeSizingObjectZone(ChangeSizingObjectZone),
    ChangeSizingObjectSizingType(ChangeSizingObjectSizingType),
    ChangeSizingObjectDesignDayType(ChangeSizingObjectDesignDayType),
    CreateRoomAirModelAssignment(CreateRoomAirModelAssignment),
    DeleteRoomAirModelAssignment(DeleteRoomAirModelAssignment),
    ChangeRoomAirModel(ChangeRoomAirModel),
    CreateSetpointManager(CreateSetpointManager),
    DeleteSetpointManager(DeleteSetpointManager),
    RenameSetpointManager(RenameSetpointManager),
    ReplaceSetpointManagerKind(ReplaceSetpointManagerKind),
    ChangeSetpointManagerSchedule(ChangeSetpointManagerSchedule),
    CreateAirLoop(CreateAirLoop),
    DeleteAirLoop(DeleteAirLoop),
    RenameAirLoop(RenameAirLoop),
    ChangeAirLoopSupplyNode(ChangeAirLoopSupplyNode),
    ChangeAirLoopReturnNode(ChangeAirLoopReturnNode),
    ChangeAirLoopDesignSupplyAirFlow(ChangeAirLoopDesignSupplyAirFlow),
    AddAirLoopTerminalZone(AddAirLoopTerminalZone),
    RemoveAirLoopTerminalZone(RemoveAirLoopTerminalZone),
    CreatePlantLoop(CreatePlantLoop),
    DeletePlantLoop(DeletePlantLoop),
    RenamePlantLoop(RenamePlantLoop),
    ChangePlantLoopType(ChangePlantLoopType),
    ChangePlantLoopSupplyTemperature(ChangePlantLoopSupplyTemperature),
    ChangePlantLoopReturnTemperature(ChangePlantLoopReturnTemperature),
    ChangePlantLoopDesignFlow(ChangePlantLoopDesignFlow),
    AddPlantLoopEquipment(AddPlantLoopEquipment),
    RemovePlantLoopEquipment(RemovePlantLoopEquipment),
    CreateOutdoorAirSystem(CreateOutdoorAirSystem),
    DeleteOutdoorAirSystem(DeleteOutdoorAirSystem),
    ChangeOutdoorAirSystemAirLoop(ChangeOutdoorAirSystemAirLoop),
    ChangeOutdoorAirSystemMinOaFlow(ChangeOutdoorAirSystemMinOaFlow),
    ChangeOutdoorAirSystemEconomizerEnabled(ChangeOutdoorAirSystemEconomizerEnabled),
    CreateElectricalLoadCenter(CreateElectricalLoadCenter),
    DeleteElectricalLoadCenter(DeleteElectricalLoadCenter),
    RenameElectricalLoadCenter(RenameElectricalLoadCenter),
    AddElectricalLoadCenterPv(AddElectricalLoadCenterPv),
    RemoveElectricalLoadCenterPv(RemoveElectricalLoadCenterPv),
    AddElectricalLoadCenterBattery(AddElectricalLoadCenterBattery),
    RemoveElectricalLoadCenterBattery(RemoveElectricalLoadCenterBattery),
    CreatePvSystem(CreatePvSystem),
    DeletePvSystem(DeletePvSystem),
    ChangePvSystemDcCapacity(ChangePvSystemDcCapacity),
    ChangePvSystemArea(ChangePvSystemArea),
    ChangePvSystemTilt(ChangePvSystemTilt),
    ChangePvSystemAzimuth(ChangePvSystemAzimuth),
    ChangePvSystemModuleEfficiency(ChangePvSystemModuleEfficiency),
    ChangePvSystemInverterEfficiency(ChangePvSystemInverterEfficiency),
    CreateBattery(CreateBattery),
    DeleteBattery(DeleteBattery),
    ChangeBatteryCapacity(ChangeBatteryCapacity),
    ChangeBatteryMaxCharge(ChangeBatteryMaxCharge),
    ChangeBatteryMaxDischarge(ChangeBatteryMaxDischarge),
    ChangeBatteryRoundTripEfficiency(ChangeBatteryRoundTripEfficiency),
    CreateShwSystem(CreateShwSystem),
    DeleteShwSystem(DeleteShwSystem),
    ChangeShwSystemHeaterCapacity(ChangeShwSystemHeaterCapacity),
    ChangeShwSystemStorageVolume(ChangeShwSystemStorageVolume),
    ChangeShwSystemSetpoint(ChangeShwSystemSetpoint),
    ChangeShwSystemSchedule(ChangeShwSystemSchedule),
    CreateSolarThermalSystem(CreateSolarThermalSystem),
    DeleteSolarThermalSystem(DeleteSolarThermalSystem),
    ChangeSolarThermalSystemCollectorArea(ChangeSolarThermalSystemCollectorArea),
    ChangeSolarThermalSystemEfficiency(ChangeSolarThermalSystemEfficiency),
    ChangeSolarThermalSystemStorageVolume(ChangeSolarThermalSystemStorageVolume),
    ChangeSolarThermalSystemTilt(ChangeSolarThermalSystemTilt),
    ChangeSolarThermalSystemAzimuth(ChangeSolarThermalSystemAzimuth),
    CreateRefrigerationSystem(CreateRefrigerationSystem),
    DeleteRefrigerationSystem(DeleteRefrigerationSystem),
    ChangeRefrigerationSystemCaseCount(ChangeRefrigerationSystemCaseCount),
    ChangeRefrigerationSystemDesignLoad(ChangeRefrigerationSystemDesignLoad),
    ChangeRefrigerationSystemDefrostSchedule(ChangeRefrigerationSystemDefrostSchedule),
    CreateWaterSystem(CreateWaterSystem),
    DeleteWaterSystem(DeleteWaterSystem),
    ChangeWaterSystemFixtureCount(ChangeWaterSystemFixtureCount),
    ChangeWaterSystemPeakFlow(ChangeWaterSystemPeakFlow),
    ChangeWaterSystemSchedule(ChangeWaterSystemSchedule),
    CreateFault(CreateFault),
    DeleteFault(DeleteFault),
    ChangeFaultTargetEquipment(ChangeFaultTargetEquipment),
    ChangeFaultType(ChangeFaultType),
    ChangeFaultSeverity(ChangeFaultSeverity),
    ChangeFaultStartSchedule(ChangeFaultStartSchedule),
    CreateSpaceList(CreateSpaceList),
    DeleteSpaceList(DeleteSpaceList),
    RenameSpaceList(RenameSpaceList),
    AddSpaceListMember(AddSpaceListMember),
    RemoveSpaceListMember(RemoveSpaceListMember),
    CreateThermalEnclosure(CreateThermalEnclosure),
    DeleteThermalEnclosure(DeleteThermalEnclosure),
    RenameThermalEnclosure(RenameThermalEnclosure),
    AddThermalEnclosureZone(AddThermalEnclosureZone),
    RemoveThermalEnclosureZone(RemoveThermalEnclosureZone),
    CreateConstantSchedule(CreateConstantSchedule),
    DeleteConstantSchedule(DeleteConstantSchedule),
    ChangeConstantScheduleValue(ChangeConstantScheduleValue),
    CreateDailySchedule(CreateDailySchedule),
    DeleteDailySchedule(DeleteDailySchedule),
    ReplaceDailyScheduleHourlyValues(ReplaceDailyScheduleHourlyValues),
    ChangeDailyScheduleInterpolation(ChangeDailyScheduleInterpolation),
    ChangeDailyScheduleLimits(ChangeDailyScheduleLimits),
    CreateWeeklySchedule(CreateWeeklySchedule),
    DeleteWeeklySchedule(DeleteWeeklySchedule),
    ChangeWeeklyScheduleDay(ChangeWeeklyScheduleDay),
    CreateAnnualSchedule(CreateAnnualSchedule),
    DeleteAnnualSchedule(DeleteAnnualSchedule),
    InsertAnnualScheduleRule(InsertAnnualScheduleRule),
    RemoveAnnualScheduleRule(RemoveAnnualScheduleRule),
    ReorderAnnualScheduleRules(ReorderAnnualScheduleRules),
    ChangeAnnualScheduleDefaultDailySchedule(ChangeAnnualScheduleDefaultDailySchedule),
    ChangeAnnualScheduleHolidayDailySchedule(ChangeAnnualScheduleHolidayDailySchedule),
    AddAnnualScheduleHoliday(AddAnnualScheduleHoliday),
    RemoveAnnualScheduleHoliday(RemoveAnnualScheduleHoliday),
    CreateTimeSeriesSchedule(CreateTimeSeriesSchedule),
    DeleteTimeSeriesSchedule(DeleteTimeSeriesSchedule),
    ReplaceTimeSeriesScheduleValues(ReplaceTimeSeriesScheduleValues),
    ChangeTimeSeriesScheduleTimestep(ChangeTimeSeriesScheduleTimestep),
}

/// 🏷️ Direct semantic roster exported for the language-neutral test adapter, in aggregate
/// declaration order — which is also the binary ordinal order `dsl::variants_binary` writes.
pub const KINDS: &[&str] = &[
    "rename-model",
    "change-model-version",
    "update-site",
    "update-ground-temperature",
    "update-run-period",
    "replace-airflow-network",
    "add-output-variable",
    "remove-output-variable",
    "bind-weather-file",
    "unbind-weather-file",
    "connect-referenced-model",
    "disconnect-referenced-model",
    "rename-zone",
    "change-zone-volume",
    "change-zone-multiplier",
    "change-zone-conditioned",
    "change-zone-floor-area-participation",
    "create-zone",
    "delete-zone",
    "create-space",
    "delete-space",
    "rename-space",
    "change-space-floor-area",
    "change-space-zone",
    "create-surface",
    "delete-surface",
    "rename-surface",
    "change-surface-zone",
    "change-surface-class",
    "replace-surface-vertices",
    "change-surface-construction",
    "change-surface-boundary-condition",
    "change-surface-sun-exposed",
    "change-surface-wind-exposed",
    "change-surface-multiplier",
    "create-fenestration",
    "delete-fenestration",
    "rename-fenestration",
    "change-fenestration-surface",
    "change-fenestration-u-value",
    "change-fenestration-shgc",
    "change-fenestration-vlt",
    "change-fenestration-area",
    "change-fenestration-frame-conductance",
    "change-fenestration-divider-conductance",
    "create-shading-surface",
    "delete-shading-surface",
    "rename-shading-surface",
    "replace-shading-surface-vertices",
    "change-shading-surface-transmittance-schedule",
    "connect-surfaces",
    "disconnect-surfaces",
    "bind-fenestration-glazing-construction",
    "clear-fenestration-glazing-construction",
    "change-fenestration-height",
    "change-fenestration-sill-height",
    "change-fenestration-overhang-depth",
    "change-fenestration-overhang-offset",
    "change-fenestration-fin-depth",
    "change-fenestration-fin-offset",
    "create-material",
    "delete-material",
    "rename-material",
    "change-material-thickness",
    "change-material-conductivity",
    "change-material-density",
    "change-material-specific-heat",
    "change-material-thermal-absorptance",
    "change-material-solar-absorptance",
    "change-material-visible-absorptance",
    "create-construction",
    "delete-construction",
    "rename-construction",
    "add-construction-layer",
    "remove-construction-layer",
    "reorder-construction-layers",
    "create-people-gain",
    "delete-people-gain",
    "change-people-gain-zone",
    "change-people-gain-schedule",
    "change-people-gain-activity-schedule",
    "change-people-gain-people-per-area",
    "change-people-gain-sensible-fraction",
    "change-people-gain-latent-fraction",
    "change-people-gain-radiant-fraction",
    "create-lighting-gain",
    "delete-lighting-gain",
    "change-lighting-gain-zone",
    "change-lighting-gain-schedule",
    "change-lighting-gain-watts-per-area",
    "change-lighting-gain-radiant-fraction",
    "change-lighting-gain-visible-fraction",
    "change-lighting-gain-return-air-fraction",
    "create-equipment-gain",
    "delete-equipment-gain",
    "change-equipment-gain-zone",
    "change-equipment-gain-schedule",
    "change-equipment-gain-watts-per-area",
    "change-equipment-gain-radiant-fraction",
    "change-equipment-gain-latent-fraction",
    "create-infiltration",
    "delete-infiltration",
    "change-infiltration-zone",
    "change-infiltration-schedule",
    "change-infiltration-flow-per-exterior-area",
    "change-infiltration-constant-term-coefficient",
    "change-infiltration-temperature-term-coefficient",
    "change-infiltration-velocity-term-coefficient",
    "change-infiltration-velocity-squared-term-coefficient",
    "create-mechanical-ventilation",
    "delete-mechanical-ventilation",
    "change-mechanical-ventilation-zone",
    "change-mechanical-ventilation-schedule",
    "change-mechanical-ventilation-design-flow",
    "change-mechanical-ventilation-fan-total-efficiency",
    "change-mechanical-ventilation-fan-delta-pressure",
    "change-infiltration-method",
    "change-infiltration-design-flow-ach",
    "change-infiltration-effective-leakage-area",
    "change-infiltration-discharge-coefficient",
    "change-infiltration-stack-height",
    "create-thermostat",
    "delete-thermostat",
    "change-thermostat-zone",
    "change-thermostat-heating-setpoint-schedule",
    "change-thermostat-cooling-setpoint-schedule",
    "change-thermostat-heating-throttle-range",
    "change-thermostat-cooling-throttle-range",
    "create-humidistat",
    "delete-humidistat",
    "change-humidistat-zone",
    "change-humidistat-humidifying-setpoint-schedule",
    "change-humidistat-dehumidifying-setpoint-schedule",
    "change-humidistat-humidifying-throttle-range",
    "change-humidistat-dehumidifying-throttle-range",
    "create-ideal-loads-system",
    "delete-ideal-loads-system",
    "change-ideal-loads-system-zone",
    "change-ideal-loads-system-max-heating-supply-air-temp",
    "change-ideal-loads-system-min-cooling-supply-air-temp",
    "change-ideal-loads-system-max-heating-capacity",
    "change-ideal-loads-system-max-cooling-capacity",
    "change-ideal-loads-system-outdoor-air-per-person",
    "change-ideal-loads-system-outdoor-air-per-area",
    "create-zone-equipment",
    "delete-zone-equipment",
    "change-zone-equipment-zone",
    "change-zone-equipment-type",
    "change-zone-equipment-priority",
    "change-zone-equipment-heating-capacity",
    "change-zone-equipment-cooling-capacity",
    "create-daylight-zone",
    "delete-daylight-zone",
    "change-daylight-zone-zone",
    "change-daylight-zone-illuminance-target",
    "change-daylight-zone-glare-limit",
    "change-daylight-zone-window-transmittance",
    "create-sizing-object",
    "delete-sizing-object",
    "change-sizing-object-zone",
    "change-sizing-object-sizing-type",
    "change-sizing-object-design-day-type",
    "create-room-air-model-assignment",
    "delete-room-air-model-assignment",
    "change-room-air-model",
    "create-setpoint-manager",
    "delete-setpoint-manager",
    "rename-setpoint-manager",
    "replace-setpoint-manager-kind",
    "change-setpoint-manager-schedule",
    "create-air-loop",
    "delete-air-loop",
    "rename-air-loop",
    "change-air-loop-supply-node",
    "change-air-loop-return-node",
    "change-air-loop-design-supply-air-flow",
    "add-air-loop-terminal-zone",
    "remove-air-loop-terminal-zone",
    "create-plant-loop",
    "delete-plant-loop",
    "rename-plant-loop",
    "change-plant-loop-type",
    "change-plant-loop-supply-temperature",
    "change-plant-loop-return-temperature",
    "change-plant-loop-design-flow",
    "add-plant-loop-equipment",
    "remove-plant-loop-equipment",
    "create-outdoor-air-system",
    "delete-outdoor-air-system",
    "change-outdoor-air-system-air-loop",
    "change-outdoor-air-system-min-oa-flow",
    "change-outdoor-air-system-economizer-enabled",
    "create-electrical-load-center",
    "delete-electrical-load-center",
    "rename-electrical-load-center",
    "add-electrical-load-center-pv",
    "remove-electrical-load-center-pv",
    "add-electrical-load-center-battery",
    "remove-electrical-load-center-battery",
    "create-pv-system",
    "delete-pv-system",
    "change-pv-system-dc-capacity",
    "change-pv-system-area",
    "change-pv-system-tilt",
    "change-pv-system-azimuth",
    "change-pv-system-module-efficiency",
    "change-pv-system-inverter-efficiency",
    "create-battery",
    "delete-battery",
    "change-battery-capacity",
    "change-battery-max-charge",
    "change-battery-max-discharge",
    "change-battery-round-trip-efficiency",
    "create-shw-system",
    "delete-shw-system",
    "change-shw-system-heater-capacity",
    "change-shw-system-storage-volume",
    "change-shw-system-setpoint",
    "change-shw-system-schedule",
    "create-solar-thermal-system",
    "delete-solar-thermal-system",
    "change-solar-thermal-system-collector-area",
    "change-solar-thermal-system-efficiency",
    "change-solar-thermal-system-storage-volume",
    "change-solar-thermal-system-tilt",
    "change-solar-thermal-system-azimuth",
    "create-refrigeration-system",
    "delete-refrigeration-system",
    "change-refrigeration-system-case-count",
    "change-refrigeration-system-design-load",
    "change-refrigeration-system-defrost-schedule",
    "create-water-system",
    "delete-water-system",
    "change-water-system-fixture-count",
    "change-water-system-peak-flow",
    "change-water-system-schedule",
    "create-fault",
    "delete-fault",
    "change-fault-target-equipment",
    "change-fault-type",
    "change-fault-severity",
    "change-fault-start-schedule",
    "create-space-list",
    "delete-space-list",
    "rename-space-list",
    "add-space-list-member",
    "remove-space-list-member",
    "create-thermal-enclosure",
    "delete-thermal-enclosure",
    "rename-thermal-enclosure",
    "add-thermal-enclosure-zone",
    "remove-thermal-enclosure-zone",
    "create-constant-schedule",
    "delete-constant-schedule",
    "change-constant-schedule-value",
    "create-daily-schedule",
    "delete-daily-schedule",
    "replace-daily-schedule-hourly-values",
    "change-daily-schedule-interpolation",
    "change-daily-schedule-limits",
    "create-weekly-schedule",
    "delete-weekly-schedule",
    "change-weekly-schedule-day",
    "create-annual-schedule",
    "delete-annual-schedule",
    "insert-annual-schedule-rule",
    "remove-annual-schedule-rule",
    "reorder-annual-schedule-rules",
    "change-annual-schedule-default-daily-schedule",
    "change-annual-schedule-holiday-daily-schedule",
    "add-annual-schedule-holiday",
    "remove-annual-schedule-holiday",
    "create-time-series-schedule",
    "delete-time-series-schedule",
    "replace-time-series-schedule-values",
    "change-time-series-schedule-timestep",
];

/// 🗂️ `(semanticKind, leaf directory name)` for every declared kind — the single place the
/// emoji-carrying directory names are stated in Rust.
pub const DIRECTORIES: &[(&str, &str)] = &[
    ("rename-model", "🏷️rename-model"),
    ("change-model-version", "🔢️change-model-version"),
    ("update-site", "🌍️update-site"),
    ("update-ground-temperature", "🌡️update-ground-temperature"),
    ("update-run-period", "📅️update-run-period"),
    ("replace-airflow-network", "🫧️replace-airflow-network"),
    ("add-output-variable", "📊️add-output-variable"),
    ("remove-output-variable", "📉️remove-output-variable"),
    ("bind-weather-file", "🌦️bind-weather-file"),
    ("unbind-weather-file", "🌤️unbind-weather-file"),
    ("connect-referenced-model", "🪢️connect-referenced-model"),
    ("disconnect-referenced-model", "✂️disconnect-referenced-model"),
    ("rename-zone", "🏠️rename-zone"),
    ("change-zone-volume", "📦️change-zone-volume"),
    ("change-zone-multiplier", "✖️change-zone-multiplier"),
    ("change-zone-conditioned", "🌬️change-zone-conditioned"),
    ("change-zone-floor-area-participation", "📐️change-zone-floor-area-participation"),
    ("create-zone", "🏘️create-zone"),
    ("delete-zone", "🏚️delete-zone"),
    ("create-space", "🪑️create-space"),
    ("delete-space", "🧹️delete-space"),
    ("rename-space", "🔤️rename-space"),
    ("change-space-floor-area", "🧮️change-space-floor-area"),
    ("change-space-zone", "🚚️change-space-zone"),
    ("create-surface", "🟫️create-surface"),
    ("delete-surface", "🪚️delete-surface"),
    ("rename-surface", "🏳️rename-surface"),
    ("change-surface-zone", "🗜️change-surface-zone"),
    ("change-surface-class", "🧩️change-surface-class"),
    ("replace-surface-vertices", "🔺️replace-surface-vertices"),
    ("change-surface-construction", "🧰️change-surface-construction"),
    ("change-surface-boundary-condition", "🚧️change-surface-boundary-condition"),
    ("change-surface-sun-exposed", "🌅️change-surface-sun-exposed"),
    ("change-surface-wind-exposed", "🍃️change-surface-wind-exposed"),
    ("change-surface-multiplier", "🔁️change-surface-multiplier"),
    ("create-fenestration", "🪟️create-fenestration"),
    ("delete-fenestration", "🚪️delete-fenestration"),
    ("rename-fenestration", "🏁️rename-fenestration"),
    ("change-fenestration-surface", "🧲️change-fenestration-surface"),
    ("change-fenestration-u-value", "🌐️change-fenestration-u-value"),
    ("change-fenestration-shgc", "🌇️change-fenestration-shgc"),
    ("change-fenestration-vlt", "🌈️change-fenestration-vlt"),
    ("change-fenestration-area", "🟥️change-fenestration-area"),
    ("change-fenestration-frame-conductance", "🖼️change-fenestration-frame-conductance"),
    ("change-fenestration-divider-conductance", "🧷️change-fenestration-divider-conductance"),
    ("create-shading-surface", "🌳️create-shading-surface"),
    ("delete-shading-surface", "🪵️delete-shading-surface"),
    ("rename-shading-surface", "🏕️rename-shading-surface"),
    ("replace-shading-surface-vertices", "🗺️replace-shading-surface-vertices"),
    ("change-shading-surface-transmittance-schedule", "⛱️change-shading-surface-transmittance-schedule"),
    ("connect-surfaces", "🤝️connect-surfaces"),
    ("disconnect-surfaces", "💔️disconnect-surfaces"),
    ("bind-fenestration-glazing-construction", "🧊️bind-fenestration-glazing-construction"),
    ("clear-fenestration-glazing-construction", "🫗️clear-fenestration-glazing-construction"),
    ("change-fenestration-height", "⬆️change-fenestration-height"),
    ("change-fenestration-sill-height", "⬇️change-fenestration-sill-height"),
    ("change-fenestration-overhang-depth", "🧢️change-fenestration-overhang-depth"),
    ("change-fenestration-overhang-offset", "🎩️change-fenestration-overhang-offset"),
    ("change-fenestration-fin-depth", "🐬️change-fenestration-fin-depth"),
    ("change-fenestration-fin-offset", "🐋️change-fenestration-fin-offset"),
    ("create-material", "🧱️create-material"),
    ("delete-material", "🪨️delete-material"),
    ("rename-material", "🪧️rename-material"),
    ("change-material-thickness", "📏️change-material-thickness"),
    ("change-material-conductivity", "🔥️change-material-conductivity"),
    ("change-material-density", "⚖️change-material-density"),
    ("change-material-specific-heat", "♨️change-material-specific-heat"),
    ("change-material-thermal-absorptance", "🔆️change-material-thermal-absorptance"),
    ("change-material-solar-absorptance", "☀️change-material-solar-absorptance"),
    ("change-material-visible-absorptance", "👁️change-material-visible-absorptance"),
    ("create-construction", "🏗️create-construction"),
    ("delete-construction", "🧨️delete-construction"),
    ("rename-construction", "🪪️rename-construction"),
    ("add-construction-layer", "➕️add-construction-layer"),
    ("remove-construction-layer", "➖️remove-construction-layer"),
    ("reorder-construction-layers", "🔀️reorder-construction-layers"),
    ("create-people-gain", "👤️create-people-gain"),
    ("delete-people-gain", "🚷️delete-people-gain"),
    ("change-people-gain-zone", "🚶️change-people-gain-zone"),
    ("change-people-gain-schedule", "⏰️change-people-gain-schedule"),
    ("change-people-gain-activity-schedule", "🏃️change-people-gain-activity-schedule"),
    ("change-people-gain-people-per-area", "👥️change-people-gain-people-per-area"),
    ("change-people-gain-sensible-fraction", "🌞️change-people-gain-sensible-fraction"),
    ("change-people-gain-latent-fraction", "💧️change-people-gain-latent-fraction"),
    ("change-people-gain-radiant-fraction", "📡️change-people-gain-radiant-fraction"),
    ("create-lighting-gain", "💡️create-lighting-gain"),
    ("delete-lighting-gain", "🕯️delete-lighting-gain"),
    ("change-lighting-gain-zone", "🔦️change-lighting-gain-zone"),
    ("change-lighting-gain-schedule", "⏱️change-lighting-gain-schedule"),
    ("change-lighting-gain-watts-per-area", "🔌️change-lighting-gain-watts-per-area"),
    ("change-lighting-gain-radiant-fraction", "🌟️change-lighting-gain-radiant-fraction"),
    ("change-lighting-gain-visible-fraction", "🔅️change-lighting-gain-visible-fraction"),
    ("change-lighting-gain-return-air-fraction", "🎐️change-lighting-gain-return-air-fraction"),
    ("create-equipment-gain", "🖥️create-equipment-gain"),
    ("delete-equipment-gain", "🧯️delete-equipment-gain"),
    ("change-equipment-gain-zone", "🖨️change-equipment-gain-zone"),
    ("change-equipment-gain-schedule", "⌛️change-equipment-gain-schedule"),
    ("change-equipment-gain-watts-per-area", "⚡️change-equipment-gain-watts-per-area"),
    ("change-equipment-gain-radiant-fraction", "🌠️change-equipment-gain-radiant-fraction"),
    ("change-equipment-gain-latent-fraction", "💦️change-equipment-gain-latent-fraction"),
    ("create-infiltration", "💨️create-infiltration"),
    ("delete-infiltration", "🧽️delete-infiltration"),
    ("change-infiltration-zone", "🌀️change-infiltration-zone"),
    ("change-infiltration-schedule", "⏳️change-infiltration-schedule"),
    ("change-infiltration-flow-per-exterior-area", "🌫️change-infiltration-flow-per-exterior-area"),
    ("change-infiltration-constant-term-coefficient", "🅰️change-infiltration-constant-term-coefficient"),
    ("change-infiltration-temperature-term-coefficient", "🅱️change-infiltration-temperature-term-coefficient"),
    ("change-infiltration-velocity-term-coefficient", "🆎️change-infiltration-velocity-term-coefficient"),
    ("change-infiltration-velocity-squared-term-coefficient", "🆑️change-infiltration-velocity-squared-term-coefficient"),
    ("create-mechanical-ventilation", "🌪️create-mechanical-ventilation"),
    ("delete-mechanical-ventilation", "🚫️delete-mechanical-ventilation"),
    ("change-mechanical-ventilation-zone", "🧭️change-mechanical-ventilation-zone"),
    ("change-mechanical-ventilation-schedule", "📆️change-mechanical-ventilation-schedule"),
    ("change-mechanical-ventilation-design-flow", "🚿️change-mechanical-ventilation-design-flow"),
    ("change-mechanical-ventilation-fan-total-efficiency", "💠️change-mechanical-ventilation-fan-total-efficiency"),
    ("change-mechanical-ventilation-fan-delta-pressure", "🎈️change-mechanical-ventilation-fan-delta-pressure"),
    ("change-infiltration-method", "🔬️change-infiltration-method"),
    ("change-infiltration-design-flow-ach", "🔄️change-infiltration-design-flow-ach"),
    ("change-infiltration-effective-leakage-area", "🕳️change-infiltration-effective-leakage-area"),
    ("change-infiltration-discharge-coefficient", "🚰️change-infiltration-discharge-coefficient"),
    ("change-infiltration-stack-height", "🏭️change-infiltration-stack-height"),
    ("create-thermostat", "🩺️create-thermostat"),
    ("delete-thermostat", "🛑️delete-thermostat"),
    ("change-thermostat-zone", "🛖️change-thermostat-zone"),
    ("change-thermostat-heating-setpoint-schedule", "🥵️change-thermostat-heating-setpoint-schedule"),
    ("change-thermostat-cooling-setpoint-schedule", "🐧️change-thermostat-cooling-setpoint-schedule"),
    ("change-thermostat-heating-throttle-range", "🎚️change-thermostat-heating-throttle-range"),
    ("change-thermostat-cooling-throttle-range", "🎛️change-thermostat-cooling-throttle-range"),
    ("create-humidistat", "🌂️create-humidistat"),
    ("delete-humidistat", "🏜️delete-humidistat"),
    ("change-humidistat-zone", "🏙️change-humidistat-zone"),
    ("change-humidistat-humidifying-setpoint-schedule", "☔️change-humidistat-humidifying-setpoint-schedule"),
    ("change-humidistat-dehumidifying-setpoint-schedule", "🏝️change-humidistat-dehumidifying-setpoint-schedule"),
    ("change-humidistat-humidifying-throttle-range", "🌧️change-humidistat-humidifying-throttle-range"),
    ("change-humidistat-dehumidifying-throttle-range", "🧻️change-humidistat-dehumidifying-throttle-range"),
    ("create-ideal-loads-system", "🫁️create-ideal-loads-system"),
    ("delete-ideal-loads-system", "🫥️delete-ideal-loads-system"),
    ("change-ideal-loads-system-zone", "🏢️change-ideal-loads-system-zone"),
    ("change-ideal-loads-system-max-heating-supply-air-temp", "🔴️change-ideal-loads-system-max-heating-supply-air-temp"),
    ("change-ideal-loads-system-min-cooling-supply-air-temp", "🔵️change-ideal-loads-system-min-cooling-supply-air-temp"),
    ("change-ideal-loads-system-max-heating-capacity", "⛽️change-ideal-loads-system-max-heating-capacity"),
    ("change-ideal-loads-system-max-cooling-capacity", "🟧️change-ideal-loads-system-max-cooling-capacity"),
    ("change-ideal-loads-system-outdoor-air-per-person", "🧍️change-ideal-loads-system-outdoor-air-per-person"),
    ("change-ideal-loads-system-outdoor-air-per-area", "🔳️change-ideal-loads-system-outdoor-air-per-area"),
    ("create-zone-equipment", "🛠️create-zone-equipment"),
    ("delete-zone-equipment", "🗑️delete-zone-equipment"),
    ("change-zone-equipment-zone", "🏬️change-zone-equipment-zone"),
    ("change-zone-equipment-type", "🔧️change-zone-equipment-type"),
    ("change-zone-equipment-priority", "🎗️change-zone-equipment-priority"),
    ("change-zone-equipment-heating-capacity", "🧇️change-zone-equipment-heating-capacity"),
    ("change-zone-equipment-cooling-capacity", "🍧️change-zone-equipment-cooling-capacity"),
    ("create-daylight-zone", "🔭️create-daylight-zone"),
    ("delete-daylight-zone", "🌗️delete-daylight-zone"),
    ("change-daylight-zone-zone", "🏫️change-daylight-zone-zone"),
    ("change-daylight-zone-illuminance-target", "🪔️change-daylight-zone-illuminance-target"),
    ("change-daylight-zone-glare-limit", "🕶️change-daylight-zone-glare-limit"),
    ("change-daylight-zone-window-transmittance", "🥃️change-daylight-zone-window-transmittance"),
    ("create-sizing-object", "📶️create-sizing-object"),
    ("delete-sizing-object", "🪒️delete-sizing-object"),
    ("change-sizing-object-zone", "🏨️change-sizing-object-zone"),
    ("change-sizing-object-sizing-type", "🧾️change-sizing-object-sizing-type"),
    ("change-sizing-object-design-day-type", "🌥️change-sizing-object-design-day-type"),
    ("create-room-air-model-assignment", "🛏️create-room-air-model-assignment"),
    ("delete-room-air-model-assignment", "🧺️delete-room-air-model-assignment"),
    ("change-room-air-model", "🪭️change-room-air-model"),
    ("create-setpoint-manager", "📌️create-setpoint-manager"),
    ("delete-setpoint-manager", "🍄️delete-setpoint-manager"),
    ("rename-setpoint-manager", "🖇️rename-setpoint-manager"),
    ("replace-setpoint-manager-kind", "🔃️replace-setpoint-manager-kind"),
    ("change-setpoint-manager-schedule", "🎼️change-setpoint-manager-schedule"),
    ("create-air-loop", "🛞️create-air-loop"),
    ("delete-air-loop", "🥀️delete-air-loop"),
    ("rename-air-loop", "📇️rename-air-loop"),
    ("change-air-loop-supply-node", "↗️change-air-loop-supply-node"),
    ("change-air-loop-return-node", "↘️change-air-loop-return-node"),
    ("change-air-loop-design-supply-air-flow", "🍥️change-air-loop-design-supply-air-flow"),
    ("add-air-loop-terminal-zone", "🪺️add-air-loop-terminal-zone"),
    ("remove-air-loop-terminal-zone", "🪹️remove-air-loop-terminal-zone"),
    ("create-plant-loop", "⚗️create-plant-loop"),
    ("delete-plant-loop", "💣️delete-plant-loop"),
    ("rename-plant-loop", "📛️rename-plant-loop"),
    ("change-plant-loop-type", "♻️change-plant-loop-type"),
    ("change-plant-loop-supply-temperature", "☕️change-plant-loop-supply-temperature"),
    ("change-plant-loop-return-temperature", "🧫️change-plant-loop-return-temperature"),
    ("change-plant-loop-design-flow", "🚤️change-plant-loop-design-flow"),
    ("add-plant-loop-equipment", "🔩️add-plant-loop-equipment"),
    ("remove-plant-loop-equipment", "⚙️remove-plant-loop-equipment"),
    ("create-outdoor-air-system", "🌲️create-outdoor-air-system"),
    ("delete-outdoor-air-system", "🍂️delete-outdoor-air-system"),
    ("change-outdoor-air-system-air-loop", "⛓️change-outdoor-air-system-air-loop"),
    ("change-outdoor-air-system-min-oa-flow", "🦋️change-outdoor-air-system-min-oa-flow"),
    ("change-outdoor-air-system-economizer-enabled", "💰️change-outdoor-air-system-economizer-enabled"),
    ("create-electrical-load-center", "🏦️create-electrical-load-center"),
    ("delete-electrical-load-center", "🔻️delete-electrical-load-center"),
    ("rename-electrical-load-center", "🖊️rename-electrical-load-center"),
    ("add-electrical-load-center-pv", "☄️add-electrical-load-center-pv"),
    ("remove-electrical-load-center-pv", "🌘️remove-electrical-load-center-pv"),
    ("add-electrical-load-center-battery", "🔋️add-electrical-load-center-battery"),
    ("remove-electrical-load-center-battery", "🪝️remove-electrical-load-center-battery"),
    ("create-pv-system", "✨️create-pv-system"),
    ("delete-pv-system", "🌒️delete-pv-system"),
    ("change-pv-system-dc-capacity", "⚛️change-pv-system-dc-capacity"),
    ("change-pv-system-area", "🟨️change-pv-system-area"),
    ("change-pv-system-tilt", "📈️change-pv-system-tilt"),
    ("change-pv-system-azimuth", "🧿️change-pv-system-azimuth"),
    ("change-pv-system-module-efficiency", "🎖️change-pv-system-module-efficiency"),
    ("change-pv-system-inverter-efficiency", "♌️change-pv-system-inverter-efficiency"),
    ("create-battery", "🪙️create-battery"),
    ("delete-battery", "♒️delete-battery"),
    ("change-battery-capacity", "🥫️change-battery-capacity"),
    ("change-battery-max-charge", "⏫️change-battery-max-charge"),
    ("change-battery-max-discharge", "⏬️change-battery-max-discharge"),
    ("change-battery-round-trip-efficiency", "🥉️change-battery-round-trip-efficiency"),
    ("create-shw-system", "🛀️create-shw-system"),
    ("delete-shw-system", "🚱️delete-shw-system"),
    ("change-shw-system-heater-capacity", "🍵️change-shw-system-heater-capacity"),
    ("change-shw-system-storage-volume", "🛢️change-shw-system-storage-volume"),
    ("change-shw-system-setpoint", "🏹️change-shw-system-setpoint"),
    ("change-shw-system-schedule", "🕐️change-shw-system-schedule"),
    ("create-solar-thermal-system", "🌄️create-solar-thermal-system"),
    ("delete-solar-thermal-system", "🌆️delete-solar-thermal-system"),
    ("change-solar-thermal-system-collector-area", "🟩️change-solar-thermal-system-collector-area"),
    ("change-solar-thermal-system-efficiency", "🏅️change-solar-thermal-system-efficiency"),
    ("change-solar-thermal-system-storage-volume", "🧃️change-solar-thermal-system-storage-volume"),
    ("change-solar-thermal-system-tilt", "🔼️change-solar-thermal-system-tilt"),
    ("change-solar-thermal-system-azimuth", "⛵️change-solar-thermal-system-azimuth"),
    ("create-refrigeration-system", "❄️create-refrigeration-system"),
    ("delete-refrigeration-system", "🫠️delete-refrigeration-system"),
    ("change-refrigeration-system-case-count", "🗄️change-refrigeration-system-case-count"),
    ("change-refrigeration-system-design-load", "🏋️change-refrigeration-system-design-load"),
    ("change-refrigeration-system-defrost-schedule", "🕑️change-refrigeration-system-defrost-schedule"),
    ("create-water-system", "🚽️create-water-system"),
    ("delete-water-system", "🧼️delete-water-system"),
    ("change-water-system-fixture-count", "🪣️change-water-system-fixture-count"),
    ("change-water-system-peak-flow", "🚾️change-water-system-peak-flow"),
    ("change-water-system-schedule", "🕒️change-water-system-schedule"),
    ("create-fault", "⚠️create-fault"),
    ("delete-fault", "🩹️delete-fault"),
    ("change-fault-target-equipment", "🎣️change-fault-target-equipment"),
    ("change-fault-type", "🐛️change-fault-type"),
    ("change-fault-severity", "🌶️change-fault-severity"),
    ("change-fault-start-schedule", "🕓️change-fault-start-schedule"),
    ("create-space-list", "📋️create-space-list"),
    ("delete-space-list", "🗒️delete-space-list"),
    ("rename-space-list", "🪶️rename-space-list"),
    ("add-space-list-member", "➡️add-space-list-member"),
    ("remove-space-list-member", "🪤️remove-space-list-member"),
    ("create-thermal-enclosure", "🏟️create-thermal-enclosure"),
    ("delete-thermal-enclosure", "🏯️delete-thermal-enclosure"),
    ("rename-thermal-enclosure", "🖋️rename-thermal-enclosure"),
    ("add-thermal-enclosure-zone", "🔒️add-thermal-enclosure-zone"),
    ("remove-thermal-enclosure-zone", "🔓️remove-thermal-enclosure-zone"),
    ("create-constant-schedule", "🕜️create-constant-schedule"),
    ("delete-constant-schedule", "📍️delete-constant-schedule"),
    ("change-constant-schedule-value", "🕝️change-constant-schedule-value"),
    ("create-daily-schedule", "🕞️create-daily-schedule"),
    ("delete-daily-schedule", "🌓️delete-daily-schedule"),
    ("replace-daily-schedule-hourly-values", "🕔️replace-daily-schedule-hourly-values"),
    ("change-daily-schedule-interpolation", "🕕️change-daily-schedule-interpolation"),
    ("change-daily-schedule-limits", "🕟️change-daily-schedule-limits"),
    ("create-weekly-schedule", "🗓️create-weekly-schedule"),
    ("delete-weekly-schedule", "🕖️delete-weekly-schedule"),
    ("change-weekly-schedule-day", "🕗️change-weekly-schedule-day"),
    ("create-annual-schedule", "📚️create-annual-schedule"),
    ("delete-annual-schedule", "📕️delete-annual-schedule"),
    ("insert-annual-schedule-rule", "📗️insert-annual-schedule-rule"),
    ("remove-annual-schedule-rule", "📙️remove-annual-schedule-rule"),
    ("reorder-annual-schedule-rules", "🗂️reorder-annual-schedule-rules"),
    ("change-annual-schedule-default-daily-schedule", "🎌️change-annual-schedule-default-daily-schedule"),
    ("change-annual-schedule-holiday-daily-schedule", "🎄️change-annual-schedule-holiday-daily-schedule"),
    ("add-annual-schedule-holiday", "🎉️add-annual-schedule-holiday"),
    ("remove-annual-schedule-holiday", "🎊️remove-annual-schedule-holiday"),
    ("create-time-series-schedule", "🪗️create-time-series-schedule"),
    ("delete-time-series-schedule", "🎞️delete-time-series-schedule"),
    ("replace-time-series-schedule-values", "🕘️replace-time-series-schedule-values"),
    ("change-time-series-schedule-timestep", "🕙️change-time-series-schedule-timestep"),
];
//#endregion 🔖️Aggregate

//#region 🌉️TestBridge
/// 🔮️ Reports the forward and inverse behavior of one committed language-neutral vector.
pub fn energy_model_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    use semio_framework_os_kernel::ToValue;
    let decode_snapshot = |text: &str| -> Result<EnergyModelSnapshot, String> { pack::json::from_json_str::<EnergyModelSnapshot>(text).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: EnergyModelMutation = pack::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    // 🌉️ `MutationMessage` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`) is a
    // framework-owned type that has not itself gained `ToValue`/`FromValue` — its two call sites
    // here go through the PRE-EXISTING `protocol::to_dsl_value` serde bridge (framework-internal,
    // exempt) and land in `pack::json::Value` via `pack::json::from_dsl_value`.
    let messages_json = protocol::to_dsl_value(forward.messages()).map(|value| pack::json::from_dsl_value(&value)).map_err(|error| error)?;
    let inverse_messages_json = protocol::to_dsl_value(&inverse_messages).map(|value| pack::json::from_dsl_value(&value)).map_err(|error| error)?;
    let report = pack::json::object([
        ("base".to_string(), pack::json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), pack::json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), pack::json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), pack::json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), messages_json),
        ("inverseSteps".to_string(), pack::json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), pack::json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), inverse_messages_json),
    ]);
    Ok(pack::json::to_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧵️WireProbes
/// 🧵️ One representative value per declared variant — the codec round-trip corpus, so wire coverage
/// grows with the enum instead of with a hand-maintained list of tests.
#[cfg(test)]
pub fn wire_probes() -> Vec<EnergyModelMutation> {
    vec![
        rename_model("Probe".to_string()),
        change_model_version("2".to_string()),
        update_site(52.4, 9.7, 55.0, 1.0, 0.0),
        update_ground_temperature(vec![10.0; 12], vec![9.0; 12], 8.0),
        update_run_period(1, 1, 1, 31, 2026),
        replace_airflow_network(true, vec![1], vec![1], 0, vec![7]),
        add_output_variable("Zone Mean Air Temperature".to_string(), "ZONE ONE".to_string(), crate::model::OutputReportFrequency::Hourly),
        remove_output_variable("Zone Mean Air Temperature".to_string(), "ZONE ONE".to_string()),
        bind_weather_file("hannover!s.stdio.semio@v1/epw".to_string()),
        unbind_weather_file(),
        connect_referenced_model("doc-2!s.stdio.semio@v1/model".to_string()),
        disconnect_referenced_model(),
        rename_zone(crate::model::EntityId(1), "ZONE 1".to_string()),
        change_zone_volume(crate::model::EntityId(1), 129.6),
        change_zone_multiplier(crate::model::EntityId(1), 4),
        change_zone_conditioned(crate::model::EntityId(1), false),
        change_zone_floor_area_participation(crate::model::EntityId(1), false),
        create_zone(crate::model::EntityId(2), "ZONE TWO".to_string(), 129.6, 1, true, true),
        delete_zone(crate::model::EntityId(2)),
        create_space(crate::model::EntityId(2), "SPACE ONE".to_string(), crate::model::EntityId(1), 48.0),
        delete_space(crate::model::EntityId(2)),
        rename_space(crate::model::EntityId(2), "SPACE 1".to_string()),
        change_space_floor_area(crate::model::EntityId(2), 48.0),
        change_space_zone(crate::model::EntityId(2), crate::model::EntityId(3)),
        create_surface(crate::model::EntityId(3), "WALL SOUTH".to_string(), crate::model::EntityId(1), crate::model::SurfaceClass::ExteriorWall, vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7]], crate::model::EntityId(2), crate::model::OutsideBoundaryKind::OutdoorAir, None, true, true, 1),
        delete_surface(crate::model::EntityId(3)),
        rename_surface(crate::model::EntityId(3), "WALL S".to_string()),
        change_surface_zone(crate::model::EntityId(3), crate::model::EntityId(7)),
        change_surface_class(crate::model::EntityId(3), crate::model::SurfaceClass::Roof),
        replace_surface_vertices(crate::model::EntityId(3), vec![[0.0, 0.0, 0.0], [6.0, 0.0, 0.0], [6.0, 0.0, 2.7]]),
        change_surface_construction(crate::model::EntityId(3), crate::model::EntityId(8)),
        change_surface_boundary_condition(crate::model::EntityId(3), crate::model::OutsideBoundaryKind::Ground, None),
        change_surface_sun_exposed(crate::model::EntityId(3), false),
        change_surface_wind_exposed(crate::model::EntityId(3), false),
        change_surface_multiplier(crate::model::EntityId(3), 4),
        create_fenestration(crate::model::EntityId(5), "WINDOW SOUTH".to_string(), crate::model::EntityId(3), 3.0, 0.787, 0.86, 6.0, 2.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None),
        delete_fenestration(crate::model::EntityId(5)),
        rename_fenestration(crate::model::EntityId(5), "WINDOW S".to_string()),
        change_fenestration_surface(crate::model::EntityId(5), crate::model::EntityId(6)),
        change_fenestration_u_value(crate::model::EntityId(5), 2.74),
        change_fenestration_shgc(crate::model::EntityId(5), 0.76),
        change_fenestration_vlt(crate::model::EntityId(5), 0.84),
        change_fenestration_area(crate::model::EntityId(5), 12.0),
        change_fenestration_frame_conductance(crate::model::EntityId(5), 1.2),
        change_fenestration_divider_conductance(crate::model::EntityId(5), 0.4),
        create_shading_surface(crate::model::EntityId(11), "SITE AWNING".to_string(), vec![[0.0, -2.0, 3.0], [8.0, -2.0, 3.0], [8.0, 0.0, 3.0]], None),
        delete_shading_surface(crate::model::EntityId(11)),
        rename_shading_surface(crate::model::EntityId(11), "AWNING".to_string()),
        replace_shading_surface_vertices(crate::model::EntityId(11), vec![[0.0, -3.0, 3.0], [8.0, -3.0, 3.0], [8.0, 0.0, 3.0]]),
        change_shading_surface_transmittance_schedule(crate::model::EntityId(11), Some(crate::model::ScheduleId(1))),
        connect_surfaces(crate::model::EntityId(3), crate::model::EntityId(6)),
        disconnect_surfaces(crate::model::EntityId(3), crate::model::EntityId(6)),
        bind_fenestration_glazing_construction(crate::model::EntityId(5), crate::model::EntityId(7)),
        clear_fenestration_glazing_construction(crate::model::EntityId(5)),
        change_fenestration_height(crate::model::EntityId(5), 2.4),
        change_fenestration_sill_height(crate::model::EntityId(5), 0.9),
        change_fenestration_overhang_depth(crate::model::EntityId(5), 1.0),
        change_fenestration_overhang_offset(crate::model::EntityId(5), 0.5),
        change_fenestration_fin_depth(crate::model::EntityId(5), 1.0),
        change_fenestration_fin_offset(crate::model::EntityId(5), 0.2),
        create_material(2, crate::model::EntityId(3), "CONCRETE SLAB".into(), 0.08, 1.13, 1400.0, 1000.0, 0.9, 0.6, 0.6),
        delete_material(crate::model::EntityId(1)),
        rename_material(crate::model::EntityId(1), "TIMBER FLOORING".to_string()),
        change_material_thickness(crate::model::EntityId(1), 0.02),
        change_material_conductivity(crate::model::EntityId(1), 0.25),
        change_material_density(crate::model::EntityId(1), 1200.0),
        change_material_specific_heat(crate::model::EntityId(1), 1000.0),
        change_material_thermal_absorptance(crate::model::EntityId(1), 0.85),
        change_material_solar_absorptance(crate::model::EntityId(1), 0.7),
        change_material_visible_absorptance(crate::model::EntityId(1), 0.75),
        create_construction(2, crate::model::EntityId(12), "ROOF".into(), vec![crate::model::EntityId(2), crate::model::EntityId(1)]),
        delete_construction(crate::model::EntityId(11)),
        rename_construction(crate::model::EntityId(10), "LTWALL INSULATED".to_string()),
        add_construction_layer(crate::model::EntityId(10), 2, crate::model::EntityId(1)),
        remove_construction_layer(crate::model::EntityId(10), 1),
        reorder_construction_layers(crate::model::EntityId(10), vec![crate::model::EntityId(2), crate::model::EntityId(1)]),
        create_people_gain(1, crate::model::EntityId(21), crate::model::EntityId(2), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 0.1, 0.6, 0.4, 0.3),
        delete_people_gain(crate::model::EntityId(20)),
        change_people_gain_zone(crate::model::EntityId(20), crate::model::EntityId(2)),
        change_people_gain_schedule(crate::model::EntityId(20), crate::model::ScheduleId(3)),
        change_people_gain_activity_schedule(crate::model::EntityId(20), crate::model::ScheduleId(3)),
        change_people_gain_people_per_area(crate::model::EntityId(20), 0.1),
        change_people_gain_sensible_fraction(crate::model::EntityId(20), 0.7),
        change_people_gain_latent_fraction(crate::model::EntityId(20), 0.3),
        change_people_gain_radiant_fraction(crate::model::EntityId(20), 0.5),
        create_lighting_gain(1, crate::model::EntityId(31), crate::model::EntityId(2), crate::model::ScheduleId(1), 8.0, 0.6, 0.2, 0.0),
        delete_lighting_gain(crate::model::EntityId(30)),
        change_lighting_gain_zone(crate::model::EntityId(30), crate::model::EntityId(2)),
        change_lighting_gain_schedule(crate::model::EntityId(30), crate::model::ScheduleId(3)),
        change_lighting_gain_watts_per_area(crate::model::EntityId(30), 12.5),
        change_lighting_gain_radiant_fraction(crate::model::EntityId(30), 0.7),
        change_lighting_gain_visible_fraction(crate::model::EntityId(30), 0.25),
        change_lighting_gain_return_air_fraction(crate::model::EntityId(30), 0.1),
        create_equipment_gain(1, crate::model::EntityId(41), crate::model::EntityId(2), crate::model::ScheduleId(1), 2.5, 0.5, 0.0),
        delete_equipment_gain(crate::model::EntityId(40)),
        change_equipment_gain_zone(crate::model::EntityId(40), crate::model::EntityId(2)),
        change_equipment_gain_schedule(crate::model::EntityId(40), crate::model::ScheduleId(3)),
        change_equipment_gain_watts_per_area(crate::model::EntityId(40), 5.0),
        change_equipment_gain_radiant_fraction(crate::model::EntityId(40), 0.4),
        change_equipment_gain_latent_fraction(crate::model::EntityId(40), 0.2),
        create_infiltration(1, crate::model::EntityId(51), crate::model::EntityId(2), crate::model::ScheduleId(1), crate::air_exchange::InfiltrationMethod::ScheduledAch, 0.5, 0.0, 0.0, 1.0, 2.7, 1.0, 0.0, 0.0, 0.0),
        delete_infiltration(crate::model::EntityId(50)),
        change_infiltration_zone(crate::model::EntityId(50), crate::model::EntityId(2)),
        change_infiltration_schedule(crate::model::EntityId(50), crate::model::ScheduleId(3)),
        change_infiltration_flow_per_exterior_area(crate::model::EntityId(50), 0.0003),
        change_infiltration_constant_term_coefficient(crate::model::EntityId(50), 0.606),
        change_infiltration_temperature_term_coefficient(crate::model::EntityId(50), 0.03636),
        change_infiltration_velocity_term_coefficient(crate::model::EntityId(50), 0.1177),
        change_infiltration_velocity_squared_term_coefficient(crate::model::EntityId(50), 0.000103),
        create_mechanical_ventilation(1, crate::model::EntityId(61), crate::model::EntityId(2), crate::model::ScheduleId(1), 0.08, 0.65, 250.0),
        delete_mechanical_ventilation(crate::model::EntityId(60)),
        change_mechanical_ventilation_zone(crate::model::EntityId(60), crate::model::EntityId(2)),
        change_mechanical_ventilation_schedule(crate::model::EntityId(60), crate::model::ScheduleId(3)),
        change_mechanical_ventilation_design_flow(crate::model::EntityId(60), 0.12),
        change_mechanical_ventilation_fan_total_efficiency(crate::model::EntityId(60), 0.8),
        change_mechanical_ventilation_fan_delta_pressure(crate::model::EntityId(60), 500.0),
        change_infiltration_method(crate::model::EntityId(50), crate::air_exchange::InfiltrationMethod::PerExteriorArea),
        change_infiltration_design_flow_ach(crate::model::EntityId(50), 1.0),
        change_infiltration_effective_leakage_area(crate::model::EntityId(50), 0.05),
        change_infiltration_discharge_coefficient(crate::model::EntityId(50), 0.65),
        change_infiltration_stack_height(crate::model::EntityId(50), 3.0),
        create_thermostat(crate::model::EntityId(10), crate::model::EntityId(1), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 1.0, 1.0),
        delete_thermostat(crate::model::EntityId(10)),
        change_thermostat_zone(crate::model::EntityId(10), crate::model::EntityId(2)),
        change_thermostat_heating_setpoint_schedule(crate::model::EntityId(10), crate::model::ScheduleId(2)),
        change_thermostat_cooling_setpoint_schedule(crate::model::EntityId(10), crate::model::ScheduleId(1)),
        change_thermostat_heating_throttle_range(crate::model::EntityId(10), 2.0),
        change_thermostat_cooling_throttle_range(crate::model::EntityId(10), 2.0),
        create_humidistat(crate::model::EntityId(11), crate::model::EntityId(1), crate::model::ScheduleId(1), crate::model::ScheduleId(2), 5.0, 5.0),
        delete_humidistat(crate::model::EntityId(11)),
        change_humidistat_zone(crate::model::EntityId(11), crate::model::EntityId(2)),
        change_humidistat_humidifying_setpoint_schedule(crate::model::EntityId(11), crate::model::ScheduleId(2)),
        change_humidistat_dehumidifying_setpoint_schedule(crate::model::EntityId(11), crate::model::ScheduleId(1)),
        change_humidistat_humidifying_throttle_range(crate::model::EntityId(11), 10.0),
        change_humidistat_dehumidifying_throttle_range(crate::model::EntityId(11), 10.0),
        create_ideal_loads_system(crate::model::EntityId(12), crate::model::EntityId(1), 50.0, 13.0, false, 0.0, false, 0.0, 0.0, 0.0),
        delete_ideal_loads_system(crate::model::EntityId(12)),
        change_ideal_loads_system_zone(crate::model::EntityId(12), crate::model::EntityId(2)),
        change_ideal_loads_system_max_heating_supply_air_temp(crate::model::EntityId(12), 45.0),
        change_ideal_loads_system_min_cooling_supply_air_temp(crate::model::EntityId(12), 12.0),
        change_ideal_loads_system_max_heating_capacity(crate::model::EntityId(12), true, 4000.0),
        change_ideal_loads_system_max_cooling_capacity(crate::model::EntityId(12), true, 5000.0),
        change_ideal_loads_system_outdoor_air_per_person(crate::model::EntityId(12), 0.0025),
        change_ideal_loads_system_outdoor_air_per_area(crate::model::EntityId(12), 0.0003),
        create_zone_equipment(crate::model::EntityId(13), crate::model::EntityId(1), crate::model::ZoneEquipmentType::Baseboard, 1, 2000.0, 0.0),
        delete_zone_equipment(crate::model::EntityId(13)),
        change_zone_equipment_zone(crate::model::EntityId(13), crate::model::EntityId(2)),
        change_zone_equipment_type(crate::model::EntityId(13), crate::model::ZoneEquipmentType::FanCoil),
        change_zone_equipment_priority(crate::model::EntityId(13), 2),
        change_zone_equipment_heating_capacity(crate::model::EntityId(13), 3000.0),
        change_zone_equipment_cooling_capacity(crate::model::EntityId(13), 1500.0),
        create_daylight_zone(crate::model::EntityId(14), crate::model::EntityId(1), 500.0, 22.0, 0.6),
        delete_daylight_zone(crate::model::EntityId(14)),
        change_daylight_zone_zone(crate::model::EntityId(14), crate::model::EntityId(2)),
        change_daylight_zone_illuminance_target(crate::model::EntityId(14), 300.0),
        change_daylight_zone_glare_limit(crate::model::EntityId(14), 20.0),
        change_daylight_zone_window_transmittance(crate::model::EntityId(14), 0.5),
        create_sizing_object(crate::model::EntityId(15), crate::model::EntityId(1), crate::model::SizingType::Heating, crate::model::DesignDayType::Heating),
        delete_sizing_object(crate::model::EntityId(15)),
        change_sizing_object_zone(crate::model::EntityId(15), crate::model::EntityId(2)),
        change_sizing_object_sizing_type(crate::model::EntityId(15), crate::model::SizingType::Cooling),
        change_sizing_object_design_day_type(crate::model::EntityId(15), crate::model::DesignDayType::Cooling),
        create_room_air_model_assignment(crate::model::EntityId(1), crate::model::RoomAirModelType::WellMixed),
        delete_room_air_model_assignment(crate::model::EntityId(1)),
        change_room_air_model(crate::model::EntityId(1), crate::model::RoomAirModelType::TwoNodeBuoyancy),
        create_setpoint_manager(crate::model::EntityId(16), "SUPPLY SPM".to_string(), "Scheduled".to_string(), 0.0, 0.0, 0.0, 0.0, true, crate::model::ScheduleId(1)),
        delete_setpoint_manager(crate::model::EntityId(16)),
        rename_setpoint_manager(crate::model::EntityId(16), "RESET SPM".to_string()),
        replace_setpoint_manager_kind(crate::model::EntityId(16), "OutdoorAirReset".to_string(), -10.0, 20.0, 16.0, 12.0),
        change_setpoint_manager_schedule(crate::model::EntityId(16), true, crate::model::ScheduleId(2)),
        create_air_loop(crate::model::EntityId(17), "MAIN AIR LOOP".to_string(), 1, 2, 1.2, vec![crate::model::EntityId(1)]),
        delete_air_loop(crate::model::EntityId(17)),
        rename_air_loop(crate::model::EntityId(17), "PRIMARY AIR LOOP".to_string()),
        change_air_loop_supply_node(crate::model::EntityId(17), 5),
        change_air_loop_return_node(crate::model::EntityId(17), 6),
        change_air_loop_design_supply_air_flow(crate::model::EntityId(17), 1.6),
        add_air_loop_terminal_zone(crate::model::EntityId(17), crate::model::EntityId(2)),
        remove_air_loop_terminal_zone(crate::model::EntityId(17), crate::model::EntityId(1)),
        create_plant_loop(crate::model::EntityId(18), "HOT WATER LOOP".to_string(), crate::model::PlantLoopType::Heating, 80.0, 60.0, 2.0, Vec::new()),
        delete_plant_loop(crate::model::EntityId(18)),
        rename_plant_loop(crate::model::EntityId(18), "BOILER LOOP".to_string()),
        change_plant_loop_type(crate::model::EntityId(18), crate::model::PlantLoopType::Cooling),
        change_plant_loop_supply_temperature(crate::model::EntityId(18), 70.0),
        change_plant_loop_return_temperature(crate::model::EntityId(18), 55.0),
        change_plant_loop_design_flow(crate::model::EntityId(18), 3.0),
        add_plant_loop_equipment(crate::model::EntityId(18), crate::model::EntityId(50)),
        remove_plant_loop_equipment(crate::model::EntityId(18), crate::model::EntityId(50)),
        create_outdoor_air_system(crate::model::EntityId(19), crate::model::EntityId(17), 0.2, false),
        delete_outdoor_air_system(crate::model::EntityId(19)),
        change_outdoor_air_system_air_loop(crate::model::EntityId(19), crate::model::EntityId(20)),
        change_outdoor_air_system_min_oa_flow(crate::model::EntityId(19), 0.35),
        change_outdoor_air_system_economizer_enabled(crate::model::EntityId(19), true),
        create_electrical_load_center(2, crate::model::EntityId(322), "ROOF PANEL".into(), Vec::new(), vec![crate::model::EntityId(301)], Vec::new()),
        delete_electrical_load_center(crate::model::EntityId(321)),
        rename_electrical_load_center(crate::model::EntityId(320), "MAIN DISTRIBUTION".to_string()),
        add_electrical_load_center_pv(crate::model::EntityId(320), 1, crate::model::EntityId(301)),
        remove_electrical_load_center_pv(crate::model::EntityId(320), crate::model::EntityId(300)),
        add_electrical_load_center_battery(crate::model::EntityId(320), 0, crate::model::EntityId(310)),
        remove_electrical_load_center_battery(crate::model::EntityId(321), crate::model::EntityId(310)),
        create_pv_system(2, crate::model::EntityId(302), 8000.0, 44.0, 25.0, 200.0, 0.19, 0.97),
        delete_pv_system(crate::model::EntityId(300)),
        change_pv_system_dc_capacity(crate::model::EntityId(300), 7000.0),
        change_pv_system_area(crate::model::EntityId(300), 42.0),
        change_pv_system_tilt(crate::model::EntityId(300), 35.0),
        change_pv_system_azimuth(crate::model::EntityId(300), 170.0),
        change_pv_system_module_efficiency(crate::model::EntityId(300), 0.21),
        change_pv_system_inverter_efficiency(crate::model::EntityId(300), 0.98),
        create_battery(1, crate::model::EntityId(311), 27.0, 7000.0, 7000.0, 0.92),
        delete_battery(crate::model::EntityId(310)),
        change_battery_capacity(crate::model::EntityId(310), 20.0),
        change_battery_max_charge(crate::model::EntityId(310), 7000.0),
        change_battery_max_discharge(crate::model::EntityId(310), 6000.0),
        change_battery_round_trip_efficiency(crate::model::EntityId(310), 0.95),
        create_shw_system(1, crate::model::EntityId(401), 6000.0, 0.2, 60.0, crate::model::ScheduleId(2)),
        delete_shw_system(crate::model::EntityId(400)),
        change_shw_system_heater_capacity(crate::model::EntityId(400), 6000.0),
        change_shw_system_storage_volume(crate::model::EntityId(400), 0.2),
        change_shw_system_setpoint(crate::model::EntityId(400), 60.0),
        change_shw_system_schedule(crate::model::EntityId(400), crate::model::ScheduleId(2)),
        create_solar_thermal_system(1, crate::model::EntityId(411), 6.0, 0.6, 0.4, 40.0, 190.0),
        delete_solar_thermal_system(crate::model::EntityId(410)),
        change_solar_thermal_system_collector_area(crate::model::EntityId(410), 6.0),
        change_solar_thermal_system_efficiency(crate::model::EntityId(410), 0.62),
        change_solar_thermal_system_storage_volume(crate::model::EntityId(410), 0.5),
        change_solar_thermal_system_tilt(crate::model::EntityId(410), 35.0),
        change_solar_thermal_system_azimuth(crate::model::EntityId(410), 200.0),
        create_refrigeration_system(1, crate::model::EntityId(421), 2, 8000.0, crate::model::ScheduleId(2)),
        delete_refrigeration_system(crate::model::EntityId(420)),
        change_refrigeration_system_case_count(crate::model::EntityId(420), 6),
        change_refrigeration_system_design_load(crate::model::EntityId(420), 15000.0),
        change_refrigeration_system_defrost_schedule(crate::model::EntityId(420), crate::model::ScheduleId(2)),
        create_water_system(1, crate::model::EntityId(431), 4, 0.15, crate::model::ScheduleId(2)),
        delete_water_system(crate::model::EntityId(430)),
        change_water_system_fixture_count(crate::model::EntityId(430), 9),
        change_water_system_peak_flow(crate::model::EntityId(430), 0.4),
        change_water_system_schedule(crate::model::EntityId(430), crate::model::ScheduleId(2)),
        create_fault(1, crate::model::EntityId(441), crate::model::EntityId(501), crate::model::FaultType::SensorBias, 0.5, crate::model::ScheduleId(2)),
        delete_fault(crate::model::EntityId(440)),
        change_fault_target_equipment(crate::model::EntityId(440), crate::model::EntityId(501)),
        change_fault_type(crate::model::EntityId(440), crate::model::FaultType::SensorBias),
        change_fault_severity(crate::model::EntityId(440), 0.6),
        change_fault_start_schedule(crate::model::EntityId(440), crate::model::ScheduleId(2)),
        create_space_list(2, crate::model::EntityId(212), "ATRIUM".into(), vec![crate::model::EntityId(201)]),
        delete_space_list(crate::model::EntityId(211)),
        rename_space_list(crate::model::EntityId(210), "CORE SPACES".to_string()),
        add_space_list_member(crate::model::EntityId(210), 1, crate::model::EntityId(201)),
        remove_space_list_member(crate::model::EntityId(210), crate::model::EntityId(200)),
        create_thermal_enclosure(2, crate::model::EntityId(222), "ROOF BLOCK".into(), vec![crate::model::EntityId(2)]),
        delete_thermal_enclosure(crate::model::EntityId(221)),
        rename_thermal_enclosure(crate::model::EntityId(220), "NORTH ENCLOSURE".to_string()),
        add_thermal_enclosure_zone(crate::model::EntityId(220), 1, crate::model::EntityId(2)),
        remove_thermal_enclosure_zone(crate::model::EntityId(220), crate::model::EntityId(1)),
        create_constant_schedule(2, crate::model::ScheduleId(3), 0.75),
        delete_constant_schedule(crate::model::ScheduleId(2)),
        change_constant_schedule_value(crate::model::ScheduleId(1), 0.25),
        create_daily_schedule(3, crate::model::ScheduleId(13), vec![21.0; 24], crate::schedule::ScheduleInterpolation::Continuous, None, None),
        delete_daily_schedule(crate::model::ScheduleId(12)),
        replace_daily_schedule_hourly_values(crate::model::ScheduleId(10), vec![22.0; 24]),
        change_daily_schedule_interpolation(crate::model::ScheduleId(10), crate::schedule::ScheduleInterpolation::Discrete),
        change_daily_schedule_limits(crate::model::ScheduleId(10), Some(0.0), Some(40.0)),
        create_weekly_schedule(1, crate::model::ScheduleId(21), vec![crate::model::ScheduleId(11); 7]),
        delete_weekly_schedule(crate::model::ScheduleId(20)),
        change_weekly_schedule_day(crate::model::ScheduleId(20), 2, crate::model::ScheduleId(11)),
        create_annual_schedule(1, crate::model::ScheduleId(31), crate::model::ScheduleId(10), None),
        delete_annual_schedule(crate::model::ScheduleId(30)),
        insert_annual_schedule_rule(crate::model::ScheduleId(30), 1, 7, 1, 8, 31, crate::model::ScheduleId(12)),
        remove_annual_schedule_rule(crate::model::ScheduleId(30), 0),
        reorder_annual_schedule_rules(crate::model::ScheduleId(30), 0, 1),
        change_annual_schedule_default_daily_schedule(crate::model::ScheduleId(30), crate::model::ScheduleId(10)),
        change_annual_schedule_holiday_daily_schedule(crate::model::ScheduleId(30), Some(crate::model::ScheduleId(11))),
        add_annual_schedule_holiday(crate::model::ScheduleId(30), 1, 2026, 1, 1),
        remove_annual_schedule_holiday(crate::model::ScheduleId(30), 2026, 12, 25),
        create_time_series_schedule(1, crate::model::ScheduleId(41), vec![1.0, 0.9], 900),
        delete_time_series_schedule(crate::model::ScheduleId(40)),
        replace_time_series_schedule_values(crate::model::ScheduleId(40), vec![1.0, 0.8, 0.6, 0.4]),
        change_time_series_schedule_timestep(crate::model::ScheduleId(40), 900),
    ]
}
//#endregion 🧵️WireProbes

//#region 🧰️Fixtures
/// 🧰️ Shared fixture-case machinery: the eight laws every committed `(before, mutation, after,
/// diff, outcome)` vector is held to, plus the `SEMIO_ENERGY_WRITE_FIXTURES=1` generator that
/// materializes those five JSON files from one typed scenario. Every `<kind>/🧪️tests/<case>/🦀️.rs`
/// is a thin declaration over this module, so a new kind's fixture case is data, not code.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixtures/🦀️.rs"]
pub mod fixtures;
//#endregion 🧰️Fixtures

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
