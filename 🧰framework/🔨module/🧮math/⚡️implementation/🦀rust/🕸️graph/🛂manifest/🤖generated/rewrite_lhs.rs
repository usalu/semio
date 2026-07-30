// Generated from rewrite-lhs.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const REWRITELHS_NODE_REWRITE_MATCH: &str = "rewrite.match";
pub const REWRITELHS_NODE_REWRITE_WHERE: &str = "rewrite.where";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RewriteLhsNodeKind {
    #[serde(rename = "rewrite.match")]
    RewriteMatch,
    #[serde(rename = "rewrite.where")]
    RewriteWhere,
}

impl RewriteLhsNodeKind {
    pub const ALL: &'static [Self] = &[RewriteLhsNodeKind::RewriteMatch, RewriteLhsNodeKind::RewriteWhere];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RewriteMatch => "rewrite.match",
            Self::RewriteWhere => "rewrite.where",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "rewrite.match" => Ok(Self::RewriteMatch),
            "rewrite.where" => Ok(Self::RewriteWhere),
            other => Err(format!("unknown node kind {other:?} for RewriteLhs")),
        }
    }
}

pub const REWRITELHS_NODE_IDS: &[&str] = &["rewrite.match", "rewrite.where"];
pub const REWRITELHS_EDGE_EDGE_FLOW: &str = "edge.flow";
pub const REWRITELHS_EDGE_EDGE_PATTERN: &str = "edge.pattern";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RewriteLhsEdgeKind {
    #[serde(rename = "edge.flow")]
    EdgeFlow,
    #[serde(rename = "edge.pattern")]
    EdgePattern,
}

impl RewriteLhsEdgeKind {
    pub const ALL: &'static [Self] = &[RewriteLhsEdgeKind::EdgeFlow, RewriteLhsEdgeKind::EdgePattern];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EdgeFlow => "edge.flow",
            Self::EdgePattern => "edge.pattern",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "edge.flow" => Ok(Self::EdgeFlow),
            "edge.pattern" => Ok(Self::EdgePattern),
            other => Err(format!("unknown edge kind {other:?} for RewriteLhs")),
        }
    }
}

pub const REWRITELHS_EDGE_IDS: &[&str] = &["edge.flow", "edge.pattern"];
pub const REWRITELHS_PORT_PORT: &str = "port";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RewriteLhsPortKind {
    #[serde(rename = "port")]
    Port,
}

impl RewriteLhsPortKind {
    pub const ALL: &'static [Self] = &[RewriteLhsPortKind::Port];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Port => "port",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "port" => Ok(Self::Port),
            other => Err(format!("unknown port kind {other:?} for RewriteLhs")),
        }
    }
}

pub const REWRITELHS_PORT_IDS: &[&str] = &["port"];
pub const REWRITELHS_WIRE_WIRE_FLOW: &str = "wire.flow";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RewriteLhsWireKind {
    #[serde(rename = "wire.flow")]
    WireFlow,
}

impl RewriteLhsWireKind {
    pub const ALL: &'static [Self] = &[RewriteLhsWireKind::WireFlow];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WireFlow => "wire.flow",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "wire.flow" => Ok(Self::WireFlow),
            other => Err(format!("unknown wire kind {other:?} for RewriteLhs")),
        }
    }
}

pub const REWRITELHS_WIRE_IDS: &[&str] = &["wire.flow"];
pub const REWRITELHS_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"rewrite-lhs\",\"name\":\"Rewrite LHS\",\"axes\":{\"portModel\":\"ported\",\"directedness\":\"directed\"},\"portKinds\":[{\"id\":\"port\",\"name\":\"Port\",\"direction\":\"out\",\"properties\":[]}],\"wireKinds\":[{\"id\":\"wire.flow\",\"name\":\"Flow\",\"presentation\":{\"defaultEdgeKind\":\"edge.flow\"}}],\"edgeKinds\":[{\"id\":\"edge.flow\",\"name\":\"Flow\",\"presentation\":{\"directed\":true,\"targetTip\":\"filled-arrow\"}},{\"id\":\"edge.pattern\",\"name\":\"Pattern\",\"presentation\":{\"directed\":true,\"targetTip\":\"filled-arrow\"}}],\"nodeKinds\":[{\"id\":\"rewrite.match\",\"name\":\"Match\",\"ports\":[\"port\"],\"presentation\":{\"color\":\"hsl(210 58% 48%)\",\"icon\":\"emoji:🎯\",\"handles\":[{\"handleKind\":\"port\",\"angle\":0,\"radius\":3},{\"handleKind\":\"port\",\"angle\":3.141592653589793,\"radius\":3}]}},{\"id\":\"rewrite.where\",\"name\":\"Where\",\"ports\":[\"port\"],\"presentation\":{\"color\":\"hsl(42 58% 48%)\",\"icon\":\"emoji:🔍\",\"handles\":[{\"handleKind\":\"port\",\"angle\":3.141592653589793,\"radius\":3},{\"handleKind\":\"port\",\"angle\":0,\"radius\":3}]}}],\"edgeTips\":[]}";

pub fn rewrite_lhs_manifest() -> Manifest {
    serde_json::from_str(REWRITELHS_MANIFEST_JSON).expect("manifest json")
}
