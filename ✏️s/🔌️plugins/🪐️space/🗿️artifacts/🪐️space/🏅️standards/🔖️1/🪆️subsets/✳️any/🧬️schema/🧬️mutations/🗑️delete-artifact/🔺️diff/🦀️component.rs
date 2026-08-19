//! 🔺️ Sparse diff builder for `DeleteArtifact` — a real filtered removal.
use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    if !base.artifacts.iter().any(|row| row.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Artifact \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let artifacts: Vec<_> = base.artifacts.iter().filter(|row| row.id != payload.id).cloned().collect();
    protocol::MutationOutcome::new(SSpaceDiff { artifacts: Some(artifacts), ..Default::default() })
}
//#endregion 🔖️Diff
