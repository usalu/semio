//! 🔺️ Diff fragment yielded by `MoveBlock`. Error `target-missing` when absent, Warning `no-op`
//! when already at that position, Fatal `invariant` when the position is non-finite.
use super::MoveBlock;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Block \"{}\" position must be finite, got ({}, {}).", payload.id, payload.new_x, payload.new_y), [payload.id.clone()]);
    }
    let (x, y, ..) = crate::schema::block_bounds(block);
    if x == payload.new_x && y == payload.new_y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" is already at ({}, {}).", payload.id, payload.new_x, payload.new_y));
    }
    let mut updated = block.clone();
    match &mut updated {
        crate::NoteBlockNode::Text { x, y, .. }
        | crate::NoteBlockNode::Image { x, y, .. }
        | crate::NoteBlockNode::Table { x, y, .. }
        | crate::NoteBlockNode::Math { x, y, .. }
        | crate::NoteBlockNode::Ink { x, y, .. }
        | crate::NoteBlockNode::Group { x, y, .. } => {
            *x = payload.new_x;
            *y = payload.new_y;
        }
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
