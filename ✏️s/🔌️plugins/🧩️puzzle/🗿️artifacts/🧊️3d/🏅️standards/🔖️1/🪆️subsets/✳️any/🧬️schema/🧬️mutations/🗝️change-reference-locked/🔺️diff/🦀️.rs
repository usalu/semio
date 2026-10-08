//! 🔺️ Sparse diff builder for `ChangeReferenceLocked` — patches the one addressed reference in place.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle3dDiff, Puzzle3dReferencePatch, Puzzle3dReferencesDelta};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeReferenceLocked, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "reference", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle3dReferencePatch {
        locked: (payload.new_locked != item.locked).then_some(payload.new_locked),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        references: Some(Puzzle3dReferencesDelta::patching(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
