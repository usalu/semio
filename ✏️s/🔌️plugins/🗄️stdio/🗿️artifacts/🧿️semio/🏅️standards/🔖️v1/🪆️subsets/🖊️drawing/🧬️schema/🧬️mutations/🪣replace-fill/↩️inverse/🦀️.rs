//! ↩️ Inverse for `ReplaceFill`.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ReplaceFill, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) => vec![SemioDrawingMutation::ReplaceFill(super::ReplaceFill { style_name: payload.style_name.clone(), new_fill: old.fill })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
