//! 🔺️ Sparse diff builder for `ResizeReference` — patches the one addressed reference in place.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dReferencePatch, Puzzle3dReferencePatchEntry, Puzzle3dReferencesDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ResizeReference, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "reference", payload.id), vec![payload.id.clone()]);
    };
    let mut next = item.clone();
    next.width_world = payload.new_width_world;
    if next == *item {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        references: Some(Puzzle3dReferencesDelta { patched: vec![Puzzle3dReferencePatchEntry { id: payload.id.clone(), patch: Puzzle3dReferencePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
