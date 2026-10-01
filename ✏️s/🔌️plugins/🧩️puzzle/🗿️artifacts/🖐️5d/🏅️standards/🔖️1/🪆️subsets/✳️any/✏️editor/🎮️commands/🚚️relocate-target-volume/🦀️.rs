//! 🚚️ `relocate-target-volume` command.

use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;
use semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main::utilities::transform::Puzzle3dSelectionRecord;

/// 🚚️ One target-volume gumball gesture `{volumeId, mode, before, after}` as ONE transform-tool transaction:
/// the RELATIVE motion from `before` to `after` — a `drag-`, `rotate-` or `scale-selection3d` leaf over the
/// volume, read off whatever base it replays on.
pub fn relocate_target_volume(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(record) = Puzzle3dSelectionRecord::from_pose_delta(args) {
        ctx.commit_selection("relocateTargetVolume", vec![Puzzle5dSelectionRecord::world(record)]);
    }
}
