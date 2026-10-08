//! 💾️ Binary representation codec surface for `s.stdio.semio.flow` (mutations) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::flow::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::flow::schema::diff::{diff_insert_edge, diff_insert_node, diff_remove_edge, diff_remove_node, diff_remove_node_param, diff_set_edge_endpoints, diff_set_edge_kind, diff_set_node_kind, diff_set_node_label, diff_set_node_param, diff_set_node_position, SemioFlowDiff};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_edge};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_edge};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_node};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_node};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_port_ref};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_port_ref};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_point2};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_point2};
use crate::standards::v1::subsets::flow::io::text::snapshot::{dec_f64};
use crate::standards::v1::subsets::flow::io::text::snapshot::{enc_f64};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef, SemioFlowSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioFlowMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).
use protocol::{OpBinary, OpText};
use crate::standards::v1::subsets::flow::io::text::mutations::{print_flow_mutation};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioFlowMutation) -> u8 {
    match m {
        SemioFlowMutation::InsertNode(_) => TAG_INSERT_NODE,
        SemioFlowMutation::RemoveNode(_) => TAG_REMOVE_NODE,
        SemioFlowMutation::SetNodeKind(_) => TAG_SET_NODE_KIND,
        SemioFlowMutation::SetNodeLabel(_) => TAG_SET_NODE_LABEL,
        SemioFlowMutation::SetNodePosition(_) => TAG_SET_NODE_POSITION,
        SemioFlowMutation::SetNodeParam(_) => TAG_SET_NODE_PARAM,
        SemioFlowMutation::RemoveNodeParam(_) => TAG_REMOVE_NODE_PARAM,
        SemioFlowMutation::InsertEdge(_) => TAG_INSERT_EDGE,
        SemioFlowMutation::RemoveEdge(_) => TAG_REMOVE_EDGE,
        SemioFlowMutation::SetEdgeEndpoints(_) => TAG_SET_EDGE_ENDPOINTS,
        SemioFlowMutation::SetEdgeKind(_) => TAG_SET_EDGE_KIND,
        SemioFlowMutation::DragNodes(_) => TAG_DRAG_NODES,
    }
}

/// ✂️ Just the `key=value ...` argument tail of `print_flow_mutation` — the binary frame's `tag`
/// byte already carries the keyword, so the text keyword itself is redundant in the binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_flow_mutation_args(m: &SemioFlowMutation) -> String {
    match print_flow_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

impl OpBinary for SemioFlowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_flow_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioFlowMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_INSERT_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-node");
const TAG_REMOVE_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-node");
const TAG_SET_NODE_KIND: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-node-kind");
const TAG_SET_NODE_LABEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-node-label");
const TAG_SET_NODE_POSITION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-node-position");
const TAG_SET_NODE_PARAM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-node-param");
const TAG_REMOVE_NODE_PARAM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-node-param");
const TAG_INSERT_EDGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-edge");
const TAG_REMOVE_EDGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-edge");
const TAG_SET_EDGE_ENDPOINTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-edge-endpoints");
const TAG_SET_EDGE_KIND: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-edge-kind");
const TAG_DRAG_NODES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "drag-nodes");
//#endregion 🏷️WireTags

#[path = "📦️codec/🫳️borrowed/🦀️.rs"]
mod borrowed_operation_source;
