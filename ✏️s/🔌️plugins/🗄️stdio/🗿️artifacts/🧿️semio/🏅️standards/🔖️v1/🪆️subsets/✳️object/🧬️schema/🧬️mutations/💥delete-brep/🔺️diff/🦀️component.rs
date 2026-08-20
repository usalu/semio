//! 🔺️ `delete-brep` — sparse diff construction: clears `brep`, built directly from `(payload,
//! base)`. An already-empty `brep` slot is `mutation.target-missing` (Error, empty diff — nothing
//! to delete).

use super::mutation::DeleteBrep;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(_payload: &DeleteBrep, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    if base.brep.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The object has no brep to delete.".to_string(), ["brep".to_string()]);
    }
    protocol::MutationOutcome::new(SemioObjectDiff { brep: Some(None), ..Default::default() })
}
//#endregion 🔖️Diff
