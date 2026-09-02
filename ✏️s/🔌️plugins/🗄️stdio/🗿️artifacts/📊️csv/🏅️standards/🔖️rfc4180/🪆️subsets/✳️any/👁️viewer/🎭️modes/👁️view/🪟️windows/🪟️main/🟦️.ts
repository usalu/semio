/** 📊️ Csv viewer — `main` window: typed twin of `🦀️.rs`'s `TableWindowKit` view-model.
 * Read-only mirror of the editor's own `main` window payload shape — no `set-cell` payload type
 * here, a viewer never emits an artifact mutation. */

/** 👁️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * input (a bare `CsvSnapshot`). */
export interface CsvMainViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: string[][];
}

export const CSV_MAIN_WINDOW_KIND_ID = "framework.window.table" as const;
export const CSV_MAIN_BODY_KEY = "framework.window.table" as const;
