//! 🧬️ Imperative diff schema — sparse field delta over the artifact.

use crate::artifacts::imperative::{Dictionary, Path, PathRef, Step};
use neural_engine::Value;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the imperative artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.imperative.imperative")]
pub struct ImperativeDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::imperative::schema::ImperativeArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub path: Option<ImperativePathDelta>,
    #[state(persistent)]
    pub seed: Option<BTreeMap<String, Value>>,
    #[state(shared_ui)]
    pub selected_step_ids: Option<ImperativeStringList>,
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
pub struct ImperativeStringList {
    pub values: Vec<String>,
}

/// 🧭 Step-list edit at a nested `PathRef`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ImperativePathDelta {
    pub path_ref: PathRef,
    pub steps: ImperativeStepsDelta,
}

/// 🧩 Identified-collection delta for a step list at `pathRef`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ImperativeStepsDelta {
    pub added: Vec<Step>,
    pub removed: Vec<String>,
    pub patched: Vec<ImperativeStepPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched step entry (params dictionary patch).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeStepPatchEntry {
    pub id: String,
    pub patch: Dictionary,
}
//#endregion 🔖️DeltaHelpers
