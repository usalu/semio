//! 🎥️ Puzzle 5d play app commands — camera pose. Session-only view state (`ActionKind::View`): these
//! arms write the runtime only and never touch the document, so they never emit a VCS operation.

use crate::apps::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dCamera3d};
use crate::apps::puzzle5d::modes::edit::windows::board2d;
use crate::apps::puzzle5d::{gumball_target_world, Puzzle5dActionCtx};
use serde_json::Value;

/// 📷️ The surface-agnostic setter: the flat camera wins when the surface is the board (or the payload
/// carries no `position`), otherwise the volume camera.
pub fn set_camera(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(camera) = args.and_then(|value| value.get("camera")) else {
        return;
    };
    let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
    if surface_id == board2d::SURFACE_ID || camera.get("position").is_none() {
        if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera2d>(camera.clone()) {
            ctx.scene.runtime.camera2d = parsed;
        }
    } else if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera3d>(camera.clone()) {
        ctx.scene.runtime.camera3d = parsed;
    }
}

pub fn set_camera_2d(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
            ctx.scene.runtime.camera2d = parsed;
        }
    }
}

pub fn set_camera_3d(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
            ctx.scene.runtime.camera3d = parsed;
        }
    }
}

/// 🔍️ `zoomToSelection`/`focusSelection`: recenters both cameras on the selection, preserving the
/// volume camera's orbit offset. Aborts (emitting nothing at all) when nothing is selected — the
/// pre-migration `return Emit::default()`.
pub fn zoom_to_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let Some(target) = gumball_target_world(ctx.scene) else {
        ctx.abort = true;
        return;
    };
    let camera = &mut ctx.scene.runtime.camera3d;
    let offset = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
    camera.target = target;
    camera.position = [target[0] + offset[0], target[1] + offset[1], target[2] + offset[2]];
    let selected_2d: Vec<(f64, f64)> = ctx.scene.document.parts.iter().filter(|part| ctx.scene.runtime.selection.part_ids.contains(&part.id)).map(|part| (part.part_2d.x, part.part_2d.y)).collect();
    if !selected_2d.is_empty() {
        ctx.scene.runtime.camera2d.x = selected_2d.iter().map(|(x, _)| x).sum::<f64>() / selected_2d.len() as f64;
        ctx.scene.runtime.camera2d.y = selected_2d.iter().map(|(_, y)| y).sum::<f64>() / selected_2d.len() as f64;
    }
}
