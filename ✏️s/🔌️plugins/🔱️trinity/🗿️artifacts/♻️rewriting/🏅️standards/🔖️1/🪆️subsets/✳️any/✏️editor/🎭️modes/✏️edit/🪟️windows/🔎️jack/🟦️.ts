/** 🔎️ Trinity Rewriting editor — Jack window: typed twin of `build_text_editor_scene`'s boundary,
 * showing the compiled jack query for this rewrite rule. Read-only (no `on_change`/`commit`
 * wiring on the Rust side); `occurrencesJson` deliberately absent — the trinity-wide gap this
 * window's own doc comment documents (`ArtifactApp::render` has no `InteractionView` yet). */

export interface TrinityRewritingEditJackViewModel {
  windowKindId: "trinity-rewriting-edit-jack";
  bodyKey: "trinity.rewriting.edit.jack";
  surfaceId: "trinity.rewriting.edit.jack";
  query: string;
  tokensJson?: string;
  readOnly: true;
}

export const TRINITY_REWRITING_EDIT_JACK_WINDOW_KIND_ID = "trinity-rewriting-edit-jack" as const;
export const TRINITY_REWRITING_EDIT_JACK_BODY_KEY = "trinity.rewriting.edit.jack" as const;
export const TRINITY_REWRITING_EDIT_JACK_SURFACE_ID = "trinity.rewriting.edit.jack" as const;
