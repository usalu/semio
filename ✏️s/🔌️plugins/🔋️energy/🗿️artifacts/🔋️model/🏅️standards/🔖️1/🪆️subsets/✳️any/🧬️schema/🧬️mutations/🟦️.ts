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
  | ChangeZoneFloorAreaParticipation;
