//! ↩️ Inverse for `DragBlocks`.
use super::DragBlocks;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DragBlocks, _base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![NoteMutation::DragBlocks(DragBlocks { ids: payload.ids.clone(), dx: -payload.dx, dy: -payload.dy })]

    })())
}
//#endregion 🔖️Inverse
