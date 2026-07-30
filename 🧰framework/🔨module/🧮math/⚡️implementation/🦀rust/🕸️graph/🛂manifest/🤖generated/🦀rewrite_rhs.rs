// Generated from 🛂manifest.jsonrewrite-rhs.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const REWRITERHS_NODE_REWRITE_SET: &str = "rewrite.set";
pub const REWRITERHS_NODE_REWRITE_PARAMETER: &str = "rewrite.parameter";
pub const REWRITERHS_NODE_REWRITE_CREATE: &str = "rewrite.create";
pub const REWRITERHS_NODE_REWRITE_DELETE: &str = "rewrite.delete";
pub const REWRITERHS_NODE_REWRITE_MERGE: &str = "rewrite.merge";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RewriteRhsNodeKind {
    #[serde(rename = "rewrite.set")]
    RewriteSet,
    #[serde(rename = "rewrite.parameter")]
    RewriteParameter,
    #[serde(rename = "rewrite.create")]
    RewriteCreate,
    #[serde(rename = "rewrite.delete")]
    RewriteDelete,
    #[serde(rename = "rewrite.merge")]
    RewriteMerge,
}

impl RewriteRhsNodeKind {
    pub const ALL: &'static [Self] = &[RewriteRhsNodeKind::RewriteSet, RewriteRhsNodeKind::RewriteParameter, RewriteRhsNodeKind::RewriteCreate, RewriteRhsNodeKind::RewriteDelete, RewriteRhsNodeKind::RewriteMerge];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RewriteSet => "rewrite.set",
            Self::RewriteParameter => "rewrite.parameter",
            Self::RewriteCreate => "rewrite.create",
            Self::RewriteDelete => "rewrite.delete",
            Self::RewriteMerge => "rewrite.merge",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "rewrite.set" => Ok(Self::RewriteSet),
            "rewrite.parameter" => Ok(Self::RewriteParameter),
            "rewrite.create" => Ok(Self::RewriteCreate),
            "rewrite.delete" => Ok(Self::RewriteDelete),
            "rewrite.merge" => Ok(Self::RewriteMerge),
            other => Err(format!("unknown node kind {other:?} for RewriteRhs")),
        }
    }
}

pub const REWRITERHS_NODE_IDS: &[&str] = &["rewrite.set", "rewrite.parameter", "rewrite.create", "rewrite.delete", "rewrite.merge"];
pub const REWRITERHS_EDGE_EDGE_FLOW: &str = "edge.flow";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RewriteRhsEdgeKind {
    #[serde(rename = "edge.flow")]
    EdgeFlow,
}

impl RewriteRhsEdgeKind {
    pub const ALL: &'static [Self] = &[RewriteRhsEdgeKind::EdgeFlow];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EdgeFlow => "edge.flow",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "edge.flow" => Ok(Self::EdgeFlow),
            other => Err(format!("unknown edge kind {other:?} for RewriteRhs")),
        }
    }
}

pub const REWRITERHS_EDGE_IDS: &[&str] = &["edge.flow"];
pub const REWRITERHS_PORT_PORT: &str = "port";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RewriteRhsPortKind {
    #[serde(rename = "port")]
    Port,
}

impl RewriteRhsPortKind {
    pub const ALL: &'static [Self] = &[RewriteRhsPortKind::Port];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Port => "port",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "port" => Ok(Self::Port),
            other => Err(format!("unknown port kind {other:?} for RewriteRhs")),
        }
    }
}

pub const REWRITERHS_PORT_IDS: &[&str] = &["port"];
pub const REWRITERHS_WIRE_WIRE_FLOW: &str = "wire.flow";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RewriteRhsWireKind {
    #[serde(rename = "wire.flow")]
    WireFlow,
}

impl RewriteRhsWireKind {
    pub const ALL: &'static [Self] = &[RewriteRhsWireKind::WireFlow];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WireFlow => "wire.flow",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "wire.flow" => Ok(Self::WireFlow),
            other => Err(format!("unknown wire kind {other:?} for RewriteRhs")),
        }
    }
}

pub const REWRITERHS_WIRE_IDS: &[&str] = &["wire.flow"];
pub const REWRITERHS_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"rewrite-rhs\",\"name\":\"Rewrite RHS\",\"axes\":{\"portModel\":\"ported\",\"directedness\":\"directed\"},\"portKinds\":[{\"id\":\"port\",\"name\":\"Port\",\"direction\":\"out\",\"properties\":[]}],\"wireKinds\":[{\"id\":\"wire.flow\",\"name\":\"Flow\",\"presentation\":{\"defaultEdgeKind\":\"edge.flow\"}}],\"edgeKinds\":[{\"id\":\"edge.flow\",\"name\":\"Flow\",\"presentation\":{\"directed\":true,\"targetTip\":\"filled-arrow\"}}],\"nodeKinds\":[{\"id\":\"rewrite.set\",\"name\":\"Set\",\"ports\":[\"port\"],\"presentation\":{\"color\":\"hsl(150 52% 42%)\",\"icon\":\"emoji:✏️\",\"handles\":[{\"handleKind\":\"port\",\"angle\":3.141592653589793,\"radius\":3},{\"handleKind\":\"port\",\"angle\":0,\"radius\":3}]}},{\"id\":\"rewrite.parameter\",\"name\":\"Parameter\",\"ports\":[\"port\"],\"presentation\":{\"color\":\"hsl(280 52% 52%)\",\"icon\":\"emoji:🎛️\",\"handles\":[{\"handleKind\":\"port\",\"angle\":0,\"radius\":3}]}},{\"id\":\"rewrite.create\",\"name\":\"Create\",\"ports\":[\"port\"],\"presentation\":{\"color\":\"hsl(95 52% 42%)\",\"icon\":\"emoji:➕\",\"handles\":[{\"handleKind\":\"port\",\"angle\":0,\"radius\":3}]}},{\"id\":\"rewrite.delete\",\"name\":\"Delete\",\"ports\":[\"port\"],\"presentation\":{\"color\":\"hsl(4 58% 50%)\",\"icon\":\"emoji:➖\",\"handles\":[{\"handleKind\":\"port\",\"angle\":0,\"radius\":3}]}},{\"id\":\"rewrite.merge\",\"name\":\"Merge\",\"ports\":[\"port\"],\"presentation\":{\"color\":\"hsl(200 52% 48%)\",\"icon\":\"emoji:🔀\",\"handles\":[{\"handleKind\":\"port\",\"angle\":0,\"radius\":3}]}}],\"edgeTips\":[]}";

pub fn rewrite_rhs_manifest() -> Manifest {
    serde_json::from_str(REWRITERHS_MANIFEST_JSON).expect("manifest json")
}
