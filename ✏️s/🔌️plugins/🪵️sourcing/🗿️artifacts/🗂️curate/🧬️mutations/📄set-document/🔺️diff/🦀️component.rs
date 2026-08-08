use crate::artifacts::curate::diff::SourcingDiff;
use crate::artifacts::curate::SourcingDocument;
use crate::artifacts::curate::mutations::SourcingMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &SourcingMutation, base: &SourcingDocument) -> SourcingDiff {
    <SourcingMutation as protocol::Mutation<SourcingDocument>>::diff(mutation, base)
}
