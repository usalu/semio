//! 🔺️ `move-node` — sparse diff construction; Error `mutation.target-missing` when the BASE `id`
//! is absent, Fatal `mutation.invariant` on a non-finite target coordinate, Warning `mutation.no-op`
//! when the new position already equals the current one.

use super::mutation::MoveNode;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &MoveNode, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    };
    if !payload.new_position.x.is_finite() || !payload.new_position.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" target position ({}, {}) is not finite.", payload.id.value, payload.new_position.x, payload.new_position.y), [payload.id.value.clone()]);
    }
    if node.position == payload.new_position {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" is already at ({}, {}).", payload.id.value, payload.new_position.x, payload.new_position.y));
    }
    let mut nodes = base.nodes.clone();
    let node = nodes.iter_mut().find(|n| n.id == payload.id).expect("checked above");
    node.position = payload.new_position.clone();
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
