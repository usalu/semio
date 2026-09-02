//! 🧬️ Jack diff schema — sparse field delta over the artifact.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` deltas collapsed into a
//! single `content: Option<JackContentChild>` (always-present slot, never-absent-only-replaced shape
//! — matches `dag`'s/`writer`'s precedent, not `lowpoly`'s `Option<Option<_>>` optional-slot shape,
//! since jack's content always exists). `JackNodesDelta`/`JackEdgesDelta`/`JackNodePatch*`/
//! `JackEdgePatch*` are gone — every triad's `🔺️diff` now reads the current scene via
//! `jack_working_scene(base)`, applies its own specific semantics to a clone, and calls
//! `diff_replace_content`.

use crate::artifacts::jack::schema::JackEditorSelection;
use crate::artifacts::jack::{Camera, JackContentChild};
use schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the jack artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub name: Option<String>,
    #[state(artifact)]
    pub manifest_id: Option<Option<String>>,
    #[state(artifact)]
    pub manifest: Option<crate::artifacts::jack::Manifest>,
    #[state(artifact)]
    pub camera: Option<Camera>,
    #[state(artifact)]
    pub content: Option<JackContentChild>,
    #[state(artifact)]
    pub root_node_id: Option<Option<String>>,
    #[state(presence)]
    pub active_fixture_id: Option<String>,
    #[state(presence)]
    pub jack_query: Option<String>,
    #[state(presence)]
    pub lod_mode_by_window: Option<BTreeMap<String, Option<String>>>,
    #[state(config)]
    pub viewport_camera: Option<Camera>,
    #[state(config)]
    pub jack_result_json: Option<String>,
    #[state(config)]
    pub editor_engagement_input: Option<String>,
    #[state(config)]
    pub graph_engagement_input: Option<String>,
    #[state(config)]
    pub results_engagement_input: Option<String>,
    #[state(config)]
    pub reorganize_epoch: Option<u64>,
    #[state(config)]
    pub editor_selection: Option<Option<JackEditorSelection>>,
    #[state(config)]
    pub revision: Option<u64>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff
