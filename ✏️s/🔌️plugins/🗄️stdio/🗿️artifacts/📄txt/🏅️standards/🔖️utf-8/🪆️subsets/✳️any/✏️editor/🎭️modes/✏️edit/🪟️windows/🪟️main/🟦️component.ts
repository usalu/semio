/** 📄️ Txt editor — `main` window: typed twin of `🦀️component.rs`'s `TextWindowKit` view-model. */

export interface TxtMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: string | null;
  readOnly: boolean;
}

/** ✏️ `replace-text` payload shape — mirrors `TxtEditorCommand::ReplaceText`, a whole-document
 * replace (re-split into `lines` on the document's own line ending). */
export interface TxtReplaceText {
  text: string;
}

export const TXT_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const TXT_MAIN_BODY_KEY = "framework.window.text" as const;
