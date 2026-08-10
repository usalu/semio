use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::mutations::NoteMutation;

pub fn apply(projection: &mut NoteSnapshot, mutation: &NoteMutation) {
    *projection = crate::artifacts::note::schema::mutations::apply_note_mutation(projection, mutation);
}
