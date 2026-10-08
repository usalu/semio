//! ↩️ Inverse for `DeleteMaterial`.

use crate::standards::v1::subsets::mesh::schema::mutations::{create_material, SemioMeshMutation};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteMaterial, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(pos) = base.materials.iter().position(|m| m.id == payload.id) else {
        return Vec::new();
    };
    vec![SemioMeshMutation::CreateMaterial(create_material::CreateMaterial { material: base.materials[pos].clone(), at: Some(pos) })]

    })())
}
//#endregion 🔖️Inverse
