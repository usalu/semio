//! ⚙️ `set-brush-placement-contact-tolerance` command.

use {semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_absolute_or_delta, semio_s_artifact_puzzle_3d_editor_session::sync_precompute_session, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx};
use semio_framework_pack_json::Value;

/// 🖌️ The collision budget every brush/fill placement is tested against — re-syncs the precompute
/// session immediately so already-cached candidates are recomputed under the new budget.
pub fn set_brush_placement_contact_tolerance(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.contact_tolerance) {
        ctx.scene.runtime.contact_tolerance = value.clamp(0.0, 1.0);
        sync_precompute_session(&mut ctx.session.precompute.borrow_mut(), ctx.scene);
    }
}
