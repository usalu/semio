//! 🤝️ `engagement-input` command.

use crate::apps::puzzle3d::modes::edit::windows::main::utilities;
use crate::apps::puzzle3d::{apply_puzzle3d_fill_count, apply_puzzle3d_focus_selection, drive_precompute, puzzle3d_clear_selection, Puzzle3dActionCtx, PUZZLE3D_DEFAULT_UTILITY, PUZZLE3D_FILL_COUNT_MAX};
use semio_framework_plugin::strip_engagement_prefix;
use serde_json::Value;

pub fn engagement_input(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.engagement_input = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").to_string();
}
