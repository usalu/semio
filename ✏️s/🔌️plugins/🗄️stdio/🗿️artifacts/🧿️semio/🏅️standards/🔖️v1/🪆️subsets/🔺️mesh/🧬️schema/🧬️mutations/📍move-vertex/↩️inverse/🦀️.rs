//! ↩️ Inverse for `MoveVertex`.

use crate::schema::diff::primitive_at;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::MoveVertex, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::move_vertex::MoveVertex>,
{
    Ok((|| {
    let Some(primitive) = primitive_at(base, &payload.mesh_id, &payload.primitive_id) else {
        return Vec::new();
    };
    let Some(old_point) = primitive.positions.get(payload.vertex_index).copied() else {
        return Vec::new();
    };
    vec![Op::from(super::MoveVertex { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive_id.clone(), vertex_index: payload.vertex_index, new_point: old_point })]

    })())
}
//#endregion 🔖️Inverse
