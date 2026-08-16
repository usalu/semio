//! 🎥️ `focus-selection` command.

use crate::editor::puzzle3d::config::puzzle3d_camera_distance;
use crate::editor::puzzle3d::{apply_puzzle3d_focus_selection, Puzzle3dActionCtx};
use semio_framework_plugin::{apply_world3d_projection_action, world3d_projection_action_moves_pose, world3d_projection_pose};
use serde_json::Value;

pub fn focus_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let object_ids = ctx.selected_object_ids();
    apply_puzzle3d_focus_selection(ctx.scene, &object_ids);
}
