//! 🔺️ Diff fragment yielded by `ChangeBlockFontSize`. Error `target-missing` when the block is
//! absent or not a text block, Warning `no-op` when already at that size.
use super::ChangeBlockFontSize;
use crate::artifacts::note::schema::diff::note_block_patch_diff;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeBlockFontSize, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let crate::artifacts::note::NoteBlockNode::Text { font_size, .. } = block else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a text block.", payload.id), [payload.id.clone()]);
    };
    if *font_size == payload.new_font_size {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" font size is already {}.", payload.id, payload.new_font_size));
    }
    let mut updated = block.clone();
    if let crate::artifacts::note::NoteBlockNode::Text { font_size, .. } = &mut updated {
        *font_size = payload.new_font_size;
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
