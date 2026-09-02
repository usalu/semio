//! 📝️ Trinity Jack app — Jack Query editor window (text editor render with tokens/diagnostics/completions).

use crate::artifacts::jack::JackSnapshot;
use crate::core;
use crate::editor::jack::config::JackConfig;
use semio_framework_plugin::{scene_surface, text_identifier_occurrences_json, BuiltNode, TextEditorScene, UiAssemblyResult};
use semio_framework_ui_contract::SurfaceKind;

pub(crate) fn render(surface_id: &str, _controller_id: &str, fixture: &JackSnapshot, cfg: &JackConfig) -> UiAssemblyResult<BuiltNode> {
    let query = &cfg.jack_query;
    let graph = crate::editor::jack::graph_from_fixture_or_default(fixture);
    let cursor = cfg.editor_selection.as_ref().map_or(0, |selection| selection.end as usize);
    let selection_json = cfg.editor_selection.as_ref().map(|selection| json!({ "start": selection.start, "end": selection.end }).to_string());
    scene_surface(
        surface_id,
        SurfaceKind::TextEditor,
        &TextEditorScene {
            selection_json,
            tokens_json: Some(pack::to_json_string(&core::semantic_tokens(query))),
            diagnostics_json: Some(pack::to_json_string(&core::lint(&graph, query))),
            completions_json: Some(pack::to_json_string(&core::complete(&graph, query, cursor))),
            occurrences_json: text_identifier_occurrences_json(query, cursor),
            ..TextEditorScene::base(query.clone(), Some("jack".into()), None)
        },
    )
}
