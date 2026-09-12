//! ↩️ Inverse for `CreateObject`.

use crate::standards::v1::subsets::kit::schema::mutations::{delete_object, SemioKitMutation};
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateObject, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    if base.objects.iter().any(|object| object.child_id == payload.child_id) || crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "object").is_err() {
        return Vec::new();
    }
    vec![SemioKitMutation::DeleteObject(delete_object::DeleteObject { child_id: payload.child_id.clone() })]
}
//#endregion 🔖️Inverse
