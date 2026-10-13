//! ↩️ Inverse for `ChangeStrokeColor`.

use crate::schema::mutations::SemioDrawingMutation;
use crate::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ChangeStrokeColor, base: &SemioDrawingSnapshot) -> Result<Vec<SemioDrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) => vec![SemioDrawingMutation::ChangeStrokeColor(super::ChangeStrokeColor { style_name: payload.style_name.clone(), new_color: old.stroke })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
