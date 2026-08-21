//! 🔺️ Diff fragment yielded by `MoveBlock`. Error `target-missing` when absent, Warning `no-op`
//! when already at that position, Fatal `invariant` when the position is non-finite.
use super::mutation::MoveBlock;
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &MoveBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Block \"{}\" position must be finite, got ({}, {}).", payload.id, payload.new_x, payload.new_y), [payload.id.clone()]);
    }
    let (x, y, ..) = crate::artifacts::note::schema::block_bounds(block);
    if x == payload.new_x && y == payload.new_y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" is already at ({}, {}).", payload.id, payload.new_x, payload.new_y));
    }
    let mut updated = block.clone();
    match &mut updated {
        crate::artifacts::note::NoteBlockNode::Text { x, y, .. }
        | crate::artifacts::note::NoteBlockNode::Image { x, y, .. }
        | crate::artifacts::note::NoteBlockNode::Table { x, y, .. }
        | crate::artifacts::note::NoteBlockNode::Math { x, y, .. }
        | crate::artifacts::note::NoteBlockNode::Ink { x, y, .. }
        | crate::artifacts::note::NoteBlockNode::Group { x, y, .. } => {
            *x = payload.new_x;
            *y = payload.new_y;
        }
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
