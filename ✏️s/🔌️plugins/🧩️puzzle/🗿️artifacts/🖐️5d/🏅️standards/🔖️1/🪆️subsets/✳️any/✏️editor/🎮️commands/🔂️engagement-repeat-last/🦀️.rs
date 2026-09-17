//! 🔂️ `engagement-repeat-last` command.

use crate::editor::puzzle5d::commands::set_fill_count;
use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle5d::Puzzle5dActionCtx;

/// 🔂️ Repeating `fill` asks for one more part: the count is shared configuration, so a live fill run is
/// reconfigured by the framework driver and CONTINUES its deterministic sequence (`Resume`) instead of
/// restarting it. With any other gesture armed there is nothing to repeat.
pub fn engagement_repeat_last(ctx: &mut Puzzle5dActionCtx<'_>) {
    if ctx.scene.active_utility != fill_tool::TOOL_ID {
        ctx.abort = true;
        return;
    }
    let count = ctx.scene.runtime.fill_count.saturating_add(1);
    ctx.effects.push(set_fill_count::request(count));
}
