//! 🔁 Note mutation — `ReplaceAssetPayload`: whole-value swap of an existing asset's image payload.
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-asset-payload` payload — whole-value swap of an existing asset's image payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-asset-payload")]
pub struct ReplaceAssetPayload {
    pub key: String,
    #[dsl(block)]
    pub new_asset: crate::artifacts::note::NoteImageAsset,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_asset_payload(key: String, new_asset: crate::artifacts::note::NoteImageAsset) -> NoteMutation {
    NoteMutation::ReplaceAssetPayload(ReplaceAssetPayload { key, new_asset })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ReplaceAssetPayload {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "asset", kind: "replace-asset-payload", record: "ReplacedAsset" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace asset \"{}\"", self.key)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
