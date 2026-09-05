//! ↩️ Inverse for `CreateVertex`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, delete_vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepVertex, SemioBrepSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateVertex, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteVertex(delete_vertex::DeleteVertex { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
