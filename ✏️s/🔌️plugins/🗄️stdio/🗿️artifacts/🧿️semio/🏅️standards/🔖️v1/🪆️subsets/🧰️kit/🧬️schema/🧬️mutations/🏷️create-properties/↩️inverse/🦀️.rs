//! ↩️ Inverse for `CreateProperties`.

use crate::standards::v1::subsets::kit::schema::mutations::{delete_properties, SemioKitMutation};
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateProperties, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    if base.properties.is_some() || crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "value").is_err() {
        return Vec::new();
    }
    vec![SemioKitMutation::DeleteProperties(delete_properties::DeleteProperties {})]
}
//#endregion 🔖️Inverse
