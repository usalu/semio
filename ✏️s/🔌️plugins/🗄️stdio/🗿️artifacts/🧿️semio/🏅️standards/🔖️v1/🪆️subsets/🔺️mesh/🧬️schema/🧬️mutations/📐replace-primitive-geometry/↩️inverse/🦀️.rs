//! ↩️ Inverse for `ReplacePrimitiveGeometry`.

use crate::schema::diff::primitive_at;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ReplacePrimitiveGeometry, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::replace_primitive_geometry::ReplacePrimitiveGeometry>,
{
    Ok((|| {
    match primitive_at(base, &payload.mesh_id, &payload.primitive_id) {
        Some(primitive) => vec![Op::from(super::ReplacePrimitiveGeometry {
            mesh_id: payload.mesh_id.clone(),
            primitive_id: payload.primitive_id.clone(),
            positions: primitive.positions.clone(),
            normals: primitive.normals.clone(),
            uvs: primitive.uvs.clone(),
            colors: primitive.colors.clone(),
            indices: primitive.indices.clone(),
        })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
