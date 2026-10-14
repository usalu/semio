//! ↩️ Inverse for `ResizeGrip3d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::resize_grip_3d::ResizeGrip3d, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::resize_grip_3d::ResizeGrip3d> {
    Ok((|| {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::resize_grip_3d::resize_grip_3d(payload.id.clone(), existing.radius_3d)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
