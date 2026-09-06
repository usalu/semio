/** 📊️ Energy model editor — `zones` window: typed twin of `🦀️.rs`'s `TableWindowKit`
 * view-model. Mirrors the Rust `render()` boundary's output shape — one row per `crate::model::Zone`. */

/** ✏️ The `zones` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `EnergyModelSnapshot`). Columns fixed at `id`/`name`/`volumeM3`/`multiplier`/`conditioned`/
 * `partOfTotalFloorArea`, matching `crate::artifacts::model::energy_zones_table_from_model`'s shape. */
export interface EnergyModelZonesViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: ["id", "name", "volumeM3", "multiplier", "conditioned", "partOfTotalFloorArea"];
  rows: string[][];
}

/** ✏️ `set-cell` payload shape — mirrors `EnergyModelEditorCommand::SetZoneCell`. `row` indexes
 * `model.zones` directly; `column` is every column but `id` (id is not a real edit target). */
export interface EnergyModelSetZoneCell {
  row: number;
  column: "name" | "volumeM3" | "multiplier" | "conditioned" | "partOfTotalFloorArea";
  value: string;
}

/** 📊️ The window's authored verbs beside the generic `set-cell` kit action — the typed twin of
 * `🦀️.rs`'s `zone_actions()`. A zone that is still referenced by a space, surface or thermostat is
 * refused with `mutation.target-in-use` rather than cascaded. */
export const ENERGY_MODEL_ZONE_ACTIONS = [
  { id: "create-zone", label: { en: "Create zone", de: "Zone anlegen" } },
  { id: "rename-zone", label: { en: "Rename zone", de: "Zone umbenennen" } },
  { id: "delete-zone", label: { en: "Delete zone", de: "Zone löschen" } },
] as const;

export const ENERGY_MODEL_ZONES_WINDOW_KIND_ID = "framework.window.table" as const;
export const ENERGY_MODEL_ZONES_BODY_KEY = "framework.window.table" as const;
