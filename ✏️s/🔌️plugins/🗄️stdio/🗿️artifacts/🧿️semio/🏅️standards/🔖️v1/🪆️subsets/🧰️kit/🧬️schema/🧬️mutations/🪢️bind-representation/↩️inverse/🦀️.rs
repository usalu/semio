//! ↩️ Inverse for `BindRepresentation`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation, unbind_representation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &super::BindRepresentation, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    vec![SemioKitMutation::UnbindRepresentation(unbind_representation::UnbindRepresentation { index: base.representations.len() })]
}
//#endregion 🔖️Inverse
