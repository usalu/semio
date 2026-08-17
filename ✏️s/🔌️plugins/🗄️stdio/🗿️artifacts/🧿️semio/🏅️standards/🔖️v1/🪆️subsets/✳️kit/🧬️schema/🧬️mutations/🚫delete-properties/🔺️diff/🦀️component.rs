//! 🔺️ `delete-properties` — clears `properties`; Error `mutation.target-missing` if there is
//! nothing to delete.

use super::mutation::DeleteProperties;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteProperties, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.properties.is_none() {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            "The kit has no properties to delete.".to_string(),
            ["properties".to_string()],
        );
    }
    protocol::MutationOutcome::new(SemioKitDiff { properties: Some(None), ..Default::default() })
}
//#endregion 🔖️Diff
