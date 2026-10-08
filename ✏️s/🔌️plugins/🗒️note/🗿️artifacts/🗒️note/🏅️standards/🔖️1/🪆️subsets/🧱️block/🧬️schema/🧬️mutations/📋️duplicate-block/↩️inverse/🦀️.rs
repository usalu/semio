//! ↩️ Inverse for `DuplicateBlock`.
use super::DuplicateBlock;
use crate::schema::mutations::DeleteBlock;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DuplicateBlock, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    let id = crate::schema::block_id(&payload.block);
    let refused = crate::schema::find_block(&base.blocks, id).is_some() || crate::schema::find_block_location(&base.blocks, &payload.source_id).is_none();
    Ok(match refused {
        true => Vec::new(),
        false => vec![NoteMutation::DeleteBlock(DeleteBlock { id: id.to_string() })],
    })
}
//#endregion 🔖️Inverse
