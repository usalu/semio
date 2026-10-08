//! ↩️ Inverse for `CreateBlock`.
use super::CreateBlock;
use crate::schema::mutations::DeleteBlock;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateBlock, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    let id = crate::schema::block_id(&payload.block);
    Ok(match crate::schema::find_block(&base.blocks, id) {
        Some(_) => Vec::new(),
        None => vec![NoteMutation::DeleteBlock(DeleteBlock { id: id.to_string() })],
    })
}
//#endregion 🔖️Inverse
