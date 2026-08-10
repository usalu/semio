use crate::artifacts::rewrite::RewriteSnapshot;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;

pub fn inverse(base: &RewriteSnapshot, mutation: &RewriteRuleMutation) -> Vec<RewriteRuleMutation> {
    <RewriteRuleMutation as protocol::Mutation<RewriteSnapshot>>::inverse(mutation, base)
}
