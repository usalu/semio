//! 🔺️ Sparse diff builder for `ChangeTargetVolumeHidden` — patches the one addressed target-volume in place.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dTargetVolumePatch, Puzzle5dTargetVolumePatchEntry, Puzzle5dTargetVolumesDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeTargetVolumeHidden, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target-volume", payload.id), vec![payload.id.clone()]);
    };
    let mut next = item.clone();
    next.hidden = payload.new_hidden;
    if next == *item {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        target_volumes: Some(Puzzle5dTargetVolumesDelta { patched: vec![Puzzle5dTargetVolumePatchEntry { id: payload.id.clone(), patch: Puzzle5dTargetVolumePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
