//! ↩️ Inverse for `CreatePrimitive`.

use crate::schema::mutations::delete_primitive;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreatePrimitive, _base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_primitive::DeletePrimitive>,
{
    Ok((|| {
    vec![Op::from(delete_primitive::DeletePrimitive { mesh_id: payload.mesh_id.clone(), primitive_id: payload.primitive.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
