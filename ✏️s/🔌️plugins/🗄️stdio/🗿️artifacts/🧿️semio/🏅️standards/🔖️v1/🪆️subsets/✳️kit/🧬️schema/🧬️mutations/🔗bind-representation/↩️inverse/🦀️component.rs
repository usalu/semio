//! ↩️ `bind-representation` — undo is `unbind-representation` at the FINAL-state index the new
//! link landed at (the end, since `bind` always appends).

use super::mutation::BindRepresentation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{unbind_representation, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &BindRepresentation, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    vec![SemioKitMutation::UnbindRepresentation(unbind_representation::mutation::UnbindRepresentation { index: base.representations.len() })]
}
//#endregion 🔖️Inverse
