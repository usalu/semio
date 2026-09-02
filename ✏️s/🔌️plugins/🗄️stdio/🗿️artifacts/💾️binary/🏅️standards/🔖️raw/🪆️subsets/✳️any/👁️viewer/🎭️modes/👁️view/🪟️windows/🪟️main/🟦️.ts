/** 💾️ Binary viewer — main window: typed twin of `🦀️.rs`'s read-only `TextWindowKit`
 * view-model. */

/** 👁️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `BinarySnapshot`). */
export interface BinaryViewMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: "hex";
  readOnly: true;
}

export const BINARY_HEX_PREVIEW_CAP_BYTES = 4096 as const;

export const BINARY_VIEW_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const BINARY_VIEW_MAIN_BODY_KEY = "framework.window.text" as const;
