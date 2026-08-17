//! 🔺️ `create-properties` — sparse diff construction, built directly from `(payload, base)`. A
//! `properties` slot already occupied in `base` is `mutation.duplicate-id` (Fatal, empty diff — a
//! true "create" never silently overwrites).

use super::mutation::CreateProperties;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateProperties, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.properties.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The object already has a properties child.".to_string(), ["properties".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { properties: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() })
}
//#endregion 🔖️Diff
