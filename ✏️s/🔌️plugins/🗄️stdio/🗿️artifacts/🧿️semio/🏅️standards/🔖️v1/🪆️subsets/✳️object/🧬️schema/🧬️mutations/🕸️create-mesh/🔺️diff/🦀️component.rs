//! 🔺️ `create-mesh` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::CreateMesh;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateMesh, _base: &SemioObjectSnapshot) -> SemioObjectDiff {
    SemioObjectDiff { mesh: Some(Some(store::ArtifactChild::new(payload.child_id.clone(), payload.target.clone()))), ..Default::default() }
}
//#endregion 🔖️Diff
