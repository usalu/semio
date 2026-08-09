use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::mutations::NoteMutation;
use protocol::Mutation;

pub fn inverse(base: &NoteSnapshot, mutation: &NoteMutation) -> Vec<NoteMutation> {
    <NoteMutation as Mutation<NoteSnapshot>>::inverse(mutation, base)
}
