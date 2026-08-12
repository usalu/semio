//! 🔺️ `unbind-representation` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::UnbindRepresentation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitLinkList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &UnbindRepresentation, base: &SemioKitSnapshot) -> SemioKitDiff {
    let mut representations = base.representations.clone();
    if payload.index < representations.len() {
        representations.remove(payload.index);
    }
    SemioKitDiff { representations: Some(SemioKitLinkList { values: representations }), ..Default::default() }
}
//#endregion 🔖️Diff
