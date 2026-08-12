//! 🔺️ `bind-representation` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::BindRepresentation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitLinkList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &BindRepresentation, base: &SemioKitSnapshot) -> SemioKitDiff {
    let mut representations = base.representations.clone();
    representations.push(store::ArtifactLink { target: payload.target.clone(), pin: payload.pin.clone(), role: payload.role.clone() });
    SemioKitDiff { representations: Some(SemioKitLinkList { values: representations }), ..Default::default() }
}
//#endregion 🔖️Diff
