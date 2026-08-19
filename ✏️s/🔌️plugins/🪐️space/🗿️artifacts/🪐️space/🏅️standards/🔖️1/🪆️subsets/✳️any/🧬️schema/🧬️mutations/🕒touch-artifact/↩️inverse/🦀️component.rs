//! ↩️ Inverse for `TouchArtifact` — the OLD `updatedAtMs`/`updatedBy` looked up from BASE.
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::TouchArtifact, base: &SSpaceSnapshot) -> Vec<SSpaceMutation> {
    base.artifacts
        .iter()
        .find(|row| row.id == payload.id)
        .map(|row| vec![super::super::touch_artifact::mutation::touch_artifact(payload.id.clone(), row.updated_at_ms, row.updated_by.clone())])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
