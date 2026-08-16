//! 🎥️ `zoom-to-selection` command.

use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dCamera3d};
use crate::editor::puzzle5d::modes::edit::windows::board2d;
use crate::editor::puzzle5d::{gumball_target_world, Puzzle5dActionCtx};
use serde_json::Value;

/// 🔍️ `zoomToSelection`/`focusSelection`: recenters both cameras on the selection, preserving the
/// volume camera's orbit offset. Aborts (emitting nothing at all) when nothing is selected — the
/// pre-migration `return Emit::default()`.
pub fn zoom_to_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let part_ids = ctx.selected_part_ids();
    let Some(target) = gumball_target_world(ctx.scene, &part_ids) else {
        ctx.abort = true;
        return;
    };
    let camera = &mut ctx.scene.runtime.camera3d;
    let offset = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
    camera.target = target;
    camera.position = [target[0] + offset[0], target[1] + offset[1], target[2] + offset[2]];
    let selected_2d: Vec<(f64, f64)> = ctx.scene.document.parts.iter().filter(|part| part_ids.contains(&part.id)).map(|part| (part.part_2d.x, part.part_2d.y)).collect();
    if !selected_2d.is_empty() {
        ctx.scene.runtime.camera2d.x = selected_2d.iter().map(|(x, _)| x).sum::<f64>() / selected_2d.len() as f64;
        ctx.scene.runtime.camera2d.y = selected_2d.iter().map(|(_, y)| y).sum::<f64>() / selected_2d.len() as f64;
    }
}
