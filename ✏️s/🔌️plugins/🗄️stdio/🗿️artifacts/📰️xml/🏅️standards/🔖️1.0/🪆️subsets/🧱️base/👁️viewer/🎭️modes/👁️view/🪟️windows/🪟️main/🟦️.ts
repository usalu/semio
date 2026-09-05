/** 📰 Xml viewer — `main` window: typed twin of `🦀️.rs`'s `TreeWindowKit` view-model.
 * Read-only mirror of the editor's own `main` window payload shape. */

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

export const XML_MAIN_WINDOW_KIND_ID = "framework.window.tree" as const;
export const XML_MAIN_BODY_KEY = "framework.window.tree" as const;
