//! 🔺️ Sparse diff builder for `ResizeReference` — patches the one addressed reference in place.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dDiff, Puzzle3dReferencePatch, Puzzle3dReferencesDelta};
use protocol::list_delta::RowPatch;
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ResizeReference, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "reference", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle3dReferencePatch {
        width_world: (payload.new_width_world != item.width_world).then_some(payload.new_width_world),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        references: Some(Puzzle3dReferencesDelta::modification(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
