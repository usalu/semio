/** 📊 BCF editor — Main window: typed twin of
 * `🦀️component.rs`'s view-model. Editable mirror of the shared `framework.window.table` scene payload
 * `render()` produces (one row per BCF topic: GUID/Title/Status/Priority/Author). */

export interface BcfAnyEditViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: string[][];
}

export const BCF_ANY_EDIT_WINDOW_KIND_ID = "framework.window.table" as const;
export const BCF_ANY_EDIT_BODY_KEY = "framework.window.table" as const;
