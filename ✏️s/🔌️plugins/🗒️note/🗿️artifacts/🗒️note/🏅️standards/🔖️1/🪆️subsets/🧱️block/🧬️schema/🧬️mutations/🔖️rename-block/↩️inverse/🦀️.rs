//! ↩️ Inverse for `RenameBlock`.
use super::RenameBlock;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RenameBlock, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => vec![NoteMutation::RenameBlock(RenameBlock { id: payload.id.clone(), new_name: crate::schema::block_name(block).to_string() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
