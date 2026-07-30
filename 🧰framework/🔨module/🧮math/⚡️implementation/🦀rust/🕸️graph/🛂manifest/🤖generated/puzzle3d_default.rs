// Generated from puzzle3d-default.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const PUZZLE3DDEFAULT_EDGE_PUZZLE3D_ATTRACTION_LINK: &str = "puzzle3d.attraction.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle3dDefaultEdgeKind {
    #[serde(rename = "puzzle3d.attraction.link")]
    Puzzle3dAttractionLink,
}

impl Puzzle3dDefaultEdgeKind {
    pub const ALL: &'static [Self] = &[Puzzle3dDefaultEdgeKind::Puzzle3dAttractionLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Puzzle3dAttractionLink => "puzzle3d.attraction.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "puzzle3d.attraction.link" => Ok(Self::Puzzle3dAttractionLink),
            other => Err(format!("unknown edge kind {other:?} for Puzzle3dDefault")),
        }
    }
}

pub const PUZZLE3DDEFAULT_EDGE_IDS: &[&str] = &["puzzle3d.attraction.link"];
pub const PUZZLE3DDEFAULT_PORT_VORTEX: &str = "vortex";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle3dDefaultPortKind {
    #[serde(rename = "vortex")]
    Vortex,
}

impl Puzzle3dDefaultPortKind {
    pub const ALL: &'static [Self] = &[Puzzle3dDefaultPortKind::Vortex];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vortex => "vortex",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "vortex" => Ok(Self::Vortex),
            other => Err(format!("unknown port kind {other:?} for Puzzle3dDefault")),
        }
    }
}

pub const PUZZLE3DDEFAULT_PORT_IDS: &[&str] = &["vortex"];
pub const PUZZLE3DDEFAULT_WIRE_CABLE_LINK: &str = "cable.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle3dDefaultWireKind {
    #[serde(rename = "cable.link")]
    CableLink,
}

impl Puzzle3dDefaultWireKind {
    pub const ALL: &'static [Self] = &[Puzzle3dDefaultWireKind::CableLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CableLink => "cable.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "cable.link" => Ok(Self::CableLink),
            other => Err(format!("unknown wire kind {other:?} for Puzzle3dDefault")),
        }
    }
}

pub const PUZZLE3DDEFAULT_WIRE_IDS: &[&str] = &["cable.link"];
pub const PUZZLE3DDEFAULT_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"puzzle3d-default\",\"name\":\"Puzzle 3D Default\",\"axes\":{\"portModel\":\"ported\",\"directedness\":\"directed\"},\"portKinds\":[{\"id\":\"vortex\",\"name\":\"Vortex\",\"presentation\":{\"defaultWireKind\":\"cable.link\"}}],\"wireKinds\":[{\"id\":\"cable.link\",\"name\":\"Cable\",\"presentation\":{\"defaultEdgeKind\":\"puzzle3d.attraction.link\"}}],\"edgeKinds\":[{\"id\":\"puzzle3d.attraction.link\",\"name\":\"Attraction\"}],\"nodeKinds\":[]}";

pub fn puzzle3d_default_manifest() -> Manifest {
    serde_json::from_str(PUZZLE3DDEFAULT_MANIFEST_JSON).expect("manifest json")
}
