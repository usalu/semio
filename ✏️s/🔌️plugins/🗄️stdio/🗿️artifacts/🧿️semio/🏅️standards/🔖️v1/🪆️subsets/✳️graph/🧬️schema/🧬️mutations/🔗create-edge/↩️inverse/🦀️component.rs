//! ↩️ `create-edge` — undo is `delete-edge` at the same id.

use super::mutation::CreateEdge;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{delete_edge, SemioGraphMutation};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &CreateEdge, _base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    vec![SemioGraphMutation::DeleteEdge(delete_edge::mutation::DeleteEdge { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
