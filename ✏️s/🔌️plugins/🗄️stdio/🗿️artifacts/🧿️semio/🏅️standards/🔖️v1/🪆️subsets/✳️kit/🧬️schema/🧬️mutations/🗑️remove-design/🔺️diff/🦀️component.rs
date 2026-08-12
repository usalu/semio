//! 🔺️ `remove-design` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::RemoveDesign;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitDesignList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveDesign, base: &SemioKitSnapshot) -> SemioKitDiff {
    let designs: Vec<_> = base.designs.iter().filter(|d| d.id != payload.id).cloned().collect();
    SemioKitDiff { designs: Some(SemioKitDesignList { values: designs }), ..Default::default() }
}
//#endregion 🔖️Diff
