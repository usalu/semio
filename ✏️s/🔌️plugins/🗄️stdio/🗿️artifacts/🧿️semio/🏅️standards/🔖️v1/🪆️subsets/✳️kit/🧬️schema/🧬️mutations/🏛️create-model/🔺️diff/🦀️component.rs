//! 🔺️ `create-model` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::CreateModel;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitModelChildList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateModel, base: &SemioKitSnapshot) -> SemioKitDiff {
    let mut models = base.models.clone();
    models.push(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()));
    SemioKitDiff { models: Some(SemioKitModelChildList { values: models }), ..Default::default() }
}
//#endregion 🔖️Diff
