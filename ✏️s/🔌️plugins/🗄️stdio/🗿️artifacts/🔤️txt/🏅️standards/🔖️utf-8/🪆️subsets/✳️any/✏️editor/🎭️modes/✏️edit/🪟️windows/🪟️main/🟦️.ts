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

/** ✂️ One range of the draft's change set: `delete` Unicode scalars at `offset` of the body as the document's line ending joins it
 * were replaced by `insert` (ascending, disjoint, in the coordinates of the text the draft started from). */
export interface TxtTextSplice {
  offset: number;
  delete: number;
  insert: string;
}

/** ✏️ `textEdit` payload shape — mirrors `TxtEditorCommand::SpliceText`: the draft's change set as JSON text
 * (`JSON.stringify(TxtTextSplice[])`), never the draft itself. */
export interface TxtTextEdit {
  revision: string;
  splices: string;
}

export const TXT_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const TXT_MAIN_BODY_KEY = "framework.window.text" as const;
