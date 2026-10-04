//! ↩️ Inverse for `ChangeSnapGridSpacing`.
use super::ChangeSnapGridSpacing;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSnapGridSpacing, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![NoteMutation::ChangeSnapGridSpacing(ChangeSnapGridSpacing { new_spacing: base.snap_grid_spacing })]

    })())
}
//#endregion 🔖️Inverse
