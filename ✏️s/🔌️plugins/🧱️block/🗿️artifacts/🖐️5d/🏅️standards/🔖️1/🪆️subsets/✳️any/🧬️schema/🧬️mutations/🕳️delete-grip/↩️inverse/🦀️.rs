//! ↩️ Inverse for `DeleteGrip`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::delete_grip::DeleteGrip, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::create_grip::CreateGrip> {
    Ok((|| {
    match base.grips.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((position, existing)) => vec![crate::create_grip::create_grip_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
