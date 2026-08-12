//! 🔺️ `delete-node` — sparse diff construction; CASCADES: removes the node with this `id` from
//! `nodes`, AND removes every edge in `edges` whose `source == id` or `target == id`. Absent node
//! is a no-op clone (never apply-then-capture).

use super::mutation::DeleteNode;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut nodes = base.nodes.clone();
    nodes.retain(|n| n.id != payload.id);
    let mut edges = base.edges.clone();
    edges.retain(|e| e.source != payload.id && e.target != payload.id);
    SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: Some(SemioGraphEdgeList { values: edges }) }
}
//#endregion 🔖️Diff
