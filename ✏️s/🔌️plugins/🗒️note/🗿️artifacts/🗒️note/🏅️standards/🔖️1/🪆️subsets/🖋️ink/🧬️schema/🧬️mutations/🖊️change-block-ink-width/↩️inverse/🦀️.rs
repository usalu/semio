//! ↩️ Inverse for `ChangeBlockInkWidth`.
use super::ChangeBlockInkWidth;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeBlockInkWidth, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Ink { stroke_width, .. }) => vec![NoteMutation::ChangeBlockInkWidth(ChangeBlockInkWidth { id: payload.id.clone(), new_stroke_width: *stroke_width })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
