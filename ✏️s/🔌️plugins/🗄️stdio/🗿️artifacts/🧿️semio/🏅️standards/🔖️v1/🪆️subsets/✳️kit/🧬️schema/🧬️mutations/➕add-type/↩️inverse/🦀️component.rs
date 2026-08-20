//! ↩️ `add-type` — undo is `remove-type` for the just-added `id`.

use super::mutation::AddType;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{remove_type, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &AddType, _base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    vec![SemioKitMutation::RemoveType(remove_type::mutation::RemoveType { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
