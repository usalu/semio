/** 🧾️ Energy-model mutation payloads of the `infiltration` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
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
