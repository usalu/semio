//! ↩️ Inverse for `MoveBlockToContainer`.
use super::MoveBlockToContainer;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &MoveBlockToContainer, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::schema::find_block_location(&base.blocks, &payload.id) {
        Some((parent_id, index)) => vec![NoteMutation::MoveBlockToContainer(MoveBlockToContainer { id: payload.id.clone(), new_parent_id: parent_id, index })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
