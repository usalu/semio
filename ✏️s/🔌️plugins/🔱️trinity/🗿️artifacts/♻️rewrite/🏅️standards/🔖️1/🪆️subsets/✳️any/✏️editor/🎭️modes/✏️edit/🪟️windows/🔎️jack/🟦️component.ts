/** 🔎️ Trinity Rewrite editor — Jack window: typed twin of `build_text_editor_scene`'s boundary,
 * showing the compiled jack query for this rewrite rule. Read-only (no `on_change`/`commit`
 * wiring on the Rust side); `occurrencesJson` deliberately absent — the trinity-wide gap this
 * window's own doc comment documents (`ArtifactApp::render` has no `InteractionView` yet). */

export interface TrinityRewriteEditJackViewModel {
  windowKindId: "trinity-rewrite-edit-jack";
  bodyKey: "trinity.rewrite.edit.jack";
  surfaceId: "trinity.rewrite.edit.jack";
  query: string;
  tokensJson?: string;
  readOnly: true;
}

export const TRINITY_REWRITE_EDIT_JACK_WINDOW_KIND_ID = "trinity-rewrite-edit-jack" as const;
export const TRINITY_REWRITE_EDIT_JACK_BODY_KEY = "trinity.rewrite.edit.jack" as const;
export const TRINITY_REWRITE_EDIT_JACK_SURFACE_ID = "trinity.rewrite.edit.jack" as const;
