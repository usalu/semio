//! ↩️ Inverse for `ChangeMaterialBaseColor`.

use crate::schema::diff::material_at;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ChangeMaterialBaseColor, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::change_material_base_color::ChangeMaterialBaseColor>,
{
    Ok((|| {
    match material_at(base, &payload.id) {
        Some(material) => vec![Op::from(super::ChangeMaterialBaseColor { id: payload.id.clone(), new_base_color: material.base_color })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
