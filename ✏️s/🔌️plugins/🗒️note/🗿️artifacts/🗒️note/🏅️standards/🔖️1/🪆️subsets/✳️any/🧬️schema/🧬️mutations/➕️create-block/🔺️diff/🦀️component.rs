//! 🔺️ Diff fragment yielded by `CreateBlock`. Fatal `duplicate-id` on an existing id, Fatal
//! `invariant` on an unknown/non-group container.
use super::mutation::CreateBlock;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_added_diff;

//#region 🔖️Diff
pub async fn diff(payload: &CreateBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let new_id = crate::artifacts::note::schema::block_id(&payload.block);
    if crate::artifacts::note::schema::find_block(&base.blocks, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A block with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    if let Some(parent_id) = &payload.parent_id {
        match crate::artifacts::note::schema::find_block(&base.blocks, parent_id) {
            None => return protocol::MutationOutcome::fatal("mutation.invariant", format!("Container \"{}\" does not exist.", parent_id), [parent_id.clone()]),
            Some(crate::artifacts::note::NoteBlockNode::Group { .. }) => {}
            Some(_) => return protocol::MutationOutcome::fatal("mutation.invariant", format!("Container \"{}\" is not a group.", parent_id), [parent_id.clone()]),
        }
    }
    protocol::MutationOutcome::new(note_block_added_diff(payload.parent_id.clone(), payload.index, (*payload.block).clone()))
}
//#endregion 🔖️Diff
