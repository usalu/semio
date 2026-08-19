//! ↩️ Inverse for `DeleteArtifact` — re-creating the exact row it removed, looked up from BASE.
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DeleteArtifact, base: &SSpaceSnapshot) -> Vec<SSpaceMutation> {
    base.artifacts.iter().find(|row| row.id == payload.id).map(|row| vec![super::super::create_artifact::mutation::create_artifact(row.clone())]).unwrap_or_default()
}
//#endregion 🔖️Inverse
