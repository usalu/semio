//! 🔺️ Diff fragment yielded by `ChangeBlockFontSize`. Error `target-missing` when the block is
//! absent or not a text block, Warning `no-op` when already at that size.
use super::ChangeBlockFontSize;
use crate::schema::diff::NoteBlockPatch;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockFontSize, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let crate::NoteBlockNode::Text { font_size, .. } = block else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a text block.", payload.id), [payload.id.clone()]);
    };
    if *font_size == payload.new_font_size {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Block \"{}\" font size is already {}.", payload.id, payload.new_font_size));
    }
    protocol::MutationOutcome::new(NoteDiff::block_patches([(payload.id.clone(), NoteBlockPatch { font_size: Some(payload.new_font_size), ..Default::default() })]))
}
//#endregion 🔖️Diff
