/** 📄️ Txt editor — `main` window: typed twin of `🦀️.rs`'s `TextWindowKit` view-model. */

export interface TxtMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: string | null;
  /** ✍️ The kit's explicit-draft policy: edited locally, ONE `textEdit` on Apply, refused over a changed `revision`. */
  commit: "explicit";
  revision: string;
}

/** ✏️ `textEdit` payload shape — mirrors `TxtEditorCommand::ReplaceText`, a whole-document
 * replace (re-split into `lines` on the document's own line ending). */
export interface TxtTextEdit {
  revision: string;
  text: string;
}

export const TXT_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const TXT_MAIN_BODY_KEY = "framework.window.text" as const;
