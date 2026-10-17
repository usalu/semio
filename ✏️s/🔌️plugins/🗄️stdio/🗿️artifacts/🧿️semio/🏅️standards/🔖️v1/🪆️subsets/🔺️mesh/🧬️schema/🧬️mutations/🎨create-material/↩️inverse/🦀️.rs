//! ↩️ Inverse for `CreateMaterial`.

use crate::schema::mutations::delete_material;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateMaterial, _base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_material::DeleteMaterial>,
{
    Ok((|| {
    vec![Op::from(delete_material::DeleteMaterial { id: payload.material.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
