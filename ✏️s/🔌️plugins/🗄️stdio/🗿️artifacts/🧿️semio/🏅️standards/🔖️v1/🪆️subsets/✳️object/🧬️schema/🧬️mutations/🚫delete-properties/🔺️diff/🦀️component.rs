//! 🔺️ `delete-properties` — sparse diff construction: clears `properties`. An already-empty
//! `properties` slot is `mutation.target-missing` (Error, empty diff — nothing to delete).

use super::mutation::DeleteProperties;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(_payload: &DeleteProperties, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.properties.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The object has no properties to delete.".to_string(), ["properties".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { properties: Some(None), ..Default::default() })
}
//#endregion 🔖️Diff
