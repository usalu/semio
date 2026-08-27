//! 🔺️ Sparse diff builder for `TouchArtifact` — target-missing ⇒ Error; otherwise a real
//! timestamp/author stamp (never a no-op check — repeated touches with the same values are legal).
use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::TouchArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    if !base.artifacts.iter().any(|row| row.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Artifact \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let artifacts: Vec<_> = base
        .artifacts
        .iter()
        .cloned()
        .map(|mut row| {
            if row.id == payload.id {
                row.updated_at_ms = payload.updated_at_ms;
                row.updated_by = payload.updated_by.clone();
            }
            row
        })
        .collect();
    protocol::MutationOutcome::new(SSpaceDiff { artifacts: Some(artifacts), ..Default::default() })
}
//#endregion 🔖️Diff
