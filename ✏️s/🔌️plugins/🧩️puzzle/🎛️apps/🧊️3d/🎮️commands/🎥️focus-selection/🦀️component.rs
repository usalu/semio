//! 🎥️ `focus-selection` command.

use crate::apps::puzzle3d::config::puzzle3d_camera_distance;
use crate::apps::puzzle3d::{apply_puzzle3d_focus_selection, Puzzle3dActionCtx};
use semio_framework_plugin::{apply_world3d_projection_action, world3d_projection_action_moves_pose, world3d_projection_pose};
use serde_json::Value;

pub fn focus_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    apply_puzzle3d_focus_selection(ctx.scene);
}
