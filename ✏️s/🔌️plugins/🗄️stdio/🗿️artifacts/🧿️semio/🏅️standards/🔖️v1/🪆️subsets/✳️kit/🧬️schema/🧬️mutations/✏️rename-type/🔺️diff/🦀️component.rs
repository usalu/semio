//! 🔺️ `rename-type` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::RenameType;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitTypeList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameType, base: &SemioKitSnapshot) -> SemioKitDiff {
    let mut types = base.types.clone();
    if let Some(t) = types.iter_mut().find(|t| t.id == payload.id) {
        t.name = payload.new_name.clone();
    }
    SemioKitDiff { types: Some(SemioKitTypeList { values: types }), ..Default::default() }
}
//#endregion 🔖️Diff
