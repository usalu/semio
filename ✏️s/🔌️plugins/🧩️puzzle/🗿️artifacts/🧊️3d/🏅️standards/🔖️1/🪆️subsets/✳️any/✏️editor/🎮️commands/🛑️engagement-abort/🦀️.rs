//! 🤝️ `engagement-abort` command.

use crate::editor::puzzle3d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle3d::{puzzle3d_fill_tool_active, Puzzle3dActionCtx, PUZZLE3D_DEFAULT_UTILITY};

pub fn engagement_abort(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.engagement_input = String::new();
    ctx.scene.runtime.brush_candidate_index = 0;
    if puzzle3d_fill_tool_active(ctx.config) || ctx.scene.active_utility == fill_tool::TOOL_ID {
        return;
    }
    ctx.scene.active_utility = PUZZLE3D_DEFAULT_UTILITY.into();
}
