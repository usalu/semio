//! ↩️ Inverse for `CreateGcp` — `delete-gcp` of the id it created. A duplicate create was a no-op,
//! so its inverse is too.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateGcp, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    if base.gcps.iter().any(|gcp| gcp.id == payload.gcp.id) {
        return Vec::new();
    }
    vec![crate::mutations::delete_gcp::delete_gcp(payload.gcp.id.clone())]
}
//#endregion 🔖️Inverse
