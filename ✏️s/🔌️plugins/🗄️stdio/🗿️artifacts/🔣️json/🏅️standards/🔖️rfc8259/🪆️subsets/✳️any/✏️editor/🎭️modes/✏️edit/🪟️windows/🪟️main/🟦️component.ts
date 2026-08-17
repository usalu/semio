/** 🔣️ Json editor — `main` window: typed twin of `🦀️component.rs`'s `TreeWindowKit` view-model. */

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

/** ✏️ `set-node` payload shape — mirrors `JsonAnyEditorCommand::SetScalar`. `nodeId` is the
 * `k=<key>`/`i=<index>` path encoding (`/`-joined, root is `""`) the Rust window's
 * `encode_path_id` produces. */
export interface JsonSetNode {
  nodeId: string;
  value: string;
}

export const JSON_MAIN_WINDOW_KIND_ID = "framework.window.tree" as const;
export const JSON_MAIN_BODY_KEY = "framework.window.tree" as const;
