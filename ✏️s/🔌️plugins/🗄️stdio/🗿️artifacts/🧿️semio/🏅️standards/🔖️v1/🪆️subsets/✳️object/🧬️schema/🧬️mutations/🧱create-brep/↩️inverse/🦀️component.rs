//! ↩️ `create-brep` — undo restores whichever handle occupied `brep` BEFORE this create ran (a
//! real prior handle if the slot was occupied, or `delete-brep` if it was empty) — never a bare
//! "delete", since `create-brep` may have OVERWRITTEN an existing handle.

use super::mutation::CreateBrep;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{delete_brep, SemioObjectMutation};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &CreateBrep, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    match &base.brep {
        Some(existing) => vec![SemioObjectMutation::CreateBrep(CreateBrep { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => vec![SemioObjectMutation::DeleteBrep(delete_brep::mutation::DeleteBrep {})],
    }
}
//#endregion 🔖️Inverse
