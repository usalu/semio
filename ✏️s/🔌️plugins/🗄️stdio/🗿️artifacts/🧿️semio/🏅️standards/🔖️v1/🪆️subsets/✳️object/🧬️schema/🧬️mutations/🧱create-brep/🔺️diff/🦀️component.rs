//! 🔺️ `create-brep` — sparse diff construction, built directly from `(payload, base)`. A `brep`
//! slot already occupied in `base` is `mutation.duplicate-id` (Fatal, empty diff — a true "create"
//! never silently overwrites).

use super::mutation::CreateBrep;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateBrep, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.brep.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The object already has a brep child.".to_string(), ["brep".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { brep: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() })
}
//#endregion 🔖️Diff
