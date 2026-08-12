//! 🔺️ `delete-model` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::DeleteModel;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitModelChildList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteModel, base: &SemioKitSnapshot) -> SemioKitDiff {
    let models: Vec<_> = base.models.iter().filter(|c| c.child_id != payload.child_id).cloned().collect();
    SemioKitDiff { models: Some(SemioKitModelChildList { values: models }), ..Default::default() }
}
//#endregion 🔖️Diff
