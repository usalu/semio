//! 🔺️ `change-representation-pin` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::ChangeRepresentationPin;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitLinkList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRepresentationPin, base: &SemioKitSnapshot) -> SemioKitDiff {
    let mut representations = base.representations.clone();
    if let Some(link) = representations.get_mut(payload.index) {
        link.pin = payload.pin.clone();
    }
    SemioKitDiff { representations: Some(SemioKitLinkList { values: representations }), ..Default::default() }
}
//#endregion 🔖️Diff
