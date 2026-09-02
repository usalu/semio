/** 📊️ Xlsx editor (ecma-376/✳️strict) — `main` window: typed twin of `🦀️.rs`'s
 * `TableWindowKit` view-model. Mirrors the Rust `render()` boundary's output shape — one row per
 * `(sheet, row, col, value)` cell. */

/** ✏️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `XlsxSnapshot`). Columns fixed at `sheet`/`row`/`col`/`value`. */
export interface XlsxMainViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: ["sheet", "row", "col", "value"];
  rows: string[][];
}

/** ✏️ `set-cell` payload shape — mirrors `XlsxStrictEditorCommand::SetCell`. `row` indexes the
 * flattened cell list `xlsx_flat_cells` emits (sheet order, then each sheet's own cell storage
 * order); `value` is the edited cell's raw display text. */
export interface XlsxSetCell {
  row: number;
  value: string;
}

export const XLSX_MAIN_WINDOW_KIND_ID = "framework.window.table" as const;
export const XLSX_MAIN_BODY_KEY = "framework.window.table" as const;
