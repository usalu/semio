//! 🔺️ Sparse diff builder for `EditObjectLabel` — patches the one addressed object in place.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dObjectPatch, Puzzle3dObjectPatchEntry, Puzzle3dObjectsDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::EditObjectLabel, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object", payload.id), vec![payload.id.clone()]);
    };
    let mut next = item.clone();
    next.label = payload.new_label.clone();
    if next == *item {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        objects: Some(Puzzle3dObjectsDelta { patched: vec![Puzzle3dObjectPatchEntry { id: payload.id.clone(), patch: Puzzle3dObjectPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
