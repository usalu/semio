//! ↩️ `change-stroke-width` — undo restores the named style's BASE-state stroke width; absent
//! style ⇒ `Vec::new()`.

use super::mutation::ChangeStrokeWidth;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeStrokeWidth, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) => vec![SemioDrawingMutation::ChangeStrokeWidth(ChangeStrokeWidth { style_name: payload.style_name.clone(), new_width: old.stroke_width })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
