//! ↩️ Inverse for `MoveBlock`.
use super::MoveBlock;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &MoveBlock, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => {
            let (x, y, ..) = crate::artifacts::note::schema::block_bounds(block);
            vec![NoteMutation::MoveBlock(MoveBlock { id: payload.id.clone(), new_x: x, new_y: y })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
