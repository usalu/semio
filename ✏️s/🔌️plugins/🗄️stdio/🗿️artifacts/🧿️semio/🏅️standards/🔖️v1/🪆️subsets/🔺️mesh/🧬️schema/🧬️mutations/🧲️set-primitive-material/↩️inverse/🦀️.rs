//! ↩️ Inverse for `SetPrimitiveMaterial`.

use crate::schema::diff::primitive_at;
use crate::schema::mutations::SemioMeshMutation;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetPrimitiveMaterial, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match primitive_at(base, &payload.mesh_id, &payload.primitive_id) {
        Some(primitive) => vec![SemioMeshMutation::SetPrimitiveMaterial(super::SetPrimitiveMaterial { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive_id.clone(), material_id: primitive.material_id.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
