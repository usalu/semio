/** 🚿️ Energy-model mutation payloads of the `shw-system` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
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
