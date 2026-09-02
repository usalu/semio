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

export const ENERGY_MODEL_STRUCTURE_WINDOW_KIND_ID = "framework.window.tree" as const;
export const ENERGY_MODEL_STRUCTURE_BODY_KEY = "framework.window.tree" as const;
