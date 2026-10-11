//! ➖️ Block2d mutation — `RemoveCompatibilityRule`: a handle-kind compatibility rule attachment.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Mutation
/// ➖️ `remove-compatibility-rule` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "remove-compatibility-rule")]
pub struct RemoveCompatibilityRule {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_compatibility_rule(id: String) -> Block2dMutation {
    Block2dMutation::RemoveCompatibilityRule(RemoveCompatibilityRule { id })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for RemoveCompatibilityRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "compatibility-rule", kind: "remove-compatibility-rule", record: "RemovedCompatibilityRule" };

    fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove compatibility rule \"{}\"", self.id), &format!("Kompatibilitätsregel \"{}\" entfernen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
