//! 🔺️ `delete-properties` — sparse diff construction: always clears `properties`.

use super::mutation::DeleteProperties;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteProperties, _base: &SemioKitSnapshot) -> SemioKitDiff {
    SemioKitDiff { properties: Some(None), ..Default::default() }
}
//#endregion 🔖️Diff
