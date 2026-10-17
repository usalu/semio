/** 🎨️ Energy-model mutation payloads of the `finish` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🔆️ `change-material-thermal-absorptance` payload. */
export interface ChangeMaterialThermalAbsorptance {
  readonly mutation: "changeMaterialThermalAbsorptance";
  readonly id: number;
  readonly newThermalAbsorptance: number;
}

/** ☀️ `change-material-solar-absorptance` payload. */
export interface ChangeMaterialSolarAbsorptance {
  readonly mutation: "changeMaterialSolarAbsorptance";
  readonly id: number;
  readonly newSolarAbsorptance: number;
}

/** 👁️ `change-material-visible-absorptance` payload. */
export interface ChangeMaterialVisibleAbsorptance {
  readonly mutation: "changeMaterialVisibleAbsorptance";
  readonly id: number;
  readonly newVisibleAbsorptance: number;
}

/** 🗻️ `change-material-roughness` payload. */
export interface ChangeMaterialRoughness {
  readonly mutation: "changeMaterialRoughness";
  readonly id: number;
  readonly newRoughness: "VeryRough" | "Rough" | "MediumRough" | "MediumSmooth" | "Smooth" | "VerySmooth";
}
