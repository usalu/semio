//! ↩️ Inverse for `CreateBlock`.
use super::CreateBlock;
use crate::schema::mutations::DeleteBlock;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateBlock, _base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![NoteMutation::DeleteBlock(DeleteBlock { id: crate::schema::block_id(&payload.block).to_string() })]

    })())
}
//#endregion 🔖️Inverse
