//! 🧬️ Transparent energy-model semantic mutation aggregate. Every variant is a single-field tuple
//! wrapping a handcrafted `protocol::MutationKind` payload (the `🧬️mutations/<slug>/` leaves);
//! `#[derive(dsl::Mutations)]` generates the `protocol::Mutation`/`protocol::SemanticMutation`
//! dispatch and `#[derive(dsl::DslEnum)]` generates the `dsl::DslVariants` binding that
//! `📝️text/🦀️.rs`'s op codecs are written against — so a new kind costs one enum variant here, never a hand-written match arm, an opcode constant or a tag registry
//! (ticket 26/09/06/ENERGY-PLUGIN-END-TO-END, adopting `📸️remodel`'s architecture).
//!
//! There is deliberately NO whole-document replace in this enum: `📓️derivation-rules.md` rule 6
//! routes file-open / import / load-example through `store::ArtifactStore::reset`, outside history.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

#[cfg(test)]
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_air_loop as air_loop_mutations;
extern crate semio_s_artifact_energy_model_mut_battery as battery_mutations;
extern crate semio_s_artifact_energy_model_mut_boundary as boundary_mutations;
extern crate semio_s_artifact_energy_model_mut_construction as construction_mutations;
extern crate semio_s_artifact_energy_model_mut_day_schedule as day_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_daylight_zone as daylight_zone_mutations;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_mut_enclosure as enclosure_mutations;
extern crate semio_s_artifact_energy_model_mut_equipment_gain as equipment_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_fault as fault_mutations;
extern crate semio_s_artifact_energy_model_mut_fenestration as fenestration_mutations;
extern crate semio_s_artifact_energy_model_mut_finish as finish_mutations;
extern crate semio_s_artifact_energy_model_mut_fixed_schedule as fixed_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_glazing as glazing_mutations;
extern crate semio_s_artifact_energy_model_mut_humidistat as humidistat_mutations;
extern crate semio_s_artifact_energy_model_mut_ideal_loads as ideal_loads_mutations;
extern crate semio_s_artifact_energy_model_mut_infiltration as infiltration_mutations;
extern crate semio_s_artifact_energy_model_mut_leakage as leakage_mutations;
extern crate semio_s_artifact_energy_model_mut_lighting_gain as lighting_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_load_center as load_center_mutations;
extern crate semio_s_artifact_energy_model_mut_material as material_mutations;
extern crate semio_s_artifact_energy_model_mut_model as model_mutations;
extern crate semio_s_artifact_energy_model_mut_outdoor_air as outdoor_air_mutations;
extern crate semio_s_artifact_energy_model_mut_people_gain as people_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_plant_loop as plant_loop_mutations;
extern crate semio_s_artifact_energy_model_mut_pv_system as pv_system_mutations;
extern crate semio_s_artifact_energy_model_mut_pv_system_performance as pv_system_performance_mutations;
extern crate semio_s_artifact_energy_model_mut_refrigeration as refrigeration_mutations;
extern crate semio_s_artifact_energy_model_mut_room_air as room_air_mutations;
extern crate semio_s_artifact_energy_model_mut_run as run_mutations;
extern crate semio_s_artifact_energy_model_mut_setpoint as setpoint_mutations;
extern crate semio_s_artifact_energy_model_mut_shading_device as shading_device_mutations;
extern crate semio_s_artifact_energy_model_mut_shading as shading_mutations;
extern crate semio_s_artifact_energy_model_mut_shw_system as shw_system_mutations;
extern crate semio_s_artifact_energy_model_mut_site as site_mutations;
extern crate semio_s_artifact_energy_model_mut_sizing as sizing_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_solar_thermal as solar_thermal_mutations;
extern crate semio_s_artifact_energy_model_mut_space_list as space_list_mutations;
extern crate semio_s_artifact_energy_model_mut_space as space_mutations;
extern crate semio_s_artifact_energy_model_mut_surface as surface_mutations;
extern crate semio_s_artifact_energy_model_mut_thermostat as thermostat_mutations;
extern crate semio_s_artifact_energy_model_mut_time_series as time_series_mutations;
extern crate semio_s_artifact_energy_model_mut_ventilation as ventilation_mutations;
extern crate semio_s_artifact_energy_model_mut_water_system as water_system_mutations;
extern crate semio_s_artifact_energy_model_mut_week_schedule as week_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_window_layer as window_layer_mutations;
extern crate semio_s_artifact_energy_model_mut_window_layer_optics as window_layer_optics_mutations;
extern crate semio_s_artifact_energy_model_mut_year_schedule as year_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_zone_equipment as zone_equipment_mutations;
extern crate semio_s_artifact_energy_model_mut_zone as zone_mutations;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};



