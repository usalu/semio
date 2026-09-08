//! ↩️ Inverse for `EditBlockText`.
use super::EditBlockText;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditBlockText, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Text { content, .. }) => {
            let paragraphs = crate::note_block_text(content);
            vec![NoteMutation::EditBlockText(EditBlockText { id: payload.id.clone(), new_paragraphs: paragraphs })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
