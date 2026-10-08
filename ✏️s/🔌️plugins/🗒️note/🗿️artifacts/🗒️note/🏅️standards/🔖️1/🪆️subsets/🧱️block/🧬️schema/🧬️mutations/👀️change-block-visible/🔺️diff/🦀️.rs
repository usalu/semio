//! 🔺️ Diff fragment yielded by `ChangeBlockVisible`. Error `target-missing` when absent, Warning
//! `no-op` when already at that visibility.
use super::ChangeBlockVisible;
use crate::schema::diff::NoteBlockPatch;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockVisible, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if crate::schema::block_visible(block) == payload.new_visible {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" visible is already {}.", payload.id, payload.new_visible));
    }
    protocol::MutationOutcome::new(NoteDiff::block_patches([(payload.id.clone(), NoteBlockPatch { visible: Some(payload.new_visible), ..Default::default() })]))
}
//#endregion 🔖️Diff
