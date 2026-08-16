//! 🔺️ Sparse diff builder for `CreateTargetVolume` — a real append-only insert. No-op when the id already
//! exists in `base`.
use crate::artifacts::puzzle3d::diff::{Puzzle3dTargetVolumesDelta, Puzzle3dDiff};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateTargetVolume, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.target_volumes.iter().any(|entry| entry.id == payload.target_volume.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "target volume"), vec![payload.target_volume.id.clone()]);
    }
    let mut delta = Puzzle3dTargetVolumesDelta { added: vec![payload.target_volume.clone()], ..Default::default() };
    protocol::MutationOutcome::new(if let Some(index) = payload.index {
        let mut order: Vec<String> = base.target_volumes.iter().map(|entry| entry.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, payload.target_volume.id.clone());
        delta.reordered = Some(order);
    }
    Puzzle3dDiff { target_volumes: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
