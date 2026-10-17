//! ↩️ Inverse for `RenameNote`.
use super::RenameNote;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &RenameNote, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::document::schema::mutations::rename_note::RenameNote>,
{
    Ok((|| {
    vec![Op::from(RenameNote { new_title: base.title.clone() })]

    })())
}
//#endregion 🔖️Inverse
