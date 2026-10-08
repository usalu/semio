//! ↩️ Inverse for `DeleteBlocks`.
use super::DeleteBlocks;
use crate::schema::mutations::CreateBlock;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteBlocks, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok(super::removal_roots(payload, base).into_iter().map(|(parent_id, index, block)| NoteMutation::CreateBlock(CreateBlock { block: Box::new(block), parent_id, index: Some(index) })).collect())
}
//#endregion 🔖️Inverse
