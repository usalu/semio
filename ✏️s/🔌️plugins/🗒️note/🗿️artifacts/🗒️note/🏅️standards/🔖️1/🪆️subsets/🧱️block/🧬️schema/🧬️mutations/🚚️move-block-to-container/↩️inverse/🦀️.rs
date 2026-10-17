//! ↩️ Inverse for `MoveBlockToContainer`.
use super::MoveBlockToContainer;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &MoveBlockToContainer, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::move_block_to_container::MoveBlockToContainer>,
{
    Ok((|| {
    match crate::schema::find_block_location(&base.blocks, &payload.id) {
        Some((parent_id, index)) => vec![Op::from(MoveBlockToContainer { id: payload.id.clone(), new_parent_id: parent_id, index })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
