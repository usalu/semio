//! 🔺️ Sparse diff builder for `ChangeNodeAnchor` — patches the one addressed node in place.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodesDelta};
use protocol::list_delta::RowPatch;
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeAnchor, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node", payload.id), vec![payload.id.to_string_owner()]);
    };
    let patch = Puzzle2dNodePatch {
        anchor: (payload.new_anchor != node.anchor).then_some(payload.new_anchor),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.to_string_owner()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta::modification(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
