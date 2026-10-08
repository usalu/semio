//! 🔺️ Diff fragment yielded by `DuplicateBlocks`. Fatal `duplicate-id` when a new block's id
//! already exists, Error `target-missing` when none of the sources exist, Warning `partial` when
//! some sources do not.
use super::DuplicateBlocks;
use crate::schema::diff::NoteBlockRow;
use crate::{NoteDiff, NoteSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DuplicateBlocks, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let duplicate_ids: Vec<String> = payload.blocks.iter().map(|block| crate::schema::block_id(block).to_string()).filter(|id| crate::schema::find_block(&base.blocks, id).is_some()).collect();
    if !duplicate_ids.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Block id(s) already exist: {}.", duplicate_ids.join(", ")), duplicate_ids);
    }
    let (placed, missing_sources) = super::placements(payload, base);
    if placed.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} source block(s) exist.", payload.source_ids.len()), payload.source_ids.clone());
    }
    let outcome = protocol::MutationOutcome::new(NoteDiff::block_rows(placed.into_iter().map(|(parent_id, index, block)| NoteBlockRow::Add { parent_id, index, block }).collect()));
    if missing_sources.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} source block(s) did not exist and were skipped.", missing_sources.len(), payload.source_ids.len())).at(missing_sources)])
    }
}
//#endregion 🔖️Diff
