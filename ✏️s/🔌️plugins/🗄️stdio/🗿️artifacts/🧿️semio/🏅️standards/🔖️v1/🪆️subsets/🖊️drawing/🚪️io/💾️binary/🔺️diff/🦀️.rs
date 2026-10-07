//! 💾️ Binary representation codec surface for `stdio.semio.drawing` (diff). The real
//! encode/decode is `SemioDrawingDiff`'s hand-rolled `protocol::DiffCodec` impl
//! (../🦀️.rs) -- no separate envelope, the bytes ARE the text-facet grammar verbatim.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::drawing::schema::diff::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, NamedModified};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_style};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_style};
use crate::standards::v1::subsets::mesh::io::text::diff::{dec_rgba};
use crate::standards::v1::subsets::mesh::io::text::diff::{enc_rgba};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_layer};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_layer};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_node};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_node};
use crate::standards::v1::subsets::cad::io::text::diff::{dec_point2};
use crate::standards::v1::subsets::cad::io::text::diff::{enc_point2};
use crate::standards::v1::subsets::model::io::text::diff::{dec_transform};
use crate::standards::v1::subsets::model::io::text::diff::{enc_transform};
use crate::standards::v1::subsets::model::io::text::diff::{dec_list};
use crate::standards::v1::subsets::model::io::text::diff::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_path_segment};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_path_segment};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1::subsets::base::schema::triples::IndexedTripleDiff;
use crate::standards::v1::subsets::base::schema::triples::NamedTripleDiff;









// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

















/// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
/// shortcut. `format u8` + `presence u8` (bit0=`canvas`, bit1=`styles`, bit2=`layers`) are two REAL
/// fixed header fields; past that, 0-3 varint-length-prefixed opaque blobs follow (one per present
/// collection, reusing the same `enc_canvas`/`enc_named_triple`/`enc_indexed_triple` text this
/// facet's own `print_diff` already emits) -- one opaque blob per present field rather than
/// per-segment `Cond`-guards (`protocol-cond-cannot-chain`: a second `if`-guard on a field that's
/// itself only conditionally decoded hard-errors `eval_cond`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}



impl protocol::DiffBinary for SemioDrawingDiff {
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.canvas.is_some() {
        presence |= 1;
    }
    if self.styles.is_some() {
        presence |= 2;
    }
    if self.layers.is_some() {
        presence |= 4;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(c) = &self.canvas {
        write_bytes_lp(&mut out, enc_canvas(c).as_bytes());
    }
    if let Some(s) = &self.styles {
        write_bytes_lp(&mut out, enc_named_triple(s, |k: &String| enc_str(k), enc_style_diff, enc_style).as_bytes());
    }
    if let Some(l) = &self.layers {
        write_bytes_lp(&mut out, enc_indexed_triple(l, enc_layer_diff, enc_layer).as_bytes());
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: e.to_string() })?;
    if format != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {format}") });
    }
    let presence = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff presence", offset: 1, detail: e.to_string() })?;
    let map_err = |what: &'static str| move |e: String| protocol::ProtocolError::Malformed { what, offset: 2, detail: e };
    let canvas = if presence & 1 != 0 {
        let blob = read_bytes_lp(&mut reader).map_err(map_err("diff canvas blob"))?;
        let text = std::str::from_utf8(&blob).map_err(|e| protocol::ProtocolError::Malformed { what: "diff canvas utf8", offset: 2, detail: e.to_string() })?;
        Some(dec_canvas(text).map_err(map_err("diff canvas"))?)
    } else {
        None
    };
    let styles = if presence & 2 != 0 {
        let blob = read_bytes_lp(&mut reader).map_err(map_err("diff styles blob"))?;
        let text = std::str::from_utf8(&blob).map_err(|e| protocol::ProtocolError::Malformed { what: "diff styles utf8", offset: 2, detail: e.to_string() })?;
        Some(dec_named_triple(text, dec_str, dec_style_diff, dec_style).map_err(map_err("diff styles"))?)
    } else {
        None
    };
    let layers = if presence & 4 != 0 {
        let blob = read_bytes_lp(&mut reader).map_err(map_err("diff layers blob"))?;
        let text = std::str::from_utf8(&blob).map_err(|e| protocol::ProtocolError::Malformed { what: "diff layers utf8", offset: 2, detail: e.to_string() })?;
        Some(dec_indexed_triple(text, dec_layer_diff, dec_layer).map_err(map_err("diff layers"))?)
    } else {
        None
    };
    Ok(SemioDrawingDiff { canvas, styles, layers })
}
}

use crate::standards::v1::subsets::drawing::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, enc_node_diff, dec_node_diff, enc_canvas, dec_canvas, enc_style_diff, dec_style_diff, enc_layer_diff, dec_layer_diff};
}
pub use diff_codec::*;
