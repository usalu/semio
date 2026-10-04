//! ↩️ Inverse for `ChangeObjectKindUnit`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindUnit, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::change_object_kind_unit::change_object_kind_unit(base.object_kind.unit.clone())]

    })())
}
//#endregion 🔖️Inverse
