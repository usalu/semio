//! 💾️ Binary representation codec surface for `stdio.semio.graph` (diff) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::graph::schema::diff::*;
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
/// 🧪️ Hand-rolled `protocol::DiffCodec` — `graph`'s two collection fields print as
/// `nodes=[<node>,...]`/`edges=[<edge>,...]` joined by `;` (empty string = no-op diff), reusing the
/// snapshot facet's own real hex/bracket node/edge encoders (duplicated locally, same convention
/// every sibling subset's `🔺️diff` facet already establishes — see that facet's own doc comment
/// for why).
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::graph::io::text::snapshot::{dec_node};
use crate::standards::v1::subsets::graph::io::text::snapshot::{enc_node};
use crate::standards::v1::subsets::graph::io::text::snapshot::{dec_edge};
use crate::standards::v1::subsets::graph::io::text::snapshot::{enc_edge};

impl protocol::DiffBinary for SemioGraphDiff {
/// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`nodes`, bit1=`edges`) are two REAL fixed fields; each present
/// section follows as a varint byte length plus the same `enc_indexed_triple` text this facet's `print_diff` emits.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::graph::io::text::diff::{enc_nodes, enc_edges};
    let presence: u8 = (if self.nodes.is_some() { 0b0000_0001 } else { 0 }) | (if self.edges.is_some() { 0b0000_0010 } else { 0 });
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    for section in [self.nodes.as_ref().map(enc_nodes), self.edges.as_ref().map(enc_edges)].into_iter().flatten() {
        store::pack_rt::write_varint_u64(&mut out, section.len() as u64);
        out.extend_from_slice(section.as_bytes());
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::graph::io::text::diff::{dec_nodes, dec_edges};
    let malformed = |what: &'static str, detail: String| protocol::ProtocolError::Malformed { what, offset: 2, detail };
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let mut section = |what: &'static str| -> Result<String, protocol::ProtocolError> {
        let length = reader.read_varint_u64().map_err(|e| malformed(what, e.to_string()))? as usize;
        let raw = reader.read_bytes(length).map_err(|e| malformed(what, e.to_string()))?;
        String::from_utf8(raw.to_vec()).map_err(|e| malformed(what, e.to_string()))
    };
    let nodes = if presence & 0b0000_0001 != 0 { Some(dec_nodes(&section("diff nodes")?).map_err(|e| malformed("diff nodes", e))?) } else { None };
    let edges = if presence & 0b0000_0010 != 0 { Some(dec_edges(&section("diff edges")?).map_err(|e| malformed("diff edges", e))?) } else { None };
    Ok(SemioGraphDiff { nodes, edges })
}
}
}
pub use diff_codec::*;
