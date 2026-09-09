//! 🔧️ Direct rewriting mutation — `ChangeParameterBinding`: upserts one key on the `parameter_bindings` map.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;
use semio_framework_graph::manifest::PropertyValue;

//#region 🔖️Mutation
/// 🔧️ `change-parameter-binding` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-parameter-binding")]
pub struct ChangeParameterBinding {
    pub key: String,
    pub new_value: PropertyValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_parameter_binding(key: String, new_value: PropertyValue) -> RewriteRuleMutation {
    RewriteRuleMutation::ChangeParameterBinding(ChangeParameterBinding { key, new_value })
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for ChangeParameterBinding {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "parameter-binding", kind: "change-parameter-binding", record: "ChangedParameterBinding" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
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
