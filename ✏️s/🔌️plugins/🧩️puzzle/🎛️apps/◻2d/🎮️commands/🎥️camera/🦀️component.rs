//! 🎥️ Puzzle 2d play app commands — the camera vocabulary. The camera is session-only
//! `Puzzle2dConfig` state (`ActionKind::View`): neither arm here ever produces a document operation.

use crate::apps::puzzle2d::{fixture_nodes, puzzle2d_window_only_scope, set_runtime_camera, Puzzle2dActionCtx};
use serde_json::{json, Value};

pub fn set_camera(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(camera) = args.and_then(|value| value.get("camera")) else {
        return;
    };
    if let (Some(x), Some(y), Some(zoom)) = (camera.get("x").and_then(|value| value.as_f64()), camera.get("y").and_then(|value| value.as_f64()), camera.get("zoom").and_then(|value| value.as_f64())) {
        ctx.host.borrow_mut().set_camera(x, y, zoom);
    }
    set_runtime_camera(&mut ctx.scene.runtime, camera);
    *ctx.ui_scope = puzzle2d_window_only_scope();
}

/// 🎯️ Centres the camera on the selection's bounding box (session state only — never the fixture).
pub fn focus_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    if ctx.scene.runtime.selected_ids.is_empty() {
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
        if !ctx.scene.runtime.selected_ids.iter().any(|selected| selected == id) {
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
