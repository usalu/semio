/** 🗜️ Deflate viewer — main window: typed twin of `🦀️.rs`'s read-only `TextWindowKit`
 * view-model. */

export type DeflateLevelHintKeyword = "fastest" | "fast" | "default" | "maximum";

/** 👁️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `DeflateSnapshot`). */
export interface DeflateViewMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: "deflate-summary";
  readOnly: true;
}

export const DEFLATE_VIEW_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const DEFLATE_VIEW_MAIN_BODY_KEY = "framework.window.text" as const;
