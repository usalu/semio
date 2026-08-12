//! 🔺️ Diff fragment yielded by `MoveBlockToContainer`.
use super::mutation::MoveBlockToContainer;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveBlockToContainer, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    let mut delta = crate::artifacts::note::schema::diff::NoteBlocksDelta::default();
    delta.removed.push(payload.id.clone());
    delta.added.push(crate::artifacts::note::schema::diff::NoteAddedBlockEntry { parent_id: payload.new_parent_id.clone(), index: Some(payload.index), block: block.clone() });
    NoteDiff { blocks: Some(delta), ..Default::default() }
}
//#endregion 🔖️Diff
