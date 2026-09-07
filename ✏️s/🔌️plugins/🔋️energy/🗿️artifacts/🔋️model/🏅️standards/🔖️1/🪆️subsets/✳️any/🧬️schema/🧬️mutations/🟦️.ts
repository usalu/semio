/** 🧬️ Energy-model mutation vocabulary — TypeScript twin of `🧬️mutations/🦀️.rs`.
 *
 *  `EnergyModelMutation` carries `#[value(tag = "mutation", rename_all = "camelCase")]`, so the
 *  wire tag is the camelCase form of the Rust variant name (`renameModel`), never the kebab-case
 *  `#[dsl(keyword)]` slug used for the directory names and for `SemanticDescriptor.kind`.
 */

/** 🏷️ `rename-model` payload. */
export interface RenameModel {
  readonly mutation: "renameModel";
  readonly newName: string;
}

/** 🔢️ `change-model-version` payload. */
export interface ChangeModelVersion {
  readonly mutation: "changeModelVersion";
  readonly newVersion: string;
}

/** 🌍️ `update-site` payload. */
export interface UpdateSite {
  readonly mutation: "updateSite";
  readonly latitudeDeg: number;
  readonly longitudeDeg: number;
  readonly elevationM: number;
  readonly timeZoneHours: number;
  readonly northAxisDeg: number;
}

/** 🌡️ `update-ground-temperature` payload. */
export interface UpdateGroundTemperature {
  readonly mutation: "updateGroundTemperature";
  readonly buildingSurfaceC: readonly number[];
  readonly shallowC: readonly number[];
  readonly deepC: number;
}

/** 📅️ `update-run-period` payload. */
export interface UpdateRunPeriod {
  readonly mutation: "updateRunPeriod";
  readonly startMonth: number;
  readonly startDay: number;
  readonly endMonth: number;
  readonly endDay: number;
  readonly year: number;
}

/** 🫧️ `replace-airflow-network` payload. */
export interface ReplaceAirflowNetwork {
  readonly mutation: "replaceAirflowNetwork";
  readonly present: boolean;
  readonly zoneIds: readonly number[];
  readonly nodeIds: readonly number[];
  readonly outdoorNodeId: number;
  readonly linkIds: readonly number[];
}

/** 📊️ `add-output-variable` payload. */
export interface AddOutputVariable {
  readonly mutation: "addOutputVariable";
  readonly name: string;
  readonly key: string;
  readonly reportingFrequency: "Timestep" | "Hourly" | "Daily" | "Monthly" | "RunPeriod";
}

/** 📉️ `remove-output-variable` payload. */
export interface RemoveOutputVariable {
  readonly mutation: "removeOutputVariable";
  readonly name: string;
  readonly key: string;
}

/** 🌦️ `bind-weather-file` payload. */
export interface BindWeatherFile {
  readonly mutation: "bindWeatherFile";
  readonly targetUri: string;
}

/** 🌤️ `unbind-weather-file` payload. */
export interface UnbindWeatherFile {
  readonly mutation: "unbindWeatherFile";
}

/** 🪢️ `connect-referenced-model` payload. */
export interface ConnectReferencedModel {
  readonly mutation: "connectReferencedModel";
  readonly targetUri: string;
}

/** ✂️ `disconnect-referenced-model` payload. */
export interface DisconnectReferencedModel {
  readonly mutation: "disconnectReferencedModel";
}

/** 🏠️ `rename-zone` payload. */
export interface RenameZone {
  readonly mutation: "renameZone";
  readonly id: number;
  readonly newName: string;
}

/** 📦️ `change-zone-volume` payload. */
export interface ChangeZoneVolume {
  readonly mutation: "changeZoneVolume";
  readonly id: number;
  readonly newVolumeM3: number;
}

/** ✖️ `change-zone-multiplier` payload. */
export interface ChangeZoneMultiplier {
  readonly mutation: "changeZoneMultiplier";
  readonly id: number;
  readonly newMultiplier: number;
}

/** 🌬️ `change-zone-conditioned` payload. */
export interface ChangeZoneConditioned {
  readonly mutation: "changeZoneConditioned";
  readonly id: number;
  readonly newConditioned: boolean;
}

/** 📐️ `change-zone-floor-area-participation` payload. */
export interface ChangeZoneFloorAreaParticipation {
  readonly mutation: "changeZoneFloorAreaParticipation";
  readonly id: number;
  readonly newPartOfTotalFloorArea: boolean;
}

/** 🏘️ `create-zone` payload. */
export interface CreateZone {
  readonly mutation: "createZone";
  readonly id: number;
  readonly name: string;
  readonly volumeM3: number;
  readonly multiplier: number;
  readonly conditioned: boolean;
  readonly partOfTotalFloorArea: boolean;
}

/** 🏚️ `delete-zone` payload. */
export interface DeleteZone {
  readonly mutation: "deleteZone";
  readonly id: number;
}

/** 🪑️ `create-space` payload. */
export interface CreateSpace {
  readonly mutation: "createSpace";
  readonly id: number;
  readonly name: string;
  readonly zoneId: number;
  readonly floorAreaM2: number;
}

/** 🧹️ `delete-space` payload. */
export interface DeleteSpace {
  readonly mutation: "deleteSpace";
  readonly id: number;
}

/** 🔤️ `rename-space` payload. */
export interface RenameSpace {
  readonly mutation: "renameSpace";
  readonly id: number;
  readonly newName: string;
}

/** 🧮️ `change-space-floor-area` payload. */
export interface ChangeSpaceFloorArea {
  readonly mutation: "changeSpaceFloorArea";
  readonly id: number;
  readonly newFloorAreaM2: number;
}

