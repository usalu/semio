//! 🌐️ Trinity Jack app — Nakagin Graph window (node-graph render + LOD control).

use crate::artifacts::jack::JackSnapshot;
use crate::editor::jack::config::JackConfig;
use semio_framework_plugin::{scene_surface, ActionDescriptor, BuiltNode, MeasureSelectItem, NodeGraphScene, NodeGraphViewport, UiAssemblyResult, WindowMeasure};
use semio_framework_ui_contract::SurfaceKind;

pub(crate) const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";

fn trinity_lod_tier_rows() -> Vec<pack::JsonValue> {
    pack::from_json_str(&crate::editor::rewriting::world::trinity_lod_scale_json()).unwrap_or_default()
}

pub(crate) fn trinity_lod_measure(window_id: &str, current_mode: &str, jack_action: impl Fn(&str, Option<pack::JsonValue>) -> ActionDescriptor) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    items.extend(trinity_lod_tier_rows().into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: format!("{window_id}-lod"), label: Some("LOD".into()), value: current_mode.into(), items, on_change: jack_action("setLodMode", Some(json!({ "windowId": window_id }))) }
}

pub(crate) fn trinity_lod_json_for_window(cfg: &JackConfig, window_id: &str) -> Option<String> {
    let mode = cfg.lod_mode_by_window.get(window_id).map_or(TRINITY_LOD_MODE_AUTOMATIC, String::as_str);
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        Some(json!({ "automatic": true }).to_string())
    } else {
        Some(json!({ "automatic": false, "forcedLabel": mode }).to_string())
    }
}

/// 🕹️ `selection`/`hover` are left unset: `ArtifactApp::render` has no `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` gained one — see `📌️panels/🔍️inspection`'s doc comment
/// for the same framework-side gap), and this static scene isn't a `UiNode::Tree` the wrapper's
/// `stamp_and_cache_interaction_ui` post-pass would stamp either. The live node-graph host reads
/// domain "ast"'s `DomainSelection`/`DomainHover` directly (`GraphHost::sync_interaction`), so the
/// interactive surface stays correct even though this snapshot doesn't carry it.
pub(crate) fn render(surface_id: &str, _controller_id: &str, window_id: &str, fixture: &JackSnapshot, cfg: &JackConfig) -> UiAssemblyResult<BuiltNode> {
    let (nodes, edges, _) = crate::editor::jack::fixture_to_workflow(fixture);
    let viewport = NodeGraphViewport { x: cfg.camera.x, y: cfg.camera.y, zoom: cfg.camera.zoom };
    scene_surface(surface_id, SurfaceKind::NodeGraph, &NodeGraphScene { lod_json: trinity_lod_json_for_window(cfg, window_id), ..NodeGraphScene::base(nodes, edges, viewport) })
}
