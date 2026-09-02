/** 📝️ Imperative editor — the script window: typed twin of `🦀️.rs`'s `render()` boundary.
 * The compiled, read-only textual form of the document (`ImperativeHost::compile_text`). */

export interface ImperativePlayScriptViewModel {
  windowKindId: "imperative-script";
  bodyKey: "imperative.play.script";
  surfaceId: "imperative.play.script";
  text: string;
  language: "imperative";
}

export const IMPERATIVE_PLAY_WINDOW_SCRIPT = "imperative-script" as const;
export const IMPERATIVE_PLAY_BODY_SCRIPT = "imperative.play.script" as const;
export const IMPERATIVE_PLAY_SURFACE_SCRIPT = "imperative.play.script" as const;
