/** 🧾️ Energy-model mutation payloads of the `fault` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
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
