//! ↩️ Inverse for `EditBlockInkStroke`.
use super::EditBlockInkStroke;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditBlockInkStroke, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Ink { points, x, y, width, height, .. }) => {
            vec![NoteMutation::EditBlockInkStroke(EditBlockInkStroke { id: payload.id.clone(), new_points: points.clone(), new_x: *x, new_y: *y, new_width: *width, new_height: *height })]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
