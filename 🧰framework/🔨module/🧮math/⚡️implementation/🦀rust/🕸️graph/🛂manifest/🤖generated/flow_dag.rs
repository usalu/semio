// Generated from flow-dag.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const FLOWDAG_NODE_COMPUTATION: &str = "computation";
pub const FLOWDAG_NODE_SLIDER: &str = "slider";
pub const FLOWDAG_NODE_SELECT: &str = "select";
pub const FLOWDAG_NODE_SCREEN: &str = "screen";
pub const FLOWDAG_NODE_NOTE: &str = "note";
pub const FLOWDAG_NODE_IMAGE: &str = "image";
pub const FLOWDAG_NODE_PREVIEW: &str = "preview";
pub const FLOWDAG_NODE_ACTION: &str = "action";
pub const FLOWDAG_NODE_EXPORT: &str = "export";
pub const FLOWDAG_NODE_CLUSTER: &str = "cluster";
pub const FLOWDAG_NODE_APP_INSTANCE: &str = "appInstance";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FlowDagNodeKind {
    #[serde(rename = "computation")]
    Computation,
    #[serde(rename = "slider")]
    Slider,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "screen")]
    Screen,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "export")]
    Export,
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "appInstance")]
    AppInstance,
}

impl FlowDagNodeKind {
    pub const ALL: &'static [Self] = &[FlowDagNodeKind::Computation, FlowDagNodeKind::Slider, FlowDagNodeKind::Select, FlowDagNodeKind::Screen, FlowDagNodeKind::Note, FlowDagNodeKind::Image, FlowDagNodeKind::Preview, FlowDagNodeKind::Action, FlowDagNodeKind::Export, FlowDagNodeKind::Cluster, FlowDagNodeKind::AppInstance];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Computation => "computation",
            Self::Slider => "slider",
            Self::Select => "select",
            Self::Screen => "screen",
            Self::Note => "note",
            Self::Image => "image",
            Self::Preview => "preview",
            Self::Action => "action",
            Self::Export => "export",
            Self::Cluster => "cluster",
            Self::AppInstance => "appInstance",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "computation" => Ok(Self::Computation),
            "slider" => Ok(Self::Slider),
            "select" => Ok(Self::Select),
            "screen" => Ok(Self::Screen),
            "note" => Ok(Self::Note),
            "image" => Ok(Self::Image),
            "preview" => Ok(Self::Preview),
            "action" => Ok(Self::Action),
            "export" => Ok(Self::Export),
            "cluster" => Ok(Self::Cluster),
            "appInstance" => Ok(Self::AppInstance),
            other => Err(format!("unknown node kind {other:?} for FlowDag")),
        }
    }
}

pub const FLOWDAG_NODE_IDS: &[&str] = &["computation", "slider", "select", "screen", "note", "image", "preview", "action", "export", "cluster", "appInstance"];
pub const FLOWDAG_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"flow-dag\",\"name\":\"Flow DAG\",\"axes\":{\"portModel\":\"ported\",\"directedness\":\"directed\"},\"nodeKinds\":[{\"id\":\"computation\",\"name\":\"Computation\"},{\"id\":\"slider\",\"name\":\"Slider\"},{\"id\":\"select\",\"name\":\"Select\"},{\"id\":\"screen\",\"name\":\"Screen\"},{\"id\":\"note\",\"name\":\"Note\"},{\"id\":\"image\",\"name\":\"Image\"},{\"id\":\"preview\",\"name\":\"Preview\"},{\"id\":\"action\",\"name\":\"Action\"},{\"id\":\"export\",\"name\":\"Export\"},{\"id\":\"cluster\",\"name\":\"Cluster\"},{\"id\":\"appInstance\",\"name\":\"App Instance\"}]}";

pub fn flow_dag_manifest() -> Manifest {
    serde_json::from_str(FLOWDAG_MANIFEST_JSON).expect("manifest json")
}
