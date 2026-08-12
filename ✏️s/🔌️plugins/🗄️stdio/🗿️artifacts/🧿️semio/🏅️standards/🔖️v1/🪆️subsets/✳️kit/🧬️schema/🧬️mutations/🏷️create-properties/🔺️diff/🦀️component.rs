//! 🔺️ `create-properties` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::CreateProperties;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateProperties, _base: &SemioKitSnapshot) -> SemioKitDiff {
    SemioKitDiff { properties: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() }
}
//#endregion 🔖️Diff
