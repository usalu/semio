//! 🔺️ Sparse diff builder for `CreateArtifact` — a real append-only insert (never a whole-snapshot
//! capture).
use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    if base.artifacts.iter().any(|row| row.id == payload.artifact.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An artifact with id \"{}\" already exists.", payload.artifact.id), [payload.artifact.id.clone()]);
    }
    let mut artifacts = base.artifacts.clone();
    artifacts.push(payload.artifact.clone());
    protocol::MutationOutcome::new(SSpaceDiff { artifacts: Some(artifacts), ..Default::default() })
}
//#endregion 🔖️Diff
