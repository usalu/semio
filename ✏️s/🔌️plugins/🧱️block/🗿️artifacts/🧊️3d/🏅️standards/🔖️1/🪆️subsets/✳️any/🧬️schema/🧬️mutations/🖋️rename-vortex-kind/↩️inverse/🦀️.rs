//! ↩️ Inverse for `RenameVortexKind`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RenameVortexKind, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::rename_vortex_kind::rename_vortex_kind(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
