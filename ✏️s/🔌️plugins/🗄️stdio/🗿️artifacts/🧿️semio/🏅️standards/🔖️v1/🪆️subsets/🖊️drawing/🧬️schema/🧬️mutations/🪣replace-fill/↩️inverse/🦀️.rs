//! ↩️ Inverse for `ReplaceFill`.

use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ReplaceFill, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::replace_fill::ReplaceFill>,
{
    Ok((|| {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) => vec![Op::from(super::ReplaceFill { style_name: payload.style_name.clone(), new_fill: old.fill })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
