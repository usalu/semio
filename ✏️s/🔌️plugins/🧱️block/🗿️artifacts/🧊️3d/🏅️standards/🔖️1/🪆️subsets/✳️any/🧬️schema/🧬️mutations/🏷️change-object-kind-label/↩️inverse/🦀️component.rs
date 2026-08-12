//! ↩️ Inverse for `ChangeObjectKindLabel` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeObjectKindLabel, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_label::mutation::change_object_kind_label(base.object_kind.label.clone())]
}
//#endregion 🔖️Inverse
