//! ↩️ Inverse for `ChangeEraserRadius`.
use super::ChangeEraserRadius;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeEraserRadius, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeEraserRadius(ChangeEraserRadius { new_radius: base.eraser_radius })]
}
//#endregion 🔖️Inverse
