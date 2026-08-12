//! 🔺️ Sparse diff builder for `RemoveGcpObservation`. Missing target or out-of-range index ⇒
//! `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelGcpList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveGcpObservation, base: &RemodelSnapshot) -> RemodelDiff {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return RemodelDiff::default();
    };
    if payload.observation_index as usize >= gcp.observations.len() {
        return RemodelDiff::default();
    }
    let mut gcps = base.gcps.clone();
    if let Some(gcp) = gcps.iter_mut().find(|gcp| gcp.id == payload.id) {
        gcp.observations.remove(payload.observation_index as usize);
    }
    RemodelDiff { gcps: Some(RemodelGcpList { values: gcps }), ..Default::default() }
}
//#endregion 🔖️Diff
