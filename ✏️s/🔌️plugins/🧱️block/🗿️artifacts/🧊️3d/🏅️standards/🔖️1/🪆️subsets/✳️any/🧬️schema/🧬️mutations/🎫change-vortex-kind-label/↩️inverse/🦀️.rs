//! ↩️ Inverse for `ChangeVortexKindLabel`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeVortexKindLabel, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_vortex_kind_label::change_vortex_kind_label(payload.id.clone(), existing.label.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
