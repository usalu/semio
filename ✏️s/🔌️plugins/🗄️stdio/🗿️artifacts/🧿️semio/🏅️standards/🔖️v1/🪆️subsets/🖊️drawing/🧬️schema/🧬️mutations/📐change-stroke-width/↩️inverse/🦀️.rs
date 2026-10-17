//! ↩️ Inverse for `ChangeStrokeWidth`.

use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ChangeStrokeWidth, base: &SemioDrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::change_stroke_width::ChangeStrokeWidth>,
{
    Ok((|| {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) => vec![Op::from(super::ChangeStrokeWidth { style_name: payload.style_name.clone(), new_width: old.stroke_width })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
