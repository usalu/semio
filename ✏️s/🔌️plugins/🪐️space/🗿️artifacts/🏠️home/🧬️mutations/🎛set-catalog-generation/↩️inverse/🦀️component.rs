use crate::artifacts::home::SHomeDocument;
use crate::artifacts::home::mutations::SHomeMutation;

pub fn inverse(base: &SHomeDocument, mutation: &SHomeMutation) -> Vec<SHomeMutation> {
    <SHomeMutation as protocol::Mutation<SHomeDocument>>::inverse(mutation, base)
}
