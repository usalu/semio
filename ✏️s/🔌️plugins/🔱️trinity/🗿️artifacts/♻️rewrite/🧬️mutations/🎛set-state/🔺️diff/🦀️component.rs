use crate::artifacts::rewrite::diff::RewriteRuleDiff;
use crate::artifacts::rewrite::RewriteRuleDocument;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &RewriteRuleMutation, base: &RewriteRuleDocument) -> RewriteRuleDiff {
    <RewriteRuleMutation as protocol::Mutation<RewriteRuleDocument>>::diff(mutation, base)
}
