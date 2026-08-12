//! ↩️ Inverse for `ResizeBlock`.
use super::mutation::ResizeBlock;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ResizeBlock, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => { let (.., width, height) = crate::artifacts::note::schema::block_bounds(block); vec![NoteMutation::ResizeBlock(ResizeBlock { id: payload.id.clone(), new_width: width, new_height: height })] }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
