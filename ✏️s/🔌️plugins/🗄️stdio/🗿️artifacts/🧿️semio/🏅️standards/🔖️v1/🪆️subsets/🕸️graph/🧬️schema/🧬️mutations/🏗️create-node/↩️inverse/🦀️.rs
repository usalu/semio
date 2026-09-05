//! ↩️ Inverse for `CreateNode`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, delete_node};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphNode, SemioGraphPort, SemioGraphSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateNode, _base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    vec![SemioGraphMutation::DeleteNode(delete_node::DeleteNode { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
