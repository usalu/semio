//! 🗞️ Remodel mutation — `DeleteAsset`: removes one key-addressed `ImageAsset`. No app call site
//! removes an asset today; this exists as `create-asset`'s inverse-only counterpart (a mutation kind
//! is real even without its own command call site) and for collection completeness.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗞️ `delete-asset` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-asset")]
pub struct DeleteAsset {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_asset(key: String) -> RemodelMutation {
    RemodelMutation::DeleteAsset(DeleteAsset { key })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for DeleteAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "asset", kind: "delete-asset", record: "DeletedAsset" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
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
