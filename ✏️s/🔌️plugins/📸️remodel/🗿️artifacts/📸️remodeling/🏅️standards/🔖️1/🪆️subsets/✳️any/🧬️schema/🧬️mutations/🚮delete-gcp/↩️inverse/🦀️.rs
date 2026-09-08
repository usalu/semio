//! ↩️ Inverse for `DeleteGcp` — recreates the captured BASE record, observations included (the GCP
//! OWNS them, so they travel inside the record), at its canonical `id` position.
//! Missing target ⇒ `Vec::new()`.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteGcp, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.gcps.iter().find(|gcp| gcp.id == payload.id) {
        Some(gcp) => vec![crate::mutations::create_gcp::create_gcp(gcp.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
