//! 🚮 Remodeling mutation — `DeleteGcp`: removes an id-keyed ground control point.

use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingGcpList};
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🚮 `delete-gcp` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-gcp")]
pub struct DeleteGcp {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_gcp(id: String) -> RemodelingMutation {
    RemodelingMutation::DeleteGcp(DeleteGcp { id })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for DeleteGcp {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "gcp", kind: "delete-gcp", record: "DeletedGcp" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete GCP \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
