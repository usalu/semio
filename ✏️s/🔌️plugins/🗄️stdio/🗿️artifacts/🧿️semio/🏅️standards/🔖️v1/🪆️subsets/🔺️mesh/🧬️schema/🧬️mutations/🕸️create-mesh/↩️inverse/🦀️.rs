//! ↩️ Inverse for `CreateMesh`.

use crate::standards::v1::subsets::mesh::schema::mutations::{delete_mesh, SemioMeshMutation};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateMesh, _base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioMeshMutation::DeleteMesh(delete_mesh::DeleteMesh { id: payload.mesh.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
