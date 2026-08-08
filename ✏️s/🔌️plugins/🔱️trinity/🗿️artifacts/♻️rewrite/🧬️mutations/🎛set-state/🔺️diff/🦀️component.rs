use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::RewriteSnapshot;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &RewriteRuleMutation, base: &RewriteSnapshot) -> RewriteDiff {
    <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::diff(mutation, base)
}
