//! 🔺️ Diff fragment yielded by `RenameBlock`. Error `target-missing` when absent, Warning `no-op`
//! when already at that name.
use super::RenameBlock;
use crate::schema::diff::NoteBlockPatch;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if crate::schema::block_name(block) == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" already has name \"{}\".", payload.id, payload.new_name));
    }
    protocol::MutationOutcome::new(NoteDiff::block_patches([(payload.id.clone(), NoteBlockPatch { name: Some(payload.new_name.clone()), ..Default::default() })]))
}
//#endregion 🔖️Diff
