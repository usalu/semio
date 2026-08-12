//! 🔺️ Diff fragment yielded by `DuplicateBlocks`.
use super::mutation::DuplicateBlocks;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DuplicateBlocks, base: &NoteSnapshot) -> NoteDiff {
    let mut delta = crate::artifacts::note::schema::diff::NoteBlocksDelta::default();
    for (source_id, block) in payload.source_ids.iter().zip(payload.blocks.iter()) {
        let (parent_id, index) = crate::artifacts::note::schema::find_block_location(&base.blocks, source_id).map(|(p, i)| (p, Some(i + 1))).unwrap_or((None, None));
        delta.added.push(crate::artifacts::note::schema::diff::NoteAddedBlockEntry { parent_id, index, block: block.clone() });
    }
    NoteDiff { blocks: Some(delta), ..Default::default() }
}
//#endregion 🔖️Diff
