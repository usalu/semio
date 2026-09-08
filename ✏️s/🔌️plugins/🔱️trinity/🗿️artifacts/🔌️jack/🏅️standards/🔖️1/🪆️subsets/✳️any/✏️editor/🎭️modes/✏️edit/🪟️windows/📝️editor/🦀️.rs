//! 📝️ Trinity Jack app — Jack Query editor window (text editor render with tokens/diagnostics/completions).

use crate::JackSnapshot;
use crate::core;
use crate::editor::jack::config::JackConfig;
use crate::editor::jack::transient::JackEditorSelection;
use semio_framework_plugin::{scene_surface, text_identifier_occurrences_json, BuiltNode, TextEditorScene, UiAssemblyResult};
use semio_framework_ui_contract::SurfaceKind;

pub(crate) fn render(surface_id: &str, _controller_id: &str, fixture: &JackSnapshot, cfg: &JackConfig, selection: Option<&JackEditorSelection>) -> UiAssemblyResult<BuiltNode> {
    let query = &cfg.jack_query;
    let graph = crate::editor::jack::graph_from_fixture_or_default(fixture);
    let cursor = selection.map_or(0, |selection| selection.end as usize);
    let selection_json = selection.map(|selection| pack::json!({ "start": selection.start, "end": selection.end }).to_string());
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
