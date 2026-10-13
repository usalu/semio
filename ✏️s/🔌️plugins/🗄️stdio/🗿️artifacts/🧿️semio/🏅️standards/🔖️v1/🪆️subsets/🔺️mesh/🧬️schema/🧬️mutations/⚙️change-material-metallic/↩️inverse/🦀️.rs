//! ↩️ Inverse for `ChangeMaterialMetallic`.

use crate::schema::diff::material_at;
use crate::schema::mutations::SemioMeshMutation;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ChangeMaterialMetallic, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match material_at(base, &payload.id) {
        Some(material) => vec![SemioMeshMutation::ChangeMaterialMetallic(super::ChangeMaterialMetallic { id: payload.id.clone(), new_metallic: material.metallic })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
