use crate::artifacts::note::NoteDocument;
use crate::artifacts::note::mutations::NoteMutation;
use protocol::Mutation;

pub fn inverse(base: &NoteDocument, mutation: &NoteMutation) -> Vec<NoteMutation> {
    <NoteMutation as Mutation<NoteDocument>>::inverse(mutation, base)
}
