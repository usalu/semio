/** 🧾️ Energy-model mutation payloads of the `material` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🧱️ `create-material` payload. */
export interface CreateMaterial {
  readonly mutation: "createMaterial";
  readonly index: number;
  readonly id: number;
  readonly name: string;
  readonly roughness: "VeryRough" | "Rough" | "MediumRough" | "MediumSmooth" | "Smooth" | "VerySmooth";
  readonly thicknessM: number;
  readonly conductivityWMK: number;
  readonly densityKgM3: number;
  readonly specificHeatJKgK: number;
  readonly thermalAbsorptance: number;
  readonly solarAbsorptance: number;
  readonly visibleAbsorptance: number;
}

/** 🪨️ `delete-material` payload. */
export interface DeleteMaterial {
  readonly mutation: "deleteMaterial";
  readonly id: number;
}

/** 🪧️ `rename-material` payload. */
export interface RenameMaterial {
  readonly mutation: "renameMaterial";
  readonly id: number;
  readonly newName: string;
}

/** 📏️ `change-material-thickness` payload. */
export interface ChangeMaterialThickness {
  readonly mutation: "changeMaterialThickness";
  readonly id: number;
  readonly newThicknessM: number;
}

/** 🔥️ `change-material-conductivity` payload. */
export interface ChangeMaterialConductivity {
  readonly mutation: "changeMaterialConductivity";
  readonly id: number;
  readonly newConductivityWMK: number;
}

/** ⚖️ `change-material-density` payload. */
export interface ChangeMaterialDensity {
  readonly mutation: "changeMaterialDensity";
  readonly id: number;
  readonly newDensityKgM3: number;
}

/** ♨️ `change-material-specific-heat` payload. */
export interface ChangeMaterialSpecificHeat {
  readonly mutation: "changeMaterialSpecificHeat";
  readonly id: number;
  readonly newSpecificHeatJKgK: number;
}
