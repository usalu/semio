//! ↩️ Inverse for `ChangeObjectKindIcon`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindIcon, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::change_object_kind_icon::change_object_kind_icon(base.object_kind.icon.clone())]

    })())
}
//#endregion 🔖️Inverse
