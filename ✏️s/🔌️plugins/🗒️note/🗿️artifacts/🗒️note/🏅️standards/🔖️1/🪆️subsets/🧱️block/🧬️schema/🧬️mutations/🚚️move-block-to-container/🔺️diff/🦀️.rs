//! 🔺️ Diff fragment yielded by `MoveBlockToContainer`. Error `target-missing` on an absent block
//! or container, Fatal `invariant` on a self-container or a non-group container.
use super::MoveBlockToContainer;
use crate::schema::diff::NoteBlockRow;
use crate::{NoteBlockNode, NoteDiff, NoteSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MoveBlockToContainer, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some((origin_parent, origin_index)) = crate::schema::find_block_location(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(parent_id) = &payload.new_parent_id {
        if parent_id == &payload.id || crate::schema::find_block(std::slice::from_ref(block), parent_id).is_some() {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Block \"{}\" cannot become its own container.", payload.id), [payload.id.clone()]);
        }
        match crate::schema::find_block(&base.blocks, parent_id) {
            None => return protocol::MutationOutcome::error("mutation.target-missing", format!("Container \"{}\" does not exist.", parent_id), [parent_id.clone()]),
            Some(NoteBlockNode::Group { .. }) => {}
            Some(_) => return protocol::MutationOutcome::fatal("mutation.invariant", format!("Container \"{}\" is not a group.", parent_id), [parent_id.clone()]),
        }
    }
    let target_len = crate::schema::container_len(&base.blocks, payload.new_parent_id.as_deref()).unwrap_or_default();
    let remaining = if origin_parent == payload.new_parent_id { target_len - 1 } else { target_len };
    let index = payload.index.min(remaining);
    if origin_parent == payload.new_parent_id && origin_index == index {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" is already at that position.", payload.id));
    }
    protocol::MutationOutcome::new(NoteDiff::block_rows(vec![NoteBlockRow::Move { id: payload.id.clone(), from_parent_id: origin_parent, from_index: origin_index, parent_id: payload.new_parent_id.clone(), index }]))
}
//#endregion 🔖️Diff
