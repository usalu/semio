//! 🔺️ Diff fragment yielded by `CreateBlock`. Fatal `duplicate-id` on an existing id, Fatal
//! `invariant` on an unknown/non-group container.
use super::CreateBlock;
use crate::schema::diff::NoteBlockRow;
use crate::{NoteDiff, NoteSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let new_id = crate::schema::block_id(&payload.block);
    if crate::schema::find_block(&base.blocks, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A block with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    let Some(container_len) = crate::schema::container_len(&base.blocks, payload.parent_id.as_deref()) else {
        let parent_id = payload.parent_id.clone().unwrap_or_default();
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Container \"{parent_id}\" does not exist or is not a group."), [parent_id]);
    };
    let index = payload.index.map_or(container_len, |index| index.min(container_len));
    protocol::MutationOutcome::new(NoteDiff::block_rows(vec![NoteBlockRow::Add { parent_id: payload.parent_id.clone(), index, block: (*payload.block).clone() }]))
}
//#endregion 🔖️Diff
