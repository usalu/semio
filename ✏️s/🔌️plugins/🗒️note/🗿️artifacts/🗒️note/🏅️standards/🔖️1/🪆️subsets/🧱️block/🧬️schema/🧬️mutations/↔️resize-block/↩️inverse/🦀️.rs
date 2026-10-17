//! ↩️ Inverse for `ResizeBlock`.
use super::ResizeBlock;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ResizeBlock, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::resize_block::ResizeBlock>,
{
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(block) => {
            let (.., width, height) = crate::schema::block_bounds(block);
            vec![Op::from(ResizeBlock { id: payload.id.clone(), new_width: width, new_height: height })]
        }
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
