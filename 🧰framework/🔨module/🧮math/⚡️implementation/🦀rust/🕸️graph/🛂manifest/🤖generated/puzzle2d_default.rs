// Generated from puzzle2d-default.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const PUZZLE2DDEFAULT_EDGE_EDGE_LINK: &str = "edge.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle2dDefaultEdgeKind {
    #[serde(rename = "edge.link")]
    EdgeLink,
}

impl Puzzle2dDefaultEdgeKind {
    pub const ALL: &'static [Self] = &[Puzzle2dDefaultEdgeKind::EdgeLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EdgeLink => "edge.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "edge.link" => Ok(Self::EdgeLink),
            other => Err(format!("unknown edge kind {other:?} for Puzzle2dDefault")),
        }
    }
}

pub const PUZZLE2DDEFAULT_EDGE_IDS: &[&str] = &["edge.link"];
pub const PUZZLE2DDEFAULT_PORT_PORT: &str = "port";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle2dDefaultPortKind {
    #[serde(rename = "port")]
    Port,
}

impl Puzzle2dDefaultPortKind {
    pub const ALL: &'static [Self] = &[Puzzle2dDefaultPortKind::Port];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Port => "port",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "port" => Ok(Self::Port),
            other => Err(format!("unknown port kind {other:?} for Puzzle2dDefault")),
        }
    }
}

pub const PUZZLE2DDEFAULT_PORT_IDS: &[&str] = &["port"];
pub const PUZZLE2DDEFAULT_WIRE_WIRE_LINK: &str = "wire.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle2dDefaultWireKind {
    #[serde(rename = "wire.link")]
    WireLink,
}

impl Puzzle2dDefaultWireKind {
    pub const ALL: &'static [Self] = &[Puzzle2dDefaultWireKind::WireLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WireLink => "wire.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "wire.link" => Ok(Self::WireLink),
            other => Err(format!("unknown wire kind {other:?} for Puzzle2dDefault")),
        }
    }
}

pub const PUZZLE2DDEFAULT_WIRE_IDS: &[&str] = &["wire.link"];
pub const PUZZLE2DDEFAULT_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"puzzle2d-default\",\"name\":\"Puzzle 2D Default\",\"axes\":{\"portModel\":\"ported\",\"directedness\":\"directed\"},\"portKinds\":[{\"id\":\"port\",\"name\":\"Port\",\"presentation\":{\"color\":\"var(--muted-foreground)\",\"defaultWireKind\":\"wire.link\"}}],\"wireKinds\":[{\"id\":\"wire.link\",\"name\":\"Link wire\",\"presentation\":{\"defaultEdgeKind\":\"edge.link\"}}],\"edgeKinds\":[{\"id\":\"edge.link\",\"name\":\"Link edge\"}],\"nodeKinds\":[],\"edgeTips\":[]}";

pub fn puzzle2d_default_manifest() -> Manifest {
    serde_json::from_str(PUZZLE2DDEFAULT_MANIFEST_JSON).expect("manifest json")
}
