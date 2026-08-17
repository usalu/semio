/** 🌳️ Energy model viewer — `structure` window: typed twin of `🦀️component.rs`'s `TreeWindowKit`
 * view-model. Read-only mirror of the Rust `render()` boundary's output — no edit-target fields, no
 * command payload types (a viewer's `Command` never carries a document mutation). */

/** 👁️ One tree leaf — mirrors the framework `TreeNodeView` shape (`framework.window.tree`). */
export interface EnergyModelStructureNode {
  id: string;
  label: string;
  children: EnergyModelStructureNode[];
}

/** 👁️ The `structure` window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * input (a bare `EnergyModelSnapshot`, decoded server-side into `crate::model::Model`). */
export interface EnergyModelStructureViewModel {
  windowKindId: "framework.window.tree";
  bodyKey: "framework.window.tree";
  roots: EnergyModelStructureNode[];
}

export const ENERGY_MODEL_STRUCTURE_WINDOW_KIND_ID = "framework.window.tree" as const;
export const ENERGY_MODEL_STRUCTURE_BODY_KEY = "framework.window.tree" as const;
