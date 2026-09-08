//! ↩️ Inverse for `ChangeObjectKindUnit`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindUnit, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_unit::change_object_kind_unit(base.object_kind.unit.clone())]
}
//#endregion 🔖️Inverse
