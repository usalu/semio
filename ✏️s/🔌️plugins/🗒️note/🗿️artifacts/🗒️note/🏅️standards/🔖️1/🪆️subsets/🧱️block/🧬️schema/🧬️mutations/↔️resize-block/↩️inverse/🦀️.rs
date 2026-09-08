//! ↩️ Inverse for `ResizeBlock`.
use super::ResizeBlock;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ResizeBlock, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => {
            let (.., width, height) = crate::schema::block_bounds(block);
            vec![NoteMutation::ResizeBlock(ResizeBlock { id: payload.id.clone(), new_width: width, new_height: height })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
