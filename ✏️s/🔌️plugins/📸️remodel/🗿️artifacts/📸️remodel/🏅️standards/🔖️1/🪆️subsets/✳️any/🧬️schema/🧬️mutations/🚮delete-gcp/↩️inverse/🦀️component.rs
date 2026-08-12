//! ↩️ Inverse for `DeleteGcp` — recreates the captured BASE record. Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteGcp, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.gcps.iter().find(|gcp| gcp.id == payload.id) {
        Some(gcp) => vec![crate::artifacts::remodel::mutations::create_gcp::mutation::create_gcp(gcp.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
