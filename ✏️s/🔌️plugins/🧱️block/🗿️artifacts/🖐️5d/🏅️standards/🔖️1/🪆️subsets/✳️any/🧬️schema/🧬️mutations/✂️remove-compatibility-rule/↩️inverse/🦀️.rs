//! ↩️ Inverse for `RemoveCompatibilityRule`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveCompatibilityRule, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.compatibility.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((position, existing)) => vec![super::super::add_compatibility_rule::add_compatibility_rule_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
