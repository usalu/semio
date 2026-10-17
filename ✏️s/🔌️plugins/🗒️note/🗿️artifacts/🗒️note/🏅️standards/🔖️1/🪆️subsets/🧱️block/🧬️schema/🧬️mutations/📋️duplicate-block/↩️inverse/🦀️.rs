//! ↩️ Inverse for `DuplicateBlock`.
use super::DuplicateBlock;
use crate::standards::v1::subsets::block::schema::mutations::delete_block::DeleteBlock;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &DuplicateBlock, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::delete_block::DeleteBlock>,
{
    let id = crate::schema::block_id(&payload.block);
    let refused = crate::schema::find_block(&base.blocks, id).is_some() || crate::schema::find_block_location(&base.blocks, &payload.source_id).is_none();
    Ok(match refused {
        true => Vec::new(),
        false => vec![Op::from(DeleteBlock { id: id.to_string() })],
    })
}
//#endregion 🔖️Inverse
