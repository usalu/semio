//! 🔺️ Diff fragment yielded by `ChangeBlockInkWidth`. Error `target-missing` when the block is
//! absent or not an ink block, Warning `no-op` when already at that width.
use super::ChangeBlockInkWidth;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockInkWidth, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let crate::NoteBlockNode::Ink { stroke_width, .. } = block else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not an ink block.", payload.id), [payload.id.clone()]);
    };
    if *stroke_width == payload.new_stroke_width {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" ink width is already {}.", payload.id, payload.new_stroke_width));
    }
    let mut updated = block.clone();
    if let crate::NoteBlockNode::Ink { stroke_width, .. } = &mut updated {
        *stroke_width = payload.new_stroke_width;
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, &updated))
}
//#endregion 🔖️Diff
