//! 🔺️ Diff for `CreateProperties`.

use crate::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateProperties, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.properties.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The object already has a properties child.".to_string(), ["properties".to_string()]);
    }
    if let Err(message) = crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "value") {
        return protocol::MutationOutcome::fatal("mutation.child-identity", message, ["properties".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { properties: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() })
}
//#endregion 🔖️Diff
