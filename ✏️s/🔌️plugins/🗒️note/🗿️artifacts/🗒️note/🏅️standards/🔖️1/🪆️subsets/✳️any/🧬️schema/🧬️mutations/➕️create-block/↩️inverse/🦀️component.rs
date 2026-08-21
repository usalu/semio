//! ↩️ Inverse for `CreateBlock`.
use super::mutation::CreateBlock;
use crate::artifacts::note::schema::mutations::DeleteBlock;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateBlock, _base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::DeleteBlock(DeleteBlock { id: crate::artifacts::note::schema::block_id(&payload.block).to_string() })]
}
//#endregion 🔖️Inverse
