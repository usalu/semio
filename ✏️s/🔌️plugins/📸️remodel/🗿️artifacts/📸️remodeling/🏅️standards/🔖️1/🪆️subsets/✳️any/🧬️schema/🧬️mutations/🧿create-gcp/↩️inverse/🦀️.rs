//! ↩️ Inverse for `CreateGcp` — `delete-gcp` of the id it created. A duplicate create was a no-op,
//! so its inverse is too.
use crate::standards::v1::subsets::any::schema::mutations::delete_gcp::DeleteGcp;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<DeleteGcp>>(payload: &crate::standards::v1::subsets::any::schema::mutations::create_gcp::component::CreateGcp, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.gcps.iter().any(|gcp| gcp.id == payload.gcp.id) {
        return Vec::new();
    }
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_gcp::delete_gcp(payload.gcp.id.clone())]

    })())
}
//#endregion 🔖️Inverse
