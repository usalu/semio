//! ↩️ Inverse for `ChangeObjectKindLabel`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindLabel, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::change_object_kind_label::change_object_kind_label(base.object_kind.label.clone())]

    })())
}
//#endregion 🔖️Inverse
