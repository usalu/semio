//! 🔄️ Sync, merge and scope of the shell schema: sync cards, artifact sync status, inference port status, merge policy and conflicts and the storage scope. See [`super`].

use super::*;

//#region 🔄️Sync
/// 🗂️ Check-in target type (audit: "check-in file/folder/remote type").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum SyncCardKind {
    File,
    Folder,
    Remote,
}

/// 🩺️ Per-document sync health (audit: `ArtifactSyncStatus`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArtifactSyncStatus {
    Clean,
    Dirty,
    Syncing,
    Errored { message: String },
}
//#endregion 🔄️Sync

/// 📣 Opaque owner-authored ephemeral service status scoped to one document.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(crate="::protocol::value",rename_all="camelCase",deny_unknown_fields)]
pub struct InferencePortStatus {
    pub owner:String,
    pub service_id:String,
    #[value(with="json_value_bridge")]
    pub payload:JsonValue,
}

//#region 🤝️Merge
/// 🤝️ Conflict resolution strategy (audit: `MergePolicy`, persisted).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum MergePolicy {
    PreferLocal,
    PreferRemote,
    Manual,
}

/// ⚠️ One open conflict on the roster (audit: `Conflict`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct Conflict {
    pub conflict_id: String,
    pub document_id: String,
    pub description: String,
}
//#endregion 🤝️Merge

//#region 💾️Host
/// 💾️ Storage backend for shell state persistence (ShellHost `scope` useState, audit §2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum ShellScope {
    LocalStorage,
    Memory,
}
//#endregion 💾️Host
