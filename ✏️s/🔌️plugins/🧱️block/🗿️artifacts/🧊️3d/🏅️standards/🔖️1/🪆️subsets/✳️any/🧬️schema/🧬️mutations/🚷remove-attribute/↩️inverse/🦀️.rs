//! ↩️ Inverse for `RemoveAttribute`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveAttribute, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.attributes.iter().find(|item| item.key == payload.key) {
        Some(existing) => vec![super::super::add_attribute::add_attribute(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
