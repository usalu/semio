//! 🔺️ Diff fragment yielded by `ChangeBlockLocked`. Error `target-missing` when absent, Warning
//! `no-op` when already at that locked state.
use super::mutation::ChangeBlockLocked;
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeBlockLocked, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if crate::artifacts::note::schema::block_locked(block) == payload.new_locked {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" locked is already {}.", payload.id, payload.new_locked));
    }
    let mut updated = block.clone();
    match &mut updated {
        crate::artifacts::note::NoteBlockNode::Text { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Image { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Table { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Math { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Ink { locked, .. }
        | crate::artifacts::note::NoteBlockNode::Group { locked, .. } => *locked = payload.new_locked,
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