/** 🚚️ `change-space-zone` payload. */
export interface ChangeSpaceZone {
  readonly mutation: "changeSpaceZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🟫️ `create-surface` payload. */
export interface CreateSurface {
  readonly mutation: "createSurface";
  readonly id: number;
  readonly name: string;
  readonly zoneId: number;
  readonly class: "ExteriorWall" | "InteriorWall" | "Roof" | "Ceiling" | "Floor" | "Interzone" | "Adiabatic" | "Ground";
  readonly verticesM: readonly (readonly [number, number, number])[];
  readonly constructionId: number;
  readonly boundary: "OutdoorAir" | "Ground" | "OtherSideTemperature" | "Adiabatic" | "Interzone";
  readonly interzoneSurfaceId: number | null;
  readonly sunExposed: boolean;
  readonly windExposed: boolean;
  readonly multiplier: number;
}

/** 🪚️ `delete-surface` payload. */
export interface DeleteSurface {
  readonly mutation: "deleteSurface";
  readonly id: number;
}

/** 🏳️ `rename-surface` payload. */
export interface RenameSurface {
  readonly mutation: "renameSurface";
  readonly id: number;
  readonly newName: string;
}

/** 🗜️ `change-surface-zone` payload. */
export interface ChangeSurfaceZone {
  readonly mutation: "changeSurfaceZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🧩️ `change-surface-class` payload. */
export interface ChangeSurfaceClass {
  readonly mutation: "changeSurfaceClass";
  readonly id: number;
  readonly newClass: "ExteriorWall" | "InteriorWall" | "Roof" | "Ceiling" | "Floor" | "Interzone" | "Adiabatic" | "Ground";
}

/** 🔺️ `replace-surface-vertices` payload. */
export interface ReplaceSurfaceVertices {
  readonly mutation: "replaceSurfaceVertices";
  readonly id: number;
  readonly newVerticesM: readonly (readonly [number, number, number])[];
}

/** 🧰️ `change-surface-construction` payload. */
export interface ChangeSurfaceConstruction {
  readonly mutation: "changeSurfaceConstruction";
  readonly id: number;
  readonly newConstructionId: number;
}

/** 🚧️ `change-surface-boundary-condition` payload. */
export interface ChangeSurfaceBoundaryCondition {
  readonly mutation: "changeSurfaceBoundaryCondition";
  readonly id: number;
  readonly newBoundary: "OutdoorAir" | "Ground" | "OtherSideTemperature" | "Adiabatic" | "Interzone";
  readonly newInterzoneSurfaceId: number | null;
}

/** 🌅️ `change-surface-sun-exposed` payload. */
export interface ChangeSurfaceSunExposed {
  readonly mutation: "changeSurfaceSunExposed";
  readonly id: number;
  readonly newSunExposed: boolean;
}

/** 🍃️ `change-surface-wind-exposed` payload. */
export interface ChangeSurfaceWindExposed {
  readonly mutation: "changeSurfaceWindExposed";
  readonly id: number;
  readonly newWindExposed: boolean;
}

/** 🔁️ `change-surface-multiplier` payload. */
export interface ChangeSurfaceMultiplier {
  readonly mutation: "changeSurfaceMultiplier";
  readonly id: number;
  readonly newMultiplier: number;
}

/** 🪟️ `create-fenestration` payload. */
export interface CreateFenestration {
  readonly mutation: "createFenestration";
  readonly id: number;
  readonly name: string;
  readonly surfaceId: number;
  readonly uValueWM2k: number;
  readonly shgc: number;
  readonly vlt: number;
  readonly areaM2: number;
  readonly heightM: number;
  readonly sillHeightM: number;
  readonly frameConductanceWK: number;
  readonly dividerConductanceWK: number;
  readonly overhangDepthM: number;
  readonly overhangOffsetM: number;
  readonly finDepthM: number;
  readonly finOffsetM: number;
  readonly glazingConstructionId: number | null;
}

/** 🚪️ `delete-fenestration` payload. */
export interface DeleteFenestration {
  readonly mutation: "deleteFenestration";
  readonly id: number;
}

/** 🏁️ `rename-fenestration` payload. */
export interface RenameFenestration {
  readonly mutation: "renameFenestration";
  readonly id: number;
  readonly newName: string;
}

/** 🧲️ `change-fenestration-surface` payload. */
export interface ChangeFenestrationSurface {
  readonly mutation: "changeFenestrationSurface";
  readonly id: number;
  readonly newSurfaceId: number;
}

/** 🌐️ `change-fenestration-u-value` payload. */
export interface ChangeFenestrationUValue {
  readonly mutation: "changeFenestrationUValue";
  readonly id: number;
  readonly newUValueWM2k: number;
}

/** 🌇️ `change-fenestration-shgc` payload. */
export interface ChangeFenestrationShgc {
  readonly mutation: "changeFenestrationShgc";
  readonly id: number;
  readonly newShgc: number;
}

/** 🌈️ `change-fenestration-vlt` payload. */
export interface ChangeFenestrationVlt {
  readonly mutation: "changeFenestrationVlt";
  readonly id: number;
  readonly newVlt: number;
}

/** 🟥️ `change-fenestration-area` payload. */
export interface ChangeFenestrationArea {
  readonly mutation: "changeFenestrationArea";
  readonly id: number;
  readonly newAreaM2: number;
}

/** 🖼️ `change-fenestration-frame-conductance` payload. */
export interface ChangeFenestrationFrameConductance {
  readonly mutation: "changeFenestrationFrameConductance";
  readonly id: number;
  readonly newFrameConductanceWK: number;
}

/** 🧷️ `change-fenestration-divider-conductance` payload. */
export interface ChangeFenestrationDividerConductance {
  readonly mutation: "changeFenestrationDividerConductance";
  readonly id: number;
  readonly newDividerConductanceWK: number;
}

/** 🌳️ `create-shading-surface` payload. */
export interface CreateShadingSurface {
  readonly mutation: "createShadingSurface";
  readonly id: number;
  readonly name: string;
  readonly verticesM: readonly (readonly [number, number, number])[];
  readonly transmittanceScheduleId: number | null;
}

/** 🪵️ `delete-shading-surface` payload. */
export interface DeleteShadingSurface {
  readonly mutation: "deleteShadingSurface";
  readonly id: number;
}

/** 🏕️ `rename-shading-surface` payload. */
export interface RenameShadingSurface {
  readonly mutation: "renameShadingSurface";
  readonly id: number;
  readonly newName: string;
}

/** 🗺️ `replace-shading-surface-vertices` payload. */
export interface ReplaceShadingSurfaceVertices {
  readonly mutation: "replaceShadingSurfaceVertices";
  readonly id: number;
  readonly newVerticesM: readonly (readonly [number, number, number])[];
}

/** ⛱️ `change-shading-surface-transmittance-schedule` payload. */
export interface ChangeShadingSurfaceTransmittanceSchedule {
  readonly mutation: "changeShadingSurfaceTransmittanceSchedule";
  readonly id: number;
  readonly newTransmittanceScheduleId: number | null;
}

/** 🤝️ `connect-surfaces` payload. */
export interface ConnectSurfaces {
  readonly mutation: "connectSurfaces";
  readonly surfaceAId: number;
  readonly surfaceBId: number;
}

/** 💔️ `disconnect-surfaces` payload. */
export interface DisconnectSurfaces {
  readonly mutation: "disconnectSurfaces";
  readonly surfaceAId: number;
  readonly surfaceBId: number;
}

/** 🧊️ `bind-fenestration-glazing-construction` payload. */
export interface BindFenestrationGlazingConstruction {
  readonly mutation: "bindFenestrationGlazingConstruction";
  readonly id: number;
  readonly constructionId: number;
}

/** 🫗️ `clear-fenestration-glazing-construction` payload. */
export interface ClearFenestrationGlazingConstruction {
  readonly mutation: "clearFenestrationGlazingConstruction";
  readonly id: number;
}

/** ⬆️ `change-fenestration-height` payload. */
export interface ChangeFenestrationHeight {
  readonly mutation: "changeFenestrationHeight";
  readonly id: number;
  readonly newHeightM: number;
}

/** ⬇️ `change-fenestration-sill-height` payload. */
export interface ChangeFenestrationSillHeight {
  readonly mutation: "changeFenestrationSillHeight";
  readonly id: number;
  readonly newSillHeightM: number;
}

/** 🧢️ `change-fenestration-overhang-depth` payload. */
export interface ChangeFenestrationOverhangDepth {
  readonly mutation: "changeFenestrationOverhangDepth";
  readonly id: number;
  readonly newOverhangDepthM: number;
}

/** 🎩️ `change-fenestration-overhang-offset` payload. */
export interface ChangeFenestrationOverhangOffset {
  readonly mutation: "changeFenestrationOverhangOffset";
  readonly id: number;
  readonly newOverhangOffsetM: number;
}

/** 🐬️ `change-fenestration-fin-depth` payload. */
export interface ChangeFenestrationFinDepth {
  readonly mutation: "changeFenestrationFinDepth";
  readonly id: number;
  readonly newFinDepthM: number;
}

/** 🐋️ `change-fenestration-fin-offset` payload. */
export interface ChangeFenestrationFinOffset {
  readonly mutation: "changeFenestrationFinOffset";
  readonly id: number;
  readonly newFinOffsetM: number;
}

/** 🧱️ `create-material` payload. */
export interface CreateMaterial {
  readonly mutation: "createMaterial";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly thicknessM: number;
  readonly conductivityWMK: number;
  readonly densityKgM3: number;
  readonly specificHeatJKgK: number;
  readonly thermalAbsorptance: number;
  readonly solarAbsorptance: number;
  readonly visibleAbsorptance: number;
}

/** 🪨️ `delete-material` payload. */
export interface DeleteMaterial {
  readonly mutation: "deleteMaterial";
  readonly id: number;
}

/** 🪧️ `rename-material` payload. */
export interface RenameMaterial {
  readonly mutation: "renameMaterial";
  readonly id: number;
  readonly newName: string;
}

/** 📏️ `change-material-thickness` payload. */
export interface ChangeMaterialThickness {
  readonly mutation: "changeMaterialThickness";
  readonly id: number;
  readonly newThicknessM: number;
}

/** 🔥️ `change-material-conductivity` payload. */
export interface ChangeMaterialConductivity {
  readonly mutation: "changeMaterialConductivity";
  readonly id: number;
  readonly newConductivityWMK: number;
}

/** ⚖️ `change-material-density` payload. */
export interface ChangeMaterialDensity {
  readonly mutation: "changeMaterialDensity";
  readonly id: number;
  readonly newDensityKgM3: number;
}

/** ♨️ `change-material-specific-heat` payload. */
export interface ChangeMaterialSpecificHeat {
  readonly mutation: "changeMaterialSpecificHeat";
  readonly id: number;
  readonly newSpecificHeatJKgK: number;
}

/** 🔆️ `change-material-thermal-absorptance` payload. */
export interface ChangeMaterialThermalAbsorptance {
  readonly mutation: "changeMaterialThermalAbsorptance";
  readonly id: number;
  readonly newThermalAbsorptance: number;
}

/** ☀️ `change-material-solar-absorptance` payload. */
export interface ChangeMaterialSolarAbsorptance {
  readonly mutation: "changeMaterialSolarAbsorptance";
  readonly id: number;
  readonly newSolarAbsorptance: number;
}

/** 👁️ `change-material-visible-absorptance` payload. */
export interface ChangeMaterialVisibleAbsorptance {
  readonly mutation: "changeMaterialVisibleAbsorptance";
  readonly id: number;
  readonly newVisibleAbsorptance: number;
}

/** 🏗️ `create-construction` payload. */
export interface CreateConstruction {
  readonly mutation: "createConstruction";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly layerMaterialIds: readonly number[];
}

/** 🧨️ `delete-construction` payload. */
export interface DeleteConstruction {
  readonly mutation: "deleteConstruction";
  readonly id: number;
}

/** 🪪️ `rename-construction` payload. */
export interface RenameConstruction {
  readonly mutation: "renameConstruction";
  readonly id: number;
  readonly newName: string;
}

/** ➕️ `add-construction-layer` payload. */
export interface AddConstructionLayer {
  readonly mutation: "addConstructionLayer";
  readonly id: number;
  readonly index: number;
  readonly materialId: number;
}

/** ➖️ `remove-construction-layer` payload. */
export interface RemoveConstructionLayer {
  readonly mutation: "removeConstructionLayer";
  readonly id: number;
  readonly index: number;
}

/** 🔀️ `reorder-construction-layers` payload. */
export interface ReorderConstructionLayers {
  readonly mutation: "reorderConstructionLayers";
  readonly id: number;
  readonly newLayerMaterialIds: readonly number[];
}

/** 👤️ `create-people-gain` payload. */
export interface CreatePeopleGain {
  readonly mutation: "createPeopleGain";
  readonly index: number;
  readonly id: number;
  readonly zoneId: number;
  readonly scheduleId: number;
  readonly activityScheduleId: number;
  readonly peoplePerArea: number;
  readonly sensibleFraction: number;
  readonly latentFraction: number;
  readonly radiantFraction: number;
}

/** 🚷️ `delete-people-gain` payload. */
export interface DeletePeopleGain {
  readonly mutation: "deletePeopleGain";
  readonly id: number;
}

/** 🚶️ `change-people-gain-zone` payload. */
export interface ChangePeopleGainZone {
  readonly mutation: "changePeopleGainZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** ⏰️ `change-people-gain-schedule` payload. */
export interface ChangePeopleGainSchedule {
  readonly mutation: "changePeopleGainSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}

/** 🏃️ `change-people-gain-activity-schedule` payload. */
export interface ChangePeopleGainActivitySchedule {
  readonly mutation: "changePeopleGainActivitySchedule";
  readonly id: number;
  readonly newActivityScheduleId: number;
}

/** 👥️ `change-people-gain-people-per-area` payload. */
export interface ChangePeopleGainPeoplePerArea {
  readonly mutation: "changePeopleGainPeoplePerArea";
  readonly id: number;
  readonly newPeoplePerArea: number;
}

/** 🌞️ `change-people-gain-sensible-fraction` payload. */
export interface ChangePeopleGainSensibleFraction {
  readonly mutation: "changePeopleGainSensibleFraction";
  readonly id: number;
  readonly newSensibleFraction: number;
}

/** 💧️ `change-people-gain-latent-fraction` payload. */
export interface ChangePeopleGainLatentFraction {
  readonly mutation: "changePeopleGainLatentFraction";
  readonly id: number;
  readonly newLatentFraction: number;
}

/** 📡️ `change-people-gain-radiant-fraction` payload. */
export interface ChangePeopleGainRadiantFraction {
  readonly mutation: "changePeopleGainRadiantFraction";
  readonly id: number;
  readonly newRadiantFraction: number;
}

/** 💡️ `create-lighting-gain` payload. */
export interface CreateLightingGain {
  readonly mutation: "createLightingGain";
  readonly index: number;
  readonly id: number;
  readonly zoneId: number;
  readonly scheduleId: number;
  readonly wattsPerArea: number;
  readonly radiantFraction: number;
  readonly visibleFraction: number;
  readonly returnAirFraction: number;
}

/** 🕯️ `delete-lighting-gain` payload. */
export interface DeleteLightingGain {
  readonly mutation: "deleteLightingGain";
  readonly id: number;
}

/** 🔦️ `change-lighting-gain-zone` payload. */
export interface ChangeLightingGainZone {
  readonly mutation: "changeLightingGainZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** ⏱️ `change-lighting-gain-schedule` payload. */
export interface ChangeLightingGainSchedule {
  readonly mutation: "changeLightingGainSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}

/** 🔌️ `change-lighting-gain-watts-per-area` payload. */
export interface ChangeLightingGainWattsPerArea {
  readonly mutation: "changeLightingGainWattsPerArea";
  readonly id: number;
  readonly newWattsPerArea: number;
}

/** 🌟️ `change-lighting-gain-radiant-fraction` payload. */
export interface ChangeLightingGainRadiantFraction {
  readonly mutation: "changeLightingGainRadiantFraction";
  readonly id: number;
  readonly newRadiantFraction: number;
}

/** 🔅️ `change-lighting-gain-visible-fraction` payload. */
export interface ChangeLightingGainVisibleFraction {
  readonly mutation: "changeLightingGainVisibleFraction";
  readonly id: number;
  readonly newVisibleFraction: number;
}

/** 🎐️ `change-lighting-gain-return-air-fraction` payload. */
export interface ChangeLightingGainReturnAirFraction {
  readonly mutation: "changeLightingGainReturnAirFraction";
  readonly id: number;
  readonly newReturnAirFraction: number;
}

/** 🖥️ `create-equipment-gain` payload. */
export interface CreateEquipmentGain {
  readonly mutation: "createEquipmentGain";
  readonly index: number;
  readonly id: number;
  readonly zoneId: number;
  readonly scheduleId: number;
  readonly wattsPerArea: number;
  readonly radiantFraction: number;
  readonly latentFraction: number;
}

/** 🧯️ `delete-equipment-gain` payload. */
export interface DeleteEquipmentGain {
  readonly mutation: "deleteEquipmentGain";
  readonly id: number;
}

/** 🖨️ `change-equipment-gain-zone` payload. */
export interface ChangeEquipmentGainZone {
  readonly mutation: "changeEquipmentGainZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** ⌛️ `change-equipment-gain-schedule` payload. */
export interface ChangeEquipmentGainSchedule {
  readonly mutation: "changeEquipmentGainSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}

/** ⚡️ `change-equipment-gain-watts-per-area` payload. */
export interface ChangeEquipmentGainWattsPerArea {
  readonly mutation: "changeEquipmentGainWattsPerArea";
  readonly id: number;
  readonly newWattsPerArea: number;
}

/** 🌠️ `change-equipment-gain-radiant-fraction` payload. */
export interface ChangeEquipmentGainRadiantFraction {
  readonly mutation: "changeEquipmentGainRadiantFraction";
  readonly id: number;
  readonly newRadiantFraction: number;
}

/** 💦️ `change-equipment-gain-latent-fraction` payload. */
export interface ChangeEquipmentGainLatentFraction {
  readonly mutation: "changeEquipmentGainLatentFraction";
  readonly id: number;
  readonly newLatentFraction: number;
}

/** 💨️ `create-infiltration` payload. */
export interface CreateInfiltration {
  readonly mutation: "createInfiltration";
  readonly index: number;
  readonly id: number;
  readonly zoneId: number;
  readonly scheduleId: number;
  readonly method: "ScheduledAch" | "PerExteriorArea" | "EffectiveLeakageArea" | "WindAndStack";
  readonly designFlowAch: number;
  readonly flowPerExteriorAreaM3SM2: number;
  readonly effectiveLeakageAreaM2: number;
  readonly dischargeCoefficient: number;
  readonly stackHeightM: number;
  readonly constantTermCoefficient: number;
  readonly temperatureTermCoefficient: number;
  readonly velocityTermCoefficient: number;
  readonly velocitySquaredTermCoefficient: number;
}

/** 🧽️ `delete-infiltration` payload. */
export interface DeleteInfiltration {
  readonly mutation: "deleteInfiltration";
  readonly id: number;
}

/** 🌀️ `change-infiltration-zone` payload. */
export interface ChangeInfiltrationZone {
  readonly mutation: "changeInfiltrationZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** ⏳️ `change-infiltration-schedule` payload. */
export interface ChangeInfiltrationSchedule {
  readonly mutation: "changeInfiltrationSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}

/** 🌫️ `change-infiltration-flow-per-exterior-area` payload. */
export interface ChangeInfiltrationFlowPerExteriorArea {
  readonly mutation: "changeInfiltrationFlowPerExteriorArea";
  readonly id: number;
  readonly newFlowPerExteriorAreaM3SM2: number;
}

/** 🅰️ `change-infiltration-constant-term-coefficient` payload. */
export interface ChangeInfiltrationConstantTermCoefficient {
  readonly mutation: "changeInfiltrationConstantTermCoefficient";
  readonly id: number;
  readonly newConstantTermCoefficient: number;
}

/** 🅱️ `change-infiltration-temperature-term-coefficient` payload. */
export interface ChangeInfiltrationTemperatureTermCoefficient {
  readonly mutation: "changeInfiltrationTemperatureTermCoefficient";
  readonly id: number;
  readonly newTemperatureTermCoefficient: number;
}

/** 🆎️ `change-infiltration-velocity-term-coefficient` payload. */
export interface ChangeInfiltrationVelocityTermCoefficient {
  readonly mutation: "changeInfiltrationVelocityTermCoefficient";
  readonly id: number;
  readonly newVelocityTermCoefficient: number;
}

/** 🆑️ `change-infiltration-velocity-squared-term-coefficient` payload. */
export interface ChangeInfiltrationVelocitySquaredTermCoefficient {
  readonly mutation: "changeInfiltrationVelocitySquaredTermCoefficient";
  readonly id: number;
  readonly newVelocitySquaredTermCoefficient: number;
}

/** 🌪️ `create-mechanical-ventilation` payload. */
export interface CreateMechanicalVentilation {
  readonly mutation: "createMechanicalVentilation";
  readonly index: number;
  readonly id: number;
  readonly zoneId: number;
  readonly scheduleId: number;
  readonly designFlowM3S: number;
  readonly fanTotalEfficiency: number;
  readonly fanDeltaPressurePa: number;
}

/** 🚫️ `delete-mechanical-ventilation` payload. */
export interface DeleteMechanicalVentilation {
  readonly mutation: "deleteMechanicalVentilation";
  readonly id: number;
}

/** 🧭️ `change-mechanical-ventilation-zone` payload. */
export interface ChangeMechanicalVentilationZone {
  readonly mutation: "changeMechanicalVentilationZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 📆️ `change-mechanical-ventilation-schedule` payload. */
export interface ChangeMechanicalVentilationSchedule {
  readonly mutation: "changeMechanicalVentilationSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}

/** 🚿️ `change-mechanical-ventilation-design-flow` payload. */
export interface ChangeMechanicalVentilationDesignFlow {
  readonly mutation: "changeMechanicalVentilationDesignFlow";
  readonly id: number;
  readonly newDesignFlowM3S: number;
}

/** 💠️ `change-mechanical-ventilation-fan-total-efficiency` payload. */
export interface ChangeMechanicalVentilationFanTotalEfficiency {
  readonly mutation: "changeMechanicalVentilationFanTotalEfficiency";
  readonly id: number;
  readonly newFanTotalEfficiency: number;
}

/** 🎈️ `change-mechanical-ventilation-fan-delta-pressure` payload. */
export interface ChangeMechanicalVentilationFanDeltaPressure {
  readonly mutation: "changeMechanicalVentilationFanDeltaPressure";
  readonly id: number;
  readonly newFanDeltaPressurePa: number;
}

/** 🔬️ `change-infiltration-method` payload. */
export interface ChangeInfiltrationMethod {
  readonly mutation: "changeInfiltrationMethod";
  readonly id: number;
  readonly newMethod: "ScheduledAch" | "PerExteriorArea" | "EffectiveLeakageArea" | "WindAndStack";
}

/** 🔄️ `change-infiltration-design-flow-ach` payload. */
export interface ChangeInfiltrationDesignFlowAch {
  readonly mutation: "changeInfiltrationDesignFlowAch";
  readonly id: number;
  readonly newDesignFlowAch: number;
}

/** 🕳️ `change-infiltration-effective-leakage-area` payload. */
export interface ChangeInfiltrationEffectiveLeakageArea {
  readonly mutation: "changeInfiltrationEffectiveLeakageArea";
  readonly id: number;
  readonly newEffectiveLeakageAreaM2: number;
}

/** 🚰️ `change-infiltration-discharge-coefficient` payload. */
export interface ChangeInfiltrationDischargeCoefficient {
  readonly mutation: "changeInfiltrationDischargeCoefficient";
  readonly id: number;
  readonly newDischargeCoefficient: number;
}

/** 🏭️ `change-infiltration-stack-height` payload. */
export interface ChangeInfiltrationStackHeight {
  readonly mutation: "changeInfiltrationStackHeight";
  readonly id: number;
  readonly newStackHeightM: number;
}

/** 🩺️ `create-thermostat` payload. */
export interface CreateThermostat {
  readonly mutation: "createThermostat";
  readonly id: number;
  readonly zoneId: number;
  readonly heatingSetpointScheduleId: number;
  readonly coolingSetpointScheduleId: number;
  readonly heatingThrottleRangeK: number;
  readonly coolingThrottleRangeK: number;
}

/** 🛑️ `delete-thermostat` payload. */
export interface DeleteThermostat {
  readonly mutation: "deleteThermostat";
  readonly id: number;
}

/** 🛖️ `change-thermostat-zone` payload. */
export interface ChangeThermostatZone {
  readonly mutation: "changeThermostatZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🥵️ `change-thermostat-heating-setpoint-schedule` payload. */
export interface ChangeThermostatHeatingSetpointSchedule {
  readonly mutation: "changeThermostatHeatingSetpointSchedule";
  readonly id: number;
  readonly newHeatingSetpointScheduleId: number;
}

/** 🐧️ `change-thermostat-cooling-setpoint-schedule` payload. */
export interface ChangeThermostatCoolingSetpointSchedule {
  readonly mutation: "changeThermostatCoolingSetpointSchedule";
  readonly id: number;
  readonly newCoolingSetpointScheduleId: number;
}

/** 🎚️ `change-thermostat-heating-throttle-range` payload. */
export interface ChangeThermostatHeatingThrottleRange {
  readonly mutation: "changeThermostatHeatingThrottleRange";
  readonly id: number;
  readonly newHeatingThrottleRangeK: number;
}

/** 🎛️ `change-thermostat-cooling-throttle-range` payload. */
export interface ChangeThermostatCoolingThrottleRange {
  readonly mutation: "changeThermostatCoolingThrottleRange";
  readonly id: number;
  readonly newCoolingThrottleRangeK: number;
}

/** 🌂️ `create-humidistat` payload. */
export interface CreateHumidistat {
  readonly mutation: "createHumidistat";
  readonly id: number;
  readonly zoneId: number;
  readonly humidifyingSetpointScheduleId: number;
  readonly dehumidifyingSetpointScheduleId: number;
  readonly humidifyingThrottleRange: number;
  readonly dehumidifyingThrottleRange: number;
}

/** 🏜️ `delete-humidistat` payload. */
export interface DeleteHumidistat {
  readonly mutation: "deleteHumidistat";
  readonly id: number;
}

/** 🏙️ `change-humidistat-zone` payload. */
export interface ChangeHumidistatZone {
  readonly mutation: "changeHumidistatZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** ☔️ `change-humidistat-humidifying-setpoint-schedule` payload. */
export interface ChangeHumidistatHumidifyingSetpointSchedule {
  readonly mutation: "changeHumidistatHumidifyingSetpointSchedule";
  readonly id: number;
  readonly newHumidifyingSetpointScheduleId: number;
}

/** 🏝️ `change-humidistat-dehumidifying-setpoint-schedule` payload. */
export interface ChangeHumidistatDehumidifyingSetpointSchedule {
  readonly mutation: "changeHumidistatDehumidifyingSetpointSchedule";
  readonly id: number;
  readonly newDehumidifyingSetpointScheduleId: number;
}

/** 🌧️ `change-humidistat-humidifying-throttle-range` payload. */
export interface ChangeHumidistatHumidifyingThrottleRange {
  readonly mutation: "changeHumidistatHumidifyingThrottleRange";
  readonly id: number;
  readonly newHumidifyingThrottleRange: number;
}

/** 🧻️ `change-humidistat-dehumidifying-throttle-range` payload. */
export interface ChangeHumidistatDehumidifyingThrottleRange {
  readonly mutation: "changeHumidistatDehumidifyingThrottleRange";
  readonly id: number;
  readonly newDehumidifyingThrottleRange: number;
}

/** 🫁️ `create-ideal-loads-system` payload. */
export interface CreateIdealLoadsSystem {
  readonly mutation: "createIdealLoadsSystem";
  readonly id: number;
  readonly zoneId: number;
  readonly maxHeatingSupplyAirTempC: number;
  readonly minCoolingSupplyAirTempC: number;
  readonly maxHeatingCapacityPresent: boolean;
  readonly maxHeatingCapacityW: number;
  readonly maxCoolingCapacityPresent: boolean;
  readonly maxCoolingCapacityW: number;
  readonly outdoorAirPerPersonM3S: number;
  readonly outdoorAirPerAreaM3SM2: number;
}

/** 🫥️ `delete-ideal-loads-system` payload. */
export interface DeleteIdealLoadsSystem {
  readonly mutation: "deleteIdealLoadsSystem";
  readonly id: number;
}

/** 🏢️ `change-ideal-loads-system-zone` payload. */
export interface ChangeIdealLoadsSystemZone {
  readonly mutation: "changeIdealLoadsSystemZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🔴️ `change-ideal-loads-system-max-heating-supply-air-temp` payload. */
export interface ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp {
  readonly mutation: "changeIdealLoadsSystemMaxHeatingSupplyAirTemp";
  readonly id: number;
  readonly newMaxHeatingSupplyAirTempC: number;
}

/** 🔵️ `change-ideal-loads-system-min-cooling-supply-air-temp` payload. */
export interface ChangeIdealLoadsSystemMinCoolingSupplyAirTemp {
  readonly mutation: "changeIdealLoadsSystemMinCoolingSupplyAirTemp";
  readonly id: number;
  readonly newMinCoolingSupplyAirTempC: number;
}

/** ⛽️ `change-ideal-loads-system-max-heating-capacity` payload. */
export interface ChangeIdealLoadsSystemMaxHeatingCapacity {
  readonly mutation: "changeIdealLoadsSystemMaxHeatingCapacity";
  readonly id: number;
  readonly newCapacityPresent: boolean;
  readonly newMaxHeatingCapacityW: number;
}

/** 🟧️ `change-ideal-loads-system-max-cooling-capacity` payload. */
export interface ChangeIdealLoadsSystemMaxCoolingCapacity {
  readonly mutation: "changeIdealLoadsSystemMaxCoolingCapacity";
  readonly id: number;
  readonly newCapacityPresent: boolean;
  readonly newMaxCoolingCapacityW: number;
}

/** 🧍️ `change-ideal-loads-system-outdoor-air-per-person` payload. */
export interface ChangeIdealLoadsSystemOutdoorAirPerPerson {
  readonly mutation: "changeIdealLoadsSystemOutdoorAirPerPerson";
  readonly id: number;
  readonly newOutdoorAirPerPersonM3S: number;
}

/** 🔳️ `change-ideal-loads-system-outdoor-air-per-area` payload. */
export interface ChangeIdealLoadsSystemOutdoorAirPerArea {
  readonly mutation: "changeIdealLoadsSystemOutdoorAirPerArea";
  readonly id: number;
  readonly newOutdoorAirPerAreaM3SM2: number;
}

/** 🛠️ `create-zone-equipment` payload. */
export interface CreateZoneEquipment {
  readonly mutation: "createZoneEquipment";
  readonly id: number;
  readonly zoneId: number;
  readonly equipmentType: "Baseboard" | "Radiant" | "FanCoil" | "Ptac" | "VrfTerminal" | "Erv" | "UnitHeater" | "WaterToAirHp";
  readonly priority: number;
  readonly heatingCapacityW: number;
  readonly coolingCapacityW: number;
}

/** 🗑️ `delete-zone-equipment` payload. */
export interface DeleteZoneEquipment {
  readonly mutation: "deleteZoneEquipment";
  readonly id: number;
}

/** 🏬️ `change-zone-equipment-zone` payload. */
export interface ChangeZoneEquipmentZone {
  readonly mutation: "changeZoneEquipmentZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🔧️ `change-zone-equipment-type` payload. */
export interface ChangeZoneEquipmentType {
  readonly mutation: "changeZoneEquipmentType";
  readonly id: number;
  readonly newEquipmentType: "Baseboard" | "Radiant" | "FanCoil" | "Ptac" | "VrfTerminal" | "Erv" | "UnitHeater" | "WaterToAirHp";
}

/** 🎗️ `change-zone-equipment-priority` payload. */
export interface ChangeZoneEquipmentPriority {
  readonly mutation: "changeZoneEquipmentPriority";
  readonly id: number;
  readonly newPriority: number;
}

/** 🧇️ `change-zone-equipment-heating-capacity` payload. */
export interface ChangeZoneEquipmentHeatingCapacity {
  readonly mutation: "changeZoneEquipmentHeatingCapacity";
  readonly id: number;
  readonly newHeatingCapacityW: number;
}

/** 🍧️ `change-zone-equipment-cooling-capacity` payload. */
export interface ChangeZoneEquipmentCoolingCapacity {
  readonly mutation: "changeZoneEquipmentCoolingCapacity";
  readonly id: number;
  readonly newCoolingCapacityW: number;
}

/** 🔭️ `create-daylight-zone` payload. */
export interface CreateDaylightZone {
  readonly mutation: "createDaylightZone";
  readonly id: number;
  readonly zoneId: number;
  readonly illuminanceTargetLux: number;
  readonly glareLimit: number;
  readonly windowTransmittance: number;
}

/** 🌗️ `delete-daylight-zone` payload. */
export interface DeleteDaylightZone {
  readonly mutation: "deleteDaylightZone";
  readonly id: number;
}

/** 🏫️ `change-daylight-zone-zone` payload. */
export interface ChangeDaylightZoneZone {
  readonly mutation: "changeDaylightZoneZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🪔️ `change-daylight-zone-illuminance-target` payload. */
export interface ChangeDaylightZoneIlluminanceTarget {
  readonly mutation: "changeDaylightZoneIlluminanceTarget";
  readonly id: number;
  readonly newIlluminanceTargetLux: number;
}

/** 🕶️ `change-daylight-zone-glare-limit` payload. */
export interface ChangeDaylightZoneGlareLimit {
  readonly mutation: "changeDaylightZoneGlareLimit";
  readonly id: number;
  readonly newGlareLimit: number;
}

/** 🥃️ `change-daylight-zone-window-transmittance` payload. */
export interface ChangeDaylightZoneWindowTransmittance {
  readonly mutation: "changeDaylightZoneWindowTransmittance";
  readonly id: number;
  readonly newWindowTransmittance: number;
}

/** 📶️ `create-sizing-object` payload. */
export interface CreateSizingObject {
  readonly mutation: "createSizingObject";
  readonly id: number;
  readonly zoneId: number;
  readonly sizingType: "Heating" | "Cooling" | "OutdoorAir";
  readonly designDayType: "Heating" | "Cooling";
}

/** 🪒️ `delete-sizing-object` payload. */
export interface DeleteSizingObject {
  readonly mutation: "deleteSizingObject";
  readonly id: number;
}

/** 🏨️ `change-sizing-object-zone` payload. */
export interface ChangeSizingObjectZone {
  readonly mutation: "changeSizingObjectZone";
  readonly id: number;
  readonly newZoneId: number;
}

/** 🧾️ `change-sizing-object-sizing-type` payload. */
export interface ChangeSizingObjectSizingType {
  readonly mutation: "changeSizingObjectSizingType";
  readonly id: number;
  readonly newSizingType: "Heating" | "Cooling" | "OutdoorAir";
}

/** 🌥️ `change-sizing-object-design-day-type` payload. */
export interface ChangeSizingObjectDesignDayType {
  readonly mutation: "changeSizingObjectDesignDayType";
  readonly id: number;
  readonly newDesignDayType: "Heating" | "Cooling";
}

/** 🛏️ `create-room-air-model-assignment` payload. */
export interface CreateRoomAirModelAssignment {
  readonly mutation: "createRoomAirModelAssignment";
  readonly zoneId: number;
  readonly model: "WellMixed" | "OneNodeDisplacement" | "TwoNodeBuoyancy" | "UnderFloorAirDistribution";
}

/** 🧺️ `delete-room-air-model-assignment` payload. */
export interface DeleteRoomAirModelAssignment {
  readonly mutation: "deleteRoomAirModelAssignment";
  readonly zoneId: number;
}

/** 🪭️ `change-room-air-model` payload. */
export interface ChangeRoomAirModel {
  readonly mutation: "changeRoomAirModel";
  readonly zoneId: number;
  readonly newModel: "WellMixed" | "OneNodeDisplacement" | "TwoNodeBuoyancy" | "UnderFloorAirDistribution";
}

/** 📌️ `create-setpoint-manager` payload. */
export interface CreateSetpointManager {
  readonly mutation: "createSetpointManager";
  readonly id: number;
  readonly name: string;
  readonly kind: string;
  readonly lowOutdoorC: number;
  readonly highOutdoorC: number;
  readonly lowSetpointC: number;
  readonly highSetpointC: number;
  readonly schedulePresent: boolean;
  readonly scheduleId: number;
}

/** 🍄️ `delete-setpoint-manager` payload. */
export interface DeleteSetpointManager {
  readonly mutation: "deleteSetpointManager";
  readonly id: number;
}

/** 🖇️ `rename-setpoint-manager` payload. */
export interface RenameSetpointManager {
  readonly mutation: "renameSetpointManager";
  readonly id: number;
  readonly newName: string;
}

/** 🔃️ `replace-setpoint-manager-kind` payload. */
export interface ReplaceSetpointManagerKind {
  readonly mutation: "replaceSetpointManagerKind";
  readonly id: number;
  readonly newKind: string;
  readonly newLowOutdoorC: number;
  readonly newHighOutdoorC: number;
  readonly newLowSetpointC: number;
  readonly newHighSetpointC: number;
}

/** 🎼️ `change-setpoint-manager-schedule` payload. */
export interface ChangeSetpointManagerSchedule {
  readonly mutation: "changeSetpointManagerSchedule";
  readonly id: number;
  readonly newSchedulePresent: boolean;
  readonly newScheduleId: number;
}

/** 🛞️ `create-air-loop` payload. */
export interface CreateAirLoop {
  readonly mutation: "createAirLoop";
  readonly id: number;
  readonly name: string;
  readonly supplyNodeId: number;
  readonly returnNodeId: number;
  readonly designSupplyAirFlowM3S: number;
  readonly terminalZoneIds: readonly number[];
}

/** 🥀️ `delete-air-loop` payload. */
export interface DeleteAirLoop {
  readonly mutation: "deleteAirLoop";
  readonly id: number;
}

/** 📇️ `rename-air-loop` payload. */
export interface RenameAirLoop {
  readonly mutation: "renameAirLoop";
  readonly id: number;
  readonly newName: string;
}

/** ↗️ `change-air-loop-supply-node` payload. */
export interface ChangeAirLoopSupplyNode {
  readonly mutation: "changeAirLoopSupplyNode";
  readonly id: number;
  readonly newSupplyNodeId: number;
}

/** ↘️ `change-air-loop-return-node` payload. */
export interface ChangeAirLoopReturnNode {
  readonly mutation: "changeAirLoopReturnNode";
  readonly id: number;
  readonly newReturnNodeId: number;
}

/** 🍥️ `change-air-loop-design-supply-air-flow` payload. */
export interface ChangeAirLoopDesignSupplyAirFlow {
  readonly mutation: "changeAirLoopDesignSupplyAirFlow";
  readonly id: number;
  readonly newDesignSupplyAirFlowM3S: number;
}

/** 🪺️ `add-air-loop-terminal-zone` payload. */
export interface AddAirLoopTerminalZone {
  readonly mutation: "addAirLoopTerminalZone";
  readonly id: number;
  readonly zoneId: number;
}

/** 🪹️ `remove-air-loop-terminal-zone` payload. */
export interface RemoveAirLoopTerminalZone {
  readonly mutation: "removeAirLoopTerminalZone";
  readonly id: number;
  readonly zoneId: number;
}

/** ⚗️ `create-plant-loop` payload. */
export interface CreatePlantLoop {
  readonly mutation: "createPlantLoop";
  readonly id: number;
  readonly name: string;
  readonly loopType: "Heating" | "Cooling" | "Condenser";
  readonly supplyTemperatureC: number;
  readonly returnTemperatureC: number;
  readonly designFlowKgS: number;
  readonly equipmentIds: readonly number[];
}

/** 💣️ `delete-plant-loop` payload. */
export interface DeletePlantLoop {
  readonly mutation: "deletePlantLoop";
  readonly id: number;
}

/** 📛️ `rename-plant-loop` payload. */
export interface RenamePlantLoop {
  readonly mutation: "renamePlantLoop";
  readonly id: number;
  readonly newName: string;
}

/** ♻️ `change-plant-loop-type` payload. */
export interface ChangePlantLoopType {
  readonly mutation: "changePlantLoopType";
  readonly id: number;
  readonly newLoopType: "Heating" | "Cooling" | "Condenser";
}

/** ☕️ `change-plant-loop-supply-temperature` payload. */
export interface ChangePlantLoopSupplyTemperature {
  readonly mutation: "changePlantLoopSupplyTemperature";
  readonly id: number;
  readonly newSupplyTemperatureC: number;
}

/** 🧫️ `change-plant-loop-return-temperature` payload. */
export interface ChangePlantLoopReturnTemperature {
  readonly mutation: "changePlantLoopReturnTemperature";
  readonly id: number;
  readonly newReturnTemperatureC: number;
}

/** 🚤️ `change-plant-loop-design-flow` payload. */
export interface ChangePlantLoopDesignFlow {
  readonly mutation: "changePlantLoopDesignFlow";
  readonly id: number;
  readonly newDesignFlowKgS: number;
}

/** 🔩️ `add-plant-loop-equipment` payload. */
export interface AddPlantLoopEquipment {
  readonly mutation: "addPlantLoopEquipment";
  readonly id: number;
  readonly equipmentId: number;
}

/** ⚙️ `remove-plant-loop-equipment` payload. */
export interface RemovePlantLoopEquipment {
  readonly mutation: "removePlantLoopEquipment";
  readonly id: number;
  readonly equipmentId: number;
}

/** 🌲️ `create-outdoor-air-system` payload. */
export interface CreateOutdoorAirSystem {
  readonly mutation: "createOutdoorAirSystem";
  readonly id: number;
  readonly airLoopId: number;
  readonly minOaFlowM3S: number;
  readonly economizerEnabled: boolean;
}

/** 🍂️ `delete-outdoor-air-system` payload. */
export interface DeleteOutdoorAirSystem {
  readonly mutation: "deleteOutdoorAirSystem";
  readonly id: number;
}

/** ⛓️ `change-outdoor-air-system-air-loop` payload. */
export interface ChangeOutdoorAirSystemAirLoop {
  readonly mutation: "changeOutdoorAirSystemAirLoop";
  readonly id: number;
  readonly newAirLoopId: number;
}

/** 🦋️ `change-outdoor-air-system-min-oa-flow` payload. */
export interface ChangeOutdoorAirSystemMinOaFlow {
  readonly mutation: "changeOutdoorAirSystemMinOaFlow";
  readonly id: number;
  readonly newMinOaFlowM3S: number;
}

/** 💰️ `change-outdoor-air-system-economizer-enabled` payload. */
export interface ChangeOutdoorAirSystemEconomizerEnabled {
  readonly mutation: "changeOutdoorAirSystemEconomizerEnabled";
  readonly id: number;
  readonly newEconomizerEnabled: boolean;
}

/** 🏦️ `create-electrical-load-center` payload. */
export interface CreateElectricalLoadCenter {
  readonly mutation: "createElectricalLoadCenter";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly generatorIds: readonly number[];
  readonly pvIds: readonly number[];
  readonly batteryIds: readonly number[];
}

/** 🔻️ `delete-electrical-load-center` payload. */
export interface DeleteElectricalLoadCenter {
  readonly mutation: "deleteElectricalLoadCenter";
  readonly id: number;
}

/** 🖊️ `rename-electrical-load-center` payload. */
export interface RenameElectricalLoadCenter {
  readonly mutation: "renameElectricalLoadCenter";
  readonly id: number;
  readonly newName: string;
}

/** ☄️ `add-electrical-load-center-pv` payload. */
export interface AddElectricalLoadCenterPv {
  readonly mutation: "addElectricalLoadCenterPv";
  readonly id: number;
  readonly index: number;
  readonly pvId: number;
}

/** 🌘️ `remove-electrical-load-center-pv` payload. */
export interface RemoveElectricalLoadCenterPv {
  readonly mutation: "removeElectricalLoadCenterPv";
  readonly id: number;
  readonly pvId: number;
}

/** 🔋️ `add-electrical-load-center-battery` payload. */
export interface AddElectricalLoadCenterBattery {
  readonly mutation: "addElectricalLoadCenterBattery";
  readonly id: number;
  readonly index: number;
  readonly batteryId: number;
}

/** 🪝️ `remove-electrical-load-center-battery` payload. */
export interface RemoveElectricalLoadCenterBattery {
  readonly mutation: "removeElectricalLoadCenterBattery";
  readonly id: number;
  readonly batteryId: number;
}

/** ✨️ `create-pv-system` payload. */
export interface CreatePvSystem {
  readonly mutation: "createPvSystem";
  readonly index: number;
  readonly id: number;
  readonly dcCapacityW: number;
  readonly areaM2: number;
  readonly tiltDeg: number;
  readonly azimuthDeg: number;
  readonly moduleEfficiency: number;
  readonly inverterEfficiency: number;
}

/** 🌒️ `delete-pv-system` payload. */
export interface DeletePvSystem {
  readonly mutation: "deletePvSystem";
  readonly id: number;
}

/** ⚛️ `change-pv-system-dc-capacity` payload. */
export interface ChangePvSystemDcCapacity {
  readonly mutation: "changePvSystemDcCapacity";
  readonly id: number;
  readonly newDcCapacityW: number;
}

/** 🟨️ `change-pv-system-area` payload. */
export interface ChangePvSystemArea {
  readonly mutation: "changePvSystemArea";
  readonly id: number;
  readonly newAreaM2: number;
}

/** 📈️ `change-pv-system-tilt` payload. */
export interface ChangePvSystemTilt {
  readonly mutation: "changePvSystemTilt";
  readonly id: number;
  readonly newTiltDeg: number;
}

/** 🧿️ `change-pv-system-azimuth` payload. */
export interface ChangePvSystemAzimuth {
  readonly mutation: "changePvSystemAzimuth";
  readonly id: number;
  readonly newAzimuthDeg: number;
}

/** 🎖️ `change-pv-system-module-efficiency` payload. */
export interface ChangePvSystemModuleEfficiency {
  readonly mutation: "changePvSystemModuleEfficiency";
  readonly id: number;
  readonly newModuleEfficiency: number;
}

/** ♌️ `change-pv-system-inverter-efficiency` payload. */
export interface ChangePvSystemInverterEfficiency {
  readonly mutation: "changePvSystemInverterEfficiency";
  readonly id: number;
  readonly newInverterEfficiency: number;
}

/** 🪙️ `create-battery` payload. */
export interface CreateBattery {
  readonly mutation: "createBattery";
  readonly index: number;
  readonly id: number;
  readonly capacityKwh: number;
  readonly maxChargeW: number;
  readonly maxDischargeW: number;
  readonly roundTripEfficiency: number;
}

/** ♒️ `delete-battery` payload. */
export interface DeleteBattery {
  readonly mutation: "deleteBattery";
  readonly id: number;
}

/** 🥫️ `change-battery-capacity` payload. */
export interface ChangeBatteryCapacity {
  readonly mutation: "changeBatteryCapacity";
  readonly id: number;
  readonly newCapacityKwh: number;
}

/** ⏫️ `change-battery-max-charge` payload. */
export interface ChangeBatteryMaxCharge {
  readonly mutation: "changeBatteryMaxCharge";
  readonly id: number;
  readonly newMaxChargeW: number;
}

/** ⏬️ `change-battery-max-discharge` payload. */
export interface ChangeBatteryMaxDischarge {
  readonly mutation: "changeBatteryMaxDischarge";
  readonly id: number;
  readonly newMaxDischargeW: number;
}

/** 🥉️ `change-battery-round-trip-efficiency` payload. */
export interface ChangeBatteryRoundTripEfficiency {
  readonly mutation: "changeBatteryRoundTripEfficiency";
  readonly id: number;
  readonly newRoundTripEfficiency: number;
}

/** 🛀️ `create-shw-system` payload. */
export interface CreateShwSystem {
  readonly mutation: "createShwSystem";
  readonly index: number;
  readonly id: number;
  readonly heaterCapacityW: number;
  readonly storageVolumeM3: number;
  readonly setpointC: number;
  readonly scheduleId: number;
}

/** 🚱️ `delete-shw-system` payload. */
export interface DeleteShwSystem {
  readonly mutation: "deleteShwSystem";
  readonly id: number;
}

/** 🍵️ `change-shw-system-heater-capacity` payload. */
export interface ChangeShwSystemHeaterCapacity {
  readonly mutation: "changeShwSystemHeaterCapacity";
  readonly id: number;
  readonly newHeaterCapacityW: number;
}

/** 🛢️ `change-shw-system-storage-volume` payload. */
export interface ChangeShwSystemStorageVolume {
  readonly mutation: "changeShwSystemStorageVolume";
  readonly id: number;
  readonly newStorageVolumeM3: number;
}

/** 🏹️ `change-shw-system-setpoint` payload. */
export interface ChangeShwSystemSetpoint {
  readonly mutation: "changeShwSystemSetpoint";
  readonly id: number;
  readonly newSetpointC: number;
}

/** 🕐️ `change-shw-system-schedule` payload. */
export interface ChangeShwSystemSchedule {
  readonly mutation: "changeShwSystemSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}

/** 🌄️ `create-solar-thermal-system` payload. */
export interface CreateSolarThermalSystem {
  readonly mutation: "createSolarThermalSystem";
  readonly index: number;
  readonly id: number;
  readonly collectorAreaM2: number;
  readonly efficiency: number;
  readonly storageVolumeM3: number;
  readonly tiltDeg: number;
  readonly azimuthDeg: number;
}

/** 🌆️ `delete-solar-thermal-system` payload. */
export interface DeleteSolarThermalSystem {
  readonly mutation: "deleteSolarThermalSystem";
  readonly id: number;
}

/** 🟩️ `change-solar-thermal-system-collector-area` payload. */
export interface ChangeSolarThermalSystemCollectorArea {
  readonly mutation: "changeSolarThermalSystemCollectorArea";
  readonly id: number;
  readonly newCollectorAreaM2: number;
}

/** 🏅️ `change-solar-thermal-system-efficiency` payload. */
export interface ChangeSolarThermalSystemEfficiency {
  readonly mutation: "changeSolarThermalSystemEfficiency";
  readonly id: number;
  readonly newEfficiency: number;
}

/** 🧃️ `change-solar-thermal-system-storage-volume` payload. */
export interface ChangeSolarThermalSystemStorageVolume {
  readonly mutation: "changeSolarThermalSystemStorageVolume";
  readonly id: number;
  readonly newStorageVolumeM3: number;
}

/** 🔼️ `change-solar-thermal-system-tilt` payload. */
export interface ChangeSolarThermalSystemTilt {
  readonly mutation: "changeSolarThermalSystemTilt";
  readonly id: number;
  readonly newTiltDeg: number;
}

/** ⛵️ `change-solar-thermal-system-azimuth` payload. */
export interface ChangeSolarThermalSystemAzimuth {
  readonly mutation: "changeSolarThermalSystemAzimuth";
  readonly id: number;
  readonly newAzimuthDeg: number;
}

/** ❄️ `create-refrigeration-system` payload. */
export interface CreateRefrigerationSystem {
  readonly mutation: "createRefrigerationSystem";
  readonly index: number;
  readonly id: number;
  readonly caseCount: number;
  readonly designLoadW: number;
  readonly defrostScheduleId: number;
}

/** 🫠️ `delete-refrigeration-system` payload. */
export interface DeleteRefrigerationSystem {
  readonly mutation: "deleteRefrigerationSystem";
  readonly id: number;
}

/** 🗄️ `change-refrigeration-system-case-count` payload. */
export interface ChangeRefrigerationSystemCaseCount {
  readonly mutation: "changeRefrigerationSystemCaseCount";
  readonly id: number;
  readonly newCaseCount: number;
}

/** 🏋️ `change-refrigeration-system-design-load` payload. */
export interface ChangeRefrigerationSystemDesignLoad {
  readonly mutation: "changeRefrigerationSystemDesignLoad";
  readonly id: number;
  readonly newDesignLoadW: number;
}

/** 🕑️ `change-refrigeration-system-defrost-schedule` payload. */
export interface ChangeRefrigerationSystemDefrostSchedule {
  readonly mutation: "changeRefrigerationSystemDefrostSchedule";
  readonly id: number;
  readonly newDefrostScheduleId: number;
}

/** 🚽️ `create-water-system` payload. */
export interface CreateWaterSystem {
  readonly mutation: "createWaterSystem";
  readonly index: number;
  readonly id: number;
  readonly fixtureCount: number;
  readonly peakFlowLS: number;
  readonly scheduleId: number;
}

/** 🧼️ `delete-water-system` payload. */
export interface DeleteWaterSystem {
  readonly mutation: "deleteWaterSystem";
  readonly id: number;
}

/** 🪣️ `change-water-system-fixture-count` payload. */
export interface ChangeWaterSystemFixtureCount {
  readonly mutation: "changeWaterSystemFixtureCount";
  readonly id: number;
  readonly newFixtureCount: number;
}

/** 🚾️ `change-water-system-peak-flow` payload. */
export interface ChangeWaterSystemPeakFlow {
  readonly mutation: "changeWaterSystemPeakFlow";
  readonly id: number;
  readonly newPeakFlowLS: number;
}

/** 🕒️ `change-water-system-schedule` payload. */
export interface ChangeWaterSystemSchedule {
  readonly mutation: "changeWaterSystemSchedule";
  readonly id: number;
  readonly newScheduleId: number;
}

/** ⚠️ `create-fault` payload. */
export interface CreateFault {
  readonly mutation: "createFault";
  readonly index: number;
  readonly id: number;
  readonly targetEquipmentId: number;
  readonly faultType: "SensorBias" | "CoilFouling" | "DamperStuck" | "ChillerFouling" | "BoilerEfficiencyDegradation";
  readonly severity: number;
  readonly startScheduleId: number;
}

/** 🩹️ `delete-fault` payload. */
export interface DeleteFault {
  readonly mutation: "deleteFault";
  readonly id: number;
}

/** 🎣️ `change-fault-target-equipment` payload. */
export interface ChangeFaultTargetEquipment {
  readonly mutation: "changeFaultTargetEquipment";
  readonly id: number;
  readonly newTargetEquipmentId: number;
}

/** 🐛️ `change-fault-type` payload. */
export interface ChangeFaultType {
  readonly mutation: "changeFaultType";
  readonly id: number;
  readonly newFaultType: "SensorBias" | "CoilFouling" | "DamperStuck" | "ChillerFouling" | "BoilerEfficiencyDegradation";
}

/** 🌶️ `change-fault-severity` payload. */
export interface ChangeFaultSeverity {
  readonly mutation: "changeFaultSeverity";
  readonly id: number;
  readonly newSeverity: number;
}

/** 🕓️ `change-fault-start-schedule` payload. */
export interface ChangeFaultStartSchedule {
  readonly mutation: "changeFaultStartSchedule";
  readonly id: number;
  readonly newStartScheduleId: number;
}

/** 📋️ `create-space-list` payload. */
export interface CreateSpaceList {
  readonly mutation: "createSpaceList";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly spaceIds: readonly number[];
}

/** 🗒️ `delete-space-list` payload. */
export interface DeleteSpaceList {
  readonly mutation: "deleteSpaceList";
  readonly id: number;
}

/** 🪶️ `rename-space-list` payload. */
export interface RenameSpaceList {
  readonly mutation: "renameSpaceList";
  readonly id: number;
  readonly newName: string;
}

/** ➡️ `add-space-list-member` payload. */
export interface AddSpaceListMember {
  readonly mutation: "addSpaceListMember";
  readonly id: number;
  readonly index: number;
  readonly spaceId: number;
}

/** 🪤️ `remove-space-list-member` payload. */
export interface RemoveSpaceListMember {
  readonly mutation: "removeSpaceListMember";
  readonly id: number;
  readonly spaceId: number;
}

/** 🏟️ `create-thermal-enclosure` payload. */
export interface CreateThermalEnclosure {
  readonly mutation: "createThermalEnclosure";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly zoneIds: readonly number[];
}

/** 🏯️ `delete-thermal-enclosure` payload. */
export interface DeleteThermalEnclosure {
  readonly mutation: "deleteThermalEnclosure";
  readonly id: number;
}

/** 🖋️ `rename-thermal-enclosure` payload. */
export interface RenameThermalEnclosure {
  readonly mutation: "renameThermalEnclosure";
  readonly id: number;
  readonly newName: string;
}

/** 🔒️ `add-thermal-enclosure-zone` payload. */
export interface AddThermalEnclosureZone {
  readonly mutation: "addThermalEnclosureZone";
  readonly id: number;
  readonly index: number;
  readonly zoneId: number;
}

/** 🔓️ `remove-thermal-enclosure-zone` payload. */
export interface RemoveThermalEnclosureZone {
  readonly mutation: "removeThermalEnclosureZone";
  readonly id: number;
  readonly zoneId: number;
}

/** 🕜️ `create-constant-schedule` payload. */
export interface CreateConstantSchedule {
  readonly mutation: "createConstantSchedule";
  readonly index: number;
  readonly id: number;
  readonly value: number;
}

/** 📍️ `delete-constant-schedule` payload. */
export interface DeleteConstantSchedule {
  readonly mutation: "deleteConstantSchedule";
  readonly id: number;
}

/** 🕝️ `change-constant-schedule-value` payload. */
export interface ChangeConstantScheduleValue {
  readonly mutation: "changeConstantScheduleValue";
  readonly id: number;
  readonly newValue: number;
}

/** 🕞️ `create-daily-schedule` payload. */
export interface CreateDailySchedule {
  readonly mutation: "createDailySchedule";
  readonly index: number;
  readonly id: number;
  readonly hourlyValues: readonly number[];
  readonly interpolation: "Continuous" | "Discrete";
  readonly limitsMin: number | null;
  readonly limitsMax: number | null;
}

/** 🌓️ `delete-daily-schedule` payload. */
export interface DeleteDailySchedule {
  readonly mutation: "deleteDailySchedule";
  readonly id: number;
}

/** 🕔️ `replace-daily-schedule-hourly-values` payload. */
export interface ReplaceDailyScheduleHourlyValues {
  readonly mutation: "replaceDailyScheduleHourlyValues";
  readonly id: number;
  readonly newHourlyValues: readonly number[];
}

/** 🕕️ `change-daily-schedule-interpolation` payload. */
export interface ChangeDailyScheduleInterpolation {
  readonly mutation: "changeDailyScheduleInterpolation";
  readonly id: number;
  readonly newInterpolation: "Continuous" | "Discrete";
}

/** 🕟️ `change-daily-schedule-limits` payload. */
export interface ChangeDailyScheduleLimits {
  readonly mutation: "changeDailyScheduleLimits";
  readonly id: number;
  readonly newLimitsMin: number | null;
  readonly newLimitsMax: number | null;
}

/** 🗓️ `create-weekly-schedule` payload. */
export interface CreateWeeklySchedule {
  readonly mutation: "createWeeklySchedule";
  readonly index: number;
  readonly id: number;
  readonly dailyScheduleIds: readonly number[];
}

/** 🕖️ `delete-weekly-schedule` payload. */
export interface DeleteWeeklySchedule {
  readonly mutation: "deleteWeeklySchedule";
  readonly id: number;
}

/** 🕗️ `change-weekly-schedule-day` payload. */
export interface ChangeWeeklyScheduleDay {
  readonly mutation: "changeWeeklyScheduleDay";
  readonly id: number;
  readonly dayIndex: number;
  readonly newDailyScheduleId: number;
}

/** 📚️ `create-annual-schedule` payload. */
export interface CreateAnnualSchedule {
  readonly mutation: "createAnnualSchedule";
  readonly index: number;
  readonly id: number;
  readonly defaultDailyScheduleId: number;
  readonly holidayDailyScheduleId: number | null;
}

/** 📕️ `delete-annual-schedule` payload. */
export interface DeleteAnnualSchedule {
  readonly mutation: "deleteAnnualSchedule";
  readonly id: number;
}

/** 📗️ `insert-annual-schedule-rule` payload. */
export interface InsertAnnualScheduleRule {
  readonly mutation: "insertAnnualScheduleRule";
  readonly id: number;
  readonly index: number;
  readonly startMonth: number;
  readonly startDay: number;
  readonly endMonth: number;
  readonly endDay: number;
  readonly dailyScheduleId: number;
}

/** 📙️ `remove-annual-schedule-rule` payload. */
export interface RemoveAnnualScheduleRule {
  readonly mutation: "removeAnnualScheduleRule";
  readonly id: number;
  readonly index: number;
}

/** 🗂️ `reorder-annual-schedule-rules` payload. */
export interface ReorderAnnualScheduleRules {
  readonly mutation: "reorderAnnualScheduleRules";
  readonly id: number;
  readonly from: number;
  readonly to: number;
}

/** 🎌️ `change-annual-schedule-default-daily-schedule` payload. */
export interface ChangeAnnualScheduleDefaultDailySchedule {
  readonly mutation: "changeAnnualScheduleDefaultDailySchedule";
  readonly id: number;
  readonly newDefaultDailyScheduleId: number;
}

/** 🎄️ `change-annual-schedule-holiday-daily-schedule` payload. */
export interface ChangeAnnualScheduleHolidayDailySchedule {
  readonly mutation: "changeAnnualScheduleHolidayDailySchedule";
  readonly id: number;
  readonly newHolidayDailyScheduleId: number | null;
}

/** 🎉️ `add-annual-schedule-holiday` payload. */
export interface AddAnnualScheduleHoliday {
  readonly mutation: "addAnnualScheduleHoliday";
  readonly id: number;
  readonly index: number;
  readonly year: number;
  readonly month: number;
  readonly day: number;
}

/** 🎊️ `remove-annual-schedule-holiday` payload. */
export interface RemoveAnnualScheduleHoliday {
  readonly mutation: "removeAnnualScheduleHoliday";
  readonly id: number;
  readonly year: number;
  readonly month: number;
  readonly day: number;
}

/** 🪗️ `create-time-series-schedule` payload. */
export interface CreateTimeSeriesSchedule {
  readonly mutation: "createTimeSeriesSchedule";
  readonly index: number;
  readonly id: number;
  readonly values: readonly number[];
  readonly timestepSeconds: number;
}

/** 🎞️ `delete-time-series-schedule` payload. */
export interface DeleteTimeSeriesSchedule {
  readonly mutation: "deleteTimeSeriesSchedule";
  readonly id: number;
}

/** 🕘️ `replace-time-series-schedule-values` payload. */
export interface ReplaceTimeSeriesScheduleValues {
  readonly mutation: "replaceTimeSeriesScheduleValues";
  readonly id: number;
  readonly newValues: readonly number[];
}

/** 🕙️ `change-time-series-schedule-timestep` payload. */
export interface ChangeTimeSeriesScheduleTimestep {
  readonly mutation: "changeTimeSeriesScheduleTimestep";
  readonly id: number;
  readonly newTimestepSeconds: number;
}

export type EnergyModelMutation =
  | RenameModel
  | ChangeModelVersion
  | UpdateSite
  | UpdateGroundTemperature
  | UpdateRunPeriod
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
  | ChangeTimeSeriesScheduleTimestep;
