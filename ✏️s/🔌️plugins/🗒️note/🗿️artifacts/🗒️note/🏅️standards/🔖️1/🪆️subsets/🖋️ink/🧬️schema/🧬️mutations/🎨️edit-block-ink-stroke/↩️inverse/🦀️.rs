//! ↩️ Inverse for `EditBlockInkStroke`.
use super::EditBlockInkStroke;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &EditBlockInkStroke, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::EditBlockInkStroke>,
{
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Ink { points, x, y, width, height, .. }) => {
            vec![Op::from(EditBlockInkStroke { id: payload.id.clone(), new_points: points.clone(), new_x: *x, new_y: *y, new_width: *width, new_height: *height })]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
