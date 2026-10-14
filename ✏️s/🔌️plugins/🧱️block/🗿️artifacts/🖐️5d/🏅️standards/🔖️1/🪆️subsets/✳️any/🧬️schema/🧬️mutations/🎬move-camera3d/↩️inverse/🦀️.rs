//! ↩️ Inverse for `MoveCamera3d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::move_camera3d::MoveCamera3d, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::move_camera3d::MoveCamera3d> {
    Ok((|| {
    vec![crate::move_camera3d::move_camera3d(base.camera3d.position, base.camera3d.target)]

    })())
}
//#endregion 🔖️Inverse
