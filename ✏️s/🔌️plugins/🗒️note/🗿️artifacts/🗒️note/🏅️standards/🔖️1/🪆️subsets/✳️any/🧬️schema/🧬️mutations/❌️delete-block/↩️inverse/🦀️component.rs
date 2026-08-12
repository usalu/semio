//! ↩️ Inverse for `DeleteBlock`.
use super::mutation::DeleteBlock;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::schema::mutations::CreateBlock;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteBlock, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match (crate::artifacts::note::engine::find_block(&base.blocks, &payload.id), crate::artifacts::note::engine::find_block_location(&base.blocks, &payload.id)) {
        (Some(block), Some((parent_id, index))) => vec![NoteMutation::CreateBlock(CreateBlock { block: block.clone(), parent_id, index: Some(index) })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
