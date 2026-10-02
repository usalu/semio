//! 📍️ Absolute rewriting mutation — `SetRuleLayoutPoints`: many keys of the `rule_layout` map at once — every `points` key set
//! to its point, every `cleared` key removed (its node returns to its default slot). The exact one-row undo of a rule-node drag,
//! and the one row a reorganize publishes.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 📍️ One rule node at an explicit position.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RuleLayoutPlacement {
    pub key: String,
    pub x: f64,
    pub y: f64,
}

/// 📍️ `set-rule-layout-points` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-rule-layout-points")]
pub struct SetRuleLayoutPoints {
    #[dsl(table)]
    pub points: Vec<RuleLayoutPlacement>,
    pub cleared: Vec<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_rule_layout_points(points: Vec<RuleLayoutPlacement>, cleared: Vec<String>) -> RewriteRuleMutation {
    RewriteRuleMutation::SetRuleLayoutPoints(SetRuleLayoutPoints { points, cleared })
}

impl SetRuleLayoutPoints {
    /// 🛂️ Whether the payload is well-formed: every key at most once across `points` and `cleared`, every point finite.
    pub fn holds_invariants(&self) -> bool {
        let keys: Vec<&str> = self.points.iter().map(|point| point.key.as_str()).chain(self.cleared.iter().map(String::as_str)).collect();
        !keys.iter().enumerate().any(|(at, key)| key.is_empty() || keys[..at].contains(key)) && self.points.iter().all(|point| point.x.is_finite() && point.y.is_finite())
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for SetRuleLayoutPoints {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "rule-layout-points", kind: "set-rule-layout-points", record: "SetRuleLayoutPoints" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.points.len() + self.cleared.len() {
            1 => semio_framework_ui_locale::LocalizedLabel::native("Place 1 rule node", "1 Regelknoten platzieren"),
            count => semio_framework_ui_locale::LocalizedLabel::native(&format!("Place {count} rule nodes"), &format!("{count} Regelknoten platzieren")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.points.iter().map(|point| point.key.clone()).chain(self.cleared.iter().cloned()).collect()
    }
}
//#endregion 🔖️Mutation
