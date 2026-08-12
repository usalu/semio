//! ↩️ Inverse for `EditBlockText`.
use super::mutation::EditBlockText;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditBlockText, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Text { paragraphs, .. }) => vec![NoteMutation::EditBlockText(EditBlockText { id: payload.id.clone(), new_paragraphs: paragraphs.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
