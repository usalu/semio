//! 🔺️ Sparse diff builder for `CreateTargetVolume` — a real append-only insert. Fatal when the id
//! already exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dTargetVolumesDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateTargetVolume, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if base.target_volumes.iter().any(|entry| entry.id == payload.target_volume.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "target volume"), vec![payload.target_volume.id.clone()]);
    }
    let mut delta = Puzzle5dTargetVolumesDelta { added: vec![payload.target_volume.clone()], ..Default::default() };
    if let Some(index) = payload.index {
        let mut order: Vec<String> = base.target_volumes.iter().map(|entry| entry.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, payload.target_volume.id.clone());
        delta.reordered = Some(order);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff { target_volumes: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
