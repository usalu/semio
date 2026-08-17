//! 🔺️ Diff fragment yielded by `DeleteBlocks`. Error `target-missing` when none of the addressed
//! blocks exist, Warning `partial` when some do not.
use super::mutation::DeleteBlocks;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_removed_diff;

//#region 🔖️Diff
pub fn diff(payload: &DeleteBlocks, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let existing: Vec<String> = payload.ids.iter().filter(|id| crate::artifacts::note::schema::find_block(&base.blocks, id).is_some()).cloned().collect();
    if existing.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} requested block(s) exist.", payload.ids.len()), payload.ids.clone());
    }
    let missing: Vec<String> = payload.ids.iter().filter(|id| !existing.contains(id)).cloned().collect();
    let outcome = protocol::MutationOutcome::new(note_block_removed_diff(existing));
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warn("mutation.partial", format!("{} of {} requested block(s) did not exist and were skipped.", missing.len(), payload.ids.len())).at(missing)])
    }
}
//#endregion 🔖️Diff
