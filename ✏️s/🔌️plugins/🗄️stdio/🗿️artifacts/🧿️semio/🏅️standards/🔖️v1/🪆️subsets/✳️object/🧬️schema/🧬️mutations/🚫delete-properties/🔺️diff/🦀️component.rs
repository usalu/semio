//! 🔺️ `delete-properties` — sparse diff construction: always clears `properties`.

use super::mutation::DeleteProperties;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteProperties, _base: &SemioObjectSnapshot) -> SemioObjectDiff {
    SemioObjectDiff { properties: Some(None), ..Default::default() }
}
//#endregion 🔖️Diff
