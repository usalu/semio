//! 🔪️ Remodeling mutation — `RemoveContent`: removes every leaf of one durable content entry from leaf
//! index `from` on; `from` 0 removes the entry. It is `append-content`'s exact inverse.

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔪️ `remove-content` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-content")]
pub struct RemoveContent {
    pub content_id: String,
    pub from: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_content(content_id: String, from: u64) -> RemodelingMutation {
    RemodelingMutation::RemoveContent(RemoveContent { content_id, from })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for RemoveContent {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "content", kind: "remove-content", record: "RemovedContent" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove content \"{}\" leaves from {}", self.content_id, self.from)
    }
    fn target(&self) -> Vec<String> {
        vec![self.content_id.clone()]
    }
}
//#endregion 🔖️Mutation
