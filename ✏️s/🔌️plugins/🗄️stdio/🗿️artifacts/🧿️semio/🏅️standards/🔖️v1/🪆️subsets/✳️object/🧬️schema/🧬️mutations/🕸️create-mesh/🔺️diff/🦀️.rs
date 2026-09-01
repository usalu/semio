//! 🔺️ Diff for `CreateMesh`.

use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateMesh, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.mesh.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The object already has a mesh child.".to_string(), ["mesh".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { mesh: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() })
}
//#endregion 🔖️Diff
