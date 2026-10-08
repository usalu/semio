//! 🔺️ Sparse diff builder for `CreateArtifact` — a real insert at `index` or append (never a whole-snapshot
//! capture).
use crate::standards::v1::subsets::any::schema::diff::{SSpaceArtifactPatch, SSpaceArtifactsDelta, SSpaceDiff};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    if base.artifacts.iter().any(|row| row.id == payload.artifact.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An artifact with id \"{}\" already exists.", payload.artifact.id), [payload.artifact.id.clone()]);
    }
    protocol::MutationOutcome::new(SSpaceDiff { artifacts: Some(SSpaceArtifactsDelta {
            added: vec![payload.artifact.clone()],
            reordered: payload.index.filter(|index| (*index as usize) < base.artifacts.len()).map(|index| {
                let mut order: Vec<String> = base.artifacts.iter().map(|row| row.id.clone()).collect();
                order.insert(index as usize, payload.artifact.id.clone());
                order
            }),
            ..Default::default()
        }), ..Default::default() })
}
//#endregion 🔖️Diff
