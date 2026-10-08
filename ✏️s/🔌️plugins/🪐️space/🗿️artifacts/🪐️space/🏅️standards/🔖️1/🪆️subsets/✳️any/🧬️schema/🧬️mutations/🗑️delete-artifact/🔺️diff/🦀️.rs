//! 🔺️ Sparse diff builder for `DeleteArtifact` — a real filtered removal.
use crate::standards::v1::subsets::any::schema::diff::{SSpaceArtifactPatch, SSpaceArtifactsDelta, SSpaceDiff};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    let Some(index) = base.artifacts.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Artifact \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(SSpaceDiff { artifacts: Some(SSpaceArtifactsDelta::removal(&base.artifacts, index)), ..Default::default() })
}
//#endregion 🔖️Diff
