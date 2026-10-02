//! 👁️ 👁️ Trinity Rewriting app command — `reorganize`.

use crate::standards::v1::subsets::any::schema::mutations::set_rule_layout_points;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;

#[cfg(test)]
#[path = "🧪️tests/📐️document-mutation-reorganization/🦀️.rs"]
mod tests;

/// 🧹️ Every rule node back at its default slot: ONE `set-rule-layout-points` clearing every layout point, nothing when no node
/// was laid out by hand.
pub(crate) fn reorganize(state: &crate::RewritingSnapshot) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    match state.rule_layout.is_empty() {
        true => Emit::default(),
        false => Emit::mutations(vec![set_rule_layout_points(Vec::new(), state.rule_layout.keys().cloned().collect())]),
    }
}
