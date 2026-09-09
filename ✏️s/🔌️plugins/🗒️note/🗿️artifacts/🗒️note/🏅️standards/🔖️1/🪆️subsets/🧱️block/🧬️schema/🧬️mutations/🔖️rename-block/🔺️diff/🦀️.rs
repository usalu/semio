//! 🔺️ Diff fragment yielded by `RenameBlock`. Error `target-missing` when absent, Warning `no-op`
//! when already at that name.
use super::RenameBlock;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if crate::schema::block_name(block) == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" already has name \"{}\".", payload.id, payload.new_name));
    }
    let mut updated = block.clone();
    match &mut updated {
        crate::NoteBlockNode::Text { name, .. }
        | crate::NoteBlockNode::Image { name, .. }
        | crate::NoteBlockNode::Table { name, .. }
        | crate::NoteBlockNode::Math { name, .. }
        | crate::NoteBlockNode::Ink { name, .. }
        | crate::NoteBlockNode::Group { name, .. } => *name = payload.new_name.clone(),
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, &updated))
}
//#endregion 🔖️Diff
