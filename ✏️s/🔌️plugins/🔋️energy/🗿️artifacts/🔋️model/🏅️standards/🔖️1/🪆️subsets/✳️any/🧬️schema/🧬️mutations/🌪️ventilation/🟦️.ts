/** 🧾️ Energy-model mutation payloads of the `ventilation` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
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
