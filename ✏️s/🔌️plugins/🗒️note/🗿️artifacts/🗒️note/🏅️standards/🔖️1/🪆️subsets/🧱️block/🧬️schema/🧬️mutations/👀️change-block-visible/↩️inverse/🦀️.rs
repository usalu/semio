//! ↩️ Inverse for `ChangeBlockVisible`.
use super::ChangeBlockVisible;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangeBlockVisible, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::change_block_visible::ChangeBlockVisible>,
{
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => vec![Op::from(ChangeBlockVisible { id: payload.id.clone(), new_visible: crate::schema::block_visible(block) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
