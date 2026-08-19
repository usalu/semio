//! ↩️ `change-stroke-color` — undo restores the named style's BASE-state stroke color; absent
//! style ⇒ `Vec::new()`.

use super::mutation::ChangeStrokeColor;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeStrokeColor, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match base.styles.iter().find(|s| s.name == payload.style_name) {
        Some(old) => vec![SemioDrawingMutation::ChangeStrokeColor(ChangeStrokeColor { style_name: payload.style_name.clone(), new_color: old.stroke })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
