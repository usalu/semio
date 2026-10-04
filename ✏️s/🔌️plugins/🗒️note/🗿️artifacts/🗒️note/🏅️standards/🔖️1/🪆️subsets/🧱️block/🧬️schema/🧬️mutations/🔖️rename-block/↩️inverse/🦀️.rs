//! ↩️ Inverse for `RenameBlock`.
use super::RenameBlock;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RenameBlock, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => vec![NoteMutation::RenameBlock(RenameBlock { id: payload.id.clone(), new_name: crate::schema::block_name(block).to_string() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
