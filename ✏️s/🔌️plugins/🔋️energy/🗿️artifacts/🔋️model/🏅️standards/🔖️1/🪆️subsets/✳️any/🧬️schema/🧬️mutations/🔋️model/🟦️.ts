/** 🔋️ Energy-model mutation payloads of the `model` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
import type { ArtifactRef } from "@semio-tech/framework/schema/artifact-reference";

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
  readonly index: number | null;
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
  readonly target: ArtifactRef;
}

/** 🌤️ `unbind-weather-file` payload. */
export interface UnbindWeatherFile {
  readonly mutation: "unbindWeatherFile";
}

/** 🪢️ `connect-referenced-model` payload. */
export interface ConnectReferencedModel {
  readonly mutation: "connectReferencedModel";
  readonly target: ArtifactRef;
}

/** ✂️ `disconnect-referenced-model` payload. */
export interface DisconnectReferencedModel {
  readonly mutation: "disconnectReferencedModel";
}
