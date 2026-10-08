//! 🔺️ Sparse diff builder for `CreateTargetVolume` — a real append-only insert. Fatal when the id
//! already exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dTargetVolumesDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateTargetVolume, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if base.target_volumes.iter().any(|entry| entry.id == payload.target_volume.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "target volume"), vec![payload.target_volume.id.clone()]);
    }
    let index = payload.index.map_or(base.target_volumes.len(), |index| index.min(base.target_volumes.len()));
    let delta = Puzzle5dTargetVolumesDelta::insertion(index, payload.target_volume.clone());
    protocol::MutationOutcome::new(Puzzle5dDiff { target_volumes: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
