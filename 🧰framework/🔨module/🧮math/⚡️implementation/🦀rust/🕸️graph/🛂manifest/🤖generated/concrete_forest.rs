// Generated from concrete-forest.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const CONCRETEFOREST_NODE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT: &str = "Hexagonal Cut Concrete Forest Left";
pub const CONCRETEFOREST_NODE_HEXAGONAL_CUT_CONCRETE_FOREST_RIGHT: &str = "Hexagonal Cut Concrete Forest Right";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConcreteForestNodeKind {
    #[serde(rename = "Hexagonal Cut Concrete Forest Left")]
    HexagonalCutConcreteForestLeft,
    #[serde(rename = "Hexagonal Cut Concrete Forest Right")]
    HexagonalCutConcreteForestRight,
}

impl ConcreteForestNodeKind {
    pub const ALL: &'static [Self] = &[ConcreteForestNodeKind::HexagonalCutConcreteForestLeft, ConcreteForestNodeKind::HexagonalCutConcreteForestRight];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HexagonalCutConcreteForestLeft => "Hexagonal Cut Concrete Forest Left",
            Self::HexagonalCutConcreteForestRight => "Hexagonal Cut Concrete Forest Right",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "Hexagonal Cut Concrete Forest Left" => Ok(Self::HexagonalCutConcreteForestLeft),
            "Hexagonal Cut Concrete Forest Right" => Ok(Self::HexagonalCutConcreteForestRight),
            other => Err(format!("unknown node kind {other:?} for ConcreteForest")),
        }
    }
}

pub const CONCRETEFOREST_NODE_IDS: &[&str] = &["Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Right"];
pub const CONCRETEFOREST_EDGE_PUZZLE3D_ATTRACTION_LINK: &str = "puzzle3d.attraction.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConcreteForestEdgeKind {
    #[serde(rename = "puzzle3d.attraction.link")]
    Puzzle3dAttractionLink,
}

impl ConcreteForestEdgeKind {
    pub const ALL: &'static [Self] = &[ConcreteForestEdgeKind::Puzzle3dAttractionLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Puzzle3dAttractionLink => "puzzle3d.attraction.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "puzzle3d.attraction.link" => Ok(Self::Puzzle3dAttractionLink),
            other => Err(format!("unknown edge kind {other:?} for ConcreteForest")),
        }
    }
}

pub const CONCRETEFOREST_EDGE_IDS: &[&str] = &["puzzle3d.attraction.link"];
pub const CONCRETEFOREST_PORT_BL: &str = "b-l";
pub const CONCRETEFOREST_PORT_BLM: &str = "b-l-m";
pub const CONCRETEFOREST_PORT_BS: &str = "b-s";
pub const CONCRETEFOREST_PORT_BSM: &str = "b-s-m";
pub const CONCRETEFOREST_PORT_CB: &str = "c-b";
pub const CONCRETEFOREST_PORT_CT: &str = "c-t";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConcreteForestPortKind {
    #[serde(rename = "b-l")]
    BL,
    #[serde(rename = "b-l-m")]
    BLM,
    #[serde(rename = "b-s")]
    BS,
    #[serde(rename = "b-s-m")]
    BSM,
    #[serde(rename = "c-b")]
    CB,
    #[serde(rename = "c-t")]
    CT,
}

impl ConcreteForestPortKind {
    pub const ALL: &'static [Self] = &[ConcreteForestPortKind::BL, ConcreteForestPortKind::BLM, ConcreteForestPortKind::BS, ConcreteForestPortKind::BSM, ConcreteForestPortKind::CB, ConcreteForestPortKind::CT];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BL => "b-l",
            Self::BLM => "b-l-m",
            Self::BS => "b-s",
            Self::BSM => "b-s-m",
            Self::CB => "c-b",
            Self::CT => "c-t",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "b-l" => Ok(Self::BL),
            "b-l-m" => Ok(Self::BLM),
            "b-s" => Ok(Self::BS),
            "b-s-m" => Ok(Self::BSM),
            "c-b" => Ok(Self::CB),
            "c-t" => Ok(Self::CT),
            other => Err(format!("unknown port kind {other:?} for ConcreteForest")),
        }
    }
}

pub const CONCRETEFOREST_PORT_IDS: &[&str] = &["b-l", "b-l-m", "b-s", "b-s-m", "c-b", "c-t"];
pub const CONCRETEFOREST_WIRE_CABLE_LINK: &str = "cable.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConcreteForestWireKind {
    #[serde(rename = "cable.link")]
    CableLink,
}

impl ConcreteForestWireKind {
    pub const ALL: &'static [Self] = &[ConcreteForestWireKind::CableLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CableLink => "cable.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "cable.link" => Ok(Self::CableLink),
            other => Err(format!("unknown wire kind {other:?} for ConcreteForest")),
        }
    }
}

