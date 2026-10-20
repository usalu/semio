//! ↩️ Inverse for `DeleteGcp` — recreates the captured BASE record, observations included (the GCP
//! OWNS them, so they travel inside the record), at its canonical `id` position.
//! Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::create_gcp::CreateGcp;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateGcp>>(payload: &crate::standards::v1::subsets::any::schema::mutations::delete_gcp::component::DeleteGcp, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.gcps.iter().find(|gcp| gcp.id == payload.id) {
        Some(gcp) => vec![crate::standards::v1::subsets::any::schema::mutations::create_gcp::create_gcp(gcp.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
