//! ↩️ Inverse for `CreateVortex`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateVortex, _base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::delete_vortex::delete_vortex(payload.vortex.id.clone())]

    })())
}
//#endregion 🔖️Inverse
