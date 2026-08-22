//! 🧩️ `delete-selection` command.

use crate::editor::puzzle5d::{remove_grips, remove_parts, Puzzle5dActionCtx};

/// 🗑️ Removes every selected part (and its fasteners), grip and fastener.
pub fn delete_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let part_ids = ctx.selected_part_ids();
    let grip_ids = ctx.selected_grip_ids();
    let fastener_ids = ctx.selected_fastener_ids();
    remove_parts(&mut ctx.scene.document, &part_ids);
    remove_grips(&mut ctx.scene.document, &grip_ids);
    ctx.scene.document.fasteners.retain(|fastener| !fastener_ids.contains(&fastener.id));
}
