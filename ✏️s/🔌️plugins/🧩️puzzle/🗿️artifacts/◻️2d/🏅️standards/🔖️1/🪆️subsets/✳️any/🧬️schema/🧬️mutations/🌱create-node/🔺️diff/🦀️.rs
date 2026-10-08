//! 🔺️ Sparse diff builder for `CreateNode` — a real append-only insert (never a whole-snapshot
//! capture). No-op when the id already exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_node_invariant;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateNode, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_node_invariant(&payload.node) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.node.id.to_string_owner()]);
    }
    if base.nodes.iter().any(|entry| entry.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "node"), vec![payload.node.id.to_string_owner()]);
    }
    let index = payload.index.map_or(base.nodes.len(), |index| index.min(base.nodes.len()));
    let delta = Puzzle2dNodesDelta::insertion(index, payload.node.clone());
    protocol::MutationOutcome::new(Puzzle2dDiff { nodes: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
