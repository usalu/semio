//! ↩️ Inverse for `EditDesign`.

use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::EditDesign, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::edit_design::EditDesign>,
{
    Ok((|| {
    match base.designs.iter().find(|d| d.id == payload.id) {
        Some(existing) => vec![Op::from(super::EditDesign { id: payload.id.clone(), pieces: existing.pieces.clone(), connections: existing.connections.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
