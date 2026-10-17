import {parseArtifactRef,type ArtifactRef} from "@semio-tech/framework/schema/artifact-reference";
import type { ChangeRunEndDay, ChangeRunEndMonth, ChangeRunStartDay, ChangeRunStartMonth, ChangeRunYear } from "./⏯️run/🟦️.ts";
import type { ChangePvSystemArea, ChangePvSystemAzimuth, ChangePvSystemDcCapacity, ChangePvSystemInverterEfficiency, ChangePvSystemModuleEfficiency, ChangePvSystemTilt, CreatePvSystem, DeletePvSystem } from "./☀️pv-system/🟦️.ts";
import type { ChangeFaultSeverity, ChangeFaultStartSchedule, ChangeFaultTargetEquipment, ChangeFaultType, CreateFault, DeleteFault } from "./⚠️fault/🟦️.ts";
import type { ChangeFenestrationFinDepth, ChangeFenestrationFinOffset, ChangeFenestrationOverhangDepth, ChangeFenestrationOverhangOffset } from "./⛱️shading-device/🟦️.ts";
import type { ChangeRefrigerationSystemCaseCount, ChangeRefrigerationSystemDefrostSchedule, ChangeRefrigerationSystemDesignLoad, CreateRefrigerationSystem, DeleteRefrigerationSystem } from "./❄️refrigeration/🟦️.ts";
import type { ChangeDaylightZoneGlareLimit, ChangeDaylightZoneIlluminanceTarget, ChangeDaylightZoneWindowTransmittance, ChangeDaylightZoneZone, CreateDaylightZone, DeleteDaylightZone } from "./🌞️daylight-zone/🟦️.ts";
import type { ChangeThermostatCoolingSetpointSchedule, ChangeThermostatCoolingThrottleRange, ChangeThermostatHeatingSetpointSchedule, ChangeThermostatHeatingThrottleRange, ChangeThermostatZone, CreateThermostat, DeleteThermostat } from "./🌡️thermostat/🟦️.ts";
import type { ChangeMechanicalVentilationDesignFlow, ChangeMechanicalVentilationFanDeltaPressure, ChangeMechanicalVentilationFanTotalEfficiency, ChangeMechanicalVentilationSchedule, ChangeMechanicalVentilationZone, CreateMechanicalVentilation, DeleteMechanicalVentilation } from "./🌪️ventilation/🟦️.ts";
import type { AddAirLoopTerminalZone, ChangeAirLoopDesignSupplyAirFlow, ChangeAirLoopReturnNode, ChangeAirLoopSupplyNode, CreateAirLoop, DeleteAirLoop, RemoveAirLoopTerminalZone, RenameAirLoop } from "./🌬️air-loop/🟦️.ts";
import type { ChangeShadingSurfaceTransmittanceSchedule, CreateShadingSurface, DeleteShadingSurface, RenameShadingSurface, ReplaceShadingSurfaceVertices } from "./🌳️shading/🟦️.ts";
import type { ChangeOutdoorAirSystemAirLoop, ChangeOutdoorAirSystemEconomizerEnabled, ChangeOutdoorAirSystemMinOaFlow, CreateOutdoorAirSystem, DeleteOutdoorAirSystem } from "./🍃️outdoor-air/🟦️.ts";
import type { ChangeSetpointManagerSchedule, CreateSetpointManager, DeleteSetpointManager, RenameSetpointManager, ReplaceSetpointManagerKind } from "./🎚️setpoint/🟦️.ts";
import type { ChangeMaterialRoughness, ChangeMaterialSolarAbsorptance, ChangeMaterialThermalAbsorptance, ChangeMaterialVisibleAbsorptance } from "./🎨️finish/🟦️.ts";
import type { AddConstructionLayer, CreateConstruction, DeleteConstruction, RemoveConstructionLayer, RenameConstruction, ReorderConstructionLayers } from "./🏗️construction/🟦️.ts";
import type { ChangeZoneConditioned, ChangeZoneFloorAreaParticipation, ChangeZoneMultiplier, ChangeZoneVolume, CreateZone, DeleteZone, RenameZone } from "./🏡️zone/🟦️.ts";
import type { AddPlantLoopEquipment, ChangePlantLoopDesignFlow, ChangePlantLoopReturnTemperature, ChangePlantLoopSupplyTemperature, ChangePlantLoopType, CreatePlantLoop, DeletePlantLoop, RemovePlantLoopEquipment, RenamePlantLoop } from "./🏭️plant-loop/🟦️.ts";
import type { ChangePeopleGainActivitySchedule, ChangePeopleGainLatentFraction, ChangePeopleGainPeoplePerArea, ChangePeopleGainRadiantFraction, ChangePeopleGainSchedule, ChangePeopleGainSensibleFraction, ChangePeopleGainZone, CreatePeopleGain, DeletePeopleGain } from "./👥️people-gain/🟦️.ts";
import type { ChangeLightingGainRadiantFraction, ChangeLightingGainReturnAirFraction, ChangeLightingGainSchedule, ChangeLightingGainVisibleFraction, ChangeLightingGainWattsPerArea, ChangeLightingGainZone, CreateLightingGain, DeleteLightingGain } from "./💡️lighting-gain/🟦️.ts";
import type { ChangeHumidistatDehumidifyingSetpointSchedule, ChangeHumidistatDehumidifyingThrottleRange, ChangeHumidistatHumidifyingSetpointSchedule, ChangeHumidistatHumidifyingThrottleRange, ChangeHumidistatZone, CreateHumidistat, DeleteHumidistat } from "./💧️humidistat/🟦️.ts";
import type { ChangeInfiltrationDesignFlowAch, ChangeInfiltrationFlowPerExteriorArea, ChangeInfiltrationMethod, ChangeInfiltrationSchedule, ChangeInfiltrationZone, CreateInfiltration, DeleteInfiltration } from "./💨️infiltration/🟦️.ts";
import type { ChangeIdealLoadsSystemMaxCoolingCapacity, ChangeIdealLoadsSystemMaxHeatingCapacity, ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp, ChangeIdealLoadsSystemMinCoolingSupplyAirTemp, ChangeIdealLoadsSystemOutdoorAirPerArea, ChangeIdealLoadsSystemOutdoorAirPerPerson, ChangeIdealLoadsSystemZone, CreateIdealLoadsSystem, DeleteIdealLoadsSystem } from "./💯️ideal-loads/🟦️.ts";
import type { AddAnnualScheduleHoliday, ChangeAnnualScheduleDefaultDailySchedule, ChangeAnnualScheduleHolidayDailySchedule, CreateAnnualSchedule, DeleteAnnualSchedule, InsertAnnualScheduleRule, RemoveAnnualScheduleHoliday, RemoveAnnualScheduleRule, ReorderAnnualScheduleRules } from "./📆️year-schedule/🟦️.ts";
import type { ChangeTimeSeriesScheduleTimestep, CreateTimeSeriesSchedule, DeleteTimeSeriesSchedule, ReplaceTimeSeriesScheduleValues } from "./📈️time-series/🟦️.ts";
import type { AddSpaceListMember, CreateSpaceList, DeleteSpaceList, RemoveSpaceListMember, RenameSpaceList } from "./📋️space-list/🟦️.ts";
import type { ChangeGroundBuilding, ChangeGroundDeep, ChangeGroundShallow, ChangeSiteElevation, ChangeSiteLatitude, ChangeSiteLongitude, ChangeSiteNorthAxis, ChangeSiteTimeZone } from "./📍️site/🟦️.ts";
import type { ChangeSizingObjectDesignDayType, ChangeSizingObjectSizingType, ChangeSizingObjectZone, CreateSizingObject, DeleteSizingObject } from "./📏️sizing/🟦️.ts";
import type { AddOutputVariable, BindWeatherFile, ChangeModelVersion, ConnectReferencedModel, DisconnectReferencedModel, RemoveOutputVariable, RenameModel, ReplaceAirflowNetwork, UnbindWeatherFile } from "./🔋️model/🟦️.ts";
import type { AddElectricalLoadCenterBattery, AddElectricalLoadCenterPv, CreateElectricalLoadCenter, DeleteElectricalLoadCenter, RemoveElectricalLoadCenterBattery, RemoveElectricalLoadCenterPv, RenameElectricalLoadCenter } from "./🔌️load-center/🟦️.ts";
import type { BindFenestrationGlazingConstruction, ChangeFenestrationDividerConductance, ChangeFenestrationFrameConductance, ChangeFenestrationShgc, ChangeFenestrationUValue, ChangeFenestrationVlt, ClearFenestrationGlazingConstruction } from "./🔍️glazing/🟦️.ts";
import type { AddThermalEnclosureZone, CreateThermalEnclosure, DeleteThermalEnclosure, RemoveThermalEnclosureZone, RenameThermalEnclosure } from "./🔒️enclosure/🟦️.ts";
import type { ChangeSolarThermalSystemAzimuth, ChangeSolarThermalSystemCollectorArea, ChangeSolarThermalSystemEfficiency, ChangeSolarThermalSystemStorageVolume, ChangeSolarThermalSystemTilt, CreateSolarThermalSystem, DeleteSolarThermalSystem } from "./🔥️solar-thermal/🟦️.ts";
import type { ChangeZoneEquipmentCoolingCapacity, ChangeZoneEquipmentHeatingCapacity, ChangeZoneEquipmentPriority, ChangeZoneEquipmentType, ChangeZoneEquipmentZone, CreateZoneEquipment, DeleteZoneEquipment } from "./🔧️zone-equipment/🟦️.ts";
import type { ChangeSurfaceBoundaryCondition, ChangeSurfaceSunExposed, ChangeSurfaceWindExposed, ConnectSurfaces, DisconnectSurfaces } from "./🔲️boundary/🟦️.ts";
import type { ChangeConstantScheduleValue, CreateConstantSchedule, DeleteConstantSchedule } from "./🕐️fixed-schedule/🟦️.ts";
import type { ChangeDailyScheduleInterpolation, ChangeDailyScheduleLimits, CreateDailySchedule, DeleteDailySchedule, ReplaceDailyScheduleHourlyValues } from "./🕒️day-schedule/🟦️.ts";
import type { ChangeInfiltrationConstantTermCoefficient, ChangeInfiltrationDischargeCoefficient, ChangeInfiltrationEffectiveLeakageArea, ChangeInfiltrationStackHeight, ChangeInfiltrationTemperatureTermCoefficient, ChangeInfiltrationVelocitySquaredTermCoefficient, ChangeInfiltrationVelocityTermCoefficient } from "./🕳️leakage/🟦️.ts";
import type { ChangeEquipmentGainLatentFraction, ChangeEquipmentGainRadiantFraction, ChangeEquipmentGainSchedule, ChangeEquipmentGainWattsPerArea, ChangeEquipmentGainZone, CreateEquipmentGain, DeleteEquipmentGain } from "./🖥️equipment-gain/🟦️.ts";
import type { ChangeWeeklyScheduleDay, CreateWeeklySchedule, DeleteWeeklySchedule } from "./🗓️week-schedule/🟦️.ts";
import type { ChangeWaterSystemFixtureCount, ChangeWaterSystemPeakFlow, ChangeWaterSystemSchedule, CreateWaterSystem, DeleteWaterSystem } from "./🚰️water-system/🟦️.ts";
import type { ChangeShwSystemHeaterCapacity, ChangeShwSystemSchedule, ChangeShwSystemSetpoint, ChangeShwSystemStorageVolume, CreateShwSystem, DeleteShwSystem } from "./🚿️shw-system/🟦️.ts";
import type { ChangeSpaceFloorArea, ChangeSpaceZone, CreateSpace, DeleteSpace, RenameSpace } from "./🛋️space/🟦️.ts";
import type { ChangeRoomAirModel, CreateRoomAirModelAssignment, DeleteRoomAirModelAssignment } from "./🛏️room-air/🟦️.ts";
import type { ChangeGasMaterialGas, ChangeGasMaterialThickness, ChangeGlazingMaterialConductivity, ChangeGlazingMaterialInfraredEmissivity, ChangeGlazingMaterialSolarTransmittance, ChangeGlazingMaterialThickness, ChangeGlazingMaterialVisibleTransmittance, RenameGasMaterial, RenameGlazingMaterial } from "./🥂️window-layer/🟦️.ts";
import type { ChangeSurfaceClass, ChangeSurfaceConstruction, ChangeSurfaceMultiplier, ChangeSurfaceZone, CreateSurface, DeleteSurface, RenameSurface, ReplaceSurfaceVertices } from "./🧱️surface/🟦️.ts";
import type { ChangeFenestrationArea, ChangeFenestrationHeight, ChangeFenestrationSillHeight, ChangeFenestrationSurface, CreateFenestration, DeleteFenestration, RenameFenestration, ReplaceFenestrationVertices } from "./🪟️fenestration/🟦️.ts";
import type { ChangeMaterialConductivity, ChangeMaterialDensity, ChangeMaterialSpecificHeat, ChangeMaterialThickness, CreateMaterial, DeleteMaterial, RenameMaterial } from "./🪨️material/🟦️.ts";
import type { ChangeBatteryCapacity, ChangeBatteryMaxCharge, ChangeBatteryMaxDischarge, ChangeBatteryRoundTripEfficiency, CreateBattery, DeleteBattery } from "./🪫️battery/🟦️.ts";

