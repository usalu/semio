// Generated from wires.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const WIRES_EDGE_WIRES_OWNS: &str = "wires.owns";
pub const WIRES_EDGE_WIRES_IS: &str = "wires.is";
pub const WIRES_EDGE_WIRES_REFERENCES: &str = "wires.references";
pub const WIRES_EDGE_WIRES_HAS: &str = "wires.has";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WiresEdgeKind {
    #[serde(rename = "wires.owns")]
    WiresOwns,
    #[serde(rename = "wires.is")]
    WiresIs,
    #[serde(rename = "wires.references")]
    WiresReferences,
    #[serde(rename = "wires.has")]
    WiresHas,
}

impl WiresEdgeKind {
    pub const ALL: &'static [Self] = &[WiresEdgeKind::WiresOwns, WiresEdgeKind::WiresIs, WiresEdgeKind::WiresReferences, WiresEdgeKind::WiresHas];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WiresOwns => "wires.owns",
            Self::WiresIs => "wires.is",
            Self::WiresReferences => "wires.references",
            Self::WiresHas => "wires.has",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "wires.owns" => Ok(Self::WiresOwns),
            "wires.is" => Ok(Self::WiresIs),
            "wires.references" => Ok(Self::WiresReferences),
            "wires.has" => Ok(Self::WiresHas),
            other => Err(format!("unknown edge kind {other:?} for Wires")),
        }
    }
}

pub const WIRES_EDGE_IDS: &[&str] = &["wires.owns", "wires.is", "wires.references", "wires.has"];
pub const WIRES_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"wires\",\"name\":\"WIRES Mindmap\",\"axes\":{\"portModel\":\"normal\",\"directedness\":\"undirected\"},\"edgeKinds\":[{\"id\":\"wires.owns\",\"name\":\"Owns\"},{\"id\":\"wires.is\",\"name\":\"Is\"},{\"id\":\"wires.references\",\"name\":\"References\"},{\"id\":\"wires.has\",\"name\":\"Has\"}]}";

pub fn wires_manifest() -> Manifest {
    serde_json::from_str(WIRES_MANIFEST_JSON).expect("manifest json")
}
