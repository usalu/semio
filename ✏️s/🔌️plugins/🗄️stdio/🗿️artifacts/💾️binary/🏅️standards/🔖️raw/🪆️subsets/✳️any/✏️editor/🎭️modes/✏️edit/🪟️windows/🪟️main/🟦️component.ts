/** 💾️ Binary editor — main window: typed twin of `🦀️component.rs`'s `TextWindowKit` view-model.
 * Editable mirror of the hex-dump summary `render()` produces. */

/** ✏️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `BinarySnapshot`). `text` is contiguous lowercase hex of the first `HEX_PREVIEW_CAP_BYTES`
 * bytes plus a trailing `#`-prefixed, non-editable byte-count comment. */
export interface BinaryEditMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: "hex";
  readOnly: false;
}

/** ✏️ `replace-text` payload shape — mirrors `BinaryEditorCommand::ReplaceText`. The hex text is
 * parsed back into bytes and spliced over the WHOLE original buffer; the `#`-prefixed comment line
 * is ignored on parse. */
export interface BinaryReplaceText {
  text: string;
}

export const BINARY_HEX_PREVIEW_CAP_BYTES = 4096 as const;

export const BINARY_EDIT_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const BINARY_EDIT_MAIN_BODY_KEY = "framework.window.text" as const;
