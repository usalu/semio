//! 🔺️ `change-node-kind` — sparse diff construction; Error `mutation.target-missing` when the
//! BASE `id` is absent, Warning `mutation.no-op` when `new_kind` already equals the current kind.

use super::mutation::ChangeNodeKind;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNodeKind, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    };
    if node.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" kind is already \"{}\".", payload.id.value, payload.new_kind));
    }
    let mut nodes = base.nodes.clone();
    let node = nodes.iter_mut().find(|n| n.id == payload.id).expect("checked above");
    node.kind = payload.new_kind.clone();
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
