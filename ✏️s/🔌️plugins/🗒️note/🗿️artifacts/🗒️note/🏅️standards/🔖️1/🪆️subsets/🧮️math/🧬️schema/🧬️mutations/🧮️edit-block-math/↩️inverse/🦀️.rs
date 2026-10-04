//! ↩️ Inverse for `EditBlockMath`.
use super::EditBlockMath;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditBlockMath, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Math { tex, .. }) => vec![NoteMutation::EditBlockMath(EditBlockMath { id: payload.id.clone(), new_tex: tex.clone() })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
