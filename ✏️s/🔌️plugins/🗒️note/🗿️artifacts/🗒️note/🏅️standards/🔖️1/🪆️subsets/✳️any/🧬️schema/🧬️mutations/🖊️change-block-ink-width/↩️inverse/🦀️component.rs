//! ↩️ Inverse for `ChangeBlockInkWidth`.
use super::mutation::ChangeBlockInkWidth;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeBlockInkWidth, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Ink { stroke_width, .. }) => vec![NoteMutation::ChangeBlockInkWidth(ChangeBlockInkWidth { id: payload.id.clone(), new_stroke_width: *stroke_width })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
