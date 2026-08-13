//! 🧩️ `delete-selection` command.

use crate::apps::puzzle5d::config::Puzzle5dSelection;
use crate::apps::puzzle5d::{add_palette_part, next_part_id, remove_grips, remove_parts, Puzzle5dActionCtx, Puzzle5dPart};
use semio_framework_plugin::SelectionSet;
use serde_json::{json, Value};

/// 🗑️ Removes every selected part (and its fasteners), grip and fastener, then clears the selection.
pub fn delete_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let selection = ctx.scene.runtime.selection.clone();
    remove_parts(&mut ctx.scene.document, selection.part_ids.as_slice());
    remove_grips(&mut ctx.scene.document, selection.grip_ids.as_slice());
    ctx.scene.document.fasteners.retain(|fastener| !selection.fastener_ids.contains(&fastener.id));
    ctx.scene.runtime.selection = Puzzle5dSelection::default();
}
