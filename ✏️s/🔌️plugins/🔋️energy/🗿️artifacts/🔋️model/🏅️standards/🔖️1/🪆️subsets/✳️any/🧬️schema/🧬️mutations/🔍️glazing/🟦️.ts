/** 🔍️ Energy-model mutation payloads of the `glazing` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🌐️ `change-fenestration-u-value` payload. */
export interface ChangeFenestrationUValue {
  readonly mutation: "changeFenestrationUValue";
  readonly id: number;
  readonly newUValueWM2k: number;
}

/** 🌇️ `change-fenestration-shgc` payload. */
export interface ChangeFenestrationShgc {
  readonly mutation: "changeFenestrationShgc";
  readonly id: number;
  readonly newShgc: number;
}

/** 🌈️ `change-fenestration-vlt` payload. */
export interface ChangeFenestrationVlt {
  readonly mutation: "changeFenestrationVlt";
  readonly id: number;
  readonly newVlt: number;
}

/** 🖼️ `change-fenestration-frame-conductance` payload. */
export interface ChangeFenestrationFrameConductance {
  readonly mutation: "changeFenestrationFrameConductance";
  readonly id: number;
  readonly newFrameConductanceWK: number;
}

/** 🧷️ `change-fenestration-divider-conductance` payload. */
export interface ChangeFenestrationDividerConductance {
  readonly mutation: "changeFenestrationDividerConductance";
  readonly id: number;
  readonly newDividerConductanceWK: number;
}

/** 🧊️ `bind-fenestration-glazing-construction` payload. */
export interface BindFenestrationGlazingConstruction {
  readonly mutation: "bindFenestrationGlazingConstruction";
  readonly id: number;
  readonly constructionId: number;
}

/** 🫗️ `clear-fenestration-glazing-construction` payload. */
export interface ClearFenestrationGlazingConstruction {
  readonly mutation: "clearFenestrationGlazingConstruction";
  readonly id: number;
}
