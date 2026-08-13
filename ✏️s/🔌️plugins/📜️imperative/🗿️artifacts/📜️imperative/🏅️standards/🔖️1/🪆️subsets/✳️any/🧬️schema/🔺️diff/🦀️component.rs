//! 🧬️ Imperative diff schema — sparse field delta over the artifact.

use crate::artifacts::imperative::{ImperativeFlowChild, ImperativeTextChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the imperative artifact; persistent entries apply via
/// [`MutationDiff`](protocol::MutationDiff). `flow`/`text` carry a whole-handle replacement
/// (content-addressed, so a changed handle IS the change signal — see
/// `📓️wave3-reports/writer-report.md`'s `document: Option<WriterDocumentChild>` precedent; both
/// slots are never absent, only ever replaced, so a single `Option<…Child>` — not the double-
/// `Option` an optional slot needs — is the sparse-vs-unchanged signal here).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.imperative.imperative")]
pub struct ImperativeDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::imperative::schema::ImperativeArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub flow: Option<ImperativeFlowChild>,
    #[state(persistent)]
    pub text: Option<ImperativeTextChild>,
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
//#endregion 🔖️DeltaHelpers
