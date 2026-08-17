/** 📑️ Tsv editor — `main` window: typed twin of `🦀️component.rs`'s `TableWindowKit` view-model.
 * One row per `TsvSnapshot.records` entry, columns synthesized positionally. */

export interface TsvMainViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: string[][];
}

/** ✏️ `set-cell` payload shape — mirrors `TsvEditorCommand::SetCell`, a direct 1:1 index into
 * `records` (no header-offset math, unlike csv). */
export interface TsvSetCell {
  row: number;
  column: number;
  value: string;
}

export const TSV_MAIN_WINDOW_KIND_ID = "framework.window.table" as const;
export const TSV_MAIN_BODY_KEY = "framework.window.table" as const;
