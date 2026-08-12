//! ↩️ Inverse for `RenameBlock`.
use super::mutation::RenameBlock;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RenameBlock, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) {
        Some(block) => vec![NoteMutation::RenameBlock(RenameBlock { id: payload.id.clone(), new_name: crate::artifacts::note::engine::block_name(block).to_string() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
