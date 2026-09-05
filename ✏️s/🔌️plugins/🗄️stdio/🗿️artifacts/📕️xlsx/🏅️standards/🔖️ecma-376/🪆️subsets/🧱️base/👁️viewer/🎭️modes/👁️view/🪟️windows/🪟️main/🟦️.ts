/** 📊️ Xlsx viewer (ecma-376/🧱️base) — `main` window: typed twin of `🦀️.rs`'s
 * `TableWindowKit` view-model. Read-only mirror — one row per `(sheet, row, col, value)` cell, no
 * mutation-shaped fields. */
export interface XlsxMainViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: ["sheet", "row", "col", "value"];
  rows: string[][];
}

export const XLSX_MAIN_WINDOW_KIND_ID = "framework.window.table" as const;
export const XLSX_MAIN_BODY_KEY = "framework.window.table" as const;
