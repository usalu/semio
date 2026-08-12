//! 🔺️ Sparse diff builder for `CreateGcp`. Duplicate `gcp.id` ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelGcpList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateGcp, base: &RemodelSnapshot) -> RemodelDiff {
    if base.gcps.iter().any(|gcp| gcp.id == payload.gcp.id) {
        return RemodelDiff::default();
    }
    let mut gcps = base.gcps.clone();
    gcps.push(payload.gcp.clone());
    RemodelDiff { gcps: Some(RemodelGcpList { values: gcps }), ..Default::default() }
}
//#endregion 🔖️Diff
