//! ↩️ `create-node` — undo is `delete-node` at the same id.

use super::mutation::CreateNode;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{delete_node, SemioGraphMutation};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateNode, _base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    vec![SemioGraphMutation::DeleteNode(delete_node::mutation::DeleteNode { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
