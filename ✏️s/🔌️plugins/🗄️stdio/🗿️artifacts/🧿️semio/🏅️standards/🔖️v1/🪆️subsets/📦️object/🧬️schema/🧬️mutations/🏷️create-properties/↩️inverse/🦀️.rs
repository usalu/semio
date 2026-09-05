//! ↩️ Inverse for `CreateProperties`.

use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, delete_properties};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &super::CreateProperties, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    match &base.properties {
        Some(existing) => vec![SemioObjectMutation::CreateProperties(super::CreateProperties { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => vec![SemioObjectMutation::DeleteProperties(delete_properties::DeleteProperties {})],
    }
}
//#endregion 🔖️Inverse
