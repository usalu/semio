// Generated from 🛂manifest.jsonnakagin.manifest.json

use serde::{Deserialize, Serialize};
use crate::Manifest;

pub const NAKAGIN_NODE_BALCONY: &str = "Balcony";
pub const NAKAGIN_NODE_BASE: &str = "Base";
pub const NAKAGIN_NODE_BASE_BLOB: &str = "Base Blob";
pub const NAKAGIN_NODE_BRIDGE: &str = "Bridge";
pub const NAKAGIN_NODE_CAPITAL: &str = "Capital";
pub const NAKAGIN_NODE_CAPSULE: &str = "Capsule";
pub const NAKAGIN_NODE_CAPSULE_BACKSLASH: &str = "Capsule Backslash";
pub const NAKAGIN_NODE_CAPSULE_J: &str = "Capsule J";
pub const NAKAGIN_NODE_CAPSULE_L: &str = "Capsule L";
pub const NAKAGIN_NODE_CAPSULE_P: &str = "Capsule P";
pub const NAKAGIN_NODE_CAPSULE_Q: &str = "Capsule q";
pub const NAKAGIN_NODE_CAPSULE_S: &str = "Capsule S";
pub const NAKAGIN_NODE_CAPSULE_SLASH: &str = "Capsule Slash";
pub const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_BACKSLASH: &str = "Capsule With Balcony Backslash";
pub const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_J: &str = "Capsule With Balcony J";
pub const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_L: &str = "Capsule With Balcony L";
pub const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_P: &str = "Capsule With Balcony P";
pub const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_Q: &str = "Capsule With Balcony Q";
pub const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_S: &str = "Capsule With Balcony S";
pub const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_SLASH: &str = "Capsule With Balcony Slash";
pub const NAKAGIN_NODE_CAPSULE_WITH_BALCONY_Z: &str = "Capsule With Balcony Z";
pub const NAKAGIN_NODE_CAPSULE_Z: &str = "Capsule Z";
pub const NAKAGIN_NODE_CYLINDRIC_CAPITAL: &str = "Cylindric Capital";
pub const NAKAGIN_NODE_CYLINDRIC_FIRST_STOREY_TAMBOUR: &str = "Cylindric First Storey Tambour";
pub const NAKAGIN_NODE_CYLINDRIC_LAST_STOREY_TAMBOUR: &str = "Cylindric Last Storey Tambour";
pub const NAKAGIN_NODE_CYLINDRIC_SINGLE_STOREY_TAMBOUR: &str = "Cylindric Single Storey Tambour";
pub const NAKAGIN_NODE_CYLINDRIC_TAMBOUR: &str = "Cylindric Tambour";
pub const NAKAGIN_NODE_ELLIPSOID: &str = "Ellipsoid";
pub const NAKAGIN_NODE_FIRST_STOREY_TAMBOUR: &str = "First Storey Tambour";
pub const NAKAGIN_NODE_LAST_STOREY_TAMBOUR: &str = "Last Storey Tambour";
pub const NAKAGIN_NODE_SINGLE_STOREY_TAMBOUR: &str = "Single Storey Tambour";
pub const NAKAGIN_NODE_TAMBOUR: &str = "Tambour";
pub const NAKAGIN_NODE_TRAPEZOID: &str = "Trapezoid";
pub const NAKAGIN_NODE_TRAPEZOID_CAPSULE_BACKSLASH: &str = "Trapezoid Capsule Backslash";
pub const NAKAGIN_NODE_TRAPEZOID_CAPSULE_J: &str = "Trapezoid Capsule J";
pub const NAKAGIN_NODE_TRAPEZOID_CAPSULE_L: &str = "Trapezoid Capsule L";
pub const NAKAGIN_NODE_TRAPEZOID_CAPSULE_P: &str = "Trapezoid Capsule P";
pub const NAKAGIN_NODE_TRAPEZOID_CAPSULE_Q: &str = "Trapezoid Capsule Q";
pub const NAKAGIN_NODE_TRAPEZOID_CAPSULE_S: &str = "Trapezoid Capsule S";
pub const NAKAGIN_NODE_TRAPEZOID_CAPSULE_SLASH: &str = "Trapezoid Capsule Slash";
pub const NAKAGIN_NODE_TRAPEZOID_CAPSULE_Z: &str = "Trapezoid Capsule Z";
pub const NAKAGIN_NODE_PIECE: &str = "Piece";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NakaginNodeKind {
    #[serde(rename = "Balcony")]
    Balcony,
    #[serde(rename = "Base")]
    Base,
    #[serde(rename = "Base Blob")]
    BaseBlob,
    #[serde(rename = "Bridge")]
    Bridge,
    #[serde(rename = "Capital")]
    Capital,
    #[serde(rename = "Capsule")]
    Capsule,
    #[serde(rename = "Capsule Backslash")]
    CapsuleBackslash,
    #[serde(rename = "Capsule J")]
    CapsuleJ,
    #[serde(rename = "Capsule L")]
    CapsuleL,
    #[serde(rename = "Capsule P")]
    CapsuleP,
    #[serde(rename = "Capsule q")]
    CapsuleQ,
    #[serde(rename = "Capsule S")]
    CapsuleS,
    #[serde(rename = "Capsule Slash")]
    CapsuleSlash,
    #[serde(rename = "Capsule With Balcony Backslash")]
    CapsuleWithBalconyBackslash,
    #[serde(rename = "Capsule With Balcony J")]
    CapsuleWithBalconyJ,
    #[serde(rename = "Capsule With Balcony L")]
    CapsuleWithBalconyL,
    #[serde(rename = "Capsule With Balcony P")]
    CapsuleWithBalconyP,
    #[serde(rename = "Capsule With Balcony Q")]
    CapsuleWithBalconyQ,
    #[serde(rename = "Capsule With Balcony S")]
    CapsuleWithBalconyS,
    #[serde(rename = "Capsule With Balcony Slash")]
    CapsuleWithBalconySlash,
    #[serde(rename = "Capsule With Balcony Z")]
    CapsuleWithBalconyZ,
    #[serde(rename = "Capsule Z")]
    CapsuleZ,
    #[serde(rename = "Cylindric Capital")]
    CylindricCapital,
    #[serde(rename = "Cylindric First Storey Tambour")]
    CylindricFirstStoreyTambour,
    #[serde(rename = "Cylindric Last Storey Tambour")]
    CylindricLastStoreyTambour,
    #[serde(rename = "Cylindric Single Storey Tambour")]
    CylindricSingleStoreyTambour,
    #[serde(rename = "Cylindric Tambour")]
    CylindricTambour,
    #[serde(rename = "Ellipsoid")]
    Ellipsoid,
    #[serde(rename = "First Storey Tambour")]
    FirstStoreyTambour,
    #[serde(rename = "Last Storey Tambour")]
    LastStoreyTambour,
    #[serde(rename = "Single Storey Tambour")]
    SingleStoreyTambour,
    #[serde(rename = "Tambour")]
    Tambour,
    #[serde(rename = "Trapezoid")]
    Trapezoid,
    #[serde(rename = "Trapezoid Capsule Backslash")]
    TrapezoidCapsuleBackslash,
    #[serde(rename = "Trapezoid Capsule J")]
    TrapezoidCapsuleJ,
    #[serde(rename = "Trapezoid Capsule L")]
    TrapezoidCapsuleL,
    #[serde(rename = "Trapezoid Capsule P")]
    TrapezoidCapsuleP,
    #[serde(rename = "Trapezoid Capsule Q")]
    TrapezoidCapsuleQ,
    #[serde(rename = "Trapezoid Capsule S")]
    TrapezoidCapsuleS,
    #[serde(rename = "Trapezoid Capsule Slash")]
    TrapezoidCapsuleSlash,
    #[serde(rename = "Trapezoid Capsule Z")]
    TrapezoidCapsuleZ,
    #[serde(rename = "Piece")]
    Piece,
}

