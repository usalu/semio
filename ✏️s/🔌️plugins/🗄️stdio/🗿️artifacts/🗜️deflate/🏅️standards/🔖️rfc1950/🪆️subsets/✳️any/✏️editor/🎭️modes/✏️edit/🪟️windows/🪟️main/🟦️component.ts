/** 🗜️ Deflate editor — main window: typed twin of `🦀️component.rs`'s `TextWindowKit` view-model.
 * Editable mirror of the RFC1950 header summary `render()` produces. */

export type DeflateLevelHintKeyword = "fastest" | "fast" | "default" | "maximum";

/** ✏️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `DeflateSnapshot`). `text` is a `key=value` summary of `method`/`windowBits`/`levelHint`/
 * `presetDictionary` plus a trailing `#`-prefixed, non-editable payload byte-count comment. */
export interface DeflateEditMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: "deflate-summary";
  readOnly: false;
}

/** ✏️ `replace-text` payload shape — mirrors `DeflateEditorCommand::ReplaceText`. The whole header
 * summary is parsed back into `SetCompressionParams`/`SetPresetDictionary`; the payload comment
 * line is ignored on parse (informational only, never round-tripped). */
export interface DeflateReplaceText {
  text: string;
}

export const DEFLATE_EDIT_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const DEFLATE_EDIT_MAIN_BODY_KEY = "framework.window.text" as const;
