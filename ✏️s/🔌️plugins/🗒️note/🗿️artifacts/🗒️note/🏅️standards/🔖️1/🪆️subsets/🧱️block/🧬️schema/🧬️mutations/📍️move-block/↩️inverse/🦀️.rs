//! ↩️ Inverse for `MoveBlock`.
use super::MoveBlock;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &MoveBlock, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::move_block::MoveBlock>,
{
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => {
            let (x, y, ..) = crate::schema::block_bounds(block);
            vec![Op::from(MoveBlock { id: payload.id.clone(), new_x: x, new_y: y })]
        }
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
