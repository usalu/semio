/** 🌳️ Energy model editor — `structure` window: typed twin of `🦀️.rs`'s `TreeWindowKit`
 * view-model. Mirrors the Rust `render()` boundary's output shape (`name`/`version` are the two
 * `set-node`-editable leaves; every other leaf is a read-only collection-count overview). */

/** 🌳️ One tree leaf — mirrors the framework `TreeNodeView` shape (`framework.window.tree`). */
export interface EnergyModelStructureNode {
  id: string;
  label: string;
  children: EnergyModelStructureNode[];
}

/** ✏️ The `structure` window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * input (a bare `EnergyModelSnapshot`, decoded server-side into `crate::model::Model`). */
export interface EnergyModelStructureViewModel {
  windowKindId: "framework.window.tree";
  bodyKey: "framework.window.tree";
  roots: EnergyModelStructureNode[];
}

/** ✏️ `set-node` payload shape — mirrors `EnergyModelEditorCommand::SetStructureField`. Only
 * `field: "name" | "version"` are real edit targets today (see the Rust window's own doc comment). */
export interface EnergyModelSetStructureField {
  field: "name" | "version";
  value: string;
}

/** 🌳️ The window's authored verbs beside the generic `set-node` kit action — the typed twin of
 * `🦀️.rs`'s `structure_actions()`. */
export const ENERGY_MODEL_STRUCTURE_ACTIONS = [
  { id: "create-surface", label: { en: "Create surface", de: "Fläche anlegen" } },
  { id: "delete-surface", label: { en: "Delete surface", de: "Fläche löschen" } },
  { id: "assign-surface-construction", label: { en: "Assign construction", de: "Konstruktion zuweisen" } },
  { id: "set-material-property", label: { en: "Set material property", de: "Materialeigenschaft setzen" } },
  { id: "set-surface-property", label: { en: "Set surface property", de: "Flächeneigenschaft setzen" } },
  { id: "set-fenestration-property", label: { en: "Set window property", de: "Fenstereigenschaft setzen" } },
  { id: "set-zone-property", label: { en: "Set zone property", de: "Zoneneigenschaft setzen" } },
  { id: "set-glazing-material-property", label: { en: "Set glazing material property", de: "Verglasungsmaterial-Eigenschaft setzen" } },
  { id: "set-gas-material-property", label: { en: "Set gas gap property", de: "Gasfüllungs-Eigenschaft setzen" } },
  { id: "set-thermostat-setpoints", label: { en: "Set thermostat setpoints", de: "Thermostat-Sollwerte setzen" } },
  { id: "set-site", label: { en: "Set site", de: "Standort setzen" } },
] as const;

/** 🧱️ The seven SI-range-checked scalars `set-material-property` addresses. */
export type EnergyModelMaterialProperty =
  | "thicknessM"
  | "conductivityWMK"
  | "densityKgM3"
  | "specificHeatJKgK"
  | "thermalAbsorptance"
  | "solarAbsorptance"
  | "visibleAbsorptance";

/** 🔍️ The properties the three generic inspector verbs address. `value` is TEXT for all of them:
 * one verb has to carry a name, an enum spelling, a flag and a scalar alike. */
export type EnergyModelSurfaceProperty = "name" | "class" | "boundary" | "construction" | "sunExposed" | "windExposed" | "multiplier";

export type EnergyModelFenestrationProperty =
  | "name"
  | "uValueWM2K"
  | "shgc"
  | "vlt"
  | "areaM2"
  | "heightM"
  | "sillHeightM"
  | "frameConductanceWK"
  | "dividerConductanceWK"
  | "overhangDepthM"
  | "overhangOffsetM"
  | "finDepthM"
  | "finOffsetM"
  | "glazingConstruction";

export type EnergyModelZoneProperty = "name" | "volumeM3" | "multiplier" | "conditioned" | "partOfTotalFloorArea";

/** 🧊️ Only the fields `change-glazing-material-*`/`rename-glazing-material` name are addressable —
 * the four reflectances and the infrared transmittance still have no mutation kind. */
export type EnergyModelGlazingMaterialProperty = "name" | "thicknessM" | "conductivityWMK" | "solarTransmittance" | "visibleTransmittance" | "infraredEmissivityFront" | "infraredEmissivityBack";

export type EnergyModelGasMaterialProperty = "name" | "thicknessM" | "gas";

export const ENERGY_MODEL_STRUCTURE_WINDOW_KIND_ID = "framework.window.tree" as const;
export const ENERGY_MODEL_STRUCTURE_BODY_KEY = "framework.window.tree" as const;
