//! 🪓 Remodeling mutation — `DeleteStream`: removes an id-keyed media stream.

use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingGcpList, RemodelingMediaStreamList};
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🪓 `delete-stream` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-stream")]
pub struct DeleteStream {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_stream(id: String) -> RemodelingMutation {
    RemodelingMutation::DeleteStream(DeleteStream { id })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for DeleteStream {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "stream", kind: "delete-stream", record: "DeletedStream" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete stream \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
