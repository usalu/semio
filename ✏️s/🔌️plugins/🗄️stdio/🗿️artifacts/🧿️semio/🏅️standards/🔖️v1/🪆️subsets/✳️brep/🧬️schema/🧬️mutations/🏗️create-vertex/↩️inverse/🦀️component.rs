//! ↩️ `create-vertex` — undo is `deletevertex` (`delete_vertex`) at the same id.

use super::mutation::CreateVertex;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{delete_vertex, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateVertex, _base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    vec![SemioBrepMutation::DeleteVertex(delete_vertex::mutation::DeleteVertex { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
