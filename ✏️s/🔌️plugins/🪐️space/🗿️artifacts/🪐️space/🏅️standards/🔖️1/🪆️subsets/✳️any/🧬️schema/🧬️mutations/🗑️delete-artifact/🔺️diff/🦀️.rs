//! 🔺️ Sparse diff builder for `DeleteArtifact` — a real filtered removal.
use crate::standards::v1::subsets::any::schema::diff::{SSpaceArtifactPatch, SSpaceArtifactsDelta, SSpaceDiff};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    if !base.artifacts.iter().any(|row| row.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Artifact \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SSpaceDiff { artifacts: Some(SSpaceArtifactsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
