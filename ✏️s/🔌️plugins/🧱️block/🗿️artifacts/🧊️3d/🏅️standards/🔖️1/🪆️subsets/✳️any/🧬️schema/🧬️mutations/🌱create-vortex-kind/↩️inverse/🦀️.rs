//! ↩️ Inverse for `CreateVortexKind`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateVortexKind, _base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::delete_vortex_kind::delete_vortex_kind(payload.vortex_kind.id.clone())]

    })())
}
//#endregion 🔖️Inverse
