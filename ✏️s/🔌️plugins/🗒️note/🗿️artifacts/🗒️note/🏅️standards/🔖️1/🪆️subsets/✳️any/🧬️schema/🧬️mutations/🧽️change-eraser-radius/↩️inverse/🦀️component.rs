//! ↩️ Inverse for `ChangeEraserRadius`.
use super::mutation::ChangeEraserRadius;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEraserRadius, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeEraserRadius(ChangeEraserRadius { new_radius: base.eraser_radius })]
}
//#endregion 🔖️Inverse
