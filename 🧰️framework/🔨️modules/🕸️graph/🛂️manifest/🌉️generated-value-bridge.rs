// 🌉️ Hand-written `ToValue`/`FromValue` bridge for the machine-generated manifest enums in
// sibling `🤖️generated/🦀️*.rs` files (headers read "// Generated from *.manifest.json" —
// never hand-edited). Each enum carries an explicit per-variant `#[serde(rename = "...")]`
// wire string (not a uniform `rename_all` case), which the `#[derive(ToValue, FromValue)]`
// macro can only apply via `#[value(rename = "...")]` attributes on the type itself — those
// attributes cannot be added to a generated file, so this file hand-writes the identical
// string mapping instead, matching each enum's `#[serde(rename = "...")]` byte for byte.
// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01/02), additive phase:
// ADDITIVE ONLY — every enum below keeps its existing `Serialize`/`Deserialize` untouched.

impl dsl_core::ToValue for flow_dag::FlowDagNodeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            flow_dag::FlowDagNodeKind::Computation => "computation",
            flow_dag::FlowDagNodeKind::Slider => "slider",
            flow_dag::FlowDagNodeKind::Select => "select",
            flow_dag::FlowDagNodeKind::Screen => "screen",
            flow_dag::FlowDagNodeKind::Note => "note",
            flow_dag::FlowDagNodeKind::Image => "image",
            flow_dag::FlowDagNodeKind::Preview => "preview",
            flow_dag::FlowDagNodeKind::Action => "action",
            flow_dag::FlowDagNodeKind::Export => "export",
            flow_dag::FlowDagNodeKind::Cluster => "cluster",
            flow_dag::FlowDagNodeKind::AppInstance => "appInstance",
        }.to_string())
    }
}
impl dsl_core::FromValue for flow_dag::FlowDagNodeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for FlowDagNodeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "computation" => flow_dag::FlowDagNodeKind::Computation,
            "slider" => flow_dag::FlowDagNodeKind::Slider,
            "select" => flow_dag::FlowDagNodeKind::Select,
            "screen" => flow_dag::FlowDagNodeKind::Screen,
            "note" => flow_dag::FlowDagNodeKind::Note,
            "image" => flow_dag::FlowDagNodeKind::Image,
            "preview" => flow_dag::FlowDagNodeKind::Preview,
            "action" => flow_dag::FlowDagNodeKind::Action,
            "export" => flow_dag::FlowDagNodeKind::Export,
            "cluster" => flow_dag::FlowDagNodeKind::Cluster,
            "appInstance" => flow_dag::FlowDagNodeKind::AppInstance,
            other => return Err(dsl_core::ValueError::new(format!("unknown FlowDagNodeKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for rewrite_lhs::RewriteLhsNodeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            rewrite_lhs::RewriteLhsNodeKind::RewriteMatch => "rewrite.match",
            rewrite_lhs::RewriteLhsNodeKind::RewriteWhere => "rewrite.where",
        }.to_string())
    }
}
impl dsl_core::FromValue for rewrite_lhs::RewriteLhsNodeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for RewriteLhsNodeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "rewrite.match" => rewrite_lhs::RewriteLhsNodeKind::RewriteMatch,
            "rewrite.where" => rewrite_lhs::RewriteLhsNodeKind::RewriteWhere,
            other => return Err(dsl_core::ValueError::new(format!("unknown RewriteLhsNodeKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for rewrite_lhs::RewriteLhsEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            rewrite_lhs::RewriteLhsEdgeKind::EdgeFlow => "edge.flow",
            rewrite_lhs::RewriteLhsEdgeKind::EdgePattern => "edge.pattern",
        }.to_string())
    }
}
impl dsl_core::FromValue for rewrite_lhs::RewriteLhsEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for RewriteLhsEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "edge.flow" => rewrite_lhs::RewriteLhsEdgeKind::EdgeFlow,
            "edge.pattern" => rewrite_lhs::RewriteLhsEdgeKind::EdgePattern,
            other => return Err(dsl_core::ValueError::new(format!("unknown RewriteLhsEdgeKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for rewrite_lhs::RewriteLhsPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            rewrite_lhs::RewriteLhsPortKind::Port => "port",
        }.to_string())
    }
}
impl dsl_core::FromValue for rewrite_lhs::RewriteLhsPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for RewriteLhsPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "port" => rewrite_lhs::RewriteLhsPortKind::Port,
            other => return Err(dsl_core::ValueError::new(format!("unknown RewriteLhsPortKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for rewrite_lhs::RewriteLhsWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            rewrite_lhs::RewriteLhsWireKind::WireFlow => "wire.flow",
        }.to_string())
    }
}
impl dsl_core::FromValue for rewrite_lhs::RewriteLhsWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for RewriteLhsWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wire.flow" => rewrite_lhs::RewriteLhsWireKind::WireFlow,
            other => return Err(dsl_core::ValueError::new(format!("unknown RewriteLhsWireKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for puzzle2d_default::Puzzle2dDefaultEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            puzzle2d_default::Puzzle2dDefaultEdgeKind::EdgeLink => "edge.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for puzzle2d_default::Puzzle2dDefaultEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle2dDefaultEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "edge.link" => puzzle2d_default::Puzzle2dDefaultEdgeKind::EdgeLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle2dDefaultEdgeKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for puzzle2d_default::Puzzle2dDefaultPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            puzzle2d_default::Puzzle2dDefaultPortKind::Port => "port",
        }.to_string())
    }
}
impl dsl_core::FromValue for puzzle2d_default::Puzzle2dDefaultPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle2dDefaultPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "port" => puzzle2d_default::Puzzle2dDefaultPortKind::Port,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle2dDefaultPortKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for puzzle2d_default::Puzzle2dDefaultWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            puzzle2d_default::Puzzle2dDefaultWireKind::WireLink => "wire.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for puzzle2d_default::Puzzle2dDefaultWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle2dDefaultWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wire.link" => puzzle2d_default::Puzzle2dDefaultWireKind::WireLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle2dDefaultWireKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for nakagin::NakaginNodeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            nakagin::NakaginNodeKind::Balcony => "Balcony",
            nakagin::NakaginNodeKind::Base => "Base",
            nakagin::NakaginNodeKind::BaseBlob => "Base Blob",
            nakagin::NakaginNodeKind::Bridge => "Bridge",
            nakagin::NakaginNodeKind::Capital => "Capital",
            nakagin::NakaginNodeKind::Capsule => "Capsule",
            nakagin::NakaginNodeKind::CapsuleBackslash => "Capsule Backslash",
            nakagin::NakaginNodeKind::CapsuleJ => "Capsule J",
            nakagin::NakaginNodeKind::CapsuleL => "Capsule L",
            nakagin::NakaginNodeKind::CapsuleP => "Capsule P",
            nakagin::NakaginNodeKind::CapsuleQ => "Capsule q",
            nakagin::NakaginNodeKind::CapsuleS => "Capsule S",
            nakagin::NakaginNodeKind::CapsuleSlash => "Capsule Slash",
            nakagin::NakaginNodeKind::CapsuleWithBalconyBackslash => "Capsule With Balcony Backslash",
            nakagin::NakaginNodeKind::CapsuleWithBalconyJ => "Capsule With Balcony J",
            nakagin::NakaginNodeKind::CapsuleWithBalconyL => "Capsule With Balcony L",
            nakagin::NakaginNodeKind::CapsuleWithBalconyP => "Capsule With Balcony P",
            nakagin::NakaginNodeKind::CapsuleWithBalconyQ => "Capsule With Balcony Q",
            nakagin::NakaginNodeKind::CapsuleWithBalconyS => "Capsule With Balcony S",
            nakagin::NakaginNodeKind::CapsuleWithBalconySlash => "Capsule With Balcony Slash",
            nakagin::NakaginNodeKind::CapsuleWithBalconyZ => "Capsule With Balcony Z",
            nakagin::NakaginNodeKind::CapsuleZ => "Capsule Z",
            nakagin::NakaginNodeKind::CylindricCapital => "Cylindric Capital",
            nakagin::NakaginNodeKind::CylindricFirstStoreyTambour => "Cylindric First Storey Tambour",
            nakagin::NakaginNodeKind::CylindricLastStoreyTambour => "Cylindric Last Storey Tambour",
            nakagin::NakaginNodeKind::CylindricSingleStoreyTambour => "Cylindric Single Storey Tambour",
            nakagin::NakaginNodeKind::CylindricTambour => "Cylindric Tambour",
            nakagin::NakaginNodeKind::Ellipsoid => "Ellipsoid",
            nakagin::NakaginNodeKind::FirstStoreyTambour => "First Storey Tambour",
            nakagin::NakaginNodeKind::LastStoreyTambour => "Last Storey Tambour",
            nakagin::NakaginNodeKind::SingleStoreyTambour => "Single Storey Tambour",
            nakagin::NakaginNodeKind::Tambour => "Tambour",
            nakagin::NakaginNodeKind::Trapezoid => "Trapezoid",
            nakagin::NakaginNodeKind::TrapezoidCapsuleBackslash => "Trapezoid Capsule Backslash",
            nakagin::NakaginNodeKind::TrapezoidCapsuleJ => "Trapezoid Capsule J",
            nakagin::NakaginNodeKind::TrapezoidCapsuleL => "Trapezoid Capsule L",
            nakagin::NakaginNodeKind::TrapezoidCapsuleP => "Trapezoid Capsule P",
            nakagin::NakaginNodeKind::TrapezoidCapsuleQ => "Trapezoid Capsule Q",
            nakagin::NakaginNodeKind::TrapezoidCapsuleS => "Trapezoid Capsule S",
            nakagin::NakaginNodeKind::TrapezoidCapsuleSlash => "Trapezoid Capsule Slash",
            nakagin::NakaginNodeKind::TrapezoidCapsuleZ => "Trapezoid Capsule Z",
            nakagin::NakaginNodeKind::Piece => "Piece",
        }.to_string())
    }
}
impl dsl_core::FromValue for nakagin::NakaginNodeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for NakaginNodeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "Balcony" => nakagin::NakaginNodeKind::Balcony,
            "Base" => nakagin::NakaginNodeKind::Base,
            "Base Blob" => nakagin::NakaginNodeKind::BaseBlob,
            "Bridge" => nakagin::NakaginNodeKind::Bridge,
            "Capital" => nakagin::NakaginNodeKind::Capital,
            "Capsule" => nakagin::NakaginNodeKind::Capsule,
            "Capsule Backslash" => nakagin::NakaginNodeKind::CapsuleBackslash,
            "Capsule J" => nakagin::NakaginNodeKind::CapsuleJ,
            "Capsule L" => nakagin::NakaginNodeKind::CapsuleL,
            "Capsule P" => nakagin::NakaginNodeKind::CapsuleP,
            "Capsule q" => nakagin::NakaginNodeKind::CapsuleQ,
            "Capsule S" => nakagin::NakaginNodeKind::CapsuleS,
            "Capsule Slash" => nakagin::NakaginNodeKind::CapsuleSlash,
            "Capsule With Balcony Backslash" => nakagin::NakaginNodeKind::CapsuleWithBalconyBackslash,
            "Capsule With Balcony J" => nakagin::NakaginNodeKind::CapsuleWithBalconyJ,
            "Capsule With Balcony L" => nakagin::NakaginNodeKind::CapsuleWithBalconyL,
            "Capsule With Balcony P" => nakagin::NakaginNodeKind::CapsuleWithBalconyP,
            "Capsule With Balcony Q" => nakagin::NakaginNodeKind::CapsuleWithBalconyQ,
            "Capsule With Balcony S" => nakagin::NakaginNodeKind::CapsuleWithBalconyS,
            "Capsule With Balcony Slash" => nakagin::NakaginNodeKind::CapsuleWithBalconySlash,
            "Capsule With Balcony Z" => nakagin::NakaginNodeKind::CapsuleWithBalconyZ,
            "Capsule Z" => nakagin::NakaginNodeKind::CapsuleZ,
            "Cylindric Capital" => nakagin::NakaginNodeKind::CylindricCapital,
            "Cylindric First Storey Tambour" => nakagin::NakaginNodeKind::CylindricFirstStoreyTambour,
            "Cylindric Last Storey Tambour" => nakagin::NakaginNodeKind::CylindricLastStoreyTambour,
            "Cylindric Single Storey Tambour" => nakagin::NakaginNodeKind::CylindricSingleStoreyTambour,
            "Cylindric Tambour" => nakagin::NakaginNodeKind::CylindricTambour,
            "Ellipsoid" => nakagin::NakaginNodeKind::Ellipsoid,
            "First Storey Tambour" => nakagin::NakaginNodeKind::FirstStoreyTambour,
            "Last Storey Tambour" => nakagin::NakaginNodeKind::LastStoreyTambour,
            "Single Storey Tambour" => nakagin::NakaginNodeKind::SingleStoreyTambour,
            "Tambour" => nakagin::NakaginNodeKind::Tambour,
            "Trapezoid" => nakagin::NakaginNodeKind::Trapezoid,
            "Trapezoid Capsule Backslash" => nakagin::NakaginNodeKind::TrapezoidCapsuleBackslash,
            "Trapezoid Capsule J" => nakagin::NakaginNodeKind::TrapezoidCapsuleJ,
            "Trapezoid Capsule L" => nakagin::NakaginNodeKind::TrapezoidCapsuleL,
            "Trapezoid Capsule P" => nakagin::NakaginNodeKind::TrapezoidCapsuleP,
            "Trapezoid Capsule Q" => nakagin::NakaginNodeKind::TrapezoidCapsuleQ,
            "Trapezoid Capsule S" => nakagin::NakaginNodeKind::TrapezoidCapsuleS,
            "Trapezoid Capsule Slash" => nakagin::NakaginNodeKind::TrapezoidCapsuleSlash,
            "Trapezoid Capsule Z" => nakagin::NakaginNodeKind::TrapezoidCapsuleZ,
            "Piece" => nakagin::NakaginNodeKind::Piece,
            other => return Err(dsl_core::ValueError::new(format!("unknown NakaginNodeKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for nakagin::NakaginEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            nakagin::NakaginEdgeKind::Connection => "Connection",
            nakagin::NakaginEdgeKind::EdgeLink => "edge.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for nakagin::NakaginEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for NakaginEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "Connection" => nakagin::NakaginEdgeKind::Connection,
            "edge.link" => nakagin::NakaginEdgeKind::EdgeLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown NakaginEdgeKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for nakagin::NakaginPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            nakagin::NakaginPortKind::Connector => "Connector",
            nakagin::NakaginPortKind::CoreCircularBottom => "core circular bottom",
            nakagin::NakaginPortKind::CoreCircularTop => "core circular top",
            nakagin::NakaginPortKind::CoreRectangularBottom => "core rectangular bottom",
            nakagin::NakaginPortKind::CoreRectangularTop => "core rectangular top",
            nakagin::NakaginPortKind::DoorCapsuleRight => "door capsule right",
            nakagin::NakaginPortKind::DoorCapsuleLeft => "door capsule left",
            nakagin::NakaginPortKind::DoorTambourLeft => "door tambour left",
            nakagin::NakaginPortKind::DoorTambourRight => "door tambour right",
            nakagin::NakaginPortKind::PlatformRight => "platform right",
            nakagin::NakaginPortKind::PlatformLeft => "platform left",
            nakagin::NakaginPortKind::RoofCircularBottom => "roof circular bottom",
            nakagin::NakaginPortKind::RoofCircularTop => "roof circular top",
            nakagin::NakaginPortKind::RoofRectangularBottom => "roof rectangular bottom",
            nakagin::NakaginPortKind::RoofRectangularTop => "roof rectangular top",
            nakagin::NakaginPortKind::TambourCircularBottom => "tambour circular bottom",
            nakagin::NakaginPortKind::TambourCircularTop => "tambour circular top",
            nakagin::NakaginPortKind::TambourRectangularBottom => "tambour rectangular bottom",
            nakagin::NakaginPortKind::TambourRectangularTop => "tambour rectangular top",
        }.to_string())
    }
}
impl dsl_core::FromValue for nakagin::NakaginPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for NakaginPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "Connector" => nakagin::NakaginPortKind::Connector,
            "core circular bottom" => nakagin::NakaginPortKind::CoreCircularBottom,
            "core circular top" => nakagin::NakaginPortKind::CoreCircularTop,
            "core rectangular bottom" => nakagin::NakaginPortKind::CoreRectangularBottom,
            "core rectangular top" => nakagin::NakaginPortKind::CoreRectangularTop,
            "door capsule right" => nakagin::NakaginPortKind::DoorCapsuleRight,
            "door capsule left" => nakagin::NakaginPortKind::DoorCapsuleLeft,
            "door tambour left" => nakagin::NakaginPortKind::DoorTambourLeft,
            "door tambour right" => nakagin::NakaginPortKind::DoorTambourRight,
            "platform right" => nakagin::NakaginPortKind::PlatformRight,
            "platform left" => nakagin::NakaginPortKind::PlatformLeft,
            "roof circular bottom" => nakagin::NakaginPortKind::RoofCircularBottom,
            "roof circular top" => nakagin::NakaginPortKind::RoofCircularTop,
            "roof rectangular bottom" => nakagin::NakaginPortKind::RoofRectangularBottom,
            "roof rectangular top" => nakagin::NakaginPortKind::RoofRectangularTop,
            "tambour circular bottom" => nakagin::NakaginPortKind::TambourCircularBottom,
            "tambour circular top" => nakagin::NakaginPortKind::TambourCircularTop,
            "tambour rectangular bottom" => nakagin::NakaginPortKind::TambourRectangularBottom,
            "tambour rectangular top" => nakagin::NakaginPortKind::TambourRectangularTop,
            other => return Err(dsl_core::ValueError::new(format!("unknown NakaginPortKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for nakagin::NakaginWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            nakagin::NakaginWireKind::WireLink => "wire.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for nakagin::NakaginWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for NakaginWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wire.link" => nakagin::NakaginWireKind::WireLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown NakaginWireKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for puzzle5d_default::Puzzle5dDefaultEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            puzzle5d_default::Puzzle5dDefaultEdgeKind::EdgeLink => "edge.link",
            puzzle5d_default::Puzzle5dDefaultEdgeKind::AttractionLink => "attraction.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for puzzle5d_default::Puzzle5dDefaultEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle5dDefaultEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "edge.link" => puzzle5d_default::Puzzle5dDefaultEdgeKind::EdgeLink,
            "attraction.link" => puzzle5d_default::Puzzle5dDefaultEdgeKind::AttractionLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle5dDefaultEdgeKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for puzzle5d_default::Puzzle5dDefaultPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            puzzle5d_default::Puzzle5dDefaultPortKind::Port => "port",
            puzzle5d_default::Puzzle5dDefaultPortKind::Vortex => "vortex",
        }.to_string())
    }
}
impl dsl_core::FromValue for puzzle5d_default::Puzzle5dDefaultPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle5dDefaultPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "port" => puzzle5d_default::Puzzle5dDefaultPortKind::Port,
            "vortex" => puzzle5d_default::Puzzle5dDefaultPortKind::Vortex,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle5dDefaultPortKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for puzzle5d_default::Puzzle5dDefaultWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            puzzle5d_default::Puzzle5dDefaultWireKind::WireLink => "wire.link",
            puzzle5d_default::Puzzle5dDefaultWireKind::CableLink => "cable.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for puzzle5d_default::Puzzle5dDefaultWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle5dDefaultWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wire.link" => puzzle5d_default::Puzzle5dDefaultWireKind::WireLink,
            "cable.link" => puzzle5d_default::Puzzle5dDefaultWireKind::CableLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle5dDefaultWireKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for writer_languages::WriterLanguagesLanguageKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            writer_languages::WriterLanguagesLanguageKind::Jack => "jack",
            writer_languages::WriterLanguagesLanguageKind::Wire => "wire",
            writer_languages::WriterLanguagesLanguageKind::Plaintext => "plaintext",
            writer_languages::WriterLanguagesLanguageKind::Markdown => "markdown",
        }.to_string())
    }
}
impl dsl_core::FromValue for writer_languages::WriterLanguagesLanguageKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for WriterLanguagesLanguageKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "jack" => writer_languages::WriterLanguagesLanguageKind::Jack,
            "wire" => writer_languages::WriterLanguagesLanguageKind::Wire,
            "plaintext" => writer_languages::WriterLanguagesLanguageKind::Plaintext,
            "markdown" => writer_languages::WriterLanguagesLanguageKind::Markdown,
            other => return Err(dsl_core::ValueError::new(format!("unknown WriterLanguagesLanguageKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for wires::WiresEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            wires::WiresEdgeKind::WiresOwns => "wires.owns",
            wires::WiresEdgeKind::WiresIs => "wires.is",
            wires::WiresEdgeKind::WiresReferences => "wires.references",
            wires::WiresEdgeKind::WiresHas => "wires.has",
        }.to_string())
    }
}
impl dsl_core::FromValue for wires::WiresEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for WiresEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wires.owns" => wires::WiresEdgeKind::WiresOwns,
            "wires.is" => wires::WiresEdgeKind::WiresIs,
            "wires.references" => wires::WiresEdgeKind::WiresReferences,
            "wires.has" => wires::WiresEdgeKind::WiresHas,
            other => return Err(dsl_core::ValueError::new(format!("unknown WiresEdgeKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for drawing_layers::DrawingLayersLayerKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            drawing_layers::DrawingLayersLayerKind::Shape => "shape",
            drawing_layers::DrawingLayersLayerKind::Path => "path",
            drawing_layers::DrawingLayersLayerKind::Text => "text",
            drawing_layers::DrawingLayersLayerKind::Image => "image",
            drawing_layers::DrawingLayersLayerKind::Group => "group",
            drawing_layers::DrawingLayersLayerKind::Boolean => "boolean",
            drawing_layers::DrawingLayersLayerKind::Trace => "trace",
        }.to_string())
    }
}
impl dsl_core::FromValue for drawing_layers::DrawingLayersLayerKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for DrawingLayersLayerKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "shape" => drawing_layers::DrawingLayersLayerKind::Shape,
            "path" => drawing_layers::DrawingLayersLayerKind::Path,
            "text" => drawing_layers::DrawingLayersLayerKind::Text,
            "image" => drawing_layers::DrawingLayersLayerKind::Image,
            "group" => drawing_layers::DrawingLayersLayerKind::Group,
            "boolean" => drawing_layers::DrawingLayersLayerKind::Boolean,
            "trace" => drawing_layers::DrawingLayersLayerKind::Trace,
            other => return Err(dsl_core::ValueError::new(format!("unknown DrawingLayersLayerKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for puzzle3d_default::Puzzle3dDefaultEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            puzzle3d_default::Puzzle3dDefaultEdgeKind::Puzzle3dAttractionLink => "puzzle3d.attraction.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for puzzle3d_default::Puzzle3dDefaultEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle3dDefaultEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "puzzle3d.attraction.link" => puzzle3d_default::Puzzle3dDefaultEdgeKind::Puzzle3dAttractionLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle3dDefaultEdgeKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for puzzle3d_default::Puzzle3dDefaultPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            puzzle3d_default::Puzzle3dDefaultPortKind::Vortex => "vortex",
        }.to_string())
    }
}
impl dsl_core::FromValue for puzzle3d_default::Puzzle3dDefaultPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle3dDefaultPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "vortex" => puzzle3d_default::Puzzle3dDefaultPortKind::Vortex,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle3dDefaultPortKind `{other}`"))),
        })
    }
}

impl dsl_core::ToValue for puzzle3d_default::Puzzle3dDefaultWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            puzzle3d_default::Puzzle3dDefaultWireKind::CableLink => "cable.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for puzzle3d_default::Puzzle3dDefaultWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle3dDefaultWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "cable.link" => puzzle3d_default::Puzzle3dDefaultWireKind::CableLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle3dDefaultWireKind `{other}`"))),
        })
    }
}
