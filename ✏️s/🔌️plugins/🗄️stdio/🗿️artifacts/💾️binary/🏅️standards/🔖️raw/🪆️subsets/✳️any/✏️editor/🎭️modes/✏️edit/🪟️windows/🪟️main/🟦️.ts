/** 💾️ Binary editor — main window: typed twin of `🦀️.rs`'s `TextWindowKit` view-model.
 * Editable mirror of the hex-dump summary `render()` produces. */

/** ✏️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `BinarySnapshot`). `text` is complete contiguous lowercase hex plus a trailing
 * `#`-prefixed byte-count comment. */
export interface BinaryEditMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: "hex";
  readOnly: false;
}

/** ✏️ `textEdit` payload shape — mirrors `BinaryEditorCommand::ReplaceText`. The hex text is
 * parsed back into bytes and spliced over the WHOLE original buffer; the `#`-prefixed comment line
 * is ignored on parse. */
export interface BinaryTextEdit {
  text: string;
}

export const BINARY_HEX_EDITOR_MAX_BYTES = 1_048_576 as const;

export const BINARY_EDIT_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const BINARY_EDIT_MAIN_BODY_KEY = "framework.window.text" as const;
