//! 🔺️ Diff fragment yielded by `ChangeBlockLocked`. Error `target-missing` when absent, Warning
//! `no-op` when already at that locked state.
use super::ChangeBlockLocked;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockLocked, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if crate::schema::block_locked(block) == payload.new_locked {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" locked is already {}.", payload.id, payload.new_locked));
    }
    let mut updated = block.clone();
    match &mut updated {
        crate::NoteBlockNode::Text { locked, .. }
        | crate::NoteBlockNode::Image { locked, .. }
        | crate::NoteBlockNode::Table { locked, .. }
        | crate::NoteBlockNode::Math { locked, .. }
        | crate::NoteBlockNode::Ink { locked, .. }
        | crate::NoteBlockNode::Group { locked, .. } => *locked = payload.new_locked,
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, &updated))
}
//#endregion 🔖️Diff
