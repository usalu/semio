//! ↩️ Inverse for `DragBlocks`.
use super::DragBlocks;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DragBlocks, _base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::DragBlocks(DragBlocks { ids: payload.ids.clone(), dx: -payload.dx, dy: -payload.dy })]
}
//#endregion 🔖️Inverse
