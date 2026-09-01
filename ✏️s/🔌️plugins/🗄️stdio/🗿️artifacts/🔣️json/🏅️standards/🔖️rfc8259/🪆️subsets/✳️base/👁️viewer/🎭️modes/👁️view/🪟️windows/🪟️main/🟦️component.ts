/** 🔣️ Json viewer — `main` window: typed twin of `🦀️component.rs`'s `TreeWindowKit` view-model.
 * Read-only mirror of the editor's own `main` window payload shape. */

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

export const JSON_MAIN_WINDOW_KIND_ID = "framework.window.tree" as const;
export const JSON_MAIN_BODY_KEY = "framework.window.tree" as const;
