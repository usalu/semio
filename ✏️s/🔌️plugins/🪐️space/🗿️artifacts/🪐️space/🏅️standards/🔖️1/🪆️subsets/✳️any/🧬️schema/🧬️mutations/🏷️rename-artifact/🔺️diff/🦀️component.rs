//! 🔺️ Sparse diff builder for `RenameArtifact` — target-missing ⇒ Error, same name ⇒ no-op Warning
//! with an empty diff, name collision with a DIFFERENT id ⇒ Fatal duplicate-id.
use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::RenameArtifact, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
    let Some(existing) = base.artifacts.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Artifact \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Artifact \"{}\" already has that name.", payload.id));
    }
    if base.artifacts.iter().any(|row| row.id != payload.id && row.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An artifact named \"{}\" already exists.", payload.new_name), [payload.new_name.clone()]);
    }
    let artifacts: Vec<_> = base
        .artifacts
        .iter()
        .cloned()
        .map(|mut row| {
            if row.id == payload.id {
                row.name = payload.new_name.clone();
            }
            row
        })
        .collect();
    protocol::MutationOutcome::new(SSpaceDiff { artifacts: Some(artifacts), ..Default::default() })
}
//#endregion 🔖️Diff
