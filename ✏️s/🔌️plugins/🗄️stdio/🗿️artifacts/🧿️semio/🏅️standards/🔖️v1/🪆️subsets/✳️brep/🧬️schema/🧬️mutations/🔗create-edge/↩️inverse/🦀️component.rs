//! ↩️ `create-edge` — undo is `deleteedge` (`delete_edge`) at the same id.

use super::mutation::CreateEdge;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{delete_edge, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &CreateEdge, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteEdge(delete_edge::mutation::DeleteEdge { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
