//! ↩️ Inverse for `MoveBlock`.
use super::mutation::MoveBlock;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &MoveBlock, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) {
        Some(block) => { let (x, y, ..) = crate::artifacts::note::engine::block_bounds(block); vec![NoteMutation::MoveBlock(MoveBlock { id: payload.id.clone(), new_x: x, new_y: y })] }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
