//! 🔺️ Diff for `ResizeNode`.

use crate::standards::v1::subsets::base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

//#region 🔖️Diff
/// 🧮️ A negative or non-finite size is a Fatal `mutation.invariant` (the schema's hard bound); a node the graph lacks is
/// `mutation.target-missing`; the node's current size is `mutation.no-op`.
pub fn diff(payload: &super::ResizeNode, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let target = [payload.id.value.clone()];
    if !(payload.width.is_finite() && payload.height.is_finite() && payload.width >= 0.0 && payload.height >= 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" size {} × {} is not a finite non-negative size.", payload.id.value, payload.width, payload.height), target);
    }
    let Some(node) = base.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), target);
    };
    if (node.width, node.height) == (payload.width, payload.height) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" is already {} × {}.", payload.id.value, payload.width, payload.height));
    }
    let at = base.nodes.iter().position(|node| node.id == payload.id).expect("checked above");
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioGraphNodeDiff { width: Some(payload.width), height: Some(payload.height), ..Default::default() } }], ..Default::default() }), edges: None })
}
//#endregion 🔖️Diff
