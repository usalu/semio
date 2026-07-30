// Generated from puzzle5d-🛂manifest.jsondefault.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const PUZZLE5DDEFAULT_EDGE_EDGE_LINK: &str = "edge.link";
pub const PUZZLE5DDEFAULT_EDGE_ATTRACTION_LINK: &str = "attraction.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle5dDefaultEdgeKind {
    #[serde(rename = "edge.link")]
    EdgeLink,
    #[serde(rename = "attraction.link")]
    AttractionLink,
}

impl Puzzle5dDefaultEdgeKind {
    pub const ALL: &'static [Self] = &[Puzzle5dDefaultEdgeKind::EdgeLink, Puzzle5dDefaultEdgeKind::AttractionLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EdgeLink => "edge.link",
            Self::AttractionLink => "attraction.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "edge.link" => Ok(Self::EdgeLink),
            "attraction.link" => Ok(Self::AttractionLink),
            other => Err(format!("unknown edge kind {other:?} for Puzzle5dDefault")),
        }
    }
}

pub const PUZZLE5DDEFAULT_EDGE_IDS: &[&str] = &["edge.link", "attraction.link"];
pub const PUZZLE5DDEFAULT_PORT_PORT: &str = "port";
pub const PUZZLE5DDEFAULT_PORT_VORTEX: &str = "vortex";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle5dDefaultPortKind {
    #[serde(rename = "port")]
    Port,
    #[serde(rename = "vortex")]
    Vortex,
}

impl Puzzle5dDefaultPortKind {
    pub const ALL: &'static [Self] = &[Puzzle5dDefaultPortKind::Port, Puzzle5dDefaultPortKind::Vortex];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Port => "port",
            Self::Vortex => "vortex",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "port" => Ok(Self::Port),
            "vortex" => Ok(Self::Vortex),
            other => Err(format!("unknown port kind {other:?} for Puzzle5dDefault")),
        }
    }
}

pub const PUZZLE5DDEFAULT_PORT_IDS: &[&str] = &["port", "vortex"];
pub const PUZZLE5DDEFAULT_WIRE_WIRE_LINK: &str = "wire.link";
pub const PUZZLE5DDEFAULT_WIRE_CABLE_LINK: &str = "cable.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle5dDefaultWireKind {
    #[serde(rename = "wire.link")]
    WireLink,
    #[serde(rename = "cable.link")]
    CableLink,
}

impl Puzzle5dDefaultWireKind {
    pub const ALL: &'static [Self] = &[Puzzle5dDefaultWireKind::WireLink, Puzzle5dDefaultWireKind::CableLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WireLink => "wire.link",
            Self::CableLink => "cable.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "wire.link" => Ok(Self::WireLink),
            "cable.link" => Ok(Self::CableLink),
            other => Err(format!("unknown wire kind {other:?} for Puzzle5dDefault")),
        }
    }
}

pub const PUZZLE5DDEFAULT_WIRE_IDS: &[&str] = &["wire.link", "cable.link"];
pub const PUZZLE5DDEFAULT_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"puzzle5d-default\",\"name\":\"Puzzle 5D Default\",\"axes\":{\"portModel\":\"ported\",\"directedness\":\"directed\"},\"portKinds\":[{\"id\":\"port\",\"name\":\"Port\",\"presentation\":{\"color\":\"var(--muted-foreground)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"vortex\",\"name\":\"Vortex\",\"presentation\":{\"defaultWireKind\":\"cable.link\"}}],\"wireKinds\":[{\"id\":\"wire.link\",\"name\":\"Link wire\",\"presentation\":{\"defaultEdgeKind\":\"edge.link\"}},{\"id\":\"cable.link\",\"name\":\"Cable\",\"presentation\":{\"defaultEdgeKind\":\"attraction.link\"}}],\"edgeKinds\":[{\"id\":\"edge.link\",\"name\":\"Link edge\"},{\"id\":\"attraction.link\",\"name\":\"Attraction\"}],\"nodeKinds\":[],\"kindCompatibility\":[{\"source\":\"port\",\"target\":\"port\",\"bidirectional\":true},{\"source\":\"vortex\",\"target\":\"vortex\",\"bidirectional\":true}]}";

pub fn puzzle5d_default_manifest() -> Manifest {
    serde_json::from_str(PUZZLE5DDEFAULT_MANIFEST_JSON).expect("manifest json")
}
