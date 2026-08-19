//! 🗂️ `duplicate-selection` command.

use crate::editor::puzzle2d::{duplicate_selection_in_fixture, Puzzle2dActionCtx};

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: no longer re-selects the
/// new duplicates afterward — see puzzle3d's `duplicate-selection` doc comment for the identical
/// limitation (selection is framework-owned and `handle` has no channel to write it).
pub async fn duplicate_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    let selected_ids = ctx.selected_ids();
    duplicate_selection_in_fixture(&mut ctx.scene.fixture, &selected_ids);
}
