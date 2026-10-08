//! 📡️ Neutral graph scene records and retained explicit-framing materialization.
use protocol::{DomainHover,DomainSelection,SelectionMethod};
use semio_framework_ui_viewport::Viewport2d;
use serde::{Deserialize,Serialize};
#[cfg(test)]
use semio_framework_value::ToValue;
#[derive(Clone, Debug, Default, Deserialize, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GraphPortRecord {
    pub id: String,
    #[serde(default)]
    #[value(default)]
    pub label: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub code: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub abbreviation: Option<String>,
    #[serde(rename = "fullName", default)]
    #[value(rename = "fullName", default)]
    pub full_name: Option<String>,
    #[serde(rename = "resourceKind", default)]
    #[value(rename = "resourceKind", default)]
    pub artifact_kind: Option<String>,
    #[serde(rename = "valueType", default)]
    #[value(rename = "valueType", default)]
    pub value_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GraphNodeRecord {
    pub id: String,
    #[serde(default)]
    #[value(default)]
    pub label: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub instance_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub plugin_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub app_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub icon: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub x: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub y: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub width: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub height: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub inputs: Option<Vec<GraphPortRecord>>,
    #[serde(default)]
    #[value(default)]
    pub outputs: Option<Vec<GraphPortRecord>>,
}

#[derive(Clone, Debug, Default, Deserialize, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct GraphEdgeRecord {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

#[derive(Clone, Debug, Default)]
pub struct NodeGraphScenePayload {
    pub nodes: Vec<GraphNodeRecord>,
    pub edges: Vec<GraphEdgeRecord>,
    pub viewport: Option<Viewport2d>,
    pub preview_off_json: Option<String>,
    pub lod_json: Option<String>,
    pub controls_json: Option<String>,
    pub clusters_json: Option<String>,
    pub computing_json: Option<String>,
    pub status_json: Option<String>,
    pub capabilities_json: Option<String>,
    pub host_snapshot_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, semio_framework_value::ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SelectionGather {
    pub target_ids: Vec<String>,
    pub method: SelectionMethod,
}

#[path="📡️scene/🦀️.rs"]
mod scene;
pub use scene::{SceneDecodeCursor,SceneDecodeLimits,SceneDecodePhase,SceneDecodeStep};
#[cfg(test)]
#[path="📡️scene/🧪️tests/🦀️.rs"]
mod scene_retained_tests;
#[cfg(test)]
#[path="🧪️tests/🔌️ports/🦀️.rs"]
mod ports_tests;
