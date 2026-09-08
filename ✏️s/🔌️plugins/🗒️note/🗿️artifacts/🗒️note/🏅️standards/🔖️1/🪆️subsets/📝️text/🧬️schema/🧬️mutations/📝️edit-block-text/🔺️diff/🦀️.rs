//! 🔺️ Diff fragment yielded by `EditBlockText`. Error `target-missing` when the block is absent
//! or not a text block.
use super::EditBlockText;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &EditBlockText, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !matches!(block, crate::NoteBlockNode::Text { .. }) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a text block.", payload.id), [payload.id.clone()]);
    }
    let mut updated = block.clone();
    if let crate::NoteBlockNode::Text { content, .. } = &mut updated {
        *content = crate::note_text_child_record(&payload.id, &payload.new_paragraphs);
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
