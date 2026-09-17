//! 🔺️ Sparse diff builder for `DeleteTargetVolume` — a real removal, never a whole-snapshot capture.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dTargetVolumesDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteTargetVolume, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !base.target_volumes.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target volume", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff { target_volumes: Some(Puzzle5dTargetVolumesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
