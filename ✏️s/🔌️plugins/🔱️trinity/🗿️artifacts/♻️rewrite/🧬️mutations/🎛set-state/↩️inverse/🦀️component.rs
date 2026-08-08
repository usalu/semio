use crate::artifacts::rewrite::RewriteRuleDocument;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;

pub fn inverse(base: &RewriteRuleDocument, mutation: &RewriteRuleMutation) -> Vec<RewriteRuleMutation> {
    <RewriteRuleMutation as protocol::Mutation<RewriteRuleDocument>>::inverse(mutation, base)
}
