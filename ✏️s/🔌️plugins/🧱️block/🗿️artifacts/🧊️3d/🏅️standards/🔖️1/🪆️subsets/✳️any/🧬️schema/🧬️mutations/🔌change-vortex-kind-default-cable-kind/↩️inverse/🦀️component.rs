//! ↩️ Inverse for `ChangeVortexKindDefaultCableKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeVortexKindDefaultCableKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortex_kinds.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_vortex_kind_default_cable_kind::mutation::change_vortex_kind_default_cable_kind(payload.id.clone(), existing.default_cable_kind.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
