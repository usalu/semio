//! ↩️ Inverse for `DeleteGcp` — recreates the captured BASE record. Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteGcp, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.gcps.iter().find(|gcp| gcp.id == payload.id) {
        Some(gcp) => vec![crate::artifacts::remodeling::mutations::create_gcp::create_gcp(gcp.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