impl NakaginNodeKind {
    pub const ALL: &'static [Self] = &[NakaginNodeKind::Balcony, NakaginNodeKind::Base, NakaginNodeKind::BaseBlob, NakaginNodeKind::Bridge, NakaginNodeKind::Capital, NakaginNodeKind::Capsule, NakaginNodeKind::CapsuleBackslash, NakaginNodeKind::CapsuleJ, NakaginNodeKind::CapsuleL, NakaginNodeKind::CapsuleP, NakaginNodeKind::CapsuleQ, NakaginNodeKind::CapsuleS, NakaginNodeKind::CapsuleSlash, NakaginNodeKind::CapsuleWithBalconyBackslash, NakaginNodeKind::CapsuleWithBalconyJ, NakaginNodeKind::CapsuleWithBalconyL, NakaginNodeKind::CapsuleWithBalconyP, NakaginNodeKind::CapsuleWithBalconyQ, NakaginNodeKind::CapsuleWithBalconyS, NakaginNodeKind::CapsuleWithBalconySlash, NakaginNodeKind::CapsuleWithBalconyZ, NakaginNodeKind::CapsuleZ, NakaginNodeKind::CylindricCapital, NakaginNodeKind::CylindricFirstStoreyTambour, NakaginNodeKind::CylindricLastStoreyTambour, NakaginNodeKind::CylindricSingleStoreyTambour, NakaginNodeKind::CylindricTambour, NakaginNodeKind::Ellipsoid, NakaginNodeKind::FirstStoreyTambour, NakaginNodeKind::LastStoreyTambour, NakaginNodeKind::SingleStoreyTambour, NakaginNodeKind::Tambour, NakaginNodeKind::Trapezoid, NakaginNodeKind::TrapezoidCapsuleBackslash, NakaginNodeKind::TrapezoidCapsuleJ, NakaginNodeKind::TrapezoidCapsuleL, NakaginNodeKind::TrapezoidCapsuleP, NakaginNodeKind::TrapezoidCapsuleQ, NakaginNodeKind::TrapezoidCapsuleS, NakaginNodeKind::TrapezoidCapsuleSlash, NakaginNodeKind::TrapezoidCapsuleZ, NakaginNodeKind::Piece];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balcony => "Balcony",
            Self::Base => "Base",
            Self::BaseBlob => "Base Blob",
            Self::Bridge => "Bridge",
            Self::Capital => "Capital",
            Self::Capsule => "Capsule",
            Self::CapsuleBackslash => "Capsule Backslash",
            Self::CapsuleJ => "Capsule J",
            Self::CapsuleL => "Capsule L",
            Self::CapsuleP => "Capsule P",
            Self::CapsuleQ => "Capsule q",
            Self::CapsuleS => "Capsule S",
            Self::CapsuleSlash => "Capsule Slash",
            Self::CapsuleWithBalconyBackslash => "Capsule With Balcony Backslash",
            Self::CapsuleWithBalconyJ => "Capsule With Balcony J",
            Self::CapsuleWithBalconyL => "Capsule With Balcony L",
            Self::CapsuleWithBalconyP => "Capsule With Balcony P",
            Self::CapsuleWithBalconyQ => "Capsule With Balcony Q",
            Self::CapsuleWithBalconyS => "Capsule With Balcony S",
            Self::CapsuleWithBalconySlash => "Capsule With Balcony Slash",
            Self::CapsuleWithBalconyZ => "Capsule With Balcony Z",
            Self::CapsuleZ => "Capsule Z",
            Self::CylindricCapital => "Cylindric Capital",
            Self::CylindricFirstStoreyTambour => "Cylindric First Storey Tambour",
            Self::CylindricLastStoreyTambour => "Cylindric Last Storey Tambour",
            Self::CylindricSingleStoreyTambour => "Cylindric Single Storey Tambour",
            Self::CylindricTambour => "Cylindric Tambour",
            Self::Ellipsoid => "Ellipsoid",
            Self::FirstStoreyTambour => "First Storey Tambour",
            Self::LastStoreyTambour => "Last Storey Tambour",
            Self::SingleStoreyTambour => "Single Storey Tambour",
            Self::Tambour => "Tambour",
            Self::Trapezoid => "Trapezoid",
            Self::TrapezoidCapsuleBackslash => "Trapezoid Capsule Backslash",
            Self::TrapezoidCapsuleJ => "Trapezoid Capsule J",
            Self::TrapezoidCapsuleL => "Trapezoid Capsule L",
            Self::TrapezoidCapsuleP => "Trapezoid Capsule P",
            Self::TrapezoidCapsuleQ => "Trapezoid Capsule Q",
            Self::TrapezoidCapsuleS => "Trapezoid Capsule S",
            Self::TrapezoidCapsuleSlash => "Trapezoid Capsule Slash",
            Self::TrapezoidCapsuleZ => "Trapezoid Capsule Z",
            Self::Piece => "Piece",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "Balcony" => Ok(Self::Balcony),
            "Base" => Ok(Self::Base),
            "Base Blob" => Ok(Self::BaseBlob),
            "Bridge" => Ok(Self::Bridge),
            "Capital" => Ok(Self::Capital),
            "Capsule" => Ok(Self::Capsule),
            "Capsule Backslash" => Ok(Self::CapsuleBackslash),
            "Capsule J" => Ok(Self::CapsuleJ),
            "Capsule L" => Ok(Self::CapsuleL),
            "Capsule P" => Ok(Self::CapsuleP),
            "Capsule q" => Ok(Self::CapsuleQ),
            "Capsule S" => Ok(Self::CapsuleS),
            "Capsule Slash" => Ok(Self::CapsuleSlash),
            "Capsule With Balcony Backslash" => Ok(Self::CapsuleWithBalconyBackslash),
            "Capsule With Balcony J" => Ok(Self::CapsuleWithBalconyJ),
            "Capsule With Balcony L" => Ok(Self::CapsuleWithBalconyL),
            "Capsule With Balcony P" => Ok(Self::CapsuleWithBalconyP),
            "Capsule With Balcony Q" => Ok(Self::CapsuleWithBalconyQ),
            "Capsule With Balcony S" => Ok(Self::CapsuleWithBalconyS),
            "Capsule With Balcony Slash" => Ok(Self::CapsuleWithBalconySlash),
            "Capsule With Balcony Z" => Ok(Self::CapsuleWithBalconyZ),
            "Capsule Z" => Ok(Self::CapsuleZ),
            "Cylindric Capital" => Ok(Self::CylindricCapital),
            "Cylindric First Storey Tambour" => Ok(Self::CylindricFirstStoreyTambour),
            "Cylindric Last Storey Tambour" => Ok(Self::CylindricLastStoreyTambour),
            "Cylindric Single Storey Tambour" => Ok(Self::CylindricSingleStoreyTambour),
            "Cylindric Tambour" => Ok(Self::CylindricTambour),
            "Ellipsoid" => Ok(Self::Ellipsoid),
            "First Storey Tambour" => Ok(Self::FirstStoreyTambour),
            "Last Storey Tambour" => Ok(Self::LastStoreyTambour),
            "Single Storey Tambour" => Ok(Self::SingleStoreyTambour),
            "Tambour" => Ok(Self::Tambour),
            "Trapezoid" => Ok(Self::Trapezoid),
            "Trapezoid Capsule Backslash" => Ok(Self::TrapezoidCapsuleBackslash),
            "Trapezoid Capsule J" => Ok(Self::TrapezoidCapsuleJ),
            "Trapezoid Capsule L" => Ok(Self::TrapezoidCapsuleL),
            "Trapezoid Capsule P" => Ok(Self::TrapezoidCapsuleP),
            "Trapezoid Capsule Q" => Ok(Self::TrapezoidCapsuleQ),
            "Trapezoid Capsule S" => Ok(Self::TrapezoidCapsuleS),
            "Trapezoid Capsule Slash" => Ok(Self::TrapezoidCapsuleSlash),
            "Trapezoid Capsule Z" => Ok(Self::TrapezoidCapsuleZ),
            "Piece" => Ok(Self::Piece),
            other => Err(format!("unknown node kind {other:?} for Nakagin")),
        }
    }
}

