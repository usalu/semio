/** 🥂️ Energy-model mutation payloads of the `window-layer` group: the TypeScript twin of the Rust leaves below `./`, one interface per leaf, wire-tagged by the camelCase variant name. */
/** 🔷️ `change-glazing-material-thickness` payload. */
export interface ChangeGlazingMaterialThickness {
  readonly mutation: "changeGlazingMaterialThickness";
  readonly id: number;
  readonly newThicknessM: number;
}

/** 🟠️ `change-glazing-material-conductivity` payload. */
export interface ChangeGlazingMaterialConductivity {
  readonly mutation: "changeGlazingMaterialConductivity";
  readonly id: number;
  readonly newConductivityWMK: number;
}

/** 🟡️ `change-glazing-material-solar-transmittance` payload. */
export interface ChangeGlazingMaterialSolarTransmittance {
  readonly mutation: "changeGlazingMaterialSolarTransmittance";
  readonly id: number;
  readonly newSolarTransmittance: number;
}

/** 🥽️ `change-glazing-material-visible-transmittance` payload. */
export interface ChangeGlazingMaterialVisibleTransmittance {
  readonly mutation: "changeGlazingMaterialVisibleTransmittance";
  readonly id: number;
  readonly newVisibleTransmittance: number;
}

/** 🩻️ `change-glazing-material-infrared-emissivity` payload. */
export interface ChangeGlazingMaterialInfraredEmissivity {
  readonly mutation: "changeGlazingMaterialInfraredEmissivity";
  readonly id: number;
  readonly newInfraredEmissivityFront: number;
  readonly newInfraredEmissivityBack: number;
}

/** 🟢️ `rename-glazing-material` payload. */
export interface RenameGlazingMaterial {
  readonly mutation: "renameGlazingMaterial";
  readonly id: number;
  readonly newName: string;
}

/** 🟣️ `change-gas-material-thickness` payload. */
export interface ChangeGasMaterialThickness {
  readonly mutation: "changeGasMaterialThickness";
  readonly id: number;
  readonly newThicknessM: number;
}

/** 🟤️ `change-gas-material-gas` payload. */
export interface ChangeGasMaterialGas {
  readonly mutation: "changeGasMaterialGas";
  readonly id: number;
  readonly newGas: "Air" | "Argon" | "Krypton" | "Xenon";
}

/** 🔘️ `rename-gas-material` payload. */
export interface RenameGasMaterial {
  readonly mutation: "renameGasMaterial";
  readonly id: number;
  readonly newName: string;
}
