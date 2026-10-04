//! ↩️ Inverse for `DuplicateBlocks`.
use super::DuplicateBlocks;
use crate::schema::mutations::DeleteBlocks;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DuplicateBlocks, _base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![NoteMutation::DeleteBlocks(DeleteBlocks { ids: payload.blocks.iter().map(|b| crate::schema::block_id(b).to_string()).collect() })]

    })())
}
//#endregion 🔖️Inverse
