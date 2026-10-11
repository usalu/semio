//! ➕ Block3d mutation — `AddCompatibilityRule`: a handle/vortex-kind compatibility rule attachment.

use crate::BlockCompatibilityRule;
use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

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
pub fn add_compatibility_rule(rule: BlockCompatibilityRule) -> Block3dMutation {
    Block3dMutation::AddCompatibilityRule(AddCompatibilityRule { rule, index: None })
}

/// 📍️ Builder — like [`add_compatibility_rule`] but inserts the row at `index`.
pub fn add_compatibility_rule_at(rule: BlockCompatibilityRule, index: u32) -> Block3dMutation {
    Block3dMutation::AddCompatibilityRule(AddCompatibilityRule { rule, index: Some(index) })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for AddCompatibilityRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "compatibility-rule", kind: "add-compatibility-rule", record: "AddedCompatibilityRule" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
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
