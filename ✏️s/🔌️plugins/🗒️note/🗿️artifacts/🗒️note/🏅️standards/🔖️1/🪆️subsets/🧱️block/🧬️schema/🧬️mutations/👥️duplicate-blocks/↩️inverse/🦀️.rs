//! ↩️ Inverse for `DuplicateBlocks`.
use super::DuplicateBlocks;
use crate::standards::v1::subsets::block::schema::mutations::delete_blocks::DeleteBlocks;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &DuplicateBlocks, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::delete_blocks::DeleteBlocks>,
{
    let refused = payload.blocks.iter().any(|block| crate::schema::find_block(&base.blocks, crate::schema::block_id(block)).is_some());
    let (placed, _) = super::placements(payload, base);
    Ok(match refused || placed.is_empty() {
        true => Vec::new(),
        false => vec![Op::from(DeleteBlocks { ids: placed.iter().map(|(_, _, block)| crate::schema::block_id(block).to_string()).collect() })],
    })
}
//#endregion 🔖️Inverse
