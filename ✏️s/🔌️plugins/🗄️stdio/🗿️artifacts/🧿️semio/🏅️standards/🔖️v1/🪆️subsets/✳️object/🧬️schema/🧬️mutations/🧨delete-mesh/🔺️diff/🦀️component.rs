//! 🔺️ `delete-mesh` — sparse diff construction: always clears `mesh`.

use super::mutation::DeleteMesh;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &DeleteMesh, _base: &SemioObjectSnapshot) -> SemioObjectDiff {
    SemioObjectDiff { mesh: Some(None), ..Default::default() }
}
//#endregion 🔖️Diff
