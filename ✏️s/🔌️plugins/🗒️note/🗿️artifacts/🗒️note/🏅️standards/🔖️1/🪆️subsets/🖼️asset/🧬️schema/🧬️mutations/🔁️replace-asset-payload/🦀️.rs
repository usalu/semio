//! 🔁 Note mutation — `ReplaceAssetPayload`: whole-value swap of an existing asset's image payload.

use crate::schema::mutations::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-asset-payload` payload — whole-value swap of an existing asset's image payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-asset-payload")]
pub struct ReplaceAssetPayload {
    pub key: String,
    #[dsl(block)]
    pub new_asset: crate::NoteImageAsset,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_asset_payload(key: String, new_asset: crate::NoteImageAsset) -> NoteMutation {
    NoteMutation::ReplaceAssetPayload(ReplaceAssetPayload { key, new_asset })
}

impl MutationKind<NoteSnapshot, NoteMutation> for ReplaceAssetPayload {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "asset", kind: "replace-asset-payload", record: "ReplacedAsset" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace asset \"{}\"", self.key)
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
