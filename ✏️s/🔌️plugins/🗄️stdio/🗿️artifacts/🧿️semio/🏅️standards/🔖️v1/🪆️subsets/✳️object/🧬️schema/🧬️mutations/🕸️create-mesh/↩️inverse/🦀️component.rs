//! ↩️ `create-mesh` — undo restores whichever handle occupied `mesh` BEFORE this create ran.

use super::mutation::CreateMesh;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{delete_mesh, SemioObjectMutation};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &CreateMesh, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    match &base.mesh {
        Some(existing) => vec![SemioObjectMutation::CreateMesh(CreateMesh { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => vec![SemioObjectMutation::DeleteMesh(delete_mesh::mutation::DeleteMesh {})],
    }
}
//#endregion 🔖️Inverse
