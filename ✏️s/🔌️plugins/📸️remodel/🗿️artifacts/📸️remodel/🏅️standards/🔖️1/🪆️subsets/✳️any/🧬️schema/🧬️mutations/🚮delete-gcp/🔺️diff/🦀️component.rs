//! 🔺️ Sparse diff builder for `DeleteGcp`. Missing target ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelGcpList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteGcp, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.gcps.iter().any(|gcp| gcp.id == payload.id) {
        return RemodelDiff::default();
    }
    let gcps: Vec<_> = base.gcps.iter().filter(|gcp| gcp.id != payload.id).cloned().collect();
    RemodelDiff { gcps: Some(RemodelGcpList { values: gcps }), ..Default::default() }
}
//#endregion 🔖️Diff
