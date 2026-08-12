//! 🔺️ `rotate-object` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::RotateObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RotateObject, base: &SemioObjectSnapshot) -> SemioObjectDiff {
    let mut transform = base.transform.clone();
    transform.rotation = payload.rotation;
    SemioObjectDiff { transform: Some(transform), ..Default::default() }
}
//#endregion 🔖️Diff
