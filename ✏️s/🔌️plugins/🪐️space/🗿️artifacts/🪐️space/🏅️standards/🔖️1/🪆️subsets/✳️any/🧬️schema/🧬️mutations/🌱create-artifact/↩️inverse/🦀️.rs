//! ↩️ Inverse for `CreateArtifact` — deleting the row it created.
use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateArtifact, _base: &SSpaceSnapshot) -> Result<Vec<SSpaceMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::delete_artifact::delete_artifact(payload.artifact.id.clone())]

    })())
}
//#endregion 🔖️Inverse
