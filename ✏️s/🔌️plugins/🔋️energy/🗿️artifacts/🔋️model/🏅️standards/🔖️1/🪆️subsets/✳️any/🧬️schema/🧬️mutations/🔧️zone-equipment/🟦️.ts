/** 🧾️ Energy-model mutation payloads of the `zone-equipment` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🛠️ `create-zone-equipment` payload. */
export interface CreateZoneEquipment {
  readonly mutation: "createZoneEquipment";
  readonly id: number;
  readonly zoneId: number;
  readonly equipmentType: "Baseboard" | "Radiant" | "FanCoil" | "Ptac" | "VrfTerminal" | "Erv" | "UnitHeater" | "WaterToAirHp";
  readonly priority: number;
  readonly heatingCapacityW: number;
  readonly coolingCapacityW: number;
  readonly index: number | null;
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
