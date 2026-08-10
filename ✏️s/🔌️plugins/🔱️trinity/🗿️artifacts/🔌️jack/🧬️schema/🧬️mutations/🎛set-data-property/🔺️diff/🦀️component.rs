use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::JackSnapshot;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &TrinityGraphMutation, base: &JackSnapshot) -> JackDiff {
    <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(mutation, base)
}
