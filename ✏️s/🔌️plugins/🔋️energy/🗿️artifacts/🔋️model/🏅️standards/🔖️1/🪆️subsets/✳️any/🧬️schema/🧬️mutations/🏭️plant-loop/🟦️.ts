/** 🏭️ Energy-model mutation payloads of the `plant-loop` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
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
  readonly index: number | null;
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
  readonly index: number | null;
}

/** ⚙️ `remove-plant-loop-equipment` payload. */
export interface RemovePlantLoopEquipment {
  readonly mutation: "removePlantLoopEquipment";
  readonly id: number;
  readonly equipmentId: number;
}
