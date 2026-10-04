//! ↩️ Inverse for `MoveBlock`.
use super::MoveBlock;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &MoveBlock, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => {
            let (x, y, ..) = crate::schema::block_bounds(block);
            vec![NoteMutation::MoveBlock(MoveBlock { id: payload.id.clone(), new_x: x, new_y: y })]
        }
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
