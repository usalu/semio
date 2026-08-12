//! ↩️ Inverse for `ChangeBlockVisible`.
use super::mutation::ChangeBlockVisible;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeBlockVisible, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) {
        Some(block) => vec![NoteMutation::ChangeBlockVisible(ChangeBlockVisible { id: payload.id.clone(), new_visible: crate::artifacts::note::engine::block_visible(block) })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
