//! 💾️ Binary representation codec surface for `stdio.semio.graph` (diff) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::graph::schema::diff::*;
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
/// 🧪️ Hand-rolled `protocol::DiffCodec` — `graph`'s two collection fields print as
/// `nodes=[<node>,...]`/`edges=[<edge>,...]` joined by `;` (empty string = no-op diff), reusing the
/// snapshot facet's own real hex/bracket node/edge encoders (duplicated locally, same convention
/// every sibling subset's `🔺️diff` facet already establishes — see that facet's own doc comment
/// for why).
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::flow::io::text::diff::{dec_node};
use crate::flow::io::text::diff::{enc_node};
use crate::flow::io::text::diff::{dec_edge};
use crate::flow::io::text::diff::{enc_edge};

impl protocol::DiffBinary for SemioGraphDiff {
/// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`nodes`, bit1=`edges`) are two
/// REAL fixed fields; when present, each list follows as a real varint count + per-record
/// binary encoding (reusing the snapshot facet's own `write_node`/`read_node`/`write_edge`/
/// `read_edge`) rather than a text-blob-in-binary shortcut.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::graph::schema::snapshot::{write_edge};
    use crate::standards::v5::subsets::any::io::text::snapshot::{write_node};
    let presence: u8 = (if self.nodes.is_some() { 0b0000_0001 } else { 0 }) | (if self.edges.is_some() { 0b0000_0010 } else { 0 });
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(list) = &self.nodes {
        store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
        for n in &list.values {
            write_node(&mut out, n);
        }
    }
    if let Some(list) = &self.edges {
        store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
        for e in &list.values {
            write_edge(&mut out, e);
        }
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::graph::schema::snapshot::{read_edge, read_node};
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let nodes = if presence & 0b0000_0001 != 0 {
        let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff nodes count", offset: 2, detail: e.to_string() })?;
        let mut values = Vec::with_capacity(count as usize);
        for _ in 0..count {
            values.push(read_node(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff node", offset: 2, detail: e })?);
        }
        Some(SemioGraphNodeList { values })
    } else {
        None
    };
    let edges = if presence & 0b0000_0010 != 0 {
        let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff edges count", offset: 2, detail: e.to_string() })?;
        let mut values = Vec::with_capacity(count as usize);
        for _ in 0..count {
            values.push(read_edge(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff edge", offset: 2, detail: e })?);
        }
        Some(SemioGraphEdgeList { values })
    } else {
        None
    };
    Ok(SemioGraphDiff { nodes, edges })
}
}
}
pub use diff_codec::*;
