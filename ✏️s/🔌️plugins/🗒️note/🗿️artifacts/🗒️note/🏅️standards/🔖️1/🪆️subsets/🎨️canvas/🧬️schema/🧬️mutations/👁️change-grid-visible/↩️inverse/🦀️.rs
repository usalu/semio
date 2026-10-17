//! ↩️ Inverse for `ChangeGridVisible`.
use super::ChangeGridVisible;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeGridVisible, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::ChangeGridVisible>,
{
    Ok((|| {
    vec![Op::from(ChangeGridVisible { new_visible: base.grid_visible })]

    })())
}
//#endregion 🔖️Inverse
