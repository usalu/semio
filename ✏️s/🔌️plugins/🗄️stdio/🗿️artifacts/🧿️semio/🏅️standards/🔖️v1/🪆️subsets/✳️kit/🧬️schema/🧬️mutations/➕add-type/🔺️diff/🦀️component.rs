//! 🔺️ `add-type` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::AddType;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitTypeList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};

//#region 🔖️Diff
pub fn diff(payload: &AddType, base: &SemioKitSnapshot) -> SemioKitDiff {
    let mut types = base.types.clone();
    types.push(SemioKitType { id: payload.id.clone(), name: payload.name.clone(), category: payload.category.clone() });
    SemioKitDiff { types: Some(SemioKitTypeList { values: types }), ..Default::default() }
}
//#endregion 🔖️Diff