//#region 🔖️Aggregate
/// 🧬️ Closed semantic mutation vocabulary for an energy model.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = EnergyModelSnapshot, diff = EnergyModelDiff, schema = "energy.model", reference = EnergyModelSnapshot, reference_diff = EnergyModelDiff)]
pub enum EnergyModelMutation {
    RenameModel(crate::model_mutations::RenameModel),
    ChangeModelVersion(crate::model_mutations::ChangeModelVersion),
    ChangeSiteLatitude(crate::site_mutations::ChangeSiteLatitude),
    ChangeSiteLongitude(crate::site_mutations::ChangeSiteLongitude),
    ChangeSiteElevation(crate::site_mutations::ChangeSiteElevation),
    ChangeSiteTimeZone(crate::site_mutations::ChangeSiteTimeZone),
    ChangeSiteNorthAxis(crate::site_mutations::ChangeSiteNorthAxis),
    ChangeGroundBuilding(crate::site_mutations::ChangeGroundBuilding),
    ChangeGroundShallow(crate::site_mutations::ChangeGroundShallow),
    ChangeGroundDeep(crate::site_mutations::ChangeGroundDeep),
    ChangeRunStartMonth(crate::run_mutations::ChangeRunStartMonth),
    ChangeRunStartDay(crate::run_mutations::ChangeRunStartDay),
    ChangeRunEndMonth(crate::run_mutations::ChangeRunEndMonth),
    ChangeRunEndDay(crate::run_mutations::ChangeRunEndDay),
    ChangeRunYear(crate::run_mutations::ChangeRunYear),
    ReplaceAirflowNetwork(crate::model_mutations::ReplaceAirflowNetwork),
    AddOutputVariable(crate::model_mutations::AddOutputVariable),
    RemoveOutputVariable(crate::model_mutations::RemoveOutputVariable),
    BindWeatherFile(crate::model_mutations::BindWeatherFile),
    UnbindWeatherFile(crate::model_mutations::UnbindWeatherFile),
    ConnectReferencedModel(crate::model_mutations::ConnectReferencedModel),
    DisconnectReferencedModel(crate::model_mutations::DisconnectReferencedModel),
    RenameZone(crate::zone_mutations::RenameZone),
    ChangeZoneVolume(crate::zone_mutations::ChangeZoneVolume),
    ChangeZoneMultiplier(crate::zone_mutations::ChangeZoneMultiplier),
    ChangeZoneConditioned(crate::zone_mutations::ChangeZoneConditioned),
    ChangeZoneFloorAreaParticipation(crate::zone_mutations::ChangeZoneFloorAreaParticipation),
    CreateZone(crate::zone_mutations::CreateZone),
    DeleteZone(crate::zone_mutations::DeleteZone),
    CreateSpace(crate::space_mutations::CreateSpace),
    DeleteSpace(crate::space_mutations::DeleteSpace),
    RenameSpace(crate::space_mutations::RenameSpace),
    ChangeSpaceFloorArea(crate::space_mutations::ChangeSpaceFloorArea),
    ChangeSpaceZone(crate::space_mutations::ChangeSpaceZone),
    CreateSurface(crate::surface_mutations::CreateSurface),
    DeleteSurface(crate::surface_mutations::DeleteSurface),
    RenameSurface(crate::surface_mutations::RenameSurface),
    ChangeSurfaceZone(crate::surface_mutations::ChangeSurfaceZone),
    ChangeSurfaceClass(crate::surface_mutations::ChangeSurfaceClass),
    ReplaceSurfaceVertices(crate::surface_mutations::ReplaceSurfaceVertices),
    ChangeSurfaceConstruction(crate::surface_mutations::ChangeSurfaceConstruction),
    ChangeSurfaceBoundaryCondition(crate::boundary_mutations::ChangeSurfaceBoundaryCondition),
    ChangeSurfaceSunExposed(crate::boundary_mutations::ChangeSurfaceSunExposed),
    ChangeSurfaceWindExposed(crate::boundary_mutations::ChangeSurfaceWindExposed),
    ChangeSurfaceMultiplier(crate::surface_mutations::ChangeSurfaceMultiplier),
    CreateFenestration(crate::fenestration_mutations::CreateFenestration),
    DeleteFenestration(crate::fenestration_mutations::DeleteFenestration),
    RenameFenestration(crate::fenestration_mutations::RenameFenestration),
    ChangeFenestrationSurface(crate::fenestration_mutations::ChangeFenestrationSurface),
    ChangeFenestrationUValue(crate::glazing_mutations::ChangeFenestrationUValue),
    ChangeFenestrationShgc(crate::glazing_mutations::ChangeFenestrationShgc),
    ChangeFenestrationVlt(crate::glazing_mutations::ChangeFenestrationVlt),
    ChangeFenestrationArea(crate::fenestration_mutations::ChangeFenestrationArea),
    ChangeFenestrationFrameConductance(crate::glazing_mutations::ChangeFenestrationFrameConductance),
    ChangeFenestrationDividerConductance(crate::glazing_mutations::ChangeFenestrationDividerConductance),
    CreateShadingSurface(crate::shading_mutations::CreateShadingSurface),
    DeleteShadingSurface(crate::shading_mutations::DeleteShadingSurface),
    RenameShadingSurface(crate::shading_mutations::RenameShadingSurface),
    ReplaceShadingSurfaceVertices(crate::shading_mutations::ReplaceShadingSurfaceVertices),
    ChangeShadingSurfaceTransmittanceSchedule(crate::shading_mutations::ChangeShadingSurfaceTransmittanceSchedule),
    ConnectSurfaces(crate::boundary_mutations::ConnectSurfaces),
    DisconnectSurfaces(crate::boundary_mutations::DisconnectSurfaces),
    BindFenestrationGlazingConstruction(crate::glazing_mutations::BindFenestrationGlazingConstruction),
    ClearFenestrationGlazingConstruction(crate::glazing_mutations::ClearFenestrationGlazingConstruction),
    ChangeFenestrationHeight(crate::fenestration_mutations::ChangeFenestrationHeight),
    ChangeFenestrationSillHeight(crate::fenestration_mutations::ChangeFenestrationSillHeight),
    ChangeFenestrationOverhangDepth(crate::shading_device_mutations::ChangeFenestrationOverhangDepth),
    ChangeFenestrationOverhangOffset(crate::shading_device_mutations::ChangeFenestrationOverhangOffset),
    ChangeFenestrationFinDepth(crate::shading_device_mutations::ChangeFenestrationFinDepth),
    ChangeFenestrationFinOffset(crate::shading_device_mutations::ChangeFenestrationFinOffset),
    CreateMaterial(crate::material_mutations::CreateMaterial),
    DeleteMaterial(crate::material_mutations::DeleteMaterial),
    RenameMaterial(crate::material_mutations::RenameMaterial),
    ChangeMaterialThickness(crate::material_mutations::ChangeMaterialThickness),
    ChangeMaterialConductivity(crate::material_mutations::ChangeMaterialConductivity),
    ChangeMaterialDensity(crate::material_mutations::ChangeMaterialDensity),
    ChangeMaterialSpecificHeat(crate::material_mutations::ChangeMaterialSpecificHeat),
    ChangeMaterialThermalAbsorptance(crate::finish_mutations::ChangeMaterialThermalAbsorptance),
    ChangeMaterialSolarAbsorptance(crate::finish_mutations::ChangeMaterialSolarAbsorptance),
    ChangeMaterialVisibleAbsorptance(crate::finish_mutations::ChangeMaterialVisibleAbsorptance),
    CreateConstruction(crate::construction_mutations::CreateConstruction),
    DeleteConstruction(crate::construction_mutations::DeleteConstruction),
    RenameConstruction(crate::construction_mutations::RenameConstruction),
    AddConstructionLayer(crate::construction_mutations::AddConstructionLayer),
    RemoveConstructionLayer(crate::construction_mutations::RemoveConstructionLayer),
    ReorderConstructionLayers(crate::construction_mutations::ReorderConstructionLayers),
    CreatePeopleGain(crate::people_gain_mutations::CreatePeopleGain),
    DeletePeopleGain(crate::people_gain_mutations::DeletePeopleGain),
    ChangePeopleGainZone(crate::people_gain_mutations::ChangePeopleGainZone),
    ChangePeopleGainSchedule(crate::people_gain_mutations::ChangePeopleGainSchedule),
    ChangePeopleGainActivitySchedule(crate::people_gain_mutations::ChangePeopleGainActivitySchedule),
    ChangePeopleGainPeoplePerArea(crate::people_gain_mutations::ChangePeopleGainPeoplePerArea),
    ChangePeopleGainSensibleFraction(crate::people_gain_mutations::ChangePeopleGainSensibleFraction),
    ChangePeopleGainLatentFraction(crate::people_gain_mutations::ChangePeopleGainLatentFraction),
    ChangePeopleGainRadiantFraction(crate::people_gain_mutations::ChangePeopleGainRadiantFraction),
    CreateLightingGain(crate::lighting_gain_mutations::CreateLightingGain),
    DeleteLightingGain(crate::lighting_gain_mutations::DeleteLightingGain),
    ChangeLightingGainZone(crate::lighting_gain_mutations::ChangeLightingGainZone),
    ChangeLightingGainSchedule(crate::lighting_gain_mutations::ChangeLightingGainSchedule),
    ChangeLightingGainWattsPerArea(crate::lighting_gain_mutations::ChangeLightingGainWattsPerArea),
    ChangeLightingGainRadiantFraction(crate::lighting_gain_mutations::ChangeLightingGainRadiantFraction),
    ChangeLightingGainVisibleFraction(crate::lighting_gain_mutations::ChangeLightingGainVisibleFraction),
    ChangeLightingGainReturnAirFraction(crate::lighting_gain_mutations::ChangeLightingGainReturnAirFraction),
    CreateEquipmentGain(crate::equipment_gain_mutations::CreateEquipmentGain),
    DeleteEquipmentGain(crate::equipment_gain_mutations::DeleteEquipmentGain),
    ChangeEquipmentGainZone(crate::equipment_gain_mutations::ChangeEquipmentGainZone),
    ChangeEquipmentGainSchedule(crate::equipment_gain_mutations::ChangeEquipmentGainSchedule),
    ChangeEquipmentGainWattsPerArea(crate::equipment_gain_mutations::ChangeEquipmentGainWattsPerArea),
    ChangeEquipmentGainRadiantFraction(crate::equipment_gain_mutations::ChangeEquipmentGainRadiantFraction),
    ChangeEquipmentGainLatentFraction(crate::equipment_gain_mutations::ChangeEquipmentGainLatentFraction),
    CreateInfiltration(crate::infiltration_mutations::CreateInfiltration),
    DeleteInfiltration(crate::infiltration_mutations::DeleteInfiltration),
    ChangeInfiltrationZone(crate::infiltration_mutations::ChangeInfiltrationZone),
    ChangeInfiltrationSchedule(crate::infiltration_mutations::ChangeInfiltrationSchedule),
    ChangeInfiltrationFlowPerExteriorArea(crate::infiltration_mutations::ChangeInfiltrationFlowPerExteriorArea),
    ChangeInfiltrationConstantTermCoefficient(crate::leakage_mutations::ChangeInfiltrationConstantTermCoefficient),
    ChangeInfiltrationTemperatureTermCoefficient(crate::leakage_mutations::ChangeInfiltrationTemperatureTermCoefficient),
    ChangeInfiltrationVelocityTermCoefficient(crate::leakage_mutations::ChangeInfiltrationVelocityTermCoefficient),
    ChangeInfiltrationVelocitySquaredTermCoefficient(crate::leakage_mutations::ChangeInfiltrationVelocitySquaredTermCoefficient),
    CreateMechanicalVentilation(crate::ventilation_mutations::CreateMechanicalVentilation),
    DeleteMechanicalVentilation(crate::ventilation_mutations::DeleteMechanicalVentilation),
    ChangeMechanicalVentilationZone(crate::ventilation_mutations::ChangeMechanicalVentilationZone),
    ChangeMechanicalVentilationSchedule(crate::ventilation_mutations::ChangeMechanicalVentilationSchedule),
    ChangeMechanicalVentilationDesignFlow(crate::ventilation_mutations::ChangeMechanicalVentilationDesignFlow),
    ChangeMechanicalVentilationFanTotalEfficiency(crate::ventilation_mutations::ChangeMechanicalVentilationFanTotalEfficiency),
    ChangeMechanicalVentilationFanDeltaPressure(crate::ventilation_mutations::ChangeMechanicalVentilationFanDeltaPressure),
    ChangeInfiltrationMethod(crate::infiltration_mutations::ChangeInfiltrationMethod),
    ChangeInfiltrationDesignFlowAch(crate::infiltration_mutations::ChangeInfiltrationDesignFlowAch),
    ChangeInfiltrationEffectiveLeakageArea(crate::leakage_mutations::ChangeInfiltrationEffectiveLeakageArea),
    ChangeInfiltrationDischargeCoefficient(crate::leakage_mutations::ChangeInfiltrationDischargeCoefficient),
    ChangeInfiltrationStackHeight(crate::leakage_mutations::ChangeInfiltrationStackHeight),
    CreateThermostat(crate::thermostat_mutations::CreateThermostat),
    DeleteThermostat(crate::thermostat_mutations::DeleteThermostat),
    ChangeThermostatZone(crate::thermostat_mutations::ChangeThermostatZone),
    ChangeThermostatHeatingSetpointSchedule(crate::thermostat_mutations::ChangeThermostatHeatingSetpointSchedule),
    ChangeThermostatCoolingSetpointSchedule(crate::thermostat_mutations::ChangeThermostatCoolingSetpointSchedule),
    ChangeThermostatHeatingThrottleRange(crate::thermostat_mutations::ChangeThermostatHeatingThrottleRange),
    ChangeThermostatCoolingThrottleRange(crate::thermostat_mutations::ChangeThermostatCoolingThrottleRange),
    CreateHumidistat(crate::humidistat_mutations::CreateHumidistat),
    DeleteHumidistat(crate::humidistat_mutations::DeleteHumidistat),
    ChangeHumidistatZone(crate::humidistat_mutations::ChangeHumidistatZone),
    ChangeHumidistatHumidifyingSetpointSchedule(crate::humidistat_mutations::ChangeHumidistatHumidifyingSetpointSchedule),
    ChangeHumidistatDehumidifyingSetpointSchedule(crate::humidistat_mutations::ChangeHumidistatDehumidifyingSetpointSchedule),
    ChangeHumidistatHumidifyingThrottleRange(crate::humidistat_mutations::ChangeHumidistatHumidifyingThrottleRange),
    ChangeHumidistatDehumidifyingThrottleRange(crate::humidistat_mutations::ChangeHumidistatDehumidifyingThrottleRange),
    CreateIdealLoadsSystem(crate::ideal_loads_mutations::CreateIdealLoadsSystem),
    DeleteIdealLoadsSystem(crate::ideal_loads_mutations::DeleteIdealLoadsSystem),
    ChangeIdealLoadsSystemZone(crate::ideal_loads_mutations::ChangeIdealLoadsSystemZone),
    ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp(crate::ideal_loads_mutations::ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp),
    ChangeIdealLoadsSystemMinCoolingSupplyAirTemp(crate::ideal_loads_mutations::ChangeIdealLoadsSystemMinCoolingSupplyAirTemp),
    ChangeIdealLoadsSystemMaxHeatingCapacity(crate::ideal_loads_mutations::ChangeIdealLoadsSystemMaxHeatingCapacity),
    ChangeIdealLoadsSystemMaxCoolingCapacity(crate::ideal_loads_mutations::ChangeIdealLoadsSystemMaxCoolingCapacity),
    ChangeIdealLoadsSystemOutdoorAirPerPerson(crate::ideal_loads_mutations::ChangeIdealLoadsSystemOutdoorAirPerPerson),
    ChangeIdealLoadsSystemOutdoorAirPerArea(crate::ideal_loads_mutations::ChangeIdealLoadsSystemOutdoorAirPerArea),
    CreateZoneEquipment(crate::zone_equipment_mutations::CreateZoneEquipment),
    DeleteZoneEquipment(crate::zone_equipment_mutations::DeleteZoneEquipment),
    ChangeZoneEquipmentZone(crate::zone_equipment_mutations::ChangeZoneEquipmentZone),
    ChangeZoneEquipmentType(crate::zone_equipment_mutations::ChangeZoneEquipmentType),
    ChangeZoneEquipmentPriority(crate::zone_equipment_mutations::ChangeZoneEquipmentPriority),
    ChangeZoneEquipmentHeatingCapacity(crate::zone_equipment_mutations::ChangeZoneEquipmentHeatingCapacity),
    ChangeZoneEquipmentCoolingCapacity(crate::zone_equipment_mutations::ChangeZoneEquipmentCoolingCapacity),
    CreateDaylightZone(crate::daylight_zone_mutations::CreateDaylightZone),
    DeleteDaylightZone(crate::daylight_zone_mutations::DeleteDaylightZone),
    ChangeDaylightZoneZone(crate::daylight_zone_mutations::ChangeDaylightZoneZone),
    ChangeDaylightZoneIlluminanceTarget(crate::daylight_zone_mutations::ChangeDaylightZoneIlluminanceTarget),
    ChangeDaylightZoneGlareLimit(crate::daylight_zone_mutations::ChangeDaylightZoneGlareLimit),
    ChangeDaylightZoneWindowTransmittance(crate::daylight_zone_mutations::ChangeDaylightZoneWindowTransmittance),
    CreateSizingObject(crate::sizing_mutations::CreateSizingObject),
    DeleteSizingObject(crate::sizing_mutations::DeleteSizingObject),
    ChangeSizingObjectZone(crate::sizing_mutations::ChangeSizingObjectZone),
    ChangeSizingObjectSizingType(crate::sizing_mutations::ChangeSizingObjectSizingType),
    ChangeSizingObjectDesignDayType(crate::sizing_mutations::ChangeSizingObjectDesignDayType),
    CreateRoomAirModelAssignment(crate::room_air_mutations::CreateRoomAirModelAssignment),
    DeleteRoomAirModelAssignment(crate::room_air_mutations::DeleteRoomAirModelAssignment),
    ChangeRoomAirModel(crate::room_air_mutations::ChangeRoomAirModel),
    CreateSetpointManager(crate::setpoint_mutations::CreateSetpointManager),
    DeleteSetpointManager(crate::setpoint_mutations::DeleteSetpointManager),
    RenameSetpointManager(crate::setpoint_mutations::RenameSetpointManager),
    ReplaceSetpointManagerKind(crate::setpoint_mutations::ReplaceSetpointManagerKind),
    ChangeSetpointManagerSchedule(crate::setpoint_mutations::ChangeSetpointManagerSchedule),
    CreateAirLoop(crate::air_loop_mutations::CreateAirLoop),
    DeleteAirLoop(crate::air_loop_mutations::DeleteAirLoop),
    RenameAirLoop(crate::air_loop_mutations::RenameAirLoop),
    ChangeAirLoopSupplyNode(crate::air_loop_mutations::ChangeAirLoopSupplyNode),
    ChangeAirLoopReturnNode(crate::air_loop_mutations::ChangeAirLoopReturnNode),
    ChangeAirLoopDesignSupplyAirFlow(crate::air_loop_mutations::ChangeAirLoopDesignSupplyAirFlow),
    AddAirLoopTerminalZone(crate::air_loop_mutations::AddAirLoopTerminalZone),
    RemoveAirLoopTerminalZone(crate::air_loop_mutations::RemoveAirLoopTerminalZone),
    CreatePlantLoop(crate::plant_loop_mutations::CreatePlantLoop),
    DeletePlantLoop(crate::plant_loop_mutations::DeletePlantLoop),
    RenamePlantLoop(crate::plant_loop_mutations::RenamePlantLoop),
    ChangePlantLoopType(crate::plant_loop_mutations::ChangePlantLoopType),
    ChangePlantLoopSupplyTemperature(crate::plant_loop_mutations::ChangePlantLoopSupplyTemperature),
    ChangePlantLoopReturnTemperature(crate::plant_loop_mutations::ChangePlantLoopReturnTemperature),
    ChangePlantLoopDesignFlow(crate::plant_loop_mutations::ChangePlantLoopDesignFlow),
    AddPlantLoopEquipment(crate::plant_loop_mutations::AddPlantLoopEquipment),
    RemovePlantLoopEquipment(crate::plant_loop_mutations::RemovePlantLoopEquipment),
    CreateOutdoorAirSystem(crate::outdoor_air_mutations::CreateOutdoorAirSystem),
    DeleteOutdoorAirSystem(crate::outdoor_air_mutations::DeleteOutdoorAirSystem),
    ChangeOutdoorAirSystemAirLoop(crate::outdoor_air_mutations::ChangeOutdoorAirSystemAirLoop),
    ChangeOutdoorAirSystemMinOaFlow(crate::outdoor_air_mutations::ChangeOutdoorAirSystemMinOaFlow),
    ChangeOutdoorAirSystemEconomizerEnabled(crate::outdoor_air_mutations::ChangeOutdoorAirSystemEconomizerEnabled),
    CreateElectricalLoadCenter(crate::load_center_mutations::CreateElectricalLoadCenter),
    DeleteElectricalLoadCenter(crate::load_center_mutations::DeleteElectricalLoadCenter),
    RenameElectricalLoadCenter(crate::load_center_mutations::RenameElectricalLoadCenter),
    AddElectricalLoadCenterPv(crate::load_center_mutations::AddElectricalLoadCenterPv),
    RemoveElectricalLoadCenterPv(crate::load_center_mutations::RemoveElectricalLoadCenterPv),
    AddElectricalLoadCenterBattery(crate::load_center_mutations::AddElectricalLoadCenterBattery),
    RemoveElectricalLoadCenterBattery(crate::load_center_mutations::RemoveElectricalLoadCenterBattery),
    CreatePvSystem(crate::pv_system_mutations::CreatePvSystem),
    DeletePvSystem(crate::pv_system_mutations::DeletePvSystem),
    ChangePvSystemDcCapacity(crate::pv_system_performance_mutations::ChangePvSystemDcCapacity),
    ChangePvSystemArea(crate::pv_system_mutations::ChangePvSystemArea),
    ChangePvSystemTilt(crate::pv_system_mutations::ChangePvSystemTilt),
    ChangePvSystemAzimuth(crate::pv_system_mutations::ChangePvSystemAzimuth),
    ChangePvSystemModuleEfficiency(crate::pv_system_performance_mutations::ChangePvSystemModuleEfficiency),
    ChangePvSystemInverterEfficiency(crate::pv_system_performance_mutations::ChangePvSystemInverterEfficiency),
    CreateBattery(crate::battery_mutations::CreateBattery),
    DeleteBattery(crate::battery_mutations::DeleteBattery),
    ChangeBatteryCapacity(crate::battery_mutations::ChangeBatteryCapacity),
    ChangeBatteryMaxCharge(crate::battery_mutations::ChangeBatteryMaxCharge),
    ChangeBatteryMaxDischarge(crate::battery_mutations::ChangeBatteryMaxDischarge),
    ChangeBatteryRoundTripEfficiency(crate::battery_mutations::ChangeBatteryRoundTripEfficiency),
    CreateShwSystem(crate::shw_system_mutations::CreateShwSystem),
    DeleteShwSystem(crate::shw_system_mutations::DeleteShwSystem),
    ChangeShwSystemHeaterCapacity(crate::shw_system_mutations::ChangeShwSystemHeaterCapacity),
    ChangeShwSystemStorageVolume(crate::shw_system_mutations::ChangeShwSystemStorageVolume),
    ChangeShwSystemSetpoint(crate::shw_system_mutations::ChangeShwSystemSetpoint),
    ChangeShwSystemSchedule(crate::shw_system_mutations::ChangeShwSystemSchedule),
    CreateSolarThermalSystem(crate::solar_thermal_mutations::CreateSolarThermalSystem),
    DeleteSolarThermalSystem(crate::solar_thermal_mutations::DeleteSolarThermalSystem),
    ChangeSolarThermalSystemCollectorArea(crate::solar_thermal_mutations::ChangeSolarThermalSystemCollectorArea),
    ChangeSolarThermalSystemEfficiency(crate::solar_thermal_mutations::ChangeSolarThermalSystemEfficiency),
    ChangeSolarThermalSystemStorageVolume(crate::solar_thermal_mutations::ChangeSolarThermalSystemStorageVolume),
    ChangeSolarThermalSystemTilt(crate::solar_thermal_mutations::ChangeSolarThermalSystemTilt),
    ChangeSolarThermalSystemAzimuth(crate::solar_thermal_mutations::ChangeSolarThermalSystemAzimuth),
    CreateRefrigerationSystem(crate::refrigeration_mutations::CreateRefrigerationSystem),
    DeleteRefrigerationSystem(crate::refrigeration_mutations::DeleteRefrigerationSystem),
    ChangeRefrigerationSystemCaseCount(crate::refrigeration_mutations::ChangeRefrigerationSystemCaseCount),
    ChangeRefrigerationSystemDesignLoad(crate::refrigeration_mutations::ChangeRefrigerationSystemDesignLoad),
    ChangeRefrigerationSystemDefrostSchedule(crate::refrigeration_mutations::ChangeRefrigerationSystemDefrostSchedule),
    CreateWaterSystem(crate::water_system_mutations::CreateWaterSystem),
    DeleteWaterSystem(crate::water_system_mutations::DeleteWaterSystem),
    ChangeWaterSystemFixtureCount(crate::water_system_mutations::ChangeWaterSystemFixtureCount),
    ChangeWaterSystemPeakFlow(crate::water_system_mutations::ChangeWaterSystemPeakFlow),
    ChangeWaterSystemSchedule(crate::water_system_mutations::ChangeWaterSystemSchedule),
    CreateFault(crate::fault_mutations::CreateFault),
    DeleteFault(crate::fault_mutations::DeleteFault),
    ChangeFaultTargetEquipment(crate::fault_mutations::ChangeFaultTargetEquipment),
    ChangeFaultType(crate::fault_mutations::ChangeFaultType),
    ChangeFaultSeverity(crate::fault_mutations::ChangeFaultSeverity),
    ChangeFaultStartSchedule(crate::fault_mutations::ChangeFaultStartSchedule),
    CreateSpaceList(crate::space_list_mutations::CreateSpaceList),
    DeleteSpaceList(crate::space_list_mutations::DeleteSpaceList),
    RenameSpaceList(crate::space_list_mutations::RenameSpaceList),
    AddSpaceListMember(crate::space_list_mutations::AddSpaceListMember),
    RemoveSpaceListMember(crate::space_list_mutations::RemoveSpaceListMember),
    CreateThermalEnclosure(crate::enclosure_mutations::CreateThermalEnclosure),
    DeleteThermalEnclosure(crate::enclosure_mutations::DeleteThermalEnclosure),
    RenameThermalEnclosure(crate::enclosure_mutations::RenameThermalEnclosure),
    AddThermalEnclosureZone(crate::enclosure_mutations::AddThermalEnclosureZone),
    RemoveThermalEnclosureZone(crate::enclosure_mutations::RemoveThermalEnclosureZone),
    CreateConstantSchedule(crate::fixed_schedule_mutations::CreateConstantSchedule),
    DeleteConstantSchedule(crate::fixed_schedule_mutations::DeleteConstantSchedule),
    ChangeConstantScheduleValue(crate::fixed_schedule_mutations::ChangeConstantScheduleValue),
    CreateDailySchedule(crate::day_schedule_mutations::CreateDailySchedule),
    DeleteDailySchedule(crate::day_schedule_mutations::DeleteDailySchedule),
    ReplaceDailyScheduleHourlyValues(crate::day_schedule_mutations::ReplaceDailyScheduleHourlyValues),
    ChangeDailyScheduleInterpolation(crate::day_schedule_mutations::ChangeDailyScheduleInterpolation),
    ChangeDailyScheduleLimits(crate::day_schedule_mutations::ChangeDailyScheduleLimits),
    CreateWeeklySchedule(crate::week_schedule_mutations::CreateWeeklySchedule),
    DeleteWeeklySchedule(crate::week_schedule_mutations::DeleteWeeklySchedule),
    ChangeWeeklyScheduleDay(crate::week_schedule_mutations::ChangeWeeklyScheduleDay),
    CreateAnnualSchedule(crate::year_schedule_mutations::CreateAnnualSchedule),
    DeleteAnnualSchedule(crate::year_schedule_mutations::DeleteAnnualSchedule),
    InsertAnnualScheduleRule(crate::year_schedule_mutations::InsertAnnualScheduleRule),
    RemoveAnnualScheduleRule(crate::year_schedule_mutations::RemoveAnnualScheduleRule),
    ReorderAnnualScheduleRules(crate::year_schedule_mutations::ReorderAnnualScheduleRules),
    ChangeAnnualScheduleDefaultDailySchedule(crate::year_schedule_mutations::ChangeAnnualScheduleDefaultDailySchedule),
    ChangeAnnualScheduleHolidayDailySchedule(crate::year_schedule_mutations::ChangeAnnualScheduleHolidayDailySchedule),
    AddAnnualScheduleHoliday(crate::year_schedule_mutations::AddAnnualScheduleHoliday),
    RemoveAnnualScheduleHoliday(crate::year_schedule_mutations::RemoveAnnualScheduleHoliday),
    CreateTimeSeriesSchedule(crate::time_series_mutations::CreateTimeSeriesSchedule),
    DeleteTimeSeriesSchedule(crate::time_series_mutations::DeleteTimeSeriesSchedule),
    ReplaceTimeSeriesScheduleValues(crate::time_series_mutations::ReplaceTimeSeriesScheduleValues),
    ChangeTimeSeriesScheduleTimestep(crate::time_series_mutations::ChangeTimeSeriesScheduleTimestep),
    ReplaceFenestrationVertices(crate::fenestration_mutations::ReplaceFenestrationVertices),
    ChangeGlazingMaterialThickness(crate::window_layer_mutations::ChangeGlazingMaterialThickness),
    ChangeGlazingMaterialConductivity(crate::window_layer_mutations::ChangeGlazingMaterialConductivity),
    ChangeGlazingMaterialSolarTransmittance(crate::window_layer_optics_mutations::ChangeGlazingMaterialSolarTransmittance),
    ChangeGlazingMaterialVisibleTransmittance(crate::window_layer_optics_mutations::ChangeGlazingMaterialVisibleTransmittance),
    ChangeGlazingMaterialInfraredEmissivity(crate::window_layer_optics_mutations::ChangeGlazingMaterialInfraredEmissivity),
    RenameGlazingMaterial(crate::window_layer_mutations::RenameGlazingMaterial),
    ChangeGasMaterialThickness(crate::window_layer_mutations::ChangeGasMaterialThickness),
    ChangeGasMaterialGas(crate::window_layer_mutations::ChangeGasMaterialGas),
    RenameGasMaterial(crate::window_layer_mutations::RenameGasMaterial),
    ChangeMaterialRoughness(crate::finish_mutations::ChangeMaterialRoughness),
    ChangeGlazingMaterialSolarReflectanceFront(crate::window_layer_optics_mutations::ChangeGlazingMaterialSolarReflectanceFront),
    ChangeGlazingMaterialSolarReflectanceBack(crate::window_layer_optics_mutations::ChangeGlazingMaterialSolarReflectanceBack),
    ChangeGlazingMaterialVisibleReflectanceFront(crate::window_layer_optics_mutations::ChangeGlazingMaterialVisibleReflectanceFront),
    ChangeGlazingMaterialVisibleReflectanceBack(crate::window_layer_optics_mutations::ChangeGlazingMaterialVisibleReflectanceBack),
    ChangeGlazingMaterialInfraredTransmittance(crate::window_layer_optics_mutations::ChangeGlazingMaterialInfraredTransmittance),
    ChangePvSystemTemperatureCoefficient(crate::pv_system_performance_mutations::ChangePvSystemTemperatureCoefficient),
    ChangePvSystemFaimanHeatLossConstant(crate::pv_system_performance_mutations::ChangePvSystemFaimanHeatLossConstant),
    ChangePvSystemFaimanHeatLossWind(crate::pv_system_performance_mutations::ChangePvSystemFaimanHeatLossWind),
    ChangePvSystemMounting(crate::pv_system_mutations::ChangePvSystemMounting),
}


//#endregion 🔖️Aggregate

//#region 🚪️Representations
#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;
#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
//#endregion 🚪️Representations

#[cfg(test)]
#[path="🧪️tests/🪟️optics/🦀️.rs"]
mod optical_field_tests;
