//! 🔺️ Sparse diff builder for `AddGcpObservation`. Missing target ⇒ `RemodelDiff::default()` —
//! the idempotent-early-return idiom (`📋️forms`'s `➕add-step/🔺️diff` uses the same shape).
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelGcpList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddGcpObservation, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.gcps.iter().any(|gcp| gcp.id == payload.id) {
        return RemodelDiff::default();
    }
    let mut gcps = base.gcps.clone();
    if let Some(gcp) = gcps.iter_mut().find(|gcp| gcp.id == payload.id) {
        gcp.observations.push(payload.observation.clone());
    }
    RemodelDiff { gcps: Some(RemodelGcpList { values: gcps }), ..Default::default() }
}
//#endregion 🔖️Diff