pub const CONCRETEFOREST_WIRE_IDS: &[&str] = &["cable.link"];
pub const CONCRETEFOREST_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"concrete-forest\",\"name\":\"Concrete Forest\",\"axes\":{\"portModel\":\"ported\",\"directedness\":\"directed\"},\"portKinds\":[{\"id\":\"b-l\",\"name\":\"b-l\",\"presentation\":{\"color\":\"hsl(206 52% 48%)\",\"defaultWireKind\":\"cable.link\"}},{\"id\":\"b-l-m\",\"name\":\"b-l-m\",\"presentation\":{\"color\":\"hsl(290 52% 48%)\",\"defaultWireKind\":\"cable.link\"}},{\"id\":\"b-s\",\"name\":\"b-s\",\"presentation\":{\"color\":\"hsl(55 52% 48%)\",\"defaultWireKind\":\"cable.link\"}},{\"id\":\"b-s-m\",\"name\":\"b-s-m\",\"presentation\":{\"color\":\"hsl(124 52% 48%)\",\"defaultWireKind\":\"cable.link\"}},{\"id\":\"c-b\",\"name\":\"c-b\",\"presentation\":{\"color\":\"hsl(37 52% 48%)\",\"defaultWireKind\":\"cable.link\"}},{\"id\":\"c-t\",\"name\":\"c-t\",\"presentation\":{\"color\":\"hsl(169 52% 48%)\",\"defaultWireKind\":\"cable.link\"}}],\"wireKinds\":[{\"id\":\"cable.link\",\"name\":\"Link\",\"presentation\":{\"defaultEdgeKind\":\"puzzle3d.attraction.link\"}}],\"edgeKinds\":[{\"id\":\"puzzle3d.attraction.link\",\"name\":\"Link\"}],\"nodeKinds\":[{\"id\":\"Hexagonal Cut Concrete Forest Left\",\"name\":\"Hexagonal Cut Concrete Forest Left\",\"presentation\":{\"meshUrl\":\"/mesh/hexagonal-cut-concrete-forest-left.glb\",\"handles\":[{\"handleKind\":\"b-l\",\"angle\":-1.5707963267948966,\"radius\":0.36},{\"handleKind\":\"b-l-m\",\"angle\":-0.9995976625058433,\"radius\":0.36},{\"handleKind\":\"b-l\",\"angle\":-0.42839899821678995,\"radius\":0.36},{\"handleKind\":\"b-s-m\",\"angle\":0.14279966607226324,\"radius\":0.36},{\"handleKind\":\"b-s\",\"angle\":0.7139983303613167,\"radius\":0.36},{\"handleKind\":\"b-s-m\",\"angle\":1.28519699465037,\"radius\":0.36},{\"handleKind\":\"b-s\",\"angle\":1.856395658939423,\"radius\":0.36},{\"handleKind\":\"c-b\",\"angle\":2.4275943232284765,\"radius\":0.36},{\"handleKind\":\"c-t\",\"angle\":2.99879298751753,\"radius\":0.36},{\"handleKind\":\"c-b\",\"angle\":3.569991651806583,\"radius\":0.36},{\"handleKind\":\"c-t\",\"angle\":4.141190316095637,\"radius\":0.36}]}},{\"id\":\"Hexagonal Cut Concrete Forest Right\",\"name\":\"Hexagonal Cut Concrete Forest Right\",\"presentation\":{\"meshUrl\":\"/mesh/hexagonal-cut-concrete-forest-right.glb\",\"handles\":[{\"handleKind\":\"b-l\",\"angle\":-1.5707963267948966,\"radius\":0.36},{\"handleKind\":\"b-l-m\",\"angle\":-0.9995976625058433,\"radius\":0.36},{\"handleKind\":\"b-l\",\"angle\":-0.42839899821678995,\"radius\":0.36},{\"handleKind\":\"b-s-m\",\"angle\":0.14279966607226324,\"radius\":0.36},{\"handleKind\":\"b-s-m\",\"angle\":0.7139983303613167,\"radius\":0.36},{\"handleKind\":\"b-s\",\"angle\":1.28519699465037,\"radius\":0.36},{\"handleKind\":\"b-s-m\",\"angle\":1.856395658939423,\"radius\":0.36},{\"handleKind\":\"c-b\",\"angle\":2.4275943232284765,\"radius\":0.36},{\"handleKind\":\"c-t\",\"angle\":2.99879298751753,\"radius\":0.36},{\"handleKind\":\"c-b\",\"angle\":3.569991651806583,\"radius\":0.36},{\"handleKind\":\"c-t\",\"angle\":4.141190316095637,\"radius\":0.36}]}}],\"edgeTips\":[]}";

pub fn concrete_forest_manifest() -> Manifest {
    serde_json::from_str(CONCRETEFOREST_MANIFEST_JSON).expect("manifest json")
}
