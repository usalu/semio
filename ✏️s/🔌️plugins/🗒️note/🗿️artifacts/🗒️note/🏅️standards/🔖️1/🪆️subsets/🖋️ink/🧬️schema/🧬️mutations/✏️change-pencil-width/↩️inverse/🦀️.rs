//! ↩️ Inverse for `ChangePencilWidth`.
use super::ChangePencilWidth;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangePencilWidth, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::ink::schema::mutations::change_pencil_width::ChangePencilWidth>,
{
    Ok((|| {
    vec![Op::from(ChangePencilWidth { new_width: base.pencil_width })]

    })())
}
//#endregion 🔖️Inverse
