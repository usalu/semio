//! ↩️ Inverse for `RemoveCompatibilityRule`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveCompatibilityRule, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.compatibility.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::add_compatibility_rule::add_compatibility_rule(existing.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
