//! 🔺️ Diff fragment yielded by `ChangeBlockLocked`. Error `target-missing` when absent, Warning
//! `no-op` when already at that locked state.
use super::ChangeBlockLocked;
use crate::schema::diff::NoteBlockPatch;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockLocked, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if crate::schema::block_locked(block) == payload.new_locked {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" locked is already {}.", payload.id, payload.new_locked));
    }
    protocol::MutationOutcome::new(NoteDiff::block_patches([(payload.id.clone(), NoteBlockPatch { locked: Some(payload.new_locked), ..Default::default() })]))
}
//#endregion 🔖️Diff
