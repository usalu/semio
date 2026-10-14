//! 🎥️ `focus-selection` command.

use {semio_s_artifact_puzzle_3d_editor_snapshot::apply_puzzle3d_focus_selection, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx};

pub fn focus_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let object_ids = ctx.selected_object_ids();
    apply_puzzle3d_focus_selection(ctx.scene, &object_ids);
}
