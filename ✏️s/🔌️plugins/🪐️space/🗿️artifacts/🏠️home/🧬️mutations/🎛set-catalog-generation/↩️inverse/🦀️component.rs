use crate::artifacts::home::SHomeSnapshot;
use crate::artifacts::home::mutations::SHomeMutation;

pub fn inverse(base: &SHomeSnapshot, mutation: &SHomeMutation) -> Vec<SHomeMutation> {
    <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::inverse(mutation, base)
}
