//! 🔺️ Diff for `ChangeNodeLabel`.

use crate::standards::v1::subsets::base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ChangeNodeLabel, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    };
    if node.label == payload.new_label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" label is already \"{}\".", payload.id.value, payload.new_label));
    }
    let at = base.nodes.iter().position(|n| n.id == payload.id).expect("checked above");
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioGraphNodeDiff { label: Some(payload.new_label.clone()), ..Default::default() } }], ..Default::default() }), edges: None })
}
//#endregion 🔖️Diff
