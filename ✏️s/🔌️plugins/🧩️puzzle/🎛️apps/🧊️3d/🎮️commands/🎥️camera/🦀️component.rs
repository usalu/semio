//! 🎥️ Puzzle 3d play app commands — the viewport camera: an absolute pose push from the renderer, a
//! projection/orientation change from the projection measure, and the keybinding/engagement-driven
//! zoom-to-selection framing. All three are session-only per-window state (`ActionKind::View`) and
//! must never touch the shared document.

use crate::apps::puzzle3d::config::puzzle3d_camera_distance;
use crate::apps::puzzle3d::{apply_puzzle3d_focus_selection, Puzzle3dActionCtx};
use semio_framework_plugin::{apply_world3d_projection_action, world3d_projection_action_moves_pose, world3d_projection_pose};
use serde_json::Value;

pub fn set_camera(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
            ctx.scene.runtime.camera = parsed;
        }
    }
}

/// 🧭️ `setProjection`/`setProjectionParam` share one arm — a projection change that also moves the
/// camera pose re-derives position/up from the new orientation around the unchanged target.
pub fn set_projection(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    let moves_pose = world3d_projection_action_moves_pose(action, args);
    apply_world3d_projection_action(&mut ctx.scene.runtime.camera.projection, action, args);
    if moves_pose {
        let distance = puzzle3d_camera_distance(&ctx.scene.runtime.camera);
        let (position, up) = world3d_projection_pose(&ctx.scene.runtime.camera.projection, ctx.scene.runtime.camera.target, distance);
        ctx.scene.runtime.camera.position = position;
        ctx.scene.runtime.camera.up = Some(up);
    }
}

pub fn focus_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    apply_puzzle3d_focus_selection(ctx.scene);
}
