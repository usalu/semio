//! 🤝️ `engagement-repeat-last` command.

use crate::set_fill_count;
use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx;

/// 🔂️ Repeating `fill` asks for one more object: the count is shared configuration, so a live fill run
/// is reconfigured by the framework driver and continues its sequence.
pub fn engagement_repeat_last(ctx: &mut Puzzle3dActionCtx<'_>) {
    if ctx.scene.active_utility == semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID {
        let count = ctx.scene.runtime.fill_count.saturating_add(1);
        ctx.effects.push(set_fill_count::request(count));
    }
}
