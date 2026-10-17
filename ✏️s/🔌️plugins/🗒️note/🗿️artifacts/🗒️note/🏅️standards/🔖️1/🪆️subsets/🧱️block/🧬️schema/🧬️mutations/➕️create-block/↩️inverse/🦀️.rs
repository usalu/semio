//! ↩️ Inverse for `CreateBlock`.
use super::CreateBlock;
use crate::standards::v1::subsets::block::schema::mutations::delete_block::DeleteBlock;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &CreateBlock, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::delete_block::DeleteBlock>,
{
    let id = crate::schema::block_id(&payload.block);
    Ok(match crate::schema::find_block(&base.blocks, id) {
        Some(_) => Vec::new(),
        None => vec![Op::from(DeleteBlock { id: id.to_string() })],
    })
}
//#endregion 🔖️Inverse
