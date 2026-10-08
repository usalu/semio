//! 🔺️ Diff fragment yielded by `ResizeBlock`. Error `target-missing` when absent, Warning `no-op`
//! when already at that size, Fatal `invariant` when the size is non-finite or non-positive.
use super::ResizeBlock;
use crate::schema::diff::NoteBlockPatch;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ResizeBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_width.is_finite() || !payload.new_height.is_finite() || payload.new_width <= 0.0 || payload.new_height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Block \"{}\" size must be finite and positive, got ({}, {}).", payload.id, payload.new_width, payload.new_height), [payload.id.clone()]);
    }
    let (_, _, width, height) = crate::schema::block_bounds(block);
    if width == payload.new_width && height == payload.new_height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" already has size ({}, {}).", payload.id, payload.new_width, payload.new_height));
    }
    protocol::MutationOutcome::new(NoteDiff::block_patches([(payload.id.clone(), NoteBlockPatch { width: Some(payload.new_width), height: Some(payload.new_height), ..Default::default() })]))
}
//#endregion 🔖️Diff
