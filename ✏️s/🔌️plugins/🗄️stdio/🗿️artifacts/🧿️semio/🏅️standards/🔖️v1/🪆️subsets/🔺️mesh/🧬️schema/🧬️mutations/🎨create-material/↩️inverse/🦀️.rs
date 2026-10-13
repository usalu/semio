//! ↩️ Inverse for `CreateMaterial`.

use crate::schema::mutations::{delete_material, SemioMeshMutation};
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateMaterial, _base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioMeshMutation::DeleteMaterial(delete_material::DeleteMaterial { id: payload.material.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
