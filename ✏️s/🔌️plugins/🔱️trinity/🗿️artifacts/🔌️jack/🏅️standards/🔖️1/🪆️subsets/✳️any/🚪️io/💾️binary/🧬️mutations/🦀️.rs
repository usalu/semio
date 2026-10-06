//! 📡️ Trinity graph mutation binary framing and registry surface.

/// 🧾️ Direct-owner binary tags in aggregate declaration order.
pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[("SetQuery", crate::standards::v1::subsets::any::schema::mutations::set_query::BINARY_TAG)];

#[path = "🔎️set-query/🦀️.rs"]
pub mod set_query;

//! 📡️ `trinity.graph` artifact — state-patch wire codec for the raw document operation
//! (constitutional: spr, renamed from the old `📡️protocol` — no `📡️protocol` segment survives).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::executor::GraphEffect;
use crate::{Edge, EntityRef, JackSnapshot, Node, Port, PropertyBag, PropertyDef, PropertyValue};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_diagnostic::TextError;
use semio_framework_value::{ValueError,ValueRefusalKind};

//#region 🔖️DslMirrors

//#region 🔖️HandcraftedOpCodecs


impl OpBinary for TrinityGraphOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("./📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("./📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




//#endregion 🔖️DslMirrors

//#region 🔖️OpText


/// ⚡️ Binary mirror of the `OpText` impl above — `TrinityGraphOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl OpBinary for TrinityGraphMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        trinity_graph_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        TrinityGraphOperationDsl::decode_op(bytes).map(trinity_graph_operation_from_dsl)
    }
}
//#endregion 🔖️OpText

/// 📦️ Encodes a Trinity graph `Mutation` to its binary command form.
pub fn encode_op(operation: &TrinityGraphMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a Trinity graph `Mutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<TrinityGraphMutation, protocol::ProtocolError> {
    TrinityGraphMutation::decode_op(bytes)
}


use crate::standards::v1::subsets::any::io::text::mutations::{TrinityGraphOperationDsl, trinity_graph_operation_to_dsl, trinity_graph_operation_from_dsl};
