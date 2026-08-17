/** 📰 Xml editor — `main` window: typed twin of `🦀️component.rs`'s `TreeWindowKit` view-model. */

export interface XmlMainNode {
  id: string;
  label: string;
  children: XmlMainNode[];
}

export interface XmlMainViewModel {
  windowKindId: "framework.window.tree";
  bodyKey: "framework.window.tree";
  roots: XmlMainNode[];
}

/** ✏️ `set-node` payload shape — mirrors `XmlValidEditorCommand::SetText`. `nodeId` is a `/`-joined
 * child-index path from the root (only `Text` nodes are real edit targets). */
export interface XmlSetNode {
  nodeId: string;
  value: string;
}

export const XML_MAIN_WINDOW_KIND_ID = "framework.window.tree" as const;
export const XML_MAIN_BODY_KEY = "framework.window.tree" as const;
