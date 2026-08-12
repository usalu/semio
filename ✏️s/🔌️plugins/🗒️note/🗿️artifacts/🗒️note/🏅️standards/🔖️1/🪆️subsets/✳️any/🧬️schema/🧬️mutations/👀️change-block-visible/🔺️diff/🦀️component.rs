//! 🔺️ Diff fragment yielded by `ChangeBlockVisible`.
use super::mutation::ChangeBlockVisible;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockVisible, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    let mut updated = block.clone();
    match &mut updated { crate::artifacts::note::NoteBlockNode::Text { visible, .. } | crate::artifacts::note::NoteBlockNode::Image { visible, .. } | crate::artifacts::note::NoteBlockNode::Table { visible, .. } | crate::artifacts::note::NoteBlockNode::Math { visible, .. } | crate::artifacts::note::NoteBlockNode::Ink { visible, .. } | crate::artifacts::note::NoteBlockNode::Group { visible, .. } => *visible = payload.new_visible, }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