pub const NAKAGIN_NODE_IDS: &[&str] = &["Balcony", "Base", "Base Blob", "Bridge", "Capital", "Capsule", "Capsule Backslash", "Capsule J", "Capsule L", "Capsule P", "Capsule q", "Capsule S", "Capsule Slash", "Capsule With Balcony Backslash", "Capsule With Balcony J", "Capsule With Balcony L", "Capsule With Balcony P", "Capsule With Balcony Q", "Capsule With Balcony S", "Capsule With Balcony Slash", "Capsule With Balcony Z", "Capsule Z", "Cylindric Capital", "Cylindric First Storey Tambour", "Cylindric Last Storey Tambour", "Cylindric Single Storey Tambour", "Cylindric Tambour", "Ellipsoid", "First Storey Tambour", "Last Storey Tambour", "Single Storey Tambour", "Tambour", "Trapezoid", "Trapezoid Capsule Backslash", "Trapezoid Capsule J", "Trapezoid Capsule L", "Trapezoid Capsule P", "Trapezoid Capsule Q", "Trapezoid Capsule S", "Trapezoid Capsule Slash", "Trapezoid Capsule Z", "Piece"];
pub const NAKAGIN_EDGE_CONNECTION: &str = "Connection";
pub const NAKAGIN_EDGE_EDGE_LINK: &str = "edge.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NakaginEdgeKind {
    #[serde(rename = "Connection")]
    Connection,
    #[serde(rename = "edge.link")]
    EdgeLink,
}

