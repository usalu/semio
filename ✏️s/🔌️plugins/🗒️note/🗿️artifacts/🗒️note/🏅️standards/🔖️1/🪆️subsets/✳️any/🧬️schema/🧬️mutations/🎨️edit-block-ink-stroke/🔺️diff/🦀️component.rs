//! 🔺️ Diff fragment yielded by `EditBlockInkStroke`. Error `target-missing` when the block is
//! absent or not an ink block, Warning `no-op` when the stroke is unchanged.
use super::mutation::EditBlockInkStroke;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub async fn diff(payload: &EditBlockInkStroke, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let crate::artifacts::note::NoteBlockNode::Ink { points, x, y, width, height, .. } = block else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not an ink block.", payload.id), [payload.id.clone()]);
    };
    if points == &payload.new_points && *x == payload.new_x && *y == payload.new_y && *width == payload.new_width && *height == payload.new_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" ink stroke is unchanged.", payload.id));
    }
    let mut updated = block.clone();
    if let crate::artifacts::note::NoteBlockNode::Ink { points, x, y, width, height, .. } = &mut updated { *points = payload.new_points.clone(); *x = payload.new_x; *y = payload.new_y; *width = payload.new_width; *height = payload.new_height; }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
