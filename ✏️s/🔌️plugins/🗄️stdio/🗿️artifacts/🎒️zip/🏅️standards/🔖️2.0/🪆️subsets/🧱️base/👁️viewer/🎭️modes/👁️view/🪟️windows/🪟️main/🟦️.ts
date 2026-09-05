/** 🎒️ Zip viewer (2.0/🧱️base) — main window: typed twin of `🦀️.rs`'s read-only
 * `TreeWindowKit` view-model. */

/** 🌳️ One tree leaf — mirrors the framework `TreeNodeView` shape (`framework.window.tree`). */
export interface ZipArchiveNode {
  id: string;
  label: string;
  children: ZipArchiveNode[];
}

/** 👁️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's input
 * (a bare `ZipSnapshot`). */
export interface ZipViewMainViewModel {
  windowKindId: "framework.window.tree";
  bodyKey: "framework.window.tree";
  roots: ZipArchiveNode[];
}

export const ZIP_COMMENT_NODE_ID = "comment" as const;
export const ZIP_ENTRY_NODE_PREFIX = "entry:" as const;

export const ZIP_VIEW_MAIN_WINDOW_KIND_ID = "framework.window.tree" as const;
export const ZIP_VIEW_MAIN_BODY_KEY = "framework.window.tree" as const;
