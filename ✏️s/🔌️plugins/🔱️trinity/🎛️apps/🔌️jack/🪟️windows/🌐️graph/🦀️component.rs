//! 🌐️ Trinity Jack app — Nakagin Graph window (node-graph render + LOD control).

use crate::apps::jack::config::JackConfig;
use crate::artifacts::jack::GraphFixture;
use semio_framework_plugin::{build_node_graph_scene, ActionDescriptor, MeasureSelectItem, NodeGraphScene, NodeGraphViewport, UiNode, WindowMeasure};
use serde_json::json;

pub(crate) const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";

fn trinity_lod_tier_rows() -> Vec<serde_json::Value> {
    serde_json::from_str(&crate::apps::rewrite::world::trinity_lod_scale_json()).unwrap_or_default()
}

pub(crate) fn trinity_lod_measure(window_id: &str, current_mode: &str, jack_action: impl Fn(&str, Option<serde_json::Value>) -> ActionDescriptor) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    items.extend(trinity_lod_tier_rows().into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: format!("{window_id}-lod"), label: Some("LOD".into()), value: current_mode.into(), items, on_change: jack_action("setLodMode", Some(json!({ "windowId": window_id }))) }
}

pub(crate) fn trinity_lod_json_for_window(cfg: &JackConfig, window_id: &str) -> Option<String> {
    let mode = cfg.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        Some(json!({ "automatic": true }).to_string())
    } else {
        Some(json!({ "automatic": false, "forcedLabel": mode }).to_string())
    }
}

pub(crate) fn render(surface_id: &str, controller_id: &str, window_id: &str, fixture: &GraphFixture, cfg: &JackConfig) -> UiNode {
    let (nodes, edges, _) = crate::apps::jack::fixture_to_workflow(fixture);
    let viewport = NodeGraphViewport { x: cfg.camera.x, y: cfg.camera.y, zoom: cfg.camera.zoom };
    let selection = cfg.selected_node_ids.clone();
    build_node_graph_scene(surface_id, controller_id, NodeGraphScene { selection, lod_json: trinity_lod_json_for_window(cfg, window_id), ..NodeGraphScene::base(nodes, edges, viewport) })
}
