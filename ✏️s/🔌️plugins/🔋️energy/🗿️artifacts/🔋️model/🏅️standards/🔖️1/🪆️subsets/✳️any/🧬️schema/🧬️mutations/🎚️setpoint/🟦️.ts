/** 🎚️ Energy-model mutation payloads of the `setpoint` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 📌️ `create-setpoint-manager` payload. */
export interface CreateSetpointManager {
  readonly mutation: "createSetpointManager";
  readonly id: number;
  readonly name: string;
  readonly kind: "Scheduled" | "OutdoorAirReset" | "WarmestZone" | "ColdestZone";
  readonly lowOutdoorC: number;
  readonly highOutdoorC: number;
  readonly lowSetpointC: number;
  readonly highSetpointC: number;
  readonly schedulePresent: boolean;
  readonly scheduleId: number;
  readonly index: number | null;
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
  readonly newKind: "Scheduled" | "OutdoorAirReset" | "WarmestZone" | "ColdestZone";
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
