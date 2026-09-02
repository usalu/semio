/** 📊️ Csv editor — `main` window: typed twin of `🦀️.rs`'s `TableWindowKit` view-model.
 * Mirrors the Rust `render()` boundary's output shape (header row, when present, supplies column
 * labels; every remaining record is one editable row). */

/** ✏️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * input (a bare `CsvSnapshot`). */
export interface CsvMainViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: string[][];
}

/** ✏️ `set-cell` payload shape — mirrors `CsvEditorCommand::SetCell`. `row`/`column` index the
 * RENDERED grid (post header-split), not the raw `CsvSnapshot.records` array directly. */
export interface CsvSetCell {
  row: number;
  column: number;
  value: string;
}

export const CSV_MAIN_WINDOW_KIND_ID = "framework.window.table" as const;
export const CSV_MAIN_BODY_KEY = "framework.window.table" as const;
