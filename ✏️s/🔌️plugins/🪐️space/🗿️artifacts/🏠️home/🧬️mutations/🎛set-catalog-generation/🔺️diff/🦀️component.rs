use crate::artifacts::home::diff::SHomeDiff;
use crate::artifacts::home::SHomeSnapshot;
use crate::artifacts::home::mutations::SHomeMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &SHomeMutation, base: &SHomeSnapshot) -> SHomeDiff {
    <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(mutation, base)
}
