//! ↩️ Inverse for `ChangeGridSubdivisions`.
use super::ChangeGridSubdivisions;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeGridSubdivisions, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::ChangeGridSubdivisions>,
{
    Ok((|| {
    vec![Op::from(ChangeGridSubdivisions { new_subdivisions: base.grid_subdivisions })]

    })())
}
//#endregion 🔖️Inverse
