//! ↩️ Inverse for `ChangeVortexKindColor` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeVortexKindColor, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match crate::artifacts::block3d::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_vortex_kind_color::mutation::change_vortex_kind_color(payload.id.clone(), existing.color.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
