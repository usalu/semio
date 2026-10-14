//! ↩️ Inverse for `UpdateMeshParams` — the OLD `MeshParams` looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::update_mesh_params::UpdateMeshParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateMeshParams>>(_payload: &super::UpdateMeshParams, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_mesh_params(base.params.mesh.clone())]

    })())
}
//#endregion 🔖️Inverse
