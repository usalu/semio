//! 🧬️ Playbook diff schema — sparse field delta over the artifact.

use crate::artifacts::playbook::{PlaybookBlock, PlaybookStep};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the playbook artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::playbook::schema::PlaybookArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub id: Option<String>,
    #[state(persistent)]
    pub version: Option<String>,
    #[state(persistent)]
    pub title: Option<Option<String>>,
    #[state(persistent)]
    pub steps: Option<PlaybookStepsDelta>,
    #[state(shared_ui)]
    pub selected_ids: Option<PlaybookStringList>,
    #[state(local_ui)]
    pub locale: Option<String>,
    #[state(local_ui)]
    pub contributions_json: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PlaybookStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `steps`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PlaybookStepsDelta {
    pub added: Vec<PlaybookStep>,
    pub removed: Vec<String>,
    pub patched: Vec<PlaybookStepPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Identified-collection delta for blocks inside a step.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PlaybookBlocksDelta {
    pub added: Vec<PlaybookBlock>,
    pub removed: Vec<String>,
    pub patched: Vec<PlaybookBlockPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched step entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookStepPatchEntry {
    pub id: String,
    pub patch: PlaybookStepPatch,
}

/// 🩹 One patched block entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookBlockPatchEntry {
    pub id: String,
    pub patch: PlaybookBlockPatch,
}

/// 🩹 Sparse patch record for a step.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PlaybookStepPatch {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub blocks: Option<PlaybookBlocksDelta>,
}

/// 🩹 Sparse patch record for a block — whole replacement when `block` is set.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PlaybookBlockPatch {
    pub block: Option<PlaybookBlock>,
}
//#endregion 🔖️DeltaHelpers
