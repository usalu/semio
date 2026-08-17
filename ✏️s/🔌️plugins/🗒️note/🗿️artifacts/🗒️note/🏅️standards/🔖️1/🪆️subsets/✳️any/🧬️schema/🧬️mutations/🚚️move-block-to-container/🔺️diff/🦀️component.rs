//! 🔺️ Diff fragment yielded by `MoveBlockToContainer`. Error `target-missing` on an absent block
//! or container, Fatal `invariant` on a self-container or a non-group container.
use super::mutation::MoveBlockToContainer;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveBlockToContainer, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(parent_id) = &payload.new_parent_id {
        if parent_id == &payload.id {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Block \"{}\" cannot become its own container.", payload.id), [payload.id.clone()]);
        }
        match crate::artifacts::note::schema::find_block(&base.blocks, parent_id) {
            None => return protocol::MutationOutcome::error("mutation.target-missing", format!("Container \"{}\" does not exist.", parent_id), [parent_id.clone()]),
            Some(crate::artifacts::note::NoteBlockNode::Group { .. }) => {}
            Some(_) => return protocol::MutationOutcome::fatal("mutation.invariant", format!("Container \"{}\" is not a group.", parent_id), [parent_id.clone()]),
        }
    }
    let mut delta = crate::artifacts::note::schema::diff::NoteBlocksDelta::default();
    delta.removed.push(payload.id.clone());
    delta.added.push(crate::artifacts::note::schema::diff::NoteAddedBlockEntry { parent_id: payload.new_parent_id.clone(), index: Some(payload.index), block: block.clone() });
    protocol::MutationOutcome::new(NoteDiff { blocks: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
