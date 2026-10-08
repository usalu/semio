//! 🔺️ Sparse diff builder for `RenameArtifact` — target-missing ⇒ Error, same name ⇒ no-op Warning
//! with an empty diff, name collision with a DIFFERENT id ⇒ Fatal duplicate-id.
use crate::standards::v1::subsets::any::schema::diff::{SSpaceArtifactPatch, SSpaceArtifactsDelta, SSpaceDiff};
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    let Some(existing) = base.artifacts.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Artifact \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Artifact \"{}\" already has that name.", payload.id));
    }
    if base.artifacts.iter().any(|row| row.id != payload.id && row.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An artifact named \"{}\" already exists.", payload.new_name), [payload.new_name.clone()]);
    }
    protocol::MutationOutcome::new(SSpaceDiff { artifacts: Some(SSpaceArtifactsDelta { patched: vec![SSpaceArtifactPatch { id: payload.id.clone(), name: Some(payload.new_name.clone()), ..Default::default() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
