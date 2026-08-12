//! ↩️ `replace-fill` — undo restores the named style's BASE-state fill; absent style ⇒
//! `Vec::new()`.

use super::mutation::ReplaceFill;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceFill, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) => vec![SemioDrawingMutation::ReplaceFill(ReplaceFill { style_name: payload.style_name.clone(), new_fill: old.fill })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
