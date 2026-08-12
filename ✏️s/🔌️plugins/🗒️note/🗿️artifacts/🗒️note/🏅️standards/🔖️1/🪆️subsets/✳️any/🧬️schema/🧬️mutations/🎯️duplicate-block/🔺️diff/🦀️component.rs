//! 🔺️ Diff fragment yielded by `DuplicateBlock`.
use super::mutation::DuplicateBlock;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_added_diff;

//#region 🔖️Diff
pub fn diff(payload: &DuplicateBlock, base: &NoteSnapshot) -> NoteDiff {
    match crate::artifacts::note::schema::find_block_location(&base.blocks, &payload.source_id) {
        Some((parent_id, index)) => note_block_added_diff(parent_id, Some(index + 1), (*payload.block).clone()),
        None => note_block_added_diff(None, None, (*payload.block).clone()),
    }
}
//#endregion 🔖️Diff
