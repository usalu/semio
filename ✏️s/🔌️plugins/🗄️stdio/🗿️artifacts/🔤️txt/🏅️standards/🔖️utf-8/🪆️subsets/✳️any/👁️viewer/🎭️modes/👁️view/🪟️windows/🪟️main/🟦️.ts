/** 📄️ Txt viewer — `main` window: typed twin of `🦀️.rs`'s `TextWindowKit` view-model.
 * `readOnly` is always `true` here. */

export interface TxtMainViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: string | null;
  readOnly: true;
}

export const TXT_MAIN_WINDOW_KIND_ID = "framework.window.text" as const;
export const TXT_MAIN_BODY_KEY = "framework.window.text" as const;
