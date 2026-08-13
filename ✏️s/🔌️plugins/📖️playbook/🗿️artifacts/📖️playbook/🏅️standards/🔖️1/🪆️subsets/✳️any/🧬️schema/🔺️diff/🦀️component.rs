//! 🧬️ Playbook diff schema — sparse field delta over the artifact.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`playbook→C:document,flow`): the identified-
//! collection `steps: Option<PlaybookStepsDelta>` (and its nested `PlaybookBlocksDelta`/
//! `PlaybookStepPatch`/`PlaybookBlockPatch`) is replaced by single-Option whole-handle-replace
//! `document`/`flow` fields — the slots are never absent, only ever replaced, matching writer's
//! `document`/flow's `content` fields exactly (not `Option<Option<…>>` — that shape is for a slot
//! whose PRESENCE itself can change, e.g. lowpoly's `mesh`, which does not apply here).

use crate::artifacts::playbook::{PlaybookDocumentChild, PlaybookFlowChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the playbook artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::playbook::schema::PlaybookArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub version: Option<String>,
    #[state(artifact)]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    pub document: Option<PlaybookDocumentChild>,
    #[state(artifact)]
    pub flow: Option<PlaybookFlowChild>,
    #[state(presence)]
    pub selected_ids: Option<PlaybookStringList>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(config)]
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
//#endregion 🔖️DeltaHelpers
