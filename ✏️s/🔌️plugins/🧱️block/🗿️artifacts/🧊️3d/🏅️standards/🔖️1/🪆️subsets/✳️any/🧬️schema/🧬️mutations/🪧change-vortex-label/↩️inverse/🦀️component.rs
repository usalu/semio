//! ↩️ Inverse for `ChangeVortexLabel` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeVortexLabel, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_vortex_label::mutation::change_vortex_label(payload.id.clone(), existing.label.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
