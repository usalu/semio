//! 🤝️ `engagement-repeat-last` command.

use crate::editor::puzzle2d::commands::set_fill_count;
use crate::editor::puzzle2d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle2d::Puzzle2dActionCtx;

/// 🔂️ Repeating `fill` asks for one more node: the count is shared configuration, so a live fill run
/// is reconfigured by the framework driver and continues its sequence — puzzle3d's exact semantics.
pub fn engagement_repeat_last(ctx: &mut Puzzle2dActionCtx<'_>) {
    if ctx.scene.active_utility == fill_tool::TOOL_ID {
        let count = ctx.scene.runtime.fill_count.saturating_add(1);
        ctx.effects.push(set_fill_count::request(count));
    }
}
