//! 🔎️ Trinity Rewriting app — Jack window (read-only text editor showing the compiled jack query for
//! this rule).
//!
//! 🕹️ ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`: variable hover/select occurrence
//! highlighting used to read `RewritingConfig.active_hover_var`/`active_select_var`, both deleted —
//! the "graph" domain's node↔variable cross-highlight is now framework-owned, but `ArtifactApp::render`
//! was NOT given an `InteractionView` (see `editor::jack::panels::inspection`'s doc comment for the
//! same trinity-wide gap), so this static scene can no longer compute which variable to highlight.
//! `occurrences_json` is left unset until a future wave threads interaction state through `render`.

use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfig;
use semio_framework_plugin::{scene_surface, BuiltNode, TextEditorScene, UiAssemblyResult};
use semio_framework_ui_contract::SurfaceKind;

pub(crate) fn render(state: &RewritingSnapshot, _cfg: &RewritingConfig) -> UiAssemblyResult<BuiltNode> {
    let query = crate::editor::rewriting::compiled_jack_query(state);
    scene_surface(
        crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_JACK,
        SurfaceKind::TextEditor,
        &TextEditorScene { tokens_json: Some(pack::to_json_string(&crate::language_service::semantic_tokens(&query))), ..TextEditorScene::base(query, Some("jack".into()), None) },
    )
}
