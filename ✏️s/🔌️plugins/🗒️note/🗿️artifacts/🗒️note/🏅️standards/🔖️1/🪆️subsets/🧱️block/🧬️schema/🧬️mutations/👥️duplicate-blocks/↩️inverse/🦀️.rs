//! ↩️ Inverse for `DuplicateBlocks`.
use super::DuplicateBlocks;
use crate::schema::mutations::DeleteBlocks;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DuplicateBlocks, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    let refused = payload.blocks.iter().any(|block| crate::schema::find_block(&base.blocks, crate::schema::block_id(block)).is_some());
    let (placed, _) = super::placements(payload, base);
    Ok(match refused || placed.is_empty() {
        true => Vec::new(),
        false => vec![NoteMutation::DeleteBlocks(DeleteBlocks { ids: placed.iter().map(|(_, _, block)| crate::schema::block_id(block).to_string()).collect() })],
    })
}
//#endregion 🔖️Inverse
