//! ↩️ Inverse for `EditBlockMath`.
use super::EditBlockMath;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditBlockMath, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Math { tex, .. }) => vec![NoteMutation::EditBlockMath(EditBlockMath { id: payload.id.clone(), new_tex: tex.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
