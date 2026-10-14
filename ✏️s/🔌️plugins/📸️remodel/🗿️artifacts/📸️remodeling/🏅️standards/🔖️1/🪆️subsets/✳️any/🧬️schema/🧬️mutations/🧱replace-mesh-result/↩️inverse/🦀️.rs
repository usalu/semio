//! ↩️ Inverse for `ReplaceMeshResult` — the OLD `RemodelingMesh` from BASE, boxed.
use crate::standards::v1::subsets::any::schema::mutations::replace_mesh_result::ReplaceMeshResult;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceMeshResult>>(_payload: &super::ReplaceMeshResult, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::replace_mesh_result(Box::new(base.results.mesh.clone()))]

    })())
}
//#endregion 🔖️Inverse
