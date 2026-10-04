//! ↩️ Inverse for `ChangeGridSubdivisions`.
use super::ChangeGridSubdivisions;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeGridSubdivisions, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![NoteMutation::ChangeGridSubdivisions(ChangeGridSubdivisions { new_subdivisions: base.grid_subdivisions })]

    })())
}
//#endregion 🔖️Inverse
