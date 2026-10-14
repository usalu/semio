//! ↩️ Inverse for `MoveCamera2d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::move_camera2d::MoveCamera2d, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::move_camera2d::MoveCamera2d> {
    Ok((|| {
    vec![crate::move_camera2d::move_camera2d(base.camera2d.x, base.camera2d.y)]

    })())
}
//#endregion 🔖️Inverse
