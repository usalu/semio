//! ↩️ Inverse for `EditBlockInkStroke`.
use super::mutation::EditBlockInkStroke;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditBlockInkStroke, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Ink { points, x, y, width, height, .. }) => vec![NoteMutation::EditBlockInkStroke(EditBlockInkStroke { id: payload.id.clone(), new_points: points.clone(), new_x: *x, new_y: *y, new_width: *width, new_height: *height })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
