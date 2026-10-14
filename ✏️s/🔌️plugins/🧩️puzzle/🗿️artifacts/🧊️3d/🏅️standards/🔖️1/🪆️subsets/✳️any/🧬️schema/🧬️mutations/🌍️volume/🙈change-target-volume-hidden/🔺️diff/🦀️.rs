//! 🔺️ Sparse diff builder for `ChangeTargetVolumeHidden` — patches the one addressed target-volume in place.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dDiff, Puzzle3dTargetVolumePatch, Puzzle3dTargetVolumesDelta};
use protocol::list_delta::RowPatch;
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeTargetVolumeHidden, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target-volume", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle3dTargetVolumePatch {
        hidden: (payload.new_hidden != item.hidden).then_some(payload.new_hidden),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        target_volumes: Some(Puzzle3dTargetVolumesDelta::modification(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
