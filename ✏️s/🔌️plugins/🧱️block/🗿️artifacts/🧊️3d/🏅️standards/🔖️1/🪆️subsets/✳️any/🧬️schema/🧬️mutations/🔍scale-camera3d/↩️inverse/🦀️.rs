//! ↩️ Inverse for `ScaleCamera3d`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::scale_camera3d::ScaleCamera3d, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::scale_camera3d::ScaleCamera3d> {
    Ok((|| {
    vec![crate::scale_camera3d::scale_camera3d(base.camera3d.zoom)]

    })())
}
//#endregion 🔖️Inverse
