//! ↩️ Inverse for `SetPrimitiveTopology`.

use crate::schema::diff::primitive_at;
use crate::schema::mutations::SemioMeshMutation;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetPrimitiveTopology, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match primitive_at(base, &payload.mesh_id, &payload.primitive_id) {
        Some(primitive) => vec![SemioMeshMutation::SetPrimitiveTopology(super::SetPrimitiveTopology { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive_id.clone(), topology: primitive.topology })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