/** 🧬️ Energy-model mutation vocabulary — TypeScript twin of `🧬️mutations/🦀️.rs`.
 *
 *  `EnergyModelMutation` carries `#[value(tag = "mutation", rename_all = "camelCase")]`, so the
 *  wire tag is the camelCase form of the Rust variant name (`renameModel`), never the kebab-case
 *  `#[dsl(keyword)]` slug used for the directory names and for `SemanticDescriptor.kind`.
 */

export type EnergyModelMutation =
  | RenameModel
  | ChangeModelVersion
  | ChangeSiteLatitude
  | ChangeSiteLongitude
  | ChangeSiteElevation
  | ChangeSiteTimeZone
  | ChangeSiteNorthAxis
  | ChangeGroundBuilding
  | ChangeGroundShallow
  | ChangeGroundDeep
  | ChangeRunStartMonth
  | ChangeRunStartDay
  | ChangeRunEndMonth
  | ChangeRunEndDay
  | ChangeRunYear
  | ReplaceAirflowNetwork
  | AddOutputVariable
  | RemoveOutputVariable
  | BindWeatherFile
  | UnbindWeatherFile
  | ConnectReferencedModel
  | DisconnectReferencedModel
  | RenameZone
  | ChangeZoneVolume
  | ChangeZoneMultiplier
  | ChangeZoneConditioned
  | ChangeZoneFloorAreaParticipation
  | CreateZone
  | DeleteZone
  | CreateSpace
  | DeleteSpace
  | RenameSpace
  | ChangeSpaceFloorArea
  | ChangeSpaceZone
  | CreateSurface
  | DeleteSurface
  | RenameSurface
  | ChangeSurfaceZone
  | ChangeSurfaceClass
  | ReplaceSurfaceVertices
  | ChangeSurfaceConstruction
  | ChangeSurfaceBoundaryCondition
  | ChangeSurfaceSunExposed
  | ChangeSurfaceWindExposed
  | ChangeSurfaceMultiplier
  | CreateFenestration
  | DeleteFenestration
  | RenameFenestration
  | ChangeFenestrationSurface
  | ChangeFenestrationUValue
  | ChangeFenestrationShgc
  | ChangeFenestrationVlt
  | ChangeFenestrationArea
  | ChangeFenestrationFrameConductance
  | ChangeFenestrationDividerConductance
  | CreateShadingSurface
  | DeleteShadingSurface
  | RenameShadingSurface
  | ReplaceShadingSurfaceVertices
  | ChangeShadingSurfaceTransmittanceSchedule
  | ConnectSurfaces
  | DisconnectSurfaces
  | BindFenestrationGlazingConstruction
  | ClearFenestrationGlazingConstruction
  | ChangeFenestrationHeight
  | ChangeFenestrationSillHeight
  | ChangeFenestrationOverhangDepth
  | ChangeFenestrationOverhangOffset
  | ChangeFenestrationFinDepth
  | ChangeFenestrationFinOffset
  | CreateMaterial
  | DeleteMaterial
  | RenameMaterial
  | ChangeMaterialThickness
  | ChangeMaterialConductivity
  | ChangeMaterialDensity
  | ChangeMaterialSpecificHeat
  | ChangeMaterialThermalAbsorptance
  | ChangeMaterialSolarAbsorptance
  | ChangeMaterialVisibleAbsorptance
  | CreateConstruction
  | DeleteConstruction
  | RenameConstruction
  | AddConstructionLayer
  | RemoveConstructionLayer
  | ReorderConstructionLayers
  | CreatePeopleGain
  | DeletePeopleGain
  | ChangePeopleGainZone
  | ChangePeopleGainSchedule
  | ChangePeopleGainActivitySchedule
  | ChangePeopleGainPeoplePerArea
  | ChangePeopleGainSensibleFraction
  | ChangePeopleGainLatentFraction
  | ChangePeopleGainRadiantFraction
  | CreateLightingGain
  | DeleteLightingGain
  | ChangeLightingGainZone
  | ChangeLightingGainSchedule
  | ChangeLightingGainWattsPerArea
  | ChangeLightingGainRadiantFraction
  | ChangeLightingGainVisibleFraction
  | ChangeLightingGainReturnAirFraction
  | CreateEquipmentGain
  | DeleteEquipmentGain
  | ChangeEquipmentGainZone
  | ChangeEquipmentGainSchedule
  | ChangeEquipmentGainWattsPerArea
  | ChangeEquipmentGainRadiantFraction
  | ChangeEquipmentGainLatentFraction
  | CreateInfiltration
  | DeleteInfiltration
  | ChangeInfiltrationZone
  | ChangeInfiltrationSchedule
  | ChangeInfiltrationFlowPerExteriorArea
  | ChangeInfiltrationConstantTermCoefficient
  | ChangeInfiltrationTemperatureTermCoefficient
  | ChangeInfiltrationVelocityTermCoefficient
  | ChangeInfiltrationVelocitySquaredTermCoefficient
  | CreateMechanicalVentilation
  | DeleteMechanicalVentilation
  | ChangeMechanicalVentilationZone
  | ChangeMechanicalVentilationSchedule
  | ChangeMechanicalVentilationDesignFlow
  | ChangeMechanicalVentilationFanTotalEfficiency
  | ChangeMechanicalVentilationFanDeltaPressure
  | ChangeInfiltrationMethod
  | ChangeInfiltrationDesignFlowAch
  | ChangeInfiltrationEffectiveLeakageArea
  | ChangeInfiltrationDischargeCoefficient
  | ChangeInfiltrationStackHeight
  | CreateThermostat
  | DeleteThermostat
  | ChangeThermostatZone
  | ChangeThermostatHeatingSetpointSchedule
  | ChangeThermostatCoolingSetpointSchedule
  | ChangeThermostatHeatingThrottleRange
  | ChangeThermostatCoolingThrottleRange
  | CreateHumidistat
  | DeleteHumidistat
  | ChangeHumidistatZone
  | ChangeHumidistatHumidifyingSetpointSchedule
  | ChangeHumidistatDehumidifyingSetpointSchedule
  | ChangeHumidistatHumidifyingThrottleRange
  | ChangeHumidistatDehumidifyingThrottleRange
  | CreateIdealLoadsSystem
  | DeleteIdealLoadsSystem
  | ChangeIdealLoadsSystemZone
  | ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp
  | ChangeIdealLoadsSystemMinCoolingSupplyAirTemp
  | ChangeIdealLoadsSystemMaxHeatingCapacity
  | ChangeIdealLoadsSystemMaxCoolingCapacity
  | ChangeIdealLoadsSystemOutdoorAirPerPerson
  | ChangeIdealLoadsSystemOutdoorAirPerArea
  | CreateZoneEquipment
  | DeleteZoneEquipment
  | ChangeZoneEquipmentZone
  | ChangeZoneEquipmentType
  | ChangeZoneEquipmentPriority
  | ChangeZoneEquipmentHeatingCapacity
  | ChangeZoneEquipmentCoolingCapacity
  | CreateDaylightZone
  | DeleteDaylightZone
  | ChangeDaylightZoneZone
  | ChangeDaylightZoneIlluminanceTarget
  | ChangeDaylightZoneGlareLimit
  | ChangeDaylightZoneWindowTransmittance
  | CreateSizingObject
  | DeleteSizingObject
  | ChangeSizingObjectZone
  | ChangeSizingObjectSizingType
  | ChangeSizingObjectDesignDayType
  | CreateRoomAirModelAssignment
  | DeleteRoomAirModelAssignment
  | ChangeRoomAirModel
  | CreateSetpointManager
  | DeleteSetpointManager
  | RenameSetpointManager
  | ReplaceSetpointManagerKind
  | ChangeSetpointManagerSchedule
  | CreateAirLoop
  | DeleteAirLoop
  | RenameAirLoop
  | ChangeAirLoopSupplyNode
  | ChangeAirLoopReturnNode
  | ChangeAirLoopDesignSupplyAirFlow
  | AddAirLoopTerminalZone
  | RemoveAirLoopTerminalZone
  | CreatePlantLoop
  | DeletePlantLoop
  | RenamePlantLoop
  | ChangePlantLoopType
  | ChangePlantLoopSupplyTemperature
  | ChangePlantLoopReturnTemperature
  | ChangePlantLoopDesignFlow
  | AddPlantLoopEquipment
  | RemovePlantLoopEquipment
  | CreateOutdoorAirSystem
  | DeleteOutdoorAirSystem
  | ChangeOutdoorAirSystemAirLoop
  | ChangeOutdoorAirSystemMinOaFlow
  | ChangeOutdoorAirSystemEconomizerEnabled
  | CreateElectricalLoadCenter
  | DeleteElectricalLoadCenter
  | RenameElectricalLoadCenter
  | AddElectricalLoadCenterPv
  | RemoveElectricalLoadCenterPv
  | AddElectricalLoadCenterBattery
  | RemoveElectricalLoadCenterBattery
  | CreatePvSystem
  | DeletePvSystem
  | ChangePvSystemDcCapacity
  | ChangePvSystemArea
  | ChangePvSystemTilt
  | ChangePvSystemAzimuth
  | ChangePvSystemModuleEfficiency
  | ChangePvSystemInverterEfficiency
  | CreateBattery
  | DeleteBattery
  | ChangeBatteryCapacity
  | ChangeBatteryMaxCharge
  | ChangeBatteryMaxDischarge
  | ChangeBatteryRoundTripEfficiency
  | CreateShwSystem
  | DeleteShwSystem
  | ChangeShwSystemHeaterCapacity
  | ChangeShwSystemStorageVolume
  | ChangeShwSystemSetpoint
  | ChangeShwSystemSchedule
  | CreateSolarThermalSystem
  | DeleteSolarThermalSystem
  | ChangeSolarThermalSystemCollectorArea
  | ChangeSolarThermalSystemEfficiency
  | ChangeSolarThermalSystemStorageVolume
  | ChangeSolarThermalSystemTilt
  | ChangeSolarThermalSystemAzimuth
  | CreateRefrigerationSystem
  | DeleteRefrigerationSystem
  | ChangeRefrigerationSystemCaseCount
  | ChangeRefrigerationSystemDesignLoad
  | ChangeRefrigerationSystemDefrostSchedule
  | CreateWaterSystem
  | DeleteWaterSystem
  | ChangeWaterSystemFixtureCount
  | ChangeWaterSystemPeakFlow
  | ChangeWaterSystemSchedule
  | CreateFault
  | DeleteFault
  | ChangeFaultTargetEquipment
  | ChangeFaultType
  | ChangeFaultSeverity
  | ChangeFaultStartSchedule
  | CreateSpaceList
  | DeleteSpaceList
  | RenameSpaceList
  | AddSpaceListMember
  | RemoveSpaceListMember
  | CreateThermalEnclosure
  | DeleteThermalEnclosure
  | RenameThermalEnclosure
  | AddThermalEnclosureZone
  | RemoveThermalEnclosureZone
  | CreateConstantSchedule
  | DeleteConstantSchedule
  | ChangeConstantScheduleValue
  | CreateDailySchedule
  | DeleteDailySchedule
  | ReplaceDailyScheduleHourlyValues
  | ChangeDailyScheduleInterpolation
  | ChangeDailyScheduleLimits
  | CreateWeeklySchedule
  | DeleteWeeklySchedule
  | ChangeWeeklyScheduleDay
  | CreateAnnualSchedule
  | DeleteAnnualSchedule
  | InsertAnnualScheduleRule
  | RemoveAnnualScheduleRule
  | ReorderAnnualScheduleRules
  | ChangeAnnualScheduleDefaultDailySchedule
  | ChangeAnnualScheduleHolidayDailySchedule
  | AddAnnualScheduleHoliday
  | RemoveAnnualScheduleHoliday
  | CreateTimeSeriesSchedule
  | DeleteTimeSeriesSchedule
  | ReplaceTimeSeriesScheduleValues
  | ChangeTimeSeriesScheduleTimestep
  | ReplaceFenestrationVertices
  | ChangeGlazingMaterialThickness
  | ChangeGlazingMaterialConductivity
  | ChangeGlazingMaterialSolarTransmittance
  | ChangeGlazingMaterialVisibleTransmittance
  | ChangeGlazingMaterialInfraredEmissivity
  | RenameGlazingMaterial
  | ChangeGasMaterialThickness
  | ChangeGasMaterialGas
  | RenameGasMaterial
  | ChangeMaterialRoughness;

/** 🪪️ Admits the owned link target without native URI interpretation. */
export function parseEnergyReferenceTarget(value:unknown):ArtifactRef{const target=parseArtifactRef(value);if(target.artifactId===""||Object.values(target.dialect).some(value=>value===""))throw new TypeError("Energy link identity components must be nonempty");return target;}
