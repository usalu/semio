//! 👁️ 👁️ Trinity Rewriting app command — `reorganize`.

use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;

#[cfg(test)]
#[path = "🧪️tests/📐️document-mutation-reorganization/🦀️.rs"]
mod tests;

pub(crate) fn reorganize(state: &crate::RewritingSnapshot) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    Emit { artifact_mutations: state.rule_layout.keys().cloned().map(crate::standards::v1::subsets::any::schema::mutations::remove_rule_layout_point).collect(), ..Default::default() }
}
