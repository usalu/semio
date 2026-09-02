// 🌉️ Hand-written `ToValue`/`FromValue` bridge for the machine-generated manifest enums in
// sibling `🤖️generated/🦀️*.rs` files (headers read "// Generated from *.manifest.json" —
// never hand-edited). Each enum carries an explicit per-variant `#[serde(rename = "...")]`
// wire string (not a uniform `rename_all` case), which the `#[derive(ToValue, FromValue)]`
// macro can only apply via `#[value(rename = "...")]` attributes on the type itself — those
// attributes cannot be added to a generated file, so this file hand-writes the identical
// string mapping instead, matching each enum's `#[serde(rename = "...")]` byte for byte.
// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01/02), additive phase:
// ADDITIVE ONLY — every enum below keeps its existing `Serialize`/`Deserialize` untouched.

impl dsl_core::ToValue for crate::manifest::generated::flow_dag::FlowDagNodeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Computation => "computation",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Slider => "slider",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Select => "select",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Screen => "screen",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Note => "note",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Image => "image",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Preview => "preview",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Action => "action",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Export => "export",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::Cluster => "cluster",
            crate::manifest::generated::flow_dag::FlowDagNodeKind::AppInstance => "appInstance",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::flow_dag::FlowDagNodeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for FlowDagNodeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "computation" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Computation,
            "slider" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Slider,
            "select" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Select,
            "screen" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Screen,
            "note" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Note,
            "image" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Image,
            "preview" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Preview,
            "action" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Action,
            "export" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Export,
            "cluster" => crate::manifest::generated::flow_dag::FlowDagNodeKind::Cluster,
            "appInstance" => crate::manifest::generated::flow_dag::FlowDagNodeKind::AppInstance,
            other => return Err(dsl_core::ValueError::new(format!("unknown FlowDagNodeKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::rewrite_lhs::RewriteLhsNodeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::rewrite_lhs::RewriteLhsNodeKind::RewriteMatch => "rewrite.match",
            crate::manifest::generated::rewrite_lhs::RewriteLhsNodeKind::RewriteWhere => "rewrite.where",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::rewrite_lhs::RewriteLhsNodeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for RewriteLhsNodeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "rewrite.match" => crate::manifest::generated::rewrite_lhs::RewriteLhsNodeKind::RewriteMatch,
            "rewrite.where" => crate::manifest::generated::rewrite_lhs::RewriteLhsNodeKind::RewriteWhere,
            other => return Err(dsl_core::ValueError::new(format!("unknown RewriteLhsNodeKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::rewrite_lhs::RewriteLhsEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::rewrite_lhs::RewriteLhsEdgeKind::EdgeFlow => "edge.flow",
            crate::manifest::generated::rewrite_lhs::RewriteLhsEdgeKind::EdgePattern => "edge.pattern",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::rewrite_lhs::RewriteLhsEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for RewriteLhsEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "edge.flow" => crate::manifest::generated::rewrite_lhs::RewriteLhsEdgeKind::EdgeFlow,
            "edge.pattern" => crate::manifest::generated::rewrite_lhs::RewriteLhsEdgeKind::EdgePattern,
            other => return Err(dsl_core::ValueError::new(format!("unknown RewriteLhsEdgeKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::rewrite_lhs::RewriteLhsPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::rewrite_lhs::RewriteLhsPortKind::Port => "port",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::rewrite_lhs::RewriteLhsPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for RewriteLhsPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "port" => crate::manifest::generated::rewrite_lhs::RewriteLhsPortKind::Port,
            other => return Err(dsl_core::ValueError::new(format!("unknown RewriteLhsPortKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::rewrite_lhs::RewriteLhsWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::rewrite_lhs::RewriteLhsWireKind::WireFlow => "wire.flow",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::rewrite_lhs::RewriteLhsWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for RewriteLhsWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wire.flow" => crate::manifest::generated::rewrite_lhs::RewriteLhsWireKind::WireFlow,
            other => return Err(dsl_core::ValueError::new(format!("unknown RewriteLhsWireKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultEdgeKind::EdgeLink => "edge.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle2dDefaultEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "edge.link" => crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultEdgeKind::EdgeLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle2dDefaultEdgeKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultPortKind::Port => "port",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle2dDefaultPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "port" => crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultPortKind::Port,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle2dDefaultPortKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultWireKind::WireLink => "wire.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle2dDefaultWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wire.link" => crate::manifest::generated::puzzle2d_default::Puzzle2dDefaultWireKind::WireLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle2dDefaultWireKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::nakagin::NakaginNodeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::nakagin::NakaginNodeKind::Balcony => "Balcony",
            crate::manifest::generated::nakagin::NakaginNodeKind::Base => "Base",
            crate::manifest::generated::nakagin::NakaginNodeKind::BaseBlob => "Base Blob",
            crate::manifest::generated::nakagin::NakaginNodeKind::Bridge => "Bridge",
            crate::manifest::generated::nakagin::NakaginNodeKind::Capital => "Capital",
            crate::manifest::generated::nakagin::NakaginNodeKind::Capsule => "Capsule",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleBackslash => "Capsule Backslash",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleJ => "Capsule J",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleL => "Capsule L",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleP => "Capsule P",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleQ => "Capsule q",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleS => "Capsule S",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleSlash => "Capsule Slash",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyBackslash => "Capsule With Balcony Backslash",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyJ => "Capsule With Balcony J",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyL => "Capsule With Balcony L",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyP => "Capsule With Balcony P",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyQ => "Capsule With Balcony Q",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyS => "Capsule With Balcony S",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconySlash => "Capsule With Balcony Slash",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyZ => "Capsule With Balcony Z",
            crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleZ => "Capsule Z",
            crate::manifest::generated::nakagin::NakaginNodeKind::CylindricCapital => "Cylindric Capital",
            crate::manifest::generated::nakagin::NakaginNodeKind::CylindricFirstStoreyTambour => "Cylindric First Storey Tambour",
            crate::manifest::generated::nakagin::NakaginNodeKind::CylindricLastStoreyTambour => "Cylindric Last Storey Tambour",
            crate::manifest::generated::nakagin::NakaginNodeKind::CylindricSingleStoreyTambour => "Cylindric Single Storey Tambour",
            crate::manifest::generated::nakagin::NakaginNodeKind::CylindricTambour => "Cylindric Tambour",
            crate::manifest::generated::nakagin::NakaginNodeKind::Ellipsoid => "Ellipsoid",
            crate::manifest::generated::nakagin::NakaginNodeKind::FirstStoreyTambour => "First Storey Tambour",
            crate::manifest::generated::nakagin::NakaginNodeKind::LastStoreyTambour => "Last Storey Tambour",
            crate::manifest::generated::nakagin::NakaginNodeKind::SingleStoreyTambour => "Single Storey Tambour",
            crate::manifest::generated::nakagin::NakaginNodeKind::Tambour => "Tambour",
            crate::manifest::generated::nakagin::NakaginNodeKind::Trapezoid => "Trapezoid",
            crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleBackslash => "Trapezoid Capsule Backslash",
            crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleJ => "Trapezoid Capsule J",
            crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleL => "Trapezoid Capsule L",
            crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleP => "Trapezoid Capsule P",
            crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleQ => "Trapezoid Capsule Q",
            crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleS => "Trapezoid Capsule S",
            crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleSlash => "Trapezoid Capsule Slash",
            crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleZ => "Trapezoid Capsule Z",
            crate::manifest::generated::nakagin::NakaginNodeKind::Piece => "Piece",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::nakagin::NakaginNodeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for NakaginNodeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "Balcony" => crate::manifest::generated::nakagin::NakaginNodeKind::Balcony,
            "Base" => crate::manifest::generated::nakagin::NakaginNodeKind::Base,
            "Base Blob" => crate::manifest::generated::nakagin::NakaginNodeKind::BaseBlob,
            "Bridge" => crate::manifest::generated::nakagin::NakaginNodeKind::Bridge,
            "Capital" => crate::manifest::generated::nakagin::NakaginNodeKind::Capital,
            "Capsule" => crate::manifest::generated::nakagin::NakaginNodeKind::Capsule,
            "Capsule Backslash" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleBackslash,
            "Capsule J" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleJ,
            "Capsule L" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleL,
            "Capsule P" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleP,
            "Capsule q" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleQ,
            "Capsule S" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleS,
            "Capsule Slash" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleSlash,
            "Capsule With Balcony Backslash" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyBackslash,
            "Capsule With Balcony J" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyJ,
            "Capsule With Balcony L" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyL,
            "Capsule With Balcony P" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyP,
            "Capsule With Balcony Q" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyQ,
            "Capsule With Balcony S" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyS,
            "Capsule With Balcony Slash" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconySlash,
            "Capsule With Balcony Z" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleWithBalconyZ,
            "Capsule Z" => crate::manifest::generated::nakagin::NakaginNodeKind::CapsuleZ,
            "Cylindric Capital" => crate::manifest::generated::nakagin::NakaginNodeKind::CylindricCapital,
            "Cylindric First Storey Tambour" => crate::manifest::generated::nakagin::NakaginNodeKind::CylindricFirstStoreyTambour,
            "Cylindric Last Storey Tambour" => crate::manifest::generated::nakagin::NakaginNodeKind::CylindricLastStoreyTambour,
            "Cylindric Single Storey Tambour" => crate::manifest::generated::nakagin::NakaginNodeKind::CylindricSingleStoreyTambour,
            "Cylindric Tambour" => crate::manifest::generated::nakagin::NakaginNodeKind::CylindricTambour,
            "Ellipsoid" => crate::manifest::generated::nakagin::NakaginNodeKind::Ellipsoid,
            "First Storey Tambour" => crate::manifest::generated::nakagin::NakaginNodeKind::FirstStoreyTambour,
            "Last Storey Tambour" => crate::manifest::generated::nakagin::NakaginNodeKind::LastStoreyTambour,
            "Single Storey Tambour" => crate::manifest::generated::nakagin::NakaginNodeKind::SingleStoreyTambour,
            "Tambour" => crate::manifest::generated::nakagin::NakaginNodeKind::Tambour,
            "Trapezoid" => crate::manifest::generated::nakagin::NakaginNodeKind::Trapezoid,
            "Trapezoid Capsule Backslash" => crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleBackslash,
            "Trapezoid Capsule J" => crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleJ,
            "Trapezoid Capsule L" => crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleL,
            "Trapezoid Capsule P" => crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleP,
            "Trapezoid Capsule Q" => crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleQ,
            "Trapezoid Capsule S" => crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleS,
            "Trapezoid Capsule Slash" => crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleSlash,
            "Trapezoid Capsule Z" => crate::manifest::generated::nakagin::NakaginNodeKind::TrapezoidCapsuleZ,
            "Piece" => crate::manifest::generated::nakagin::NakaginNodeKind::Piece,
            other => return Err(dsl_core::ValueError::new(format!("unknown NakaginNodeKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::nakagin::NakaginEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::nakagin::NakaginEdgeKind::Connection => "Connection",
            crate::manifest::generated::nakagin::NakaginEdgeKind::EdgeLink => "edge.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::nakagin::NakaginEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for NakaginEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "Connection" => crate::manifest::generated::nakagin::NakaginEdgeKind::Connection,
            "edge.link" => crate::manifest::generated::nakagin::NakaginEdgeKind::EdgeLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown NakaginEdgeKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::nakagin::NakaginPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::nakagin::NakaginPortKind::Connector => "Connector",
            crate::manifest::generated::nakagin::NakaginPortKind::CoreCircularBottom => "core circular bottom",
            crate::manifest::generated::nakagin::NakaginPortKind::CoreCircularTop => "core circular top",
            crate::manifest::generated::nakagin::NakaginPortKind::CoreRectangularBottom => "core rectangular bottom",
            crate::manifest::generated::nakagin::NakaginPortKind::CoreRectangularTop => "core rectangular top",
            crate::manifest::generated::nakagin::NakaginPortKind::DoorCapsuleRight => "door capsule right",
            crate::manifest::generated::nakagin::NakaginPortKind::DoorCapsuleLeft => "door capsule left",
            crate::manifest::generated::nakagin::NakaginPortKind::DoorTambourLeft => "door tambour left",
            crate::manifest::generated::nakagin::NakaginPortKind::DoorTambourRight => "door tambour right",
            crate::manifest::generated::nakagin::NakaginPortKind::PlatformRight => "platform right",
            crate::manifest::generated::nakagin::NakaginPortKind::PlatformLeft => "platform left",
            crate::manifest::generated::nakagin::NakaginPortKind::RoofCircularBottom => "roof circular bottom",
            crate::manifest::generated::nakagin::NakaginPortKind::RoofCircularTop => "roof circular top",
            crate::manifest::generated::nakagin::NakaginPortKind::RoofRectangularBottom => "roof rectangular bottom",
            crate::manifest::generated::nakagin::NakaginPortKind::RoofRectangularTop => "roof rectangular top",
            crate::manifest::generated::nakagin::NakaginPortKind::TambourCircularBottom => "tambour circular bottom",
            crate::manifest::generated::nakagin::NakaginPortKind::TambourCircularTop => "tambour circular top",
            crate::manifest::generated::nakagin::NakaginPortKind::TambourRectangularBottom => "tambour rectangular bottom",
            crate::manifest::generated::nakagin::NakaginPortKind::TambourRectangularTop => "tambour rectangular top",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::nakagin::NakaginPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for NakaginPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "Connector" => crate::manifest::generated::nakagin::NakaginPortKind::Connector,
            "core circular bottom" => crate::manifest::generated::nakagin::NakaginPortKind::CoreCircularBottom,
            "core circular top" => crate::manifest::generated::nakagin::NakaginPortKind::CoreCircularTop,
            "core rectangular bottom" => crate::manifest::generated::nakagin::NakaginPortKind::CoreRectangularBottom,
            "core rectangular top" => crate::manifest::generated::nakagin::NakaginPortKind::CoreRectangularTop,
            "door capsule right" => crate::manifest::generated::nakagin::NakaginPortKind::DoorCapsuleRight,
            "door capsule left" => crate::manifest::generated::nakagin::NakaginPortKind::DoorCapsuleLeft,
            "door tambour left" => crate::manifest::generated::nakagin::NakaginPortKind::DoorTambourLeft,
            "door tambour right" => crate::manifest::generated::nakagin::NakaginPortKind::DoorTambourRight,
            "platform right" => crate::manifest::generated::nakagin::NakaginPortKind::PlatformRight,
            "platform left" => crate::manifest::generated::nakagin::NakaginPortKind::PlatformLeft,
            "roof circular bottom" => crate::manifest::generated::nakagin::NakaginPortKind::RoofCircularBottom,
            "roof circular top" => crate::manifest::generated::nakagin::NakaginPortKind::RoofCircularTop,
            "roof rectangular bottom" => crate::manifest::generated::nakagin::NakaginPortKind::RoofRectangularBottom,
            "roof rectangular top" => crate::manifest::generated::nakagin::NakaginPortKind::RoofRectangularTop,
            "tambour circular bottom" => crate::manifest::generated::nakagin::NakaginPortKind::TambourCircularBottom,
            "tambour circular top" => crate::manifest::generated::nakagin::NakaginPortKind::TambourCircularTop,
            "tambour rectangular bottom" => crate::manifest::generated::nakagin::NakaginPortKind::TambourRectangularBottom,
            "tambour rectangular top" => crate::manifest::generated::nakagin::NakaginPortKind::TambourRectangularTop,
            other => return Err(dsl_core::ValueError::new(format!("unknown NakaginPortKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::nakagin::NakaginWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::nakagin::NakaginWireKind::WireLink => "wire.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::nakagin::NakaginWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for NakaginWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wire.link" => crate::manifest::generated::nakagin::NakaginWireKind::WireLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown NakaginWireKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultEdgeKind::EdgeLink => "edge.link",
            crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultEdgeKind::AttractionLink => "attraction.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle5dDefaultEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "edge.link" => crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultEdgeKind::EdgeLink,
            "attraction.link" => crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultEdgeKind::AttractionLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle5dDefaultEdgeKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultPortKind::Port => "port",
            crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultPortKind::Vortex => "vortex",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle5dDefaultPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "port" => crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultPortKind::Port,
            "vortex" => crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultPortKind::Vortex,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle5dDefaultPortKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultWireKind::WireLink => "wire.link",
            crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultWireKind::CableLink => "cable.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle5dDefaultWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wire.link" => crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultWireKind::WireLink,
            "cable.link" => crate::manifest::generated::puzzle5d_default::Puzzle5dDefaultWireKind::CableLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle5dDefaultWireKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind::Jack => "jack",
            crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind::Wire => "wire",
            crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind::Plaintext => "plaintext",
            crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind::Markdown => "markdown",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for WriterLanguagesLanguageKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "jack" => crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind::Jack,
            "wire" => crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind::Wire,
            "plaintext" => crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind::Plaintext,
            "markdown" => crate::manifest::generated::writer_languages::WriterLanguagesLanguageKind::Markdown,
            other => return Err(dsl_core::ValueError::new(format!("unknown WriterLanguagesLanguageKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::wires::WiresEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::wires::WiresEdgeKind::WiresOwns => "wires.owns",
            crate::manifest::generated::wires::WiresEdgeKind::WiresIs => "wires.is",
            crate::manifest::generated::wires::WiresEdgeKind::WiresReferences => "wires.references",
            crate::manifest::generated::wires::WiresEdgeKind::WiresHas => "wires.has",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::wires::WiresEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for WiresEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "wires.owns" => crate::manifest::generated::wires::WiresEdgeKind::WiresOwns,
            "wires.is" => crate::manifest::generated::wires::WiresEdgeKind::WiresIs,
            "wires.references" => crate::manifest::generated::wires::WiresEdgeKind::WiresReferences,
            "wires.has" => crate::manifest::generated::wires::WiresEdgeKind::WiresHas,
            other => return Err(dsl_core::ValueError::new(format!("unknown WiresEdgeKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::draw_layers::DrawLayersLayerKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::draw_layers::DrawLayersLayerKind::Shape => "shape",
            crate::manifest::generated::draw_layers::DrawLayersLayerKind::Path => "path",
            crate::manifest::generated::draw_layers::DrawLayersLayerKind::Text => "text",
            crate::manifest::generated::draw_layers::DrawLayersLayerKind::Image => "image",
            crate::manifest::generated::draw_layers::DrawLayersLayerKind::Group => "group",
            crate::manifest::generated::draw_layers::DrawLayersLayerKind::Boolean => "boolean",
            crate::manifest::generated::draw_layers::DrawLayersLayerKind::Trace => "trace",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::draw_layers::DrawLayersLayerKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for DrawLayersLayerKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "shape" => crate::manifest::generated::draw_layers::DrawLayersLayerKind::Shape,
            "path" => crate::manifest::generated::draw_layers::DrawLayersLayerKind::Path,
            "text" => crate::manifest::generated::draw_layers::DrawLayersLayerKind::Text,
            "image" => crate::manifest::generated::draw_layers::DrawLayersLayerKind::Image,
            "group" => crate::manifest::generated::draw_layers::DrawLayersLayerKind::Group,
            "boolean" => crate::manifest::generated::draw_layers::DrawLayersLayerKind::Boolean,
            "trace" => crate::manifest::generated::draw_layers::DrawLayersLayerKind::Trace,
            other => return Err(dsl_core::ValueError::new(format!("unknown DrawLayersLayerKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultEdgeKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultEdgeKind::Puzzle3dAttractionLink => "puzzle3d.attraction.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultEdgeKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle3dDefaultEdgeKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "puzzle3d.attraction.link" => crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultEdgeKind::Puzzle3dAttractionLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle3dDefaultEdgeKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultPortKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultPortKind::Vortex => "vortex",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultPortKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle3dDefaultPortKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "vortex" => crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultPortKind::Vortex,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle3dDefaultPortKind `{{other}}`"))),
        })
    }
}

impl dsl_core::ToValue for crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultWireKind {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::String(match self {
            crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultWireKind::CableLink => "cable.link",
        }.to_string())
    }
}
impl dsl_core::FromValue for crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultWireKind {
    fn from_value(value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        let dsl_core::DslValue::String(s) = value else {
            return Err(dsl_core::ValueError::new(format!("expected a string for Puzzle3dDefaultWireKind, found {value:?}")));
        };
        Ok(match s.as_str() {
            "cable.link" => crate::manifest::generated::puzzle3d_default::Puzzle3dDefaultWireKind::CableLink,
            other => return Err(dsl_core::ValueError::new(format!("unknown Puzzle3dDefaultWireKind `{{other}}`"))),
        })
    }
}
