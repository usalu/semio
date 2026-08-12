//! 🔺️ `create-brep` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::CreateBrep;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateBrep, _base: &SemioObjectSnapshot) -> SemioObjectDiff {
    SemioObjectDiff { brep: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() }
}
//#endregion 🔖️Diff
