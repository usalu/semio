/** 📊️ Energy model viewer — `zones` window: typed twin of `🦀️.rs`'s `TableWindowKit`
 * view-model. Read-only mirror of the Rust `render()` boundary's output — one row per
 * `crate::model::Zone`, no mutation-shaped fields. */

/** 👁️ The `zones` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `EnergyModelSnapshot`). Columns fixed at `id`/`name`/`volumeM3`/`multiplier`/`conditioned`/
 * `partOfTotalFloorArea`. */
export interface EnergyModelZonesViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: ["id", "name", "volumeM3", "multiplier", "conditioned", "partOfTotalFloorArea"];
  rows: string[][];
}

export const ENERGY_MODEL_ZONES_WINDOW_KIND_ID = "framework.window.table" as const;
export const ENERGY_MODEL_ZONES_BODY_KEY = "framework.window.table" as const;
