use crate::artifacts::home::diff::SHomeDiff;
use crate::artifacts::home::SHomeDocument;
use crate::artifacts::home::mutations::SHomeMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &SHomeMutation, base: &SHomeDocument) -> SHomeDiff {
    <SHomeMutation as protocol::Mutation<SHomeDocument>>::diff(mutation, base)
}
