//! 🧹️ Rewrite mutation — `RemoveParameterBinding`: takes one key out of the `parameter_bindings`
//! map.
use crate::artifacts::rewrite::diff::RewriteDiff;
use crate::artifacts::rewrite::mutations::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧹️ `remove-parameter-binding` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-parameter-binding")]
pub struct RemoveParameterBinding {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_parameter_binding(key: String) -> RewriteRuleMutation {
    RewriteRuleMutation::RemoveParameterBinding(RemoveParameterBinding { key })
}

impl protocol::MutationKind<RewriteSnapshot, RewriteRuleMutation> for RemoveParameterBinding {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "parameter-binding", kind: "remove-parameter-binding", record: "RemovedParameterBinding" };

    async fn diff(&self, base: &RewriteSnapshot) -> protocol::MutationOutcome<RewriteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RewriteSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove parameter binding \"{}\"", self.key)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
