//! ↩️ Inverse for `EditBlockMath`.
use super::mutation::EditBlockMath;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditBlockMath, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) {
        Some(crate::artifacts::note::NoteBlockNode::Math { tex, .. }) => vec![NoteMutation::EditBlockMath(EditBlockMath { id: payload.id.clone(), new_tex: tex.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
