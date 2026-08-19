//! 📐️ Rewrite mutation — `ChangeRuleLayoutPoint`: upserts one key on the `rule_layout` map (the
//! rule-editor position of a pattern var/node).
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use crate::artifacts::rewrite::{LayoutPoint, RewriteSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📐️ `change-rule-layout-point` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-rule-layout-point")]
pub struct ChangeRuleLayoutPoint {
    pub key: String,
    #[dsl(block)]
    pub new_point: LayoutPoint,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_rule_layout_point(key: String, new_point: LayoutPoint) -> RewriteRuleMutation {
    RewriteRuleMutation::ChangeRuleLayoutPoint(ChangeRuleLayoutPoint { key, new_point })
}

impl protocol::MutationKind<RewriteSnapshot, RewriteRuleMutation> for ChangeRuleLayoutPoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "rule-layout-point", kind: "change-rule-layout-point", record: "ChangedRuleLayoutPoint" };

    async fn diff(&self, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change rule layout point \"{}\"", self.key)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
