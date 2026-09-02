//! 🧷 Remodel mutation — `CreateAsset`: upserts one key-addressed `ImageAsset` (the only asset
//! write path in the app — import handlers always call this, overwriting is intentional so a retried
//! import with the same key lands cleanly).

use crate::artifacts::remodel::{ImageAsset, RemodelSnapshot, durable_remodel_asset, remodel_asset, store_remodel_asset};
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🧷 `create-asset` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-asset")]
pub struct CreateAsset {
    pub key: String,
    #[dsl(block)]
    pub asset: ImageAsset,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_asset(key: String, asset: ImageAsset) -> RemodelMutation {
    RemodelMutation::CreateAsset(CreateAsset { key, asset })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for CreateAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "asset", kind: "create-asset", record: "CreatedAsset" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create asset \"{}\"", self.key)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
