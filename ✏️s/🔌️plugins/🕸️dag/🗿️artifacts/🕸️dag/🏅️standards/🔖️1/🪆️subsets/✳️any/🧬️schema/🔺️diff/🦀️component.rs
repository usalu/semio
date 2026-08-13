//! 🧬️ DAG diff schema — sparse field delta over the artifact.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes: Option<DagNodesDelta>` /
//! `edges: Option<DagEdgesDelta>` / `set_nodes` / `set_edges` are all gone — the composed child is
//! opaque (a parent's diff never embeds a child diff, per `📓️design-full-plan.md` §1's CHILD/LINK
//! split), so every triad now diffs by minting a whole new `content` handle
//! (`diff_replace_content`, see `🔺️diff/📝️text`) rather than building a structured delta. Single
//! `Option<DagContentChild>` — the slot is never absent, only ever replaced, matching writer's
//! `document`/flow's `content` field shape, not lowpoly's optional-slot `Option<Option<_>>`.
//!
//! `artifact: Option<Box<DagArtifact>>` (a whole-artifact-replace escape hatch) is also gone — it was
//! already dead (never constructed anywhere; `DagPlayApp` never overrides `whole_document_operation`)
//! and is exactly the forbidden whole-document-replace-via-diff shape `📌️important.md`'s vocabulary
//! policy bans. `DagNodesDelta`/`DagEdgesDelta`/`DagNodePatchEntry`/`DagNodeExtraPatch*`/
//! `DagEdgePatchEntry`/`DagNodeSpecList`/`DagFixtureEdgeList` are all dead with it — confirmed zero
//! remaining references after this pass.

use crate::artifacts::dag::{DagCamera, DagContentChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the DAG artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub content: Option<DagContentChild>,
    #[state(presence)]
    pub selected_node_ids: Option<DagStringList>,
    #[state(config)]
    pub camera: Option<DagCamera>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
