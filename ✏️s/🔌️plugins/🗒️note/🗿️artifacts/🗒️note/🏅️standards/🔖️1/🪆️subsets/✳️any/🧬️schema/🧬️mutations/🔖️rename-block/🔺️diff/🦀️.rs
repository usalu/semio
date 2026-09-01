//! 🔺️ Diff fragment yielded by `RenameBlock`. Error `target-missing` when absent, Warning `no-op`
//! when already at that name.
use super::RenameBlock;
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &RenameBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if crate::artifacts::note::schema::block_name(block) == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" already has name \"{}\".", payload.id, payload.new_name));
    }
    let mut updated = block.clone();
    match &mut updated {
        crate::artifacts::note::NoteBlockNode::Text { name, .. }
        | crate::artifacts::note::NoteBlockNode::Image { name, .. }
        | crate::artifacts::note::NoteBlockNode::Table { name, .. }
        | crate::artifacts::note::NoteBlockNode::Math { name, .. }
        | crate::artifacts::note::NoteBlockNode::Ink { name, .. }
        | crate::artifacts::note::NoteBlockNode::Group { name, .. } => *name = payload.new_name.clone(),
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
