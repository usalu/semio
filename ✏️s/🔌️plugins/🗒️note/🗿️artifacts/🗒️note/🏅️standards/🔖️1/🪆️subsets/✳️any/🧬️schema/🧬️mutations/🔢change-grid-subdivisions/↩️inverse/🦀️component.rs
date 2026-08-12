//! ↩️ Inverse for `ChangeGridSubdivisions`.
use super::mutation::ChangeGridSubdivisions;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeGridSubdivisions, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeGridSubdivisions(ChangeGridSubdivisions { new_subdivisions: base.grid_subdivisions })]
}
//#endregion 🔖️Inverse
