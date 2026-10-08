//! 💾️ Binary representation codec surface for `stdio.semio.brep` (diff).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::brep::schema::diff::*;
use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_named_added, dec_named_triple, enc_named_added, enc_named_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_face};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_face};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_edge};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_edge};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_vertex};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_vertex};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

















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

































































/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`) backing the real `DiffBinary::encode_diff`/`decode_diff` below — replaces
/// the old `print_diff().into_bytes()` text-as-binary shortcut.
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

impl protocol::DiffBinary for SemioBrepDiff {
/// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
/// shortcut. `format u8` + `presence u8` (bit0=`vertices`, bit1=`edges`, bit2=`loops`,
/// bit3=`faces`, bit4=`shells`, bit5=`solids`) are two REAL fixed fields; each present
/// collection then follows as its own varint-length-prefixed opaque blob (the same
/// `enc_*_diff` bracket/hex text this type's `print_diff` already produces) — independently-
/// delimited segments rather than one bare trailing `bytes` because there can be 0-6 of them
/// (chaining a `Cond` per-segment hits the `protocol-cond-cannot-chain` gap: a second
/// `if`-guard on a field that was itself only conditionally decoded hard-errors `eval_cond`).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.vertices.is_some() {
        presence |= 0b0000_0001;
    }
    if self.edges.is_some() {
        presence |= 0b0000_0010;
    }
    if self.loops.is_some() {
        presence |= 0b0000_0100;
    }
    if self.faces.is_some() {
        presence |= 0b0000_1000;
    }
    if self.shells.is_some() {
        presence |= 0b0001_0000;
    }
    if self.solids.is_some() {
        presence |= 0b0010_0000;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = &self.vertices {
        write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_vertex_diff, |a| enc_named_added(a, enc_vertex)));
    }
    if let Some(v) = &self.edges {
        write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_edge_diff, |a| enc_named_added(a, enc_edge)));
    }
    if let Some(v) = &self.loops {
        write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_loop_diff, |a| enc_named_added(a, enc_loop)));
    }
    if let Some(v) = &self.faces {
        write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_face_diff, |a| enc_named_added(a, enc_face)));
    }
    if let Some(v) = &self.shells {
        write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_shell_diff, |a| enc_named_added(a, enc_shell)));
    }
    if let Some(v) = &self.solids {
        write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_solid_diff, |a| enc_named_added(a, enc_solid)));
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let mut next_blob = |what: &'static str| -> Result<String, protocol::ProtocolError> { read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what, offset: 2, detail: e }) };
    let vertices = if presence & 0b0000_0001 != 0 {
        Some(dec_named_triple(&next_blob("diff vertices blob")?, dec_str, dec_vertex_diff, |t| dec_named_added(t, dec_vertex)).map_err(|e| protocol::ProtocolError::Malformed { what: "diff vertices text", offset: 2, detail: e })?)
    } else {
        None
    };
    let edges =
        if presence & 0b0000_0010 != 0 { Some(dec_named_triple(&next_blob("diff edges blob")?, dec_str, dec_edge_diff, |t| dec_named_added(t, dec_edge)).map_err(|e| protocol::ProtocolError::Malformed { what: "diff edges text", offset: 2, detail: e })?) } else { None };
    let loops =
        if presence & 0b0000_0100 != 0 { Some(dec_named_triple(&next_blob("diff loops blob")?, dec_str, dec_loop_diff, |t| dec_named_added(t, dec_loop)).map_err(|e| protocol::ProtocolError::Malformed { what: "diff loops text", offset: 2, detail: e })?) } else { None };
    let faces =
        if presence & 0b0000_1000 != 0 { Some(dec_named_triple(&next_blob("diff faces blob")?, dec_str, dec_face_diff, |t| dec_named_added(t, dec_face)).map_err(|e| protocol::ProtocolError::Malformed { what: "diff faces text", offset: 2, detail: e })?) } else { None };
    let shells = if presence & 0b0001_0000 != 0 {
        Some(dec_named_triple(&next_blob("diff shells blob")?, dec_str, dec_shell_diff, |t| dec_named_added(t, dec_shell)).map_err(|e| protocol::ProtocolError::Malformed { what: "diff shells text", offset: 2, detail: e })?)
    } else {
        None
    };
    let solids = if presence & 0b0010_0000 != 0 {
        Some(dec_named_triple(&next_blob("diff solids blob")?, dec_str, dec_solid_diff, |t| dec_named_added(t, dec_solid)).map_err(|e| protocol::ProtocolError::Malformed { what: "diff solids text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioBrepDiff { vertices, edges, loops, faces, shells, solids })
}
}

use crate::standards::v1::subsets::brep::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, parse_f64, parse_u32, enc_bool, parse_bool, enc_list, dec_list, enc_point3, dec_point3, enc_curve, dec_curve, enc_surface, dec_surface, enc_loop_edge, dec_loop_edge, enc_shell_face, dec_shell_face, enc_solid_shell, dec_solid_shell, enc_loop, dec_loop, enc_shell, dec_shell, enc_solid, dec_solid, enc_vertex_diff, dec_vertex_diff, enc_edge_diff, dec_edge_diff, enc_loop_diff, dec_loop_diff, enc_face_diff, dec_face_diff, enc_shell_diff, dec_shell_diff, enc_solid_diff, dec_solid_diff};
}
pub use diff_codec::*;
