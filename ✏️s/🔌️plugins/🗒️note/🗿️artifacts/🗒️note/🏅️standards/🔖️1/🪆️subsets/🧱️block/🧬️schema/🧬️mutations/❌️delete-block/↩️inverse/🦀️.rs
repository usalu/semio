//! ↩️ Inverse for `DeleteBlock`.
use super::DeleteBlock;
use crate::schema::mutations::CreateBlock;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteBlock, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match (crate::schema::find_block(&base.blocks, &payload.id), crate::schema::find_block_location(&base.blocks, &payload.id)) {
        (Some(block), Some((parent_id, index))) => vec![NoteMutation::CreateBlock(CreateBlock { block: Box::new(block.clone()), parent_id, index: Some(index) })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
