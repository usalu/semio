//! 🔺️ `delete-object` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::DeleteObject;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitObjectChildList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteObject, base: &SemioKitSnapshot) -> SemioKitDiff {
    let objects: Vec<_> = base.objects.iter().filter(|c| c.child_id != payload.child_id).cloned().collect();
    SemioKitDiff { objects: Some(SemioKitObjectChildList { values: objects }), ..Default::default() }
}
//#endregion 🔖️Diff
