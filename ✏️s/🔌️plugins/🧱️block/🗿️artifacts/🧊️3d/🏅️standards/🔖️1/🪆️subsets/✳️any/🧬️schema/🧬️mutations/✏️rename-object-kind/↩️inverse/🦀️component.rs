//! ↩️ Inverse for `RenameObjectKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RenameObjectKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::rename_object_kind::mutation::rename_object_kind(base.object_kind.name.clone())]
}
//#endregion 🔖️Inverse
