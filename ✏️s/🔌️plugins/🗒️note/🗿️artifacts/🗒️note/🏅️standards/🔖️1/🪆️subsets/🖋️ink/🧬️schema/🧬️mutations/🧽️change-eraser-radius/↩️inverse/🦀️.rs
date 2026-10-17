//! ↩️ Inverse for `ChangeEraserRadius`.
use super::ChangeEraserRadius;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeEraserRadius, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::ChangeEraserRadius>,
{
    Ok((|| {
    vec![Op::from(ChangeEraserRadius { new_radius: base.eraser_radius })]

    })())
}
//#endregion 🔖️Inverse
