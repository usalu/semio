//! 🔺️ Diff fragment yielded by `RenameBlock`.
use super::mutation::RenameBlock;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &RenameBlock, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    let mut updated = block.clone();
    match &mut updated { crate::artifacts::note::NoteBlockNode::Text { name, .. } | crate::artifacts::note::NoteBlockNode::Image { name, .. } | crate::artifacts::note::NoteBlockNode::Table { name, .. } | crate::artifacts::note::NoteBlockNode::Math { name, .. } | crate::artifacts::note::NoteBlockNode::Ink { name, .. } | crate::artifacts::note::NoteBlockNode::Group { name, .. } => *name = payload.new_name.clone(), }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
