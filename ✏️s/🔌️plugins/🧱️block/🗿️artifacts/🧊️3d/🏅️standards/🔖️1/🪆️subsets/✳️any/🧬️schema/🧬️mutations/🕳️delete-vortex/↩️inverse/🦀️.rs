//! ↩️ Inverse for `DeleteVortex`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteVortex, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_vortex::create_vortex(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
