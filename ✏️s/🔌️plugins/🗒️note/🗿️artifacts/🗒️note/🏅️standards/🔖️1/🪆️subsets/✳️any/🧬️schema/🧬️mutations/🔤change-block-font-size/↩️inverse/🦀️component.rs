//! ↩️ Inverse for `ChangeBlockFontSize`.
use super::mutation::ChangeBlockFontSize;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeBlockFontSize, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Text { font_size, .. }) => vec![NoteMutation::ChangeBlockFontSize(ChangeBlockFontSize { id: payload.id.clone(), new_font_size: *font_size })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
