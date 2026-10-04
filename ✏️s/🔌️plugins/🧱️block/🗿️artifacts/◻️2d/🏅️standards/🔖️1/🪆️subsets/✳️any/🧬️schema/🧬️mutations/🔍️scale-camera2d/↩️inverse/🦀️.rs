//! ↩️ Inverse for `ScaleCamera2d`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ScaleCamera2d, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::scale_camera2d::scale_camera2d(base.camera2d.zoom)]

    })())
}
//#endregion 🔖️Inverse
