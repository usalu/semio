//! ↩️ Inverse for `ChangeBlockFontSize`.
use super::ChangeBlockFontSize;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeBlockFontSize, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Text { font_size, .. }) => vec![NoteMutation::ChangeBlockFontSize(ChangeBlockFontSize { id: payload.id.clone(), new_font_size: *font_size })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
