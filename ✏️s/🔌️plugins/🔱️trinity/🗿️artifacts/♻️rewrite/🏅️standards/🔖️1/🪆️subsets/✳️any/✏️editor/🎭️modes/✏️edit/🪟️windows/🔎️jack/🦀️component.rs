//! 🔎️ Trinity Rewrite app — Jack window (read-only text editor showing the compiled jack query for
//! this rule).
//!
//! 🕹️ ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`: variable hover/select occurrence
//! highlighting used to read `RewriteConfig.active_hover_var`/`active_select_var`, both deleted —
//! the "graph" domain's node↔variable cross-highlight is now framework-owned, but `ArtifactApp::render`
//! was NOT given an `InteractionView` (see `editor::jack::panels::inspection`'s doc comment for the
//! same trinity-wide gap), so this static scene can no longer compute which variable to highlight.
//! `occurrences_json` is left unset until a future wave threads interaction state through `render`.

use crate::editor::rewrite::config::RewriteConfig;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{build_text_editor_scene, TextEditorScene, UiNode};

pub(crate) async fn render(state: &RewriteSnapshot, _cfg: &RewriteConfig) -> UiNode {
    let query = crate::editor::rewrite::compiled_jack_query(state);
    build_text_editor_scene(
        crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_JACK,
        crate::editor::rewrite::TRINITY_REWRITE_PLAY_CONTROLLER_ID,
        TextEditorScene { tokens_json: serde_json::to_string(&crate::language_service::semantic_tokens(&query)).ok(), ..TextEditorScene::base(query, Some("jack".into()), None) },
    )
}
