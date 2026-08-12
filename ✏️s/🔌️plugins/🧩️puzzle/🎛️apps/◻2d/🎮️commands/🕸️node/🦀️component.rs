//! 🕸️ Puzzle 2d play app commands — the node/graph vocabulary: adding nodes, patching inspector
//! fields across the selection, re-snapping edge handles and re-running the force layout.

use crate::apps::puzzle2d::{add_node_to_fixture, patch_inspector_nodes, Puzzle2dActionCtx};
use serde_json::Value;

pub fn add_node(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
    add_node_to_fixture(&mut ctx.scene.fixture, kind, args);
}

pub fn patch_inspector(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_else(|| ctx.scene.runtime.selected_ids.clone());
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    if !field.is_empty() {
        patch_inspector_nodes(&mut ctx.scene.fixture, &ids, field, value, delta);
    }
}

/// 🧲️ Re-snaps every edge's handle angle onto its node-center line.
pub fn redraw_handles(ctx: &mut Puzzle2dActionCtx<'_>) {
    if let Ok(next) = crate::apps::puzzle2d::engine::apply_edge_handle_snap_to_fixture_v1_json(&ctx.scene.fixture.to_string()) {
        if let Ok(parsed) = serde_json::from_str(&next) {
            ctx.scene.fixture = parsed;
        }
    }
}

/// 🌀️ Re-runs the force-graph layout over the whole fixture — shared by `forceLayout` and `reorganize`.
pub fn force_layout(ctx: &mut Puzzle2dActionCtx<'_>) {
    let Ok(layout_json) = crate::apps::puzzle2d::engine::apply_force_graph_layout_to_fixture_v1_json(&ctx.scene.fixture.to_string(), r#"{"mode":"force-graph"}"#) else {
        return;
    };
    if let Ok(parsed) = serde_json::from_str(&layout_json) {
        ctx.scene.fixture = parsed;
    }
}
