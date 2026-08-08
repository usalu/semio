use crate::artifacts::note::NoteDocument;
use crate::artifacts::note::mutations::NoteMutation;

pub fn apply(projection: &mut NoteDocument, mutation: &NoteMutation) {
    *projection = crate::artifacts::note::mutations::apply_note_mutation(projection, mutation);
}
