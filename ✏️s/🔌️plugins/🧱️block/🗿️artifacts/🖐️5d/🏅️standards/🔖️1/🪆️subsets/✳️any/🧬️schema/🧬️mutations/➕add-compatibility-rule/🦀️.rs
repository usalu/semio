//! ➕ Block5d mutation — `AddCompatibilityRule`: a grip-kind compatibility rule attachment.

use semio_s_plugin_block::BlockCompatibilityRule;
use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Mutation
/// ➕ `add-compatibility-rule` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-compatibility-rule")]
pub struct AddCompatibilityRule {
    #[dsl(block)]
    pub rule: BlockCompatibilityRule,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_compatibility_rule<Op: From<crate::add_compatibility_rule::AddCompatibilityRule>>(rule: BlockCompatibilityRule) -> Op {
    Op::from(AddCompatibilityRule { rule, index: None })
}

/// 📍️ Builder — like [`add_compatibility_rule`] but inserts the row at `index`.
pub fn add_compatibility_rule_at<Op: From<crate::add_compatibility_rule::AddCompatibilityRule>>(rule: BlockCompatibilityRule, index: u32) -> Op {
    Op::from(AddCompatibilityRule { rule, index: Some(index) })
}

impl<Op> protocol::MutationKind<Block5dSnapshot, Op> for AddCompatibilityRule where Op: protocol::Mutation<Block5dSnapshot, Diff = semio_s_artifact_block_5d_diff::Block5dDiff> + From<crate::remove_compatibility_rule::RemoveCompatibilityRule> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "compatibility-rule", kind: "add-compatibility-rule", record: "AddedCompatibilityRule" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        crate::add_compatibility_rule::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::add_compatibility_rule::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add compatibility rule \"{}\"", self.rule.id), &format!("Kompatibilitätsregel \"{}\" hinzufügen", self.rule.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.rule.id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
