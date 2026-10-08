//! ↩️ Inverse for `DragBlocks`.
use super::DragBlocks;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DragBlocks, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    let ids: Vec<String> = payload.ids.iter().filter(|id| crate::schema::find_block(&base.blocks, id).is_some()).cloned().collect();
    Ok(match ids.is_empty() {
        true => Vec::new(),
        false => vec![NoteMutation::DragBlocks(DragBlocks { ids, dx: -payload.dx, dy: -payload.dy })],
    })
}
//#endregion 🔖️Inverse
