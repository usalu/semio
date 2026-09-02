//! 🗞️ Remodeling mutation — `DeleteAsset`: removes one key-addressed `ImageAsset`. No app call site
//! removes an asset today; this exists as `create-asset`'s inverse-only counterpart (a mutation kind
//! is real even without its own command call site) and for collection completeness.

use crate::artifacts::remodeling::{RemodelingSnapshot, remodeling_asset};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🗞️ `delete-asset` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-asset")]
pub struct DeleteAsset {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_asset(key: String) -> RemodelingMutation {
    RemodelingMutation::DeleteAsset(DeleteAsset { key })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for DeleteAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "asset", kind: "delete-asset", record: "DeletedAsset" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete asset \"{}\"", self.key)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
