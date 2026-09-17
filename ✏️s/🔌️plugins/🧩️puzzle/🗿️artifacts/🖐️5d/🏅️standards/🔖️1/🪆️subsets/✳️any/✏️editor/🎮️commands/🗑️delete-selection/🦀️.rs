//! 🗑️ `delete-selection` command.

use crate::editor::puzzle5d::{remove_grips, remove_parts, Puzzle5dActionCtx};

/// 🗑️ Removes every selected part (and its fasteners), grip and fastener.
///
/// 🔒️ A LOCKED part is never removed, and a selection whose every part is locked refuses with ONE
/// visible notice — deleting nothing silently is indistinguishable from a dead Delete key.
pub fn delete_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let selected_parts = ctx.selected_part_ids();
    if ctx.refuse_when_locked(&selected_parts) {
        return;
    }
    let part_ids: Vec<String> = selected_parts.into_iter().filter(|id| !ctx.scene.document.parts.iter().any(|part| &part.id == id && part.part_2d.locked.unwrap_or(false))).collect();
    let grip_ids = ctx.selected_grip_ids();
    let fastener_ids = ctx.selected_fastener_ids();
    remove_parts(&mut ctx.scene.document, &part_ids);
    remove_grips(&mut ctx.scene.document, &grip_ids);
    ctx.scene.document.fasteners.retain(|fastener| !fastener_ids.contains(&fastener.id));
}
