//! 🔺️ `add-design` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::AddDesign;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDesignList, SemioKitDiff};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &AddDesign, base: &SemioKitSnapshot) -> SemioKitDiff {
    let mut designs = base.designs.clone();
    designs.push(SemioKitDesign { id: payload.id.clone(), name: payload.name.clone(), pieces: Vec::new(), connections: Vec::new() });
    SemioKitDiff { designs: Some(SemioKitDesignList { values: designs }), ..Default::default() }
}
//#endregion 🔖️Diff
