//! ↩️ Inverse for `ChangeObjectKindDescription`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindDescription, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::change_object_kind_description::change_object_kind_description(base.object_kind.description.clone())]

    })())
}
//#endregion 🔖️Inverse
