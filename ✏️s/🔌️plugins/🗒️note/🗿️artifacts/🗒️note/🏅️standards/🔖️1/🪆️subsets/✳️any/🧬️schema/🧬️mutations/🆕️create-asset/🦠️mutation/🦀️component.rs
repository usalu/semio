//! 🆕 Note mutation — `CreateAsset`: brings a new id-keyed image asset into existence.
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🆕 `create-asset` payload — brings a new id-keyed image asset into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-asset")]
pub struct CreateAsset {
    pub key: String,
    #[dsl(block)]
    pub asset: crate::artifacts::note::NoteImageAsset,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_asset(key: String, asset: crate::artifacts::note::NoteImageAsset) -> NoteMutation {
    NoteMutation::CreateAsset(CreateAsset { key, asset })
}

impl MutationKind<NoteSnapshot, NoteMutation> for CreateAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "asset", kind: "create-asset", record: "CreatedAsset" };

    async fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &NoteSnapshot) -> Vec<NoteMutation> {
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
