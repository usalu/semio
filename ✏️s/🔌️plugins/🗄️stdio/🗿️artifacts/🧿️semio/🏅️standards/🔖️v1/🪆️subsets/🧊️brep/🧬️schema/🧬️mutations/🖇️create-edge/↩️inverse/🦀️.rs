//! ↩️ Inverse for `CreateEdge`.

use crate::standards::v1::subsets::brep::schema::mutations::{delete_edge, SemioBrepMutation};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateEdge, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteEdge(delete_edge::DeleteEdge { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
