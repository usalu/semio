//! ↩️ Inverse for `DeleteBlock`.
use super::DeleteBlock;
use crate::standards::v1::subsets::block::schema::mutations::create_block::CreateBlock;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &DeleteBlock, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::create_block::CreateBlock>,
{
    Ok((|| {
    match (crate::schema::find_block(&base.blocks, &payload.id), crate::schema::find_block_location(&base.blocks, &payload.id)) {
        (Some(block), Some((parent_id, index))) => vec![Op::from(CreateBlock { block: Box::new(block.clone()), parent_id, index: Some(index) })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
