//! 🌐️ Trinity Jack app — Nakagin Graph window (node-graph render + LOD control).

use crate::editor::jack::window_config::JackGraphWindowConfig;
use crate::JackSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use semio_framework_plugin::{scene_surface, ActionDescriptor, BuiltNode, MeasureSelectItem, NodeGraphScene, UiAssemblyResult, WindowMeasure};
use semio_framework_os_kernel::Viewport2d;
use semio_framework_ui_contract::SurfaceKind;

pub(crate) const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";

fn trinity_lod_tier_rows() -> Vec<semio_framework_pack_json::Value> {
    semio_framework_pack_json::from_json_str(&crate::editor::jack::lod::trinity_lod_scale_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default()
}

pub(crate) fn trinity_lod_measure(window_id: &str, current_mode: &str, jack_action: impl Fn(&str, Option<semio_framework_pack_json::Value>) -> ActionDescriptor) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    items.extend(trinity_lod_tier_rows().into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: format!("{window_id}-lod"), label: Some("LOD".into()), value: current_mode.into(), items, on_change: jack_action("setLodMode", None) }
}

pub(crate) fn trinity_lod_json_for_window(config: Option<&JackGraphWindowConfig>) -> String {
    let mode = config.map_or(TRINITY_LOD_MODE_AUTOMATIC, |config| config.lod_mode.as_str());
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        semio_framework_pack_json::json!({ "automatic": true }).to_string()
    } else {
        semio_framework_pack_json::json!({ "automatic": false, "forcedLabel": mode }).to_string()
    }
}

/// 🕹️ `selection`/`hover` are left unset: `ArtifactApp::render` has no `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` gained one — see `📌️panels/🔍️inspection`'s doc comment
/// for the same framework-side gap), and this static scene isn't a `UiNode::Tree` the wrapper's
/// `stamp_and_cache_interaction_ui` post-pass would stamp either. The live node-graph host reads
/// domain "ast"'s `DomainSelection`/`DomainHover` directly (`GraphHost::sync_interaction`), so the
/// interactive surface stays correct even though this snapshot doesn't carry it.
pub(crate) fn render(surface_id: &str, _controller_id: &str, snapshot: &JackSnapshot, content: &SemioGraphSnapshot, config: Option<&JackGraphWindowConfig>) -> UiAssemblyResult<BuiltNode> {
    let (nodes, edges, _) = crate::editor::jack::content_to_workflow(&snapshot.camera, content);
    let camera = config.and_then(|config| config.camera.as_ref()).unwrap_or(&snapshot.camera);
    let viewport = Viewport2d { x: camera.x, y: camera.y, zoom: camera.zoom };
    scene_surface(surface_id, SurfaceKind::NodeGraph, &NodeGraphScene { lod_json: Some(trinity_lod_json_for_window(config)), ..NodeGraphScene::base(nodes, edges, viewport) })
}
