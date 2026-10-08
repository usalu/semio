//! ↩️ Inverse for `DeletePrimitive`.

use crate::standards::v1::subsets::mesh::schema::diff::mesh_at;
use crate::standards::v1::subsets::mesh::schema::mutations::{create_primitive, SemioMeshMutation};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeletePrimitive, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(mesh) = mesh_at(base, &payload.mesh_id) else {
        return Vec::new();
    };
    let Some(pos) = mesh.primitives.iter().position(|p| p.id == payload.primitive_id) else {
        return Vec::new();
    };
    vec![SemioMeshMutation::CreatePrimitive(create_primitive::CreatePrimitive { mesh_id: payload.mesh_id.clone(), primitive: mesh.primitives[pos].clone(), at: Some(pos) })]

    })())
}
//#endregion 🔖️Inverse
