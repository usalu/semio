//! ↩️ Inverse for `ChangeGridSubdivisions`.
use super::ChangeGridSubdivisions;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeGridSubdivisions, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeGridSubdivisions(ChangeGridSubdivisions { new_subdivisions: base.grid_subdivisions })]
}
//#endregion 🔖️Inverse
