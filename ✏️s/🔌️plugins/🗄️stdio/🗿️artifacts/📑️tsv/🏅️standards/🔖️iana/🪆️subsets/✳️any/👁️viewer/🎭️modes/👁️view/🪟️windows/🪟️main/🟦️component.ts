/** 📑️ Tsv viewer — `main` window: typed twin of `🦀️component.rs`'s `TableWindowKit` view-model.
 * Read-only mirror of the editor's own `main` window payload shape. */

export interface TsvMainViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: string[][];
}

export const TSV_MAIN_WINDOW_KIND_ID = "framework.window.table" as const;
export const TSV_MAIN_BODY_KEY = "framework.window.table" as const;
