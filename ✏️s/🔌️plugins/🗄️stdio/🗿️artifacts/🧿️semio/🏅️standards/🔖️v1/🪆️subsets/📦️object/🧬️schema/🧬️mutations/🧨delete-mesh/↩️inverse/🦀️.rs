//! ↩️ Inverse for `DeleteMesh`.

use crate::schema::mutations::{create_mesh, SemioObjectMutation};
use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &super::DeleteMesh, base: &SemioObjectSnapshot) -> Result<Vec<SemioObjectMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match &base.mesh {
        Some(existing) => vec![SemioObjectMutation::CreateMesh(create_mesh::CreateMesh { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
