//! ↩️ Inverse for `MoveVortex` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::MoveVortex, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::move_vortex::mutation::move_vortex(payload.id.clone(), existing.position, existing.direction)], None => Vec::new() }
}
//#endregion 🔖️Inverse
