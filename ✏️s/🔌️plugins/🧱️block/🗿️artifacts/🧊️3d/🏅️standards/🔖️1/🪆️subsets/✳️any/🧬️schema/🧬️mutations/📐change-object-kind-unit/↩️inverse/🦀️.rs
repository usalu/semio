//! ↩️ Inverse for `ChangeObjectKindUnit`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindUnit, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_unit::change_object_kind_unit(base.object_kind.unit.clone())]
}
//#endregion 🔖️Inverse
