//! 🎥️ `focus-selection` command.

use crate::editor::puzzle2d::{fixture_nodes, set_runtime_camera, Puzzle2dActionCtx};
use serde_json::json;

/// 🎯️ Centres the camera on the selection's bounding box (session state only — never the fixture).
pub async fn focus_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    let selected_ids = ctx.selected_ids();
    if selected_ids.is_empty() {
        return;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for node in fixture_nodes(&ctx.scene.fixture) {
        let Some(id) = node.get("id").and_then(|value| value.as_str()) else {
            continue;
        };
        if !selected_ids.iter().any(|selected| selected == id) {
            continue;
        }
        let x = node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let y = node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let radius = node.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0);
        min_x = min_x.min(x - radius);
        min_y = min_y.min(y - radius);
        max_x = max_x.max(x + radius);
        max_y = max_y.max(y + radius);
    }
    if !min_x.is_finite() {
        return;
    }
    let camera = json!({
        "x": (min_x + max_x) * 0.5,
        "y": (min_y + max_y) * 0.5,
        "zoom": 1.0,
    });
    set_runtime_camera(&mut ctx.scene.runtime, &camera);
    ctx.host.borrow_mut().set_camera(ctx.scene.runtime.camera_x, ctx.scene.runtime.camera_y, ctx.scene.runtime.camera_zoom);
}
