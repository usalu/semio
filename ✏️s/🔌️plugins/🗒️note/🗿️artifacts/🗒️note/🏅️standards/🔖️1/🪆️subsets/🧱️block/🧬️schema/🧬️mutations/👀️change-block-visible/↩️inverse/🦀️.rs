//! ↩️ Inverse for `ChangeBlockVisible`.
use super::ChangeBlockVisible;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeBlockVisible, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => vec![NoteMutation::ChangeBlockVisible(ChangeBlockVisible { id: payload.id.clone(), new_visible: crate::schema::block_visible(block) })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
