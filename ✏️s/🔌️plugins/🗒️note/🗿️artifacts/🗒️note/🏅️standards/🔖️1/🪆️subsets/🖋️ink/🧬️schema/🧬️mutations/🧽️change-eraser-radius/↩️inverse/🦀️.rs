//! ↩️ Inverse for `ChangeEraserRadius`.
use super::ChangeEraserRadius;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEraserRadius, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![NoteMutation::ChangeEraserRadius(ChangeEraserRadius { new_radius: base.eraser_radius })]

    })())
}
//#endregion 🔖️Inverse
