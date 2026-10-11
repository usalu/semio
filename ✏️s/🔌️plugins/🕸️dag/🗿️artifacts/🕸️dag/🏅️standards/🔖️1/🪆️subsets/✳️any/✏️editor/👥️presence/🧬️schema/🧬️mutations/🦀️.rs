//! 🧬️ Semantic DagPresence mutation vocabulary and codecs.

use super::{DagPresence, DagPresenceDiff};

#[path = "🔄️replace-presence/🦀️.rs"]
mod replace_presence;
pub use replace_presence::ReplacePresence;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", content = "payload", rename_all = "camelCase"))]
#[mutations(snapshot = DagPresence, diff = DagPresenceDiff, schema = "dag.presence")]
pub enum DagPresenceMutation {
    ReplacePresence(ReplacePresence),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
