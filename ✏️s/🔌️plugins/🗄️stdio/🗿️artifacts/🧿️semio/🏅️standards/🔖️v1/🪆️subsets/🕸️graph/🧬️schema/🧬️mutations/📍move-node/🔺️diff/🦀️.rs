//! 🔺️ Diff for `MoveNode`.

use crate::standards::v1::subsets::base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::MoveNode, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    };
    if !payload.new_position.x.is_finite() || !payload.new_position.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" target position ({}, {}) is not finite.", payload.id.value, payload.new_position.x, payload.new_position.y), [payload.id.value.clone()]);
    }
    if node.position == payload.new_position {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" is already at ({}, {}).", payload.id.value, payload.new_position.x, payload.new_position.y));
    }
    let at = base.nodes.iter().position(|n| n.id == payload.id).expect("checked above");
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioGraphNodeDiff { position: Some(payload.new_position), ..Default::default() } }], ..Default::default() }), edges: None })
}
//#endregion 🔖️Diff
