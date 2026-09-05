//! ↩️ Inverse for `DeleteMesh`.

use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, create_mesh};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &super::DeleteMesh, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    match &base.mesh {
        Some(existing) => vec![SemioObjectMutation::CreateMesh(create_mesh::CreateMesh { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
