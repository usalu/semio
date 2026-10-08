//! ↩️ Inverse for `DeleteVortexKind`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteVortexKind, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::vortex_kinds_of(base).iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((position, existing)) => vec![super::super::create_vortex_kind::create_vortex_kind_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
