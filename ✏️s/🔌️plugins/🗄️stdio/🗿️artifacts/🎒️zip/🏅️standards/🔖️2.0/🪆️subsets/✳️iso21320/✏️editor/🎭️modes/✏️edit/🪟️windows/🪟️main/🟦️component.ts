/** 🎒️ Zip editor (2.0/✳️iso21320) — main window: typed twin of `🦀️component.rs`'s `TreeWindowKit`
 * view-model. Root node addresses the archive comment; each leaf addresses one entry's name. */

/** 🌳️ One tree leaf — mirrors the framework `TreeNodeView` shape (`framework.window.tree`). */
export interface ZipArchiveNode {
  id: string;
  label: string;
  children: ZipArchiveNode[];
}

/** ✏️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `ZipSnapshot`). */
export interface ZipEditMainViewModel {
  windowKindId: "framework.window.tree";
  bodyKey: "framework.window.tree";
  roots: ZipArchiveNode[];
}

/** ✏️ `set-node` payload shape — mirrors `ZipEditorCommand::SetNode`. `nodeId` is either the fixed
 * `"comment"` root id or `"entry:{index}"` for one archive entry's name. */
export interface ZipSetNode {
  nodeId: string;
  value: string;
}

export const ZIP_COMMENT_NODE_ID = "comment" as const;
export const ZIP_ENTRY_NODE_PREFIX = "entry:" as const;

export const ZIP_EDIT_MAIN_WINDOW_KIND_ID = "framework.window.tree" as const;
export const ZIP_EDIT_MAIN_BODY_KEY = "framework.window.tree" as const;
