//! ↩️ Inverse for `CreateArtifact` — deleting the row it created.
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateArtifact, _base: &SSpaceSnapshot) -> Vec<SSpaceMutation> {
    vec![super::super::delete_artifact::mutation::delete_artifact(payload.artifact.id.clone())]
}
//#endregion 🔖️Inverse
