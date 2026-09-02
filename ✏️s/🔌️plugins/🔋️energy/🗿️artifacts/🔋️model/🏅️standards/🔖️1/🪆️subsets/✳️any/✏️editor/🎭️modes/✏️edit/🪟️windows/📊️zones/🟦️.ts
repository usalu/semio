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

export const ENERGY_MODEL_ZONES_WINDOW_KIND_ID = "framework.window.table" as const;
export const ENERGY_MODEL_ZONES_BODY_KEY = "framework.window.table" as const;
