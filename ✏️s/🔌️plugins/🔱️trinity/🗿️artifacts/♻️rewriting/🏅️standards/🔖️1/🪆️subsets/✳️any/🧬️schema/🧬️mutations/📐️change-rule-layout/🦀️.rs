//! 📐️ Direct rewriting mutation — `ChangeRuleLayoutPoint`: upserts one key on the `rule_layout` map (the
//! rule-editor position of a pattern var/node).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::{LayoutPoint, RewritingSnapshot};

//#region 🔖️Mutation
/// 📐️ `change-rule-layout-point` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    fn inverse(&self, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change rule layout point \"{}\"", self.key), &format!("Layoutpunkt der Regel \"{}\" ändern", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
