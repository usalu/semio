//! ↩️ Inverse for `DragBlocks`.
use super::DragBlocks;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &DragBlocks, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::block::schema::mutations::drag_blocks::DragBlocks>,
{
    let ids: Vec<String> = payload.ids.iter().filter(|id| crate::schema::find_block(&base.blocks, id).is_some()).cloned().collect();
    Ok(match ids.is_empty() {
        true => Vec::new(),
        false => vec![Op::from(DragBlocks { ids, dx: -payload.dx, dy: -payload.dy })],
    })
}
//#endregion 🔖️Inverse
