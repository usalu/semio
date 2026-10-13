//! 🔺️ Diff for `CreateMesh`.

use crate::schema::diff::SemioObjectDiff;
use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateMesh, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.mesh.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The object already has a mesh child.".to_string(), ["mesh".to_string()]);
    }
    if let Err(message) = semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "mesh") {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, ["mesh".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { mesh: Some(Some(store::os_store::artifact_child::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() })
}
//#endregion 🔖️Diff
