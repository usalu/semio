/** 📝️ Imperative viewer — script window: typed twin of `🦀️.rs`'s `TextWindowKit`
 * view-model. Read-only mirror of the framework `TextView` payload `render()` produces. */

export interface ImperativeViewScriptViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: "imperative";
  readOnly: true;
}

export const IMPERATIVE_VIEW_SCRIPT_WINDOW_KIND_ID = "framework.window.text" as const;
export const IMPERATIVE_VIEW_SCRIPT_BODY_KEY = "framework.window.text" as const;
