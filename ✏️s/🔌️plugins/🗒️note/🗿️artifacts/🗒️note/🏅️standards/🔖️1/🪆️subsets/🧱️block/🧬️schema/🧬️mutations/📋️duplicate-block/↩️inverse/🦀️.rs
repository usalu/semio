//! ↩️ Inverse for `DuplicateBlock`.
use super::DuplicateBlock;
use crate::artifacts::note::schema::mutations::DeleteBlock;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DuplicateBlock, _base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::DeleteBlock(DeleteBlock { id: crate::artifacts::note::schema::block_id(&payload.block).to_string() })]
}
//#endregion 🔖️Inverse
