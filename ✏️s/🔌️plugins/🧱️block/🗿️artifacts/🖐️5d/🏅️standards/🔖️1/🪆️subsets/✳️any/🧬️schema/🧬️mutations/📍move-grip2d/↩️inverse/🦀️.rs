//! ↩️ Inverse for `MoveGrip2d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::move_grip_2d::MoveGrip2d, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::move_grip_2d::MoveGrip2d> {
    Ok((|| {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![crate::move_grip_2d::move_grip_2d(payload.id.clone(), existing.angle, existing.radius_2d)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
