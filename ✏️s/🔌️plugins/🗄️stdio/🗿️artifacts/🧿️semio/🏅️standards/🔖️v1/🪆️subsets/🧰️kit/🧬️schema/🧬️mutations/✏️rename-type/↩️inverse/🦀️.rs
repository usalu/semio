//! ↩️ Inverse for `RenameType`.

use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RenameType, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::rename_type::RenameType>,
{
    Ok((|| {
    match base.types.iter().find(|t| t.id == payload.id) {
        Some(existing) => vec![Op::from(super::RenameType { id: payload.id.clone(), new_name: existing.name.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
