//! 🔺️ Diff for `CreateBrep`.

use crate::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateBrep, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.brep.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The object already has a brep child.".to_string(), ["brep".to_string()]);
    }
    if let Err(message) = crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "brep") {
        return protocol::MutationOutcome::fatal("mutation.child-identity", message, ["brep".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { brep: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() })
}
//#endregion 🔖️Diff
