//! 🔺️ `remove-type` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::RemoveType;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitTypeList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveType, base: &SemioKitSnapshot) -> SemioKitDiff {
    let types: Vec<_> = base.types.iter().filter(|t| t.id != payload.id).cloned().collect();
    SemioKitDiff { types: Some(SemioKitTypeList { values: types }), ..Default::default() }
}
//#endregion 🔖️Diff
