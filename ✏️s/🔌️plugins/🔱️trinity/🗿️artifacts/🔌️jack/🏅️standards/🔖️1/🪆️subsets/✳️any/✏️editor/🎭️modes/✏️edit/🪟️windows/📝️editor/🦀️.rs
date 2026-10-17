//! 📝️ Trinity Jack app — Jack Query editor window (text editor render with tokens/diagnostics/completions).

use crate::standards::v1::subsets::any::schema::language_service;
use crate::editor::jack::transient::JackEditorSelection;
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_plugin::{scene_surface, text_identifier_occurrences_json, BuiltNode, TextEditorScene, UiAssemblyResult};
use semio_framework_ui_contract::SurfaceKind;

pub fn render(surface_id: &str, _controller_id: &str, snapshot: &JackSnapshot, graph: &crate::standards::v1::subsets::any::schema::memory::Graph, selection: Option<&JackEditorSelection>) -> UiAssemblyResult<BuiltNode> {
    let query = &snapshot.query;
    let cursor = selection.map_or(0, |selection| selection.end as usize);
    let selection_json = selection.map(|selection| semio_framework_pack_json::json!({ "start": selection.start, "end": selection.end }).to_string());
    scene_surface(
        surface_id,
        SurfaceKind::TextEditor,
        &TextEditorScene {
            selection_json,
            tokens_json: Some(semio_framework_pack_json::to_json_string(&language_service::semantic_tokens(query))),
            diagnostics_json: Some(semio_framework_pack_json::to_json_string(&language_service::lint(graph, query))),
            completions_json: Some(semio_framework_pack_json::to_json_string(&language_service::complete(graph, query, cursor))),
            occurrences_json: text_identifier_occurrences_json(query, cursor),
            ..TextEditorScene::base(query.clone(), Some("jack".into()), None)
        },
    )
}
