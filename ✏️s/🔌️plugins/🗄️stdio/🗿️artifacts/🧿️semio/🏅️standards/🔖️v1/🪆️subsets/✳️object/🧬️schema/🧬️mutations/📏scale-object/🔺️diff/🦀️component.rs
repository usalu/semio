//! 🔺️ `scale-object` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::ScaleObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ScaleObject, base: &SemioObjectSnapshot) -> SemioObjectDiff {
    let mut transform = base.transform.clone();
    transform.scale = payload.scale;
    SemioObjectDiff { transform: Some(transform), ..Default::default() }
}
//#endregion 🔖️Diff
