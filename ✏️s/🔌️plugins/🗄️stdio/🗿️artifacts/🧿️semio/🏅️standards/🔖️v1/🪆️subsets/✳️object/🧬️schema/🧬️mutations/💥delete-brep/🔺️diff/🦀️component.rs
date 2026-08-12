//! 🔺️ `delete-brep` — sparse diff construction: always clears `brep`, built directly from
//! `(payload, base)` (idempotent even when `base.brep` is already `None`).

use super::mutation::DeleteBrep;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteBrep, _base: &SemioObjectSnapshot) -> SemioObjectDiff {
    SemioObjectDiff { brep: Some(None), ..Default::default() }
}
//#endregion 🔖️Diff
