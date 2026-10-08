//! 🔺️ Sparse diff builder for `ChangeEdgeKind` — patches the one addressed edge in place.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle2dDiff, Puzzle2dEdgePatch, Puzzle2dEdgesDelta};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeEdgeKind, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "edge", payload.id), vec![payload.id.to_string_owner()]);
    };
    let patch = Puzzle2dEdgePatch {
        edge_kind: (payload.new_edge_kind != edge.edge_kind).then(|| payload.new_edge_kind.clone()),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.to_string_owner()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        edges: Some(Puzzle2dEdgesDelta::patching(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
