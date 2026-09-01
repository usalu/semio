//! 🔧️ Direct rewrite mutation — `ChangeParameterBinding`: upserts one key on the `parameter_bindings` map.
use crate::artifacts::jack::PropertyValue;
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔧️ `change-parameter-binding` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-parameter-binding")]
pub struct ChangeParameterBinding {
    pub key: String,
    pub new_value: PropertyValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_parameter_binding(key: String, new_value: PropertyValue) -> RewriteRuleMutation {
    RewriteRuleMutation::ChangeParameterBinding(ChangeParameterBinding { key, new_value })
}

impl protocol::MutationKind<RewriteSnapshot, RewriteRuleMutation> for ChangeParameterBinding {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "parameter-binding", kind: "change-parameter-binding", record: "ChangedParameterBinding" };

    fn diff(&self, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change parameter binding \"{}\"", self.key)
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
