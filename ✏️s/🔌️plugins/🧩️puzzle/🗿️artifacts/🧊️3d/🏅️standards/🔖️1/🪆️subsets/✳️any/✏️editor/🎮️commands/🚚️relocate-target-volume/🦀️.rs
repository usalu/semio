//! 📦️ `relocate-target-volume` command.

use crate::editor::puzzle3d::modes::edit::windows::main::utilities::transform::Puzzle3dSelectionRecord;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use semio_framework_pack_json::Value;

/// 🚚️ One target-volume gumball relocate `{volumeId, mode, before, after}` as ONE transform-tool transaction
/// yielding the relative `drag-`, `rotate-` or `scale-selection` from `before` to `after`.
pub fn relocate_target_volume(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(record) = Puzzle3dSelectionRecord::from_pose_delta(args) {
        ctx.commit_selection("relocateTargetVolume", vec![record]);
    }
}
