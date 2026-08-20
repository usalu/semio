//! 🔺️ `change-node-label` — sparse diff construction; Error `mutation.target-missing` when the
//! BASE `id` is absent, Warning `mutation.no-op` when `new_label` already equals the current label.

use super::mutation::ChangeNodeLabel;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &ChangeNodeLabel, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    };
    if node.label == payload.new_label {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Node \"{}\" label is already \"{}\".", payload.id.value, payload.new_label));
    }
    let mut nodes = base.nodes.clone();
    let node = nodes.iter_mut().find(|n| n.id == payload.id).expect("checked above");
    node.label = payload.new_label.clone();
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
