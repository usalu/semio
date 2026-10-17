//! ↩️ Inverse for `ChangeSnapGridSpacing`.
use super::ChangeSnapGridSpacing;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeSnapGridSpacing, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::ChangeSnapGridSpacing>,
{
    Ok((|| {
    vec![Op::from(ChangeSnapGridSpacing { new_spacing: base.snap_grid_spacing })]

    })())
}
//#endregion 🔖️Inverse
