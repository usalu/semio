//! ↩️ Inverse for `DeleteArtifact` — re-creating the exact row it removed at its original index, looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteArtifact, base: &SSpaceSnapshot) -> Result<Vec<SSpaceMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.artifacts.iter().position(|row| row.id == payload.id).map(|at| vec![super::super::create_artifact::create_artifact_at(base.artifacts[at].clone(), at as u32)]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
