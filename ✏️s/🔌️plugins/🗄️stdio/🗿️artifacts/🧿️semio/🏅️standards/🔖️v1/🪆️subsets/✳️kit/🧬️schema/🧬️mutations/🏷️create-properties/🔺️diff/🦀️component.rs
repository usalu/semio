//! 🔺️ `create-properties` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::CreateProperties;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &CreateProperties, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.properties.is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The kit already has a properties child.".to_string(), ["properties".to_string()]);
    }
    protocol::MutationOutcome::new(SemioKitDiff { properties: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() })
}
//#endregion 🔖️Diff
