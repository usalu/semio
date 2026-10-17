//! ↩️ Inverse for `EditBlockMath`.
use super::EditBlockMath;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &EditBlockMath, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::math::schema::mutations::edit_block_math::EditBlockMath>,
{
    Ok((|| {
    match crate::schema::find_block(&base.blocks, &payload.id) {
        Some(crate::NoteBlockNode::Math { tex, .. }) => vec![Op::from(EditBlockMath { id: payload.id.clone(), new_tex: tex.clone() })],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
