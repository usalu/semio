//! 🔺️ `move-object` — sparse diff construction: sets `transform` to base's rotation/scale plus
//! the payload's new translation, built directly from `(payload, base)`.

use super::mutation::MoveObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveObject, base: &SemioObjectSnapshot) -> SemioObjectDiff {
    let mut transform = base.transform.clone();
    transform.translation = payload.translation;
    SemioObjectDiff { transform: Some(transform), ..Default::default() }
}
//#endregion 🔖️Diff
