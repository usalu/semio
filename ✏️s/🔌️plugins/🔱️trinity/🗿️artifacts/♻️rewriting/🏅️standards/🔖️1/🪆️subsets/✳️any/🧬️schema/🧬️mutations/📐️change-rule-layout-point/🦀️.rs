//! 📐️ Direct rewriting mutation — `ChangeRuleLayoutPoint`: upserts one key on the `rule_layout` map (the
//! rule-editor position of a pattern var/node).
use crate::artifacts::rewriting::diff::RewritingDiff;
use crate::artifacts::rewriting::mutations::RewriteRuleMutation;
use crate::artifacts::rewriting::{LayoutPoint, RewritingSnapshot};

//#region 🔖️Mutation
/// 📐️ `change-rule-layout-point` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-rule-layout-point")]
pub struct ChangeRuleLayoutPoint {
    pub key: String,
    #[dsl(block)]
    pub new_point: LayoutPoint,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_rule_layout_point(key: String, new_point: LayoutPoint) -> RewriteRuleMutation {
    RewriteRuleMutation::ChangeRuleLayoutPoint(ChangeRuleLayoutPoint { key, new_point })
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for ChangeRuleLayoutPoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "rule-layout-point", kind: "change-rule-layout-point", record: "ChangedRuleLayoutPoint" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change rule layout point \"{}\"", self.key)
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
