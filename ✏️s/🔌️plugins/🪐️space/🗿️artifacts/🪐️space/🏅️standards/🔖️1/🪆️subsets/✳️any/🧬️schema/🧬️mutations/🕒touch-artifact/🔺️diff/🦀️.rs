//! 🔺️ Sparse diff builder for `TouchArtifact` — target-missing ⇒ Error; otherwise a real
//! timestamp/author stamp (never a no-op check — repeated touches with the same values are legal).
use crate::standards::v1::subsets::any::schema::diff::{SSpaceArtifactPatch, SSpaceArtifactsDelta, SSpaceDiff};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::TouchArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    if !base.artifacts.iter().any(|row| row.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Artifact \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SSpaceDiff {
        artifacts: Some(SSpaceArtifactsDelta { patched: vec![SSpaceArtifactPatch { id: payload.id.clone(), updated_at_ms: Some(payload.updated_at_ms), updated_by: Some(payload.updated_by.clone()), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
