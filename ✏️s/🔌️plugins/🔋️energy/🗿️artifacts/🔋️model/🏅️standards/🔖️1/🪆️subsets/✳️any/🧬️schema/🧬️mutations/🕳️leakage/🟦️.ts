/** 🧾️ Energy-model mutation payloads of the `leakage` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🅰️ `change-infiltration-constant-term-coefficient` payload. */
export interface ChangeInfiltrationConstantTermCoefficient {
  readonly mutation: "changeInfiltrationConstantTermCoefficient";
  readonly id: number;
  readonly newConstantTermCoefficient: number;
}

/** 🅱️ `change-infiltration-temperature-term-coefficient` payload. */
export interface ChangeInfiltrationTemperatureTermCoefficient {
  readonly mutation: "changeInfiltrationTemperatureTermCoefficient";
  readonly id: number;
  readonly newTemperatureTermCoefficient: number;
}

/** 🆎️ `change-infiltration-velocity-term-coefficient` payload. */
export interface ChangeInfiltrationVelocityTermCoefficient {
  readonly mutation: "changeInfiltrationVelocityTermCoefficient";
  readonly id: number;
  readonly newVelocityTermCoefficient: number;
}

/** 🆑️ `change-infiltration-velocity-squared-term-coefficient` payload. */
export interface ChangeInfiltrationVelocitySquaredTermCoefficient {
  readonly mutation: "changeInfiltrationVelocitySquaredTermCoefficient";
  readonly id: number;
  readonly newVelocitySquaredTermCoefficient: number;
}

/** 🕳️ `change-infiltration-effective-leakage-area` payload. */
export interface ChangeInfiltrationEffectiveLeakageArea {
  readonly mutation: "changeInfiltrationEffectiveLeakageArea";
  readonly id: number;
  readonly newEffectiveLeakageAreaM2: number;
}

/** 🚰️ `change-infiltration-discharge-coefficient` payload. */
export interface ChangeInfiltrationDischargeCoefficient {
  readonly mutation: "changeInfiltrationDischargeCoefficient";
  readonly id: number;
  readonly newDischargeCoefficient: number;
}

/** 🏭️ `change-infiltration-stack-height` payload. */
export interface ChangeInfiltrationStackHeight {
  readonly mutation: "changeInfiltrationStackHeight";
  readonly id: number;
  readonly newStackHeightM: number;
}
