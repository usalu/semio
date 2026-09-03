//! 🧬️ Imperative diff schema — sparse field delta over the artifact.

use crate::artifacts::procedure::{ProcedureFlowChild, ProcedureTextChild};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the imperative artifact; persistent entries apply via
/// [`MutationDiff`](protocol::MutationDiff). `flow`/`text` carry a whole-handle replacement
/// (content-addressed, so a changed handle IS the change signal — see
/// `📓️wave3-reports/writer-report.md`'s `document: Option<WriterDocumentChild>` precedent; both
/// slots are never absent, only ever replaced, so a single `Option<…Child>` — not the double-
/// `Option` an optional slot needs — is the sparse-vs-unchanged signal here).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.imperative.procedure")]
pub struct ProcedureDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::procedure::schema::ProcedureArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub flow: Option<ProcedureFlowChild>,
    #[state(artifact)]
    pub text: Option<ProcedureTextChild>,
    #[state(presence)]
    pub selected_step_ids: Option<ProcedureStringList>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(config)]
    pub contributions_json: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ProcedureStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
