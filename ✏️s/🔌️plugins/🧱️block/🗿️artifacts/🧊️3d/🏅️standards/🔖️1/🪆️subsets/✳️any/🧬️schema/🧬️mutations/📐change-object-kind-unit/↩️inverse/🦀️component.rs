//! ↩️ Inverse for `ChangeObjectKindUnit` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangeObjectKindUnit, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_unit::mutation::change_object_kind_unit(base.object_kind.unit.clone())]
}
//#endregion 🔖️Inverse
