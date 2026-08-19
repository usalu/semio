//! ↩️ Inverse for `RenameArtifact` — the OLD name looked up from BASE.
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RenameArtifact, base: &SSpaceSnapshot) -> Vec<SSpaceMutation> {
    base.artifacts.iter().find(|row| row.id == payload.id).map(|row| vec![super::super::rename_artifact::mutation::rename_artifact(payload.id.clone(), row.name.clone())]).unwrap_or_default()
}
//#endregion 🔖️Inverse
