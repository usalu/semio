//! ↩️ Inverse for `ChangeBlockInkWidth`.
use super::ChangeBlockInkWidth;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeBlockInkWidth, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Ink { stroke_width, .. }) => vec![NoteMutation::ChangeBlockInkWidth(ChangeBlockInkWidth { id: payload.id.clone(), new_stroke_width: *stroke_width })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
