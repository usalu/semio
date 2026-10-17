//! ↩️ Inverse for `DeletePrimitive`.

use crate::schema::diff::mesh_at;
use crate::schema::mutations::create_primitive;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::DeletePrimitive, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::create_primitive::CreatePrimitive>,
{
    Ok((|| {
    let Some(mesh) = mesh_at(base, &payload.mesh_id) else {
        return Vec::new();
    };
    let Some(pos) = mesh.primitives.iter().position(|p| p.id == payload.primitive_id) else {
        return Vec::new();
    };
    vec![Op::from(create_primitive::CreatePrimitive { mesh_id: payload.mesh_id.clone(), primitive: mesh.primitives[pos].clone(), at: Some(pos) })]

    })())
}
//#endregion 🔖️Inverse
