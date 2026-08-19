//! 🎥️ `focus-selection` command.

use crate::editor::puzzle3d::{apply_puzzle3d_focus_selection, Puzzle3dActionCtx};

pub async fn focus_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    let object_ids = ctx.selected_object_ids();
    apply_puzzle3d_focus_selection(ctx.scene, &object_ids);
}
