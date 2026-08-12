//! 🧬️ DAG diff schema — sparse field delta over the artifact.

use crate::artifacts::dag::{DagCamera, DagFixtureEdge, DagNodePatch, DagNodeSpec};
use math::graph::manifest::PropertyBag;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the DAG artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::dag::schema::DagArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub nodes: Option<DagNodesDelta>,
    #[state(persistent)]
    pub edges: Option<DagEdgesDelta>,
    #[state(persistent)]
    pub set_nodes: Option<DagNodeSpecList>,
    #[state(persistent)]
    pub set_edges: Option<DagFixtureEdgeList>,
    #[state(shared_ui)]
    pub selected_node_ids: Option<DagStringList>,
    #[state(local_ui)]
    pub camera: Option<DagCamera>,
    #[state(local_ui)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagStringList {
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagNodeSpecList {
    pub values: Vec<DagNodeSpec>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagFixtureEdgeList {
    pub values: Vec<DagFixtureEdge>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagNodesDelta {
    pub added: Vec<DagNodeSpec>,
    pub removed: Vec<String>,
    pub patched: Vec<DagNodePatchEntry>,
    /// 🩹️ Fields `infinite_board_port_directed_dag::DagNodePatch` (foreign, out of this plugin's
    /// bounds) has no slot for — the node's own `id`, `icon`, `abbreviation`, `operatorKind` and
    /// `properties` — applied in place so node order never shifts (unlike a removed+re-added entry).
    pub extra_patched: Vec<DagNodeExtraPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagEdgesDelta {
    pub added: Vec<DagFixtureEdge>,
    pub removed: Vec<String>,
    pub patched: Vec<DagEdgePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagNodePatchEntry {
    pub id: String,
    pub patch: DagNodePatch,
}

/// 🩹️ Sparse patch for the `DagNodeSpec` fields `DagNodePatch` doesn't carry. `operator_kind` is
/// double-`Option`ed (`Some(None)` clears it, `None` leaves it untouched) since the field itself
/// is already `Option<String>`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagNodeExtraPatch {
    pub new_id: Option<String>,
    pub icon: Option<String>,
    pub abbreviation: Option<String>,
    pub operator_kind: Option<Option<String>>,
    pub properties: Option<PropertyBag>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagNodeExtraPatchEntry {
    pub id: String,
    pub patch: DagNodeExtraPatch,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagEdgePatchEntry {
    pub id: String,
    pub patch: infinite_board_port_directed_dag::DagEdgePatch,
}
//#endregion 🔖️DeltaHelpers
