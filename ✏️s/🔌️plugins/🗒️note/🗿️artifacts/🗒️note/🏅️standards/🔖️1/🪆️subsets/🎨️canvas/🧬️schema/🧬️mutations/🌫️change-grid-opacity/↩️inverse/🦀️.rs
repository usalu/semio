//! ↩️ Inverse for `ChangeGridOpacity`.
use super::ChangeGridOpacity;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeGridOpacity, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::ChangeGridOpacity>,
{
    Ok((|| {
    vec![Op::from(ChangeGridOpacity { new_opacity: base.grid_opacity })]

    })())
}
//#endregion 🔖️Inverse
