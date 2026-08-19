//! ↩️ Inverse for `EditBlockText`.
use super::mutation::EditBlockText;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &EditBlockText, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Text { content, .. }) => {
            let paragraphs = crate::artifacts::note::note_block_text(content);
            vec![NoteMutation::EditBlockText(EditBlockText { id: payload.id.clone(), new_paragraphs: paragraphs })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
