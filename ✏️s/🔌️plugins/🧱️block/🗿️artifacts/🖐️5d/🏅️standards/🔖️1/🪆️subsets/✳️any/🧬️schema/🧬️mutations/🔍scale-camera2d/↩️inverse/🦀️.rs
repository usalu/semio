//! ↩️ Inverse for `ScaleCamera2d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &crate::scale_camera2d::ScaleCamera2d, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::scale_camera2d::ScaleCamera2d> {
    Ok((|| {
    vec![crate::scale_camera2d::scale_camera2d(base.camera2d.zoom)]

    })())
}
//#endregion 🔖️Inverse
