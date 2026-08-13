//! 🤝️ `engagement-abort` command.

use crate::apps::puzzle3d::modes::edit::windows::main::utilities;
use crate::apps::puzzle3d::{apply_puzzle3d_fill_count, apply_puzzle3d_focus_selection, drive_precompute, puzzle3d_clear_selection, Puzzle3dActionCtx, PUZZLE3D_DEFAULT_UTILITY, PUZZLE3D_FILL_COUNT_MAX};
use semio_framework_plugin::strip_engagement_prefix;
use serde_json::Value;

pub fn engagement_abort(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.engagement_input = String::new();
    ctx.scene.runtime.brush_candidate_index = 0;
    ctx.scene.active_utility = PUZZLE3D_DEFAULT_UTILITY.into();
}
