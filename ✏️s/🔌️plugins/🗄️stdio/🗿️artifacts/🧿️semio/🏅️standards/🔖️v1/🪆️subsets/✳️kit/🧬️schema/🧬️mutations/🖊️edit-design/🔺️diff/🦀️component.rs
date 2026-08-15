//! 🔺️ `edit-design` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::EditDesign;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDesignList, SemioKitDiff};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &EditDesign, base: &SemioKitSnapshot) -> SemioKitDiff {
    let mut designs = base.designs.clone();
    if let Some(d) = designs.iter_mut().find(|d| d.id == payload.id) {
        d.pieces = payload.pieces.clone();
        d.connections = payload.connections.clone();
    }
    SemioKitDiff { designs: Some(SemioKitDesignList { values: designs }), ..Default::default() }
}
//#endregion 🔖️Diff
