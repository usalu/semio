//! ↩️ Inverse for `DeleteBlocks`.
use super::DeleteBlocks;
use crate::standards::v1::subsets::block::schema::mutations::create_block::CreateBlock;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &DeleteBlocks, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::create_block::CreateBlock>,
{
    Ok(super::removal_roots(payload, base).into_iter().map(|(parent_id, index, block)| Op::from(CreateBlock { block: Box::new(block), parent_id, index: Some(index) })).collect())
}
//#endregion 🔖️Inverse
