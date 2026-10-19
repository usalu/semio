//! binary rep for stdio.svg 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_1::subsets::base::io::binary::snapshot::document_parts::{write_str_lp, read_str_lp, enc_prolog_bin, dec_prolog_bin, enc_doctype_bin, dec_doctype_bin, enc_declaration_bin, dec_declaration_bin, dec_svg_node_bin, enc_svg_node_bin};
use crate::standards::v1_1::subsets::base::schema::diff::*;
use crate::standards::v1_1::subsets::base::io::text::snapshot::document_parts::enc_svg_node;

use crate::SvgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlQuote};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::SvgAttr;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration;
use crate::schema::snapshot::SvgNode;













/// 🧪️ P2-FG3: real recursive binary twins of [`enc_node_diff`]/[`dec_node_diff`] -- same 1-byte tag
/// numbering scheme as [`enc_svg_node_bin`] (`0`=Element/`1`=Text) plus `2`=`Replace` (needs its own
/// arm since `Replace` wraps a whole [`SvgNode`], not a bare scalar payload). `attrs`/`children`
/// collection triples encode as three varint-counted, recursively-encoded lists (removed/modified/
/// added) -- genuinely structured binary, backing the upgraded `DiffBinary::encode_diff`/
/// `decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff_bin(diff: &SvgNodeDiff, out: &mut Vec<u8>) {
    match diff {
        SvgNodeDiff::Element(e) => {
            out.push(0);
            out.push(if e.name.is_some() { 1 } else { 0 });
            if let Some(name) = &e.name {
                write_str_lp(out, name);
            }
            out.push(if e.attributes.is_some() { 1 } else { 0 });
            if let Some(attrs) = &e.attributes {
                enc_attrs_diff_bin(attrs, out);
            }
            out.push(if e.children.is_some() { 1 } else { 0 });
            if let Some(children) = &e.children {
                enc_children_diff_bin(children, out);
            }
        }
        SvgNodeDiff::Text { text } => {
            out.push(1);
            out.push(if text.is_some() { 1 } else { 0 });
            if let Some(text) = text {
                write_str_lp(out, text);
            }
        }
        SvgNodeDiff::Replace { node } => {
            out.push(2);
            out.push(if node.is_some() { 1 } else { 0 });
            if let Some(node) = node {
                enc_svg_node_bin(node, out);
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<SvgNodeDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            let attributes = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_attrs_diff_bin(reader)?) } else { None };
            let children = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_children_diff_bin(reader)?) } else { None };
            Ok(SvgNodeDiff::Element(SvgElementDiff { name, attributes, children }))
        }
        1 => {
            let text = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            Ok(SvgNodeDiff::Text { text })
        }
        2 => {
            let node = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_svg_node_bin(reader)?) } else { None };
            Ok(SvgNodeDiff::Replace { node })
        }
        other => Err(format!("svg node diff binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attrs_diff_bin(diff: &SvgAttributesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for name in &diff.removed {
        write_str_lp(out, name);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        write_str_lp(out, &entry.name);
        write_str_lp(out, &crate::standards::v1_1::subsets::base::io::text::snapshot::print_svg_attribute(&entry.value));
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        write_str_lp(out, &entry.name);
        write_str_lp(out, &crate::standards::v1_1::subsets::base::io::text::snapshot::print_svg_attribute(&entry.value));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attrs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<SvgAttributesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(read_str_lp(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let name = read_str_lp(reader)?;
        let value = read_str_lp(reader)?;
        modified.push(SvgAttrModified { value:crate::standards::v1_1::subsets::base::io::text::snapshot::bind_svg_attribute(&name,&value)?,name });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let name = read_str_lp(reader)?;
        let value = read_str_lp(reader)?;
        added.push(SvgAttrAdded { index,value:crate::standards::v1_1::subsets::base::io::text::snapshot::bind_svg_attribute(&name,&value)?,name });
    }
    Ok(SvgAttributesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_children_diff_bin(diff: &SvgChildrenDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for index in &diff.removed {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_node_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_svg_node_bin(&entry.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_children_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<SvgChildrenDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_node_diff_bin(reader)?;
        modified.push(SvgChildModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_svg_node_bin(reader)?;
        added.push(SvgChildAdded { index, item });
    }
    Ok(SvgChildrenDiff { removed, modified, added })
}

impl protocol::DiffBinary for SvgDiff {
/// 🧪️ P2-FG3: REAL binary frame (`format u8 | flags u8 | [declaration][doctype][root]`),
/// matching `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload
/// bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100%
/// of stdio's `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` bits
/// 0/1/2/3 mark `declaration`/`doctype`/`root`/`prolog` presence; each present field's own tri-state/
/// recursive payload follows in that fixed order.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags: u8 = 0;
    if self.declaration.is_some() {
        flags |= 0b001;
    }
    if self.doctype.is_some() {
        flags |= 0b010;
    }
    if self.root.is_some() {
        flags |= 0b100;
    }
    if self.prolog.is_some() {
        flags |= 0b1000;
    }
    if self.epilog.is_some() {
        flags |= 0b1_0000;
    }
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(prolog) = &self.prolog {
        enc_prolog_bin(prolog, &mut out);
    }
    if let Some(epilog) = &self.epilog {
        enc_prolog_bin(epilog, &mut out);
    }
    if let Some(declaration) = &self.declaration {
        out.push(if declaration.is_some() { 1 } else { 0 });
        if let Some(declaration) = declaration {
            enc_declaration_bin(declaration, &mut out);
        }
    }
    if let Some(doctype) = &self.doctype {
        out.push(if doctype.is_some() { 1 } else { 0 });
        if let Some(doctype) = doctype {
            enc_doctype_bin(doctype, &mut out);
        }
    }
    if let Some(root) = &self.root {
        enc_node_diff_bin(root, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
    let prolog = if flags & 0b1000 != 0 { Some(dec_prolog_bin(&mut reader).map_err(|e| malformed("diff prolog", reader.position(), e))?) } else { None };
    let epilog = if flags & 0b1_0000 != 0 { Some(dec_prolog_bin(&mut reader).map_err(|e| malformed("diff epilog", reader.position(), e))?) } else { None };
    let declaration = if flags & 0b001 != 0 {
        let has = reader.read_u8().map_err(|e| malformed("diff declaration presence", reader.position(), e.to_string()))?;
        Some(if has != 0 { Some(dec_declaration_bin(&mut reader).map_err(|e| malformed("diff declaration", reader.position(), e))?) } else { None })
    } else {
        None
    };
    let doctype = if flags & 0b010 != 0 {
        let has = reader.read_u8().map_err(|e| malformed("diff doctype presence", reader.position(), e.to_string()))?;
        Some(if has != 0 { Some(dec_doctype_bin(&mut reader).map_err(|e| malformed("diff doctype", reader.position(), e))?) } else { None })
    } else {
        None
    };
    let root = if flags & 0b100 != 0 { Some(dec_node_diff_bin(&mut reader).map_err(|e| malformed("diff root", reader.position(), e))?) } else { None };
    Ok(SvgDiff { prolog, epilog, declaration, doctype, root })
}
}

}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v1_1::subsets::base::io::binary::snapshot::document_parts::{write_str_lp, read_str_lp, enc_prolog_bin, dec_prolog_bin, enc_doctype_bin, dec_doctype_bin, enc_declaration_bin, dec_declaration_bin, dec_svg_node_bin, enc_svg_node_bin};
use crate::standards::v1_1::subsets::base::schema::diff::*;
use crate::standards::v1_1::subsets::base::io::text::snapshot::document_parts::enc_svg_node;

use crate::SvgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlQuote};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::SvgAttr;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration;
use crate::schema::snapshot::SvgNode;

/// 🧪️ P2-FG3: real recursive binary twins of [`enc_svg_node`]/[`dec_svg_node`] and
/// [`enc_declaration`]/[`dec_declaration`] above -- a 1-byte kind tag (`0`=Element/`1`=Text/
/// `2`=CData/`3`=Comment/`4`=ProcessingInstruction, distinct numbering from the text codec's letter
/// tags) followed by the real payload (length-prefixed strings for scalars, a varint COUNT then
/// that many recursively-encoded elements for `Element`'s attrs/children -- genuinely recursive,
/// not text-as-bytes). Backs the upgraded `DiffCodec` frame below and, via `../🧬️mutations/
/// 🦀️.rs`'s own `pub(crate)` re-export, the upgraded `OpBinary` frame (same intra-artifact
/// reuse convention `📰️xml`'s own sibling module already establishes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr_bin(a: &SvgAttr, out: &mut Vec<u8>) {
    write_str_lp(out, &a.name);
    write_str_lp(out, &crate::standards::v1_1::subsets::base::io::text::snapshot::print_svg_attribute(&a.value));
}
}

