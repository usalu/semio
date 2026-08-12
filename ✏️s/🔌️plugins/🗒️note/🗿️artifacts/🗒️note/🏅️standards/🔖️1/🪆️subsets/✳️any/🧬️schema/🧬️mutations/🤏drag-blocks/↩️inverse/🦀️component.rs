//! ↩️ Inverse for `DragBlocks`.
use super::mutation::DragBlocks;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DragBlocks, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::DragBlocks(DragBlocks { ids: payload.ids.clone(), dx: -payload.dx, dy: -payload.dy })]
}
//#endregion 🔖️Inverse
