//! ↩️ Inverse for `ChangeEraserRadius`.
use super::ChangeEraserRadius;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEraserRadius, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeEraserRadius(ChangeEraserRadius { new_radius: base.eraser_radius })]
}
//#endregion 🔖️Inverse
