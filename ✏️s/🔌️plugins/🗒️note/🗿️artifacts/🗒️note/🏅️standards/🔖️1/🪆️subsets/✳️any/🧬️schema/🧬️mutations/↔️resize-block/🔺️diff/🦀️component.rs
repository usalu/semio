//! 🔺️ Diff fragment yielded by `ResizeBlock`. Error `target-missing` when absent, Warning `no-op`
//! when already at that size, Fatal `invariant` when the size is non-finite or non-positive.
use super::mutation::ResizeBlock;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub async fn diff(payload: &ResizeBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_width.is_finite() || !payload.new_height.is_finite() || payload.new_width <= 0.0 || payload.new_height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Block \"{}\" size must be finite and positive, got ({}, {}).", payload.id, payload.new_width, payload.new_height), [payload.id.clone()]);
    }
    let (_, _, width, height) = crate::artifacts::note::schema::block_bounds(block);
    if width == payload.new_width && height == payload.new_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" already has size ({}, {}).", payload.id, payload.new_width, payload.new_height));
    }
    let mut updated = block.clone();
    match &mut updated { crate::artifacts::note::NoteBlockNode::Text { width, height, .. } | crate::artifacts::note::NoteBlockNode::Image { width, height, .. } | crate::artifacts::note::NoteBlockNode::Table { width, height, .. } | crate::artifacts::note::NoteBlockNode::Math { width, height, .. } | crate::artifacts::note::NoteBlockNode::Ink { width, height, .. } | crate::artifacts::note::NoteBlockNode::Group { width, height, .. } => { *width = payload.new_width; *height = payload.new_height; } }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
