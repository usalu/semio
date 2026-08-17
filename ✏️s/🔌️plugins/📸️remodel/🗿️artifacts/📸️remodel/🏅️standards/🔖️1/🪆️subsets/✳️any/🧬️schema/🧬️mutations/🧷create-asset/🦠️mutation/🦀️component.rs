//! 🧷 Remodel mutation — `CreateAsset`: upserts one key-addressed `ImageAsset` (the only asset
//! write path in the app — import handlers always call this, overwriting is intentional so a retried
//! import with the same key lands cleanly).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{ImageAsset, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧷 `create-asset` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

    fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create asset \"{}\"", self.key)
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
