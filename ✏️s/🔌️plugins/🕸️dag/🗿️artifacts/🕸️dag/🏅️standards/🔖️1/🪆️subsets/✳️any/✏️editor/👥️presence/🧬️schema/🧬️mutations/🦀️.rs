//! 🧬️ Semantic DagPresence mutation vocabulary and codecs.

use super::DagPresence;

#[path = "🔄️replace-presence/🦀️.rs"]
mod replace_presence;
pub use replace_presence::ReplacePresence;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", content = "payload", rename_all = "camelCase"))]
#[mutations(snapshot = DagPresence, diff = DagPresence, schema = "dag.presence")]
pub enum DagPresenceMutation {
    ReplacePresence(ReplacePresence),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
