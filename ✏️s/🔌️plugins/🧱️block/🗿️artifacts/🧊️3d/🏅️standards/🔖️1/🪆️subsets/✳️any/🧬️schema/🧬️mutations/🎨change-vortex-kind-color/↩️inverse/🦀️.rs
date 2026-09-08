//! ↩️ Inverse for `ChangeVortexKindColor`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeVortexKindColor, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match crate::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_vortex_kind_color::change_vortex_kind_color(payload.id.clone(), existing.color.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
