//! ↩️ Inverse for `RenameBlock`.
use super::RenameBlock;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RenameBlock, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::rename_block::RenameBlock>,
{
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => vec![Op::from(RenameBlock { id: payload.id.clone(), new_name: crate::schema::block_name(block).to_string() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
