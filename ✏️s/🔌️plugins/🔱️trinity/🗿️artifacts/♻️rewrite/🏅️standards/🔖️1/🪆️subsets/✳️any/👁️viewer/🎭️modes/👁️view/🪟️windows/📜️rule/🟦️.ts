/** 📜️ Trinity Rewrite viewer — Rule window: typed twin of `🦀️.rs`'s view-model. Read-only
 * mirror of the `framework.window.text` scene payload `render()` produces — the LHS pattern, RHS
 * actions and live parameter bindings pretty-printed as one JSON document, no rule-applied After
 * computation, no rule-layout point positions. */

export interface TrinityRewriteViewRuleViewModel {
  windowKindId: "framework.window.text";
  bodyKey: "framework.window.text";
  text: string;
  language: "json";
  readOnly: true;
}

export const TRINITY_REWRITE_VIEW_RULE_WINDOW_KIND_ID = "framework.window.text" as const;
export const TRINITY_REWRITE_VIEW_RULE_BODY_KEY = "framework.window.text" as const;
