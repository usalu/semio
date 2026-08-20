//! ↩️ `create-object` — undo is `delete-object` for the just-minted `child_id`.

use super::mutation::CreateObject;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{delete_object, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &CreateObject, _base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    vec![SemioKitMutation::DeleteObject(delete_object::mutation::DeleteObject { child_id: payload.child_id.clone() })]
}
//#endregion 🔖️Inverse
