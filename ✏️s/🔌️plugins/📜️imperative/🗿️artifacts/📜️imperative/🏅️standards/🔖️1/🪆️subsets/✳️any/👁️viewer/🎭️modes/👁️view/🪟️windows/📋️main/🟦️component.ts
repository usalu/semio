/** 📋️ Imperative viewer — main (steps) window: typed twin of `🦀️component.rs`'s `TableWindowKit`
 * view-model. Read-only mirror of the framework `TableView` payload `render()` produces — no
 * mutation-shaped fields, matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One top-level step row, read straight off the document's working `Path`. */
export interface ImperativeViewMainRow {
  index: number;
  id: string;
  kind: string;
}

/** 👁️ The main window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `ImperativeSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface ImperativeViewMainViewModel {
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  rows: ImperativeViewMainRow[];
}

export const IMPERATIVE_VIEW_MAIN_WINDOW_KIND_ID = "framework.window.table" as const;
export const IMPERATIVE_VIEW_MAIN_BODY_KEY = "framework.window.table" as const;
