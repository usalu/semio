//! 🔺️ `change-representation-pin` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::ChangeRepresentationPin;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitLinkList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRepresentationPin, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    let Some(existing) = base.representations.get(payload.index) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("No representation link exists at index #{}.", payload.index),
            [payload.index.to_string()],
        );
    };
    if existing.pin == payload.pin {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Representation link #{} is already pinned to that value.", payload.index));
    }
    let mut representations = base.representations.clone();
    if let Some(link) = representations.get_mut(payload.index) {
        link.pin = payload.pin.clone();
    }
    protocol::MutationOutcome::new(SemioKitDiff { representations: Some(SemioKitLinkList { values: representations }), ..Default::default() })
}
//#endregion 🔖️Diff
