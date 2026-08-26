//! 📝️ Trinity Jack app — Jack Query editor window (text editor render with tokens/diagnostics/completions).

use crate::artifacts::jack::JackSnapshot;
use crate::core;
use crate::editor::jack::config::JackConfig;
use semio_framework_plugin::{build_text_editor_scene, text_identifier_occurrences_json, TextEditorScene, UiNode};
use serde_json::json;

pub(crate) fn render(surface_id: &str, controller_id: &str, fixture: &JackSnapshot, cfg: &JackConfig) -> UiNode {
    let query = &cfg.jack_query;
    let graph = crate::editor::jack::graph_from_fixture_or_default(fixture);
    let cursor = cfg.editor_selection.as_ref().map_or(0, |selection| selection.end as usize);
    let selection_json = cfg.editor_selection.as_ref().map(|selection| json!({ "start": selection.start, "end": selection.end }).to_string());
    build_text_editor_scene(
        surface_id,
        controller_id,
        TextEditorScene {
            selection_json,
            tokens_json: serde_json::to_string(&core::semantic_tokens(query)).ok(),
            diagnostics_json: serde_json::to_string(&core::lint(&graph, query)).ok(),
            completions_json: serde_json::to_string(&core::complete(&graph, query, cursor)).ok(),
            occurrences_json: text_identifier_occurrences_json(query, cursor),
            ..TextEditorScene::base(query.clone(), Some("jack".into()), None)
        },
    )
}
