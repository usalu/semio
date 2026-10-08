//! 🔺️ Diff fragment yielded by `DragBlocks`. Error `target-missing` when none of the addressed
//! blocks exist, Warning `partial` when some do not.
use super::DragBlocks;
use crate::schema::diff::NoteBlockPatch;
use crate::{NoteDiff, NoteSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DragBlocks, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let (moved, missing) = super::dragged_positions(payload, base);
    if moved.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} requested block(s) exist.", payload.ids.len()), payload.ids.clone());
    }
    let patches = moved.into_iter().map(|(id, (x, y))| (id, NoteBlockPatch { x: Some(x + payload.dx), y: Some(y + payload.dy), ..Default::default() }));
    let outcome = protocol::MutationOutcome::new(NoteDiff::block_patches(patches));
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} requested block(s) did not exist and were skipped.", missing.len(), payload.ids.len())).at(missing)])
    }
}
//#endregion 🔖️Diff
