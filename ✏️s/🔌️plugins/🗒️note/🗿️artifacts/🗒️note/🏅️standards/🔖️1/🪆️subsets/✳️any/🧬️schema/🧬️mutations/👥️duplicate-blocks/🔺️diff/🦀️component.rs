//! 🔺️ Diff fragment yielded by `DuplicateBlocks`. Fatal `duplicate-id` when a new block's id
//! already exists, Error `target-missing` when none of the sources exist, Warning `partial` when
//! some sources do not.
use super::mutation::DuplicateBlocks;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DuplicateBlocks, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let mut delta = crate::artifacts::note::schema::diff::NoteBlocksDelta::default();
    let mut missing_sources = Vec::new();
    let mut duplicate_ids = Vec::new();
    for (source_id, block) in payload.source_ids.iter().zip(payload.blocks.iter()) {
        let new_id = crate::artifacts::note::schema::block_id(block);
        if crate::artifacts::note::schema::find_block(&base.blocks, new_id).is_some() {
            duplicate_ids.push(new_id.to_string());
            continue;
        }
        match crate::artifacts::note::schema::find_block_location(&base.blocks, source_id) {
            Some((parent_id, index)) => delta.added.push(crate::artifacts::note::schema::diff::NoteAddedBlockEntry { parent_id, index: Some(index + 1), block: block.clone() }),
            None => missing_sources.push(source_id.clone()),
        }
    }
    if !duplicate_ids.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Block id(s) already exist: {}.", duplicate_ids.join(", ")), duplicate_ids);
    }
    if delta.added.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} source block(s) exist.", payload.source_ids.len()), payload.source_ids.clone());
    }
    let outcome = protocol::MutationOutcome::new(NoteDiff { blocks: Some(delta), ..Default::default() });
    if missing_sources.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warn("mutation.partial", format!("{} of {} source block(s) did not exist and were skipped.", missing_sources.len(), payload.source_ids.len())).at(missing_sources)])
    }
}
//#endregion 🔖️Diff
