//! ↩️ Inverse for `CreateGcp` — `delete-gcp` of the id it created. A duplicate create was a no-op,
//! so its inverse is too.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateGcp, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    if base.gcps.iter().any(|gcp| gcp.id == payload.gcp.id) {
        return Vec::new();
    }
    vec![crate::artifacts::remodel::mutations::delete_gcp::mutation::delete_gcp(payload.gcp.id.clone())]
}
//#endregion 🔖️Inverse