impl NakaginEdgeKind {
    pub const ALL: &'static [Self] = &[NakaginEdgeKind::Connection, NakaginEdgeKind::EdgeLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Connection => "Connection",
            Self::EdgeLink => "edge.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "Connection" => Ok(Self::Connection),
            "edge.link" => Ok(Self::EdgeLink),
            other => Err(format!("unknown edge kind {other:?} for Nakagin")),
        }
    }
}

pub const NAKAGIN_EDGE_IDS: &[&str] = &["Connection", "edge.link"];
pub const NAKAGIN_PORT_CONNECTOR: &str = "Connector";
pub const NAKAGIN_PORT_CORE_CIRCULAR_BOTTOM: &str = "core circular bottom";
pub const NAKAGIN_PORT_CORE_CIRCULAR_TOP: &str = "core circular top";
pub const NAKAGIN_PORT_CORE_RECTANGULAR_BOTTOM: &str = "core rectangular bottom";
pub const NAKAGIN_PORT_CORE_RECTANGULAR_TOP: &str = "core rectangular top";
pub const NAKAGIN_PORT_DOOR_CAPSULE_RIGHT: &str = "door capsule right";
pub const NAKAGIN_PORT_DOOR_CAPSULE_LEFT: &str = "door capsule left";
pub const NAKAGIN_PORT_DOOR_TAMBOUR_LEFT: &str = "door tambour left";
pub const NAKAGIN_PORT_DOOR_TAMBOUR_RIGHT: &str = "door tambour right";
pub const NAKAGIN_PORT_PLATFORM_RIGHT: &str = "platform right";
pub const NAKAGIN_PORT_PLATFORM_LEFT: &str = "platform left";
pub const NAKAGIN_PORT_ROOF_CIRCULAR_BOTTOM: &str = "roof circular bottom";
pub const NAKAGIN_PORT_ROOF_CIRCULAR_TOP: &str = "roof circular top";
pub const NAKAGIN_PORT_ROOF_RECTANGULAR_BOTTOM: &str = "roof rectangular bottom";
pub const NAKAGIN_PORT_ROOF_RECTANGULAR_TOP: &str = "roof rectangular top";
pub const NAKAGIN_PORT_TAMBOUR_CIRCULAR_BOTTOM: &str = "tambour circular bottom";
pub const NAKAGIN_PORT_TAMBOUR_CIRCULAR_TOP: &str = "tambour circular top";
pub const NAKAGIN_PORT_TAMBOUR_RECTANGULAR_BOTTOM: &str = "tambour rectangular bottom";
pub const NAKAGIN_PORT_TAMBOUR_RECTANGULAR_TOP: &str = "tambour rectangular top";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NakaginPortKind {
    #[serde(rename = "Connector")]
    Connector,
    #[serde(rename = "core circular bottom")]
    CoreCircularBottom,
    #[serde(rename = "core circular top")]
    CoreCircularTop,
    #[serde(rename = "core rectangular bottom")]
    CoreRectangularBottom,
    #[serde(rename = "core rectangular top")]
    CoreRectangularTop,
    #[serde(rename = "door capsule right")]
    DoorCapsuleRight,
    #[serde(rename = "door capsule left")]
    DoorCapsuleLeft,
    #[serde(rename = "door tambour left")]
    DoorTambourLeft,
    #[serde(rename = "door tambour right")]
    DoorTambourRight,
    #[serde(rename = "platform right")]
    PlatformRight,
    #[serde(rename = "platform left")]
    PlatformLeft,
    #[serde(rename = "roof circular bottom")]
    RoofCircularBottom,
    #[serde(rename = "roof circular top")]
    RoofCircularTop,
    #[serde(rename = "roof rectangular bottom")]
    RoofRectangularBottom,
    #[serde(rename = "roof rectangular top")]
    RoofRectangularTop,
    #[serde(rename = "tambour circular bottom")]
    TambourCircularBottom,
    #[serde(rename = "tambour circular top")]
    TambourCircularTop,
    #[serde(rename = "tambour rectangular bottom")]
    TambourRectangularBottom,
    #[serde(rename = "tambour rectangular top")]
    TambourRectangularTop,
}

