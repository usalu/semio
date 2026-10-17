/** 🧾️ Energy-model mutation payloads of the `equipment-gain` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
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
