//! ↩️ Inverse for `DuplicateBlocks`.
use super::mutation::DuplicateBlocks;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::schema::mutations::DeleteBlocks;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DuplicateBlocks, _base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::DeleteBlocks(DeleteBlocks { ids: payload.blocks.iter().map(|b| crate::artifacts::note::schema::block_id(b).to_string()).collect() })]
}
//#endregion 🔖️Inverse
