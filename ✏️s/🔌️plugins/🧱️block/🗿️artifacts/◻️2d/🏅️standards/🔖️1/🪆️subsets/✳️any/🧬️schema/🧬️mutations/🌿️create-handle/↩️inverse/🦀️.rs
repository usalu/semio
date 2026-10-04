//! ↩️ Inverse for `CreateHandle`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateHandle, _base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::delete_handle::delete_handle(payload.handle.id.clone())]

    })())
}
//#endregion 🔖️Inverse
