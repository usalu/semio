//! 🔺️ Diff fragment yielded by `ChangeBlockLocked`.
use super::mutation::ChangeBlockLocked;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockLocked, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    let mut updated = block.clone();
    match &mut updated { crate::artifacts::note::NoteBlockNode::Text { locked, .. } | crate::artifacts::note::NoteBlockNode::Image { locked, .. } | crate::artifacts::note::NoteBlockNode::Table { locked, .. } | crate::artifacts::note::NoteBlockNode::Math { locked, .. } | crate::artifacts::note::NoteBlockNode::Ink { locked, .. } | crate::artifacts::note::NoteBlockNode::Group { locked, .. } => *locked = payload.new_locked, }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