impl NakaginPortKind {
    pub const ALL: &'static [Self] = &[NakaginPortKind::Connector, NakaginPortKind::CoreCircularBottom, NakaginPortKind::CoreCircularTop, NakaginPortKind::CoreRectangularBottom, NakaginPortKind::CoreRectangularTop, NakaginPortKind::DoorCapsuleRight, NakaginPortKind::DoorCapsuleLeft, NakaginPortKind::DoorTambourLeft, NakaginPortKind::DoorTambourRight, NakaginPortKind::PlatformRight, NakaginPortKind::PlatformLeft, NakaginPortKind::RoofCircularBottom, NakaginPortKind::RoofCircularTop, NakaginPortKind::RoofRectangularBottom, NakaginPortKind::RoofRectangularTop, NakaginPortKind::TambourCircularBottom, NakaginPortKind::TambourCircularTop, NakaginPortKind::TambourRectangularBottom, NakaginPortKind::TambourRectangularTop];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Connector => "Connector",
            Self::CoreCircularBottom => "core circular bottom",
            Self::CoreCircularTop => "core circular top",
            Self::CoreRectangularBottom => "core rectangular bottom",
            Self::CoreRectangularTop => "core rectangular top",
            Self::DoorCapsuleRight => "door capsule right",
            Self::DoorCapsuleLeft => "door capsule left",
            Self::DoorTambourLeft => "door tambour left",
            Self::DoorTambourRight => "door tambour right",
            Self::PlatformRight => "platform right",
            Self::PlatformLeft => "platform left",
            Self::RoofCircularBottom => "roof circular bottom",
            Self::RoofCircularTop => "roof circular top",
            Self::RoofRectangularBottom => "roof rectangular bottom",
            Self::RoofRectangularTop => "roof rectangular top",
            Self::TambourCircularBottom => "tambour circular bottom",
            Self::TambourCircularTop => "tambour circular top",
            Self::TambourRectangularBottom => "tambour rectangular bottom",
            Self::TambourRectangularTop => "tambour rectangular top",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "Connector" => Ok(Self::Connector),
            "core circular bottom" => Ok(Self::CoreCircularBottom),
            "core circular top" => Ok(Self::CoreCircularTop),
            "core rectangular bottom" => Ok(Self::CoreRectangularBottom),
            "core rectangular top" => Ok(Self::CoreRectangularTop),
            "door capsule right" => Ok(Self::DoorCapsuleRight),
            "door capsule left" => Ok(Self::DoorCapsuleLeft),
            "door tambour left" => Ok(Self::DoorTambourLeft),
            "door tambour right" => Ok(Self::DoorTambourRight),
            "platform right" => Ok(Self::PlatformRight),
            "platform left" => Ok(Self::PlatformLeft),
            "roof circular bottom" => Ok(Self::RoofCircularBottom),
            "roof circular top" => Ok(Self::RoofCircularTop),
            "roof rectangular bottom" => Ok(Self::RoofRectangularBottom),
            "roof rectangular top" => Ok(Self::RoofRectangularTop),
            "tambour circular bottom" => Ok(Self::TambourCircularBottom),
            "tambour circular top" => Ok(Self::TambourCircularTop),
            "tambour rectangular bottom" => Ok(Self::TambourRectangularBottom),
            "tambour rectangular top" => Ok(Self::TambourRectangularTop),
            other => Err(format!("unknown port kind {other:?} for Nakagin")),
        }
    }
}

pub const NAKAGIN_PORT_IDS: &[&str] = &["Connector", "core circular bottom", "core circular top", "core rectangular bottom", "core rectangular top", "door capsule right", "door capsule left", "door tambour left", "door tambour right", "platform right", "platform left", "roof circular bottom", "roof circular top", "roof rectangular bottom", "roof rectangular top", "tambour circular bottom", "tambour circular top", "tambour rectangular bottom", "tambour rectangular top"];
pub const NAKAGIN_WIRE_WIRE_LINK: &str = "wire.link";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NakaginWireKind {
    #[serde(rename = "wire.link")]
    WireLink,
}

