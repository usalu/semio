//! ↩️ Inverse for `MoveCamera3d`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::MoveCamera3d, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::move_camera3d::move_camera3d(base.camera3d.position, base.camera3d.target)]

    })())
}
//#endregion 🔖️Inverse
