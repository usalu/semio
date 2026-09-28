/** 🔣️ Json editor — `main` window: typed twin of `🦀️.rs`'s `TreeWindowKit` view-model. */

export interface JsonMainNode {
  id: string;
  label: string;
  children: JsonMainNode[];
}

export interface JsonMainViewModel {
  windowKindId: "framework.window.tree";
  bodyKey: "framework.window.tree";
  roots: JsonMainNode[];
}

/** ✏️ `set-node` payload shape — mirrors `JsonAnyEditorCommand::SetNode`. `nodeId` is the
 * `m=<member-index>`/`i=<index>` source-ordinal path encoding (root is `$`) the Rust window's
 * `encode_path_id` produces. */
export interface JsonSetNode {
  nodeId: string;
  revision: string;
  value: string;
}

export const JSON_MAIN_WINDOW_KIND_ID = "framework.window.tree" as const;
export const JSON_MAIN_BODY_KEY = "framework.window.tree" as const;
