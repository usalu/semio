//! 🔺️ Sparse diff builder for `CreateArtifact` — a real insert at `index` or append (never a whole-snapshot
//! capture).
use crate::standards::v1::subsets::any::schema::diff::{SSpaceArtifactPatch, SSpaceArtifactsDelta, SSpaceDiff};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    if base.artifacts.iter().any(|row| row.id == payload.artifact.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An artifact with id \"{}\" already exists.", payload.artifact.id), [payload.artifact.id.clone()]);
    }
    let at = payload.index.map_or(base.artifacts.len(), |index| (index as usize).min(base.artifacts.len()));
    protocol::MutationOutcome::new(SSpaceDiff { artifacts: Some(SSpaceArtifactsDelta::insertion(at, payload.artifact.clone())), ..Default::default() })
}
//#endregion 🔖️Diff
