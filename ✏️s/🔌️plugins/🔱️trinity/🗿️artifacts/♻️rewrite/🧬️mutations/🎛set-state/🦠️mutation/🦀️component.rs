//! 🎛 RewriteRule mutation — `SetState` apply delegate.
use crate::artifacts::rewrite::RewriteRuleDocument;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;

pub fn apply(projection: &mut RewriteRuleDocument, mutation: &RewriteRuleMutation) {
    crate::artifacts::rewrite::mutations::apply_rewrite_rule_mutation(projection, mutation);
}
