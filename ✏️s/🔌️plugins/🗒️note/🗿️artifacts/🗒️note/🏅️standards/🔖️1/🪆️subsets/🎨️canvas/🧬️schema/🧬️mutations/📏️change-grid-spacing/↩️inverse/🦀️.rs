//! ↩️ Inverse for `ChangeGridSpacing`.
use super::ChangeGridSpacing;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeGridSpacing, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::ChangeGridSpacing>,
{
    Ok((|| {
    vec![Op::from(ChangeGridSpacing { new_spacing: base.grid_spacing })]

    })())
}
//#endregion 🔖️Inverse
