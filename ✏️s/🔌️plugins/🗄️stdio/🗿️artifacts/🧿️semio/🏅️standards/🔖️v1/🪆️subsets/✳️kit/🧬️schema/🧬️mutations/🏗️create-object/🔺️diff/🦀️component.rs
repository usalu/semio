//! 🔺️ `create-object` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::CreateObject;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitObjectChildList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateObject, base: &SemioKitSnapshot) -> SemioKitDiff {
    let mut objects = base.objects.clone();
    objects.push(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()));
    SemioKitDiff { objects: Some(SemioKitObjectChildList { values: objects }), ..Default::default() }
}
//#endregion 🔖️Diff