impl NakaginWireKind {
    pub const ALL: &'static [Self] = &[NakaginWireKind::WireLink];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WireLink => "wire.link",
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "wire.link" => Ok(Self::WireLink),
            other => Err(format!("unknown wire kind {other:?} for Nakagin")),
        }
    }
}

pub const NAKAGIN_WIRE_IDS: &[&str] = &["wire.link"];
pub const NAKAGIN_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"nakagin\",\"name\":\"Nakagin Capsule Tower\",\"axes\":{\"portModel\":\"ported\",\"directedness\":\"directed\"},\"nodeKinds\":[{\"id\":\"Balcony\",\"name\":\"Balcony\",\"ports\":[],\"presentation\":{}},{\"id\":\"Base\",\"name\":\"Base\",\"ports\":[\"core rectangular bottom\"],\"presentation\":{\"handles\":[{\"handleKind\":\"core rectangular bottom\",\"angle\":-2.3561944901923453,\"radius\":3},{\"handleKind\":\"core rectangular bottom\",\"angle\":-0.7853981633974483,\"radius\":3}]}},{\"id\":\"Base Blob\",\"name\":\"Base Blob\",\"ports\":[\"core circular bottom\"],\"presentation\":{\"handles\":[{\"handleKind\":\"core circular bottom\",\"angle\":-2.3561944901923453,\"radius\":3},{\"handleKind\":\"core circular bottom\",\"angle\":-0.7853981633974483,\"radius\":3}]}},{\"id\":\"Bridge\",\"name\":\"Bridge\",\"ports\":[\"platform right\",\"platform left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"platform right\",\"angle\":0,\"radius\":3},{\"handleKind\":\"platform left\",\"angle\":3.141592653589793,\"radius\":3}]}},{\"id\":\"Capital\",\"name\":\"Capital\",\"ports\":[\"roof rectangular top\"],\"presentation\":{\"handles\":[{\"handleKind\":\"roof rectangular top\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Capsule\",\"name\":\"Capsule\",\"ports\":[],\"presentation\":{}},{\"id\":\"Capsule Backslash\",\"name\":\"Capsule Backslash\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Capsule J\",\"name\":\"Capsule J\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Capsule L\",\"name\":\"Capsule L\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Capsule P\",\"name\":\"Capsule P\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Capsule q\",\"name\":\"Capsule q\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-0.450225596260715,\"radius\":3}]}},{\"id\":\"Capsule S\",\"name\":\"Capsule S\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Capsule Slash\",\"name\":\"Capsule Slash\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Capsule With Balcony Backslash\",\"name\":\"Capsule With Balcony Backslash\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-0.21109333322274654,\"radius\":3}]}},{\"id\":\"Capsule With Balcony J\",\"name\":\"Capsule With Balcony J\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":0.805003494254653,\"radius\":3}]}},{\"id\":\"Capsule With Balcony L\",\"name\":\"Capsule With Balcony L\",\"ports\":[\"door capsule left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule left\",\"angle\":-0.805003494254653,\"radius\":3}]}},{\"id\":\"Capsule With Balcony P\",\"name\":\"Capsule With Balcony P\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":0.21109333322274654,\"radius\":3}]}},{\"id\":\"Capsule With Balcony Q\",\"name\":\"Capsule With Balcony Q\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-0.21109333322274654,\"radius\":3}]}},{\"id\":\"Capsule With Balcony S\",\"name\":\"Capsule With Balcony S\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":0.805003494254653,\"radius\":3}]}},{\"id\":\"Capsule With Balcony Slash\",\"name\":\"Capsule With Balcony Slash\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":0.21109333322274654,\"radius\":3}]}},{\"id\":\"Capsule With Balcony Z\",\"name\":\"Capsule With Balcony Z\",\"ports\":[\"door capsule left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule left\",\"angle\":-0.805003494254653,\"radius\":3}]}},{\"id\":\"Capsule Z\",\"name\":\"Capsule Z\",\"ports\":[\"door capsule left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule left\",\"angle\":-0.805003494254653,\"radius\":3}]}},{\"id\":\"Cylindric Capital\",\"name\":\"Cylindric Capital\",\"ports\":[\"roof circular top\"],\"presentation\":{\"handles\":[{\"handleKind\":\"roof circular top\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Cylindric First Storey Tambour\",\"name\":\"Cylindric First Storey Tambour\",\"ports\":[\"core circular top\",\"tambour circular top\",\"door tambour right\",\"door tambour left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"core circular top\",\"angle\":-3.141592653589793,\"radius\":3},{\"handleKind\":\"tambour circular top\",\"angle\":-3.141592653589793,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-1.8121518132334324,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-2.9002371671512575,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":2.9002371671512575,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":1.8121518132334324,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":1.329440840356361,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":0.24135548643853572,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-0.24135548643853572,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-1.329440840356361,\"radius\":3}]}},{\"id\":\"Cylindric Last Storey Tambour\",\"name\":\"Cylindric Last Storey Tambour\",\"ports\":[\"tambour circular bottom\",\"roof circular bottom\",\"door tambour right\",\"door tambour left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"tambour circular bottom\",\"angle\":-3.141592653589793,\"radius\":3},{\"handleKind\":\"roof circular bottom\",\"angle\":-3.141592653589793,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-1.8121518132334324,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-2.9002371671512575,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":2.9002371671512575,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":1.8121518132334324,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":1.329440840356361,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":0.24135548643853572,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-0.24135548643853572,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-1.329440840356361,\"radius\":3}]}},{\"id\":\"Cylindric Single Storey Tambour\",\"name\":\"Cylindric Single Storey Tambour\",\"ports\":[\"core circular top\",\"roof circular bottom\",\"door tambour right\",\"door tambour left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"core circular top\",\"angle\":-3.141592653589793,\"radius\":3},{\"handleKind\":\"roof circular bottom\",\"angle\":-3.141592653589793,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-1.8121518132334324,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-2.9002371671512575,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":2.9002371671512575,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":1.8121518132334324,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":1.329440840356361,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":0.24135548643853572,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-0.24135548643853572,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-1.329440840356361,\"radius\":3}]}},{\"id\":\"Cylindric Tambour\",\"name\":\"Cylindric Tambour\",\"ports\":[\"tambour circular bottom\",\"tambour circular top\",\"door tambour right\",\"door tambour left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"tambour circular bottom\",\"angle\":1.5707963267948966,\"radius\":3},{\"handleKind\":\"tambour circular top\",\"angle\":-1.5707963267948966,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-0.31415926535897953,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-1.256637061435917,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-1.884955592153876,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-2.8274333882308142,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":2.827433388230814,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":1.8849555921538763,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":1.2566370614359168,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":0.31415926535897953,\"radius\":3}]}},{\"id\":\"Ellipsoid\",\"name\":\"Ellipsoid\",\"ports\":[],\"presentation\":{}},{\"id\":\"First Storey Tambour\",\"name\":\"First Storey Tambour\",\"ports\":[\"core rectangular top\",\"tambour rectangular top\",\"door tambour right\",\"door tambour left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"core rectangular top\",\"angle\":1.5707963267948966,\"radius\":3},{\"handleKind\":\"tambour rectangular top\",\"angle\":-1.5707963267948966,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-0.31415926535897953,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-1.256637061435917,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-1.884955592153876,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-2.8274333882308142,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":2.827433388230814,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":1.8849555921538763,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":1.2566370614359168,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":0.31415926535897953,\"radius\":3}]}},{\"id\":\"Last Storey Tambour\",\"name\":\"Last Storey Tambour\",\"ports\":[\"tambour rectangular bottom\",\"roof rectangular bottom\",\"door tambour right\",\"door tambour left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"tambour rectangular bottom\",\"angle\":1.5707963267948966,\"radius\":3},{\"handleKind\":\"roof rectangular bottom\",\"angle\":-1.5707963267948966,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-0.31415926535897953,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-1.256637061435917,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-1.884955592153876,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-2.8274333882308142,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":2.827433388230814,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":1.8849555921538763,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":1.2566370614359168,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":0.31415926535897953,\"radius\":3}]}},{\"id\":\"Single Storey Tambour\",\"name\":\"Single Storey Tambour\",\"ports\":[\"core circular top\",\"roof rectangular bottom\",\"door tambour right\",\"door tambour left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"core circular top\",\"angle\":-3.141592653589793,\"radius\":3},{\"handleKind\":\"roof rectangular bottom\",\"angle\":-3.141592653589793,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-1.887082454707051,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-2.8253065256776386,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":2.8253065256776386,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":1.887082454707051,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":1.254510198882742,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":0.3162861279121545,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-0.3162861279121545,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-1.254510198882742,\"radius\":3}]}},{\"id\":\"Tambour\",\"name\":\"Tambour\",\"ports\":[\"tambour rectangular bottom\",\"tambour rectangular top\",\"door tambour right\",\"door tambour left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"tambour rectangular bottom\",\"angle\":1.5707963267948966,\"radius\":3},{\"handleKind\":\"tambour rectangular top\",\"angle\":-1.5707963267948966,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-0.31415926535897953,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-1.256637061435917,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":-1.884955592153876,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":-2.8274333882308142,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":2.827433388230814,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":1.8849555921538763,\"radius\":3},{\"handleKind\":\"door tambour right\",\"angle\":1.2566370614359168,\"radius\":3},{\"handleKind\":\"door tambour left\",\"angle\":0.31415926535897953,\"radius\":3}]}},{\"id\":\"Trapezoid\",\"name\":\"Trapezoid\",\"ports\":[],\"presentation\":{}},{\"id\":\"Trapezoid Capsule Backslash\",\"name\":\"Trapezoid Capsule Backslash\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Trapezoid Capsule J\",\"name\":\"Trapezoid Capsule J\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Trapezoid Capsule L\",\"name\":\"Trapezoid Capsule L\",\"ports\":[\"door capsule left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule left\",\"angle\":1.5707963267948966,\"radius\":3}]}},{\"id\":\"Trapezoid Capsule P\",\"name\":\"Trapezoid Capsule P\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":1.5707963267948966,\"radius\":3}]}},{\"id\":\"Trapezoid Capsule Q\",\"name\":\"Trapezoid Capsule Q\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Trapezoid Capsule S\",\"name\":\"Trapezoid Capsule S\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":-1.5707963267948966,\"radius\":3}]}},{\"id\":\"Trapezoid Capsule Slash\",\"name\":\"Trapezoid Capsule Slash\",\"ports\":[\"door capsule right\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule right\",\"angle\":1.5707963267948966,\"radius\":3}]}},{\"id\":\"Trapezoid Capsule Z\",\"name\":\"Trapezoid Capsule Z\",\"ports\":[\"door capsule left\"],\"presentation\":{\"handles\":[{\"handleKind\":\"door capsule left\",\"angle\":1.5707963267948966,\"radius\":3}]}},{\"id\":\"Piece\",\"name\":\"Piece\",\"ports\":[\"Connector\"],\"properties\":[{\"name\":\"position\",\"kind\":\"data\",\"valueType\":\"object\"},{\"name\":\"label\",\"kind\":\"data\",\"valueType\":\"string\"},{\"name\":\"tier\",\"kind\":\"data\",\"valueType\":\"number\"},{\"name\":\"flatPosition\",\"kind\":\"derived\",\"valueType\":\"object\",\"expr\":\"flatFromConnections\"}]}],\"edgeKinds\":[{\"id\":\"Connection\",\"name\":\"Connection\",\"properties\":[{\"name\":\"gap\",\"kind\":\"data\",\"valueType\":\"number\"},{\"name\":\"rotation\",\"kind\":\"data\",\"valueType\":\"number\"},{\"name\":\"tilt\",\"kind\":\"data\",\"valueType\":\"number\"},{\"name\":\"rise\",\"kind\":\"data\",\"valueType\":\"number\"},{\"name\":\"turn\",\"kind\":\"data\",\"valueType\":\"number\"},{\"name\":\"shift\",\"kind\":\"data\",\"valueType\":\"number\"},{\"name\":\"u\",\"kind\":\"data\",\"valueType\":\"number\"},{\"name\":\"v\",\"kind\":\"data\",\"valueType\":\"number\"}]},{\"id\":\"edge.link\",\"name\":\"Link\",\"presentation\":{\"id\":\"edge.link\",\"name\":\"Link\"}}],\"portKinds\":[{\"id\":\"Connector\",\"name\":\"Connector\",\"direction\":\"out\",\"properties\":[]},{\"id\":\"core circular bottom\",\"name\":\"core circular bottom\",\"presentation\":{\"color\":\"hsl(206 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"core circular top\",\"name\":\"core circular top\",\"presentation\":{\"color\":\"hsl(290 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"core rectangular bottom\",\"name\":\"core rectangular bottom\",\"presentation\":{\"color\":\"hsl(55 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"core rectangular top\",\"name\":\"core rectangular top\",\"presentation\":{\"color\":\"hsl(37 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"door capsule right\",\"name\":\"door capsule right\",\"presentation\":{\"color\":\"hsl(124 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"door capsule left\",\"name\":\"door capsule left\",\"presentation\":{\"color\":\"hsl(239 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"door tambour left\",\"name\":\"door tambour left\",\"presentation\":{\"color\":\"hsl(344 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"door tambour right\",\"name\":\"door tambour right\",\"presentation\":{\"color\":\"hsl(91 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"platform right\",\"name\":\"platform right\",\"presentation\":{\"color\":\"hsl(169 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"platform left\",\"name\":\"platform left\",\"presentation\":{\"color\":\"hsl(215 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"roof circular bottom\",\"name\":\"roof circular bottom\",\"presentation\":{\"color\":\"hsl(277 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"roof circular top\",\"name\":\"roof circular top\",\"presentation\":{\"color\":\"hsl(215 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"roof rectangular bottom\",\"name\":\"roof rectangular bottom\",\"presentation\":{\"color\":\"hsl(108 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"roof rectangular top\",\"name\":\"roof rectangular top\",\"presentation\":{\"color\":\"hsl(100 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"tambour circular bottom\",\"name\":\"tambour circular bottom\",\"presentation\":{\"color\":\"hsl(231 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"tambour circular top\",\"name\":\"tambour circular top\",\"presentation\":{\"color\":\"hsl(156 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"tambour rectangular bottom\",\"name\":\"tambour rectangular bottom\",\"presentation\":{\"color\":\"hsl(223 52% 48%)\",\"defaultWireKind\":\"wire.link\"}},{\"id\":\"tambour rectangular top\",\"name\":\"tambour rectangular top\",\"presentation\":{\"color\":\"hsl(108 52% 48%)\",\"defaultWireKind\":\"wire.link\"}}],\"wireKinds\":[{\"id\":\"wire.link\",\"name\":\"Link\",\"presentation\":{\"defaultEdgeKind\":\"edge.link\"}}],\"edgeTips\":[]}";

pub fn nakagin_manifest() -> Manifest {
    serde_json::from_str(NAKAGIN_MANIFEST_JSON).expect("manifest json")
}
