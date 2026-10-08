//! ↩️ Inverse for `DeleteMesh`.

use crate::standards::v1::subsets::mesh::schema::mutations::{create_mesh, SemioMeshMutation};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteMesh, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(pos) = base.meshes.iter().position(|m| m.id == payload.id) else {
        return Vec::new();
    };
    vec![SemioMeshMutation::CreateMesh(create_mesh::CreateMesh { mesh: base.meshes[pos].clone(), at: Some(pos) })]

    })())
}
//#endregion 🔖️Inverse
