//! 🗑️ Direct rewrite mutation — `RemoveRuleLayoutPoint`: takes one key out of the `rule_layout` map.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;

//#region 🔖️Mutation
/// 🗑️ `remove-rule-layout-point` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-rule-layout-point")]
pub struct RemoveRuleLayoutPoint {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_rule_layout_point(key: String) -> RewriteRuleMutation {
    RewriteRuleMutation::RemoveRuleLayoutPoint(RemoveRuleLayoutPoint { key })
}

impl protocol::MutationKind<RewriteSnapshot, RewriteRuleMutation> for RemoveRuleLayoutPoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "rule-layout-point", kind: "remove-rule-layout-point", record: "RemovedRuleLayoutPoint" };

    fn diff(&self, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove rule layout point \"{}\"", self.key)
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
