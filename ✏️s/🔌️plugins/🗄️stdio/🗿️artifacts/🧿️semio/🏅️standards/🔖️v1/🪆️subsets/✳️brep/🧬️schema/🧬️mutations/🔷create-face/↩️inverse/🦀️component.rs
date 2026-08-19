//! ↩️ `create-face` — undo is `deleteface` (`delete_face`) at the same id.

use super::mutation::CreateFace;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{delete_face, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateFace, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteFace(delete_face::mutation::DeleteFace { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
