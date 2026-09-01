//! ↩️ Inverse for `ChangeStrokeWidth`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ChangeStrokeWidth, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) => vec![SemioDrawingMutation::ChangeStrokeWidth(super::ChangeStrokeWidth { style_name: payload.style_name.clone(), new_width: old.stroke_width })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
