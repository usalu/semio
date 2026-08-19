//! ↩️ `delete-brep` — undo is `create-brep` with the escrowed handle captured from BASE; empty
//! (`Vec::new()`) when the slot was already absent (nothing to undo).

use super::mutation::DeleteBrep;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{create_brep, SemioObjectMutation};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &DeleteBrep, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    match &base.brep {
        Some(existing) => vec![SemioObjectMutation::CreateBrep(create_brep::mutation::CreateBrep { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
