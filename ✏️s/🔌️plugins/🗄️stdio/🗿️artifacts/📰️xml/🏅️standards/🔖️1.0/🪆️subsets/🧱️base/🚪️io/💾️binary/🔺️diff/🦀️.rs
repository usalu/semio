//! binary rep for stdio.xml 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_0::subsets::base::schema::diff::*;
use crate::schema::snapshot::{validate_xml_document_boundaries, XmlAttr, XmlDeclaration, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote};
use crate::XmlSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::deserialize_double_option;
use crate::standards::v1_0::subsets::base::io::binary::snapshot::{enc_prolog_bin, dec_prolog_bin, write_str_lp, read_str_lp, enc_doctype_bin, dec_doctype_bin, enc_declaration_bin, dec_declaration_bin, enc_xml_node_bin, dec_xml_node_bin};

/// 🧪️ P2-FG1: real recursive binary twin of [`enc_node_diff`]/[`dec_node_diff`] -- same 1-byte tag
/// numbering scheme as [`enc_xml_node_bin`] (`0`=Element/`1`=Text) plus `2`=`Replace` (needs its own
/// arm since `Replace` wraps a whole [`XmlNode`], not a bare scalar payload). `attrs`/`children`
/// collection triples encode as three varint-counted, recursively-encoded lists (removed/modified/
/// added) -- genuinely structured binary, backing the upgraded `DiffBinary::encode_diff`/
/// `decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff_bin(diff: &XmlNodeDiff, out: &mut Vec<u8>) {
    match diff {
        XmlNodeDiff::Element(e) => {
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
        XmlNodeDiff::Text { text } => {
            out.push(1);
            out.push(if text.is_some() { 1 } else { 0 });
            if let Some(text) = text {
                write_str_lp(out, text);
            }
        }
        XmlNodeDiff::Replace { node } => {
            out.push(2);
            out.push(if node.is_some() { 1 } else { 0 });
            if let Some(node) = node {
                enc_xml_node_bin(node, out);
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlNodeDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            let attributes = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_attrs_diff_bin(reader)?) } else { None };
            let children = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_children_diff_bin(reader)?) } else { None };
            Ok(XmlNodeDiff::Element(XmlElementDiff { name, attributes, children }))
        }
        1 => {
            let text = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            Ok(XmlNodeDiff::Text { text })
        }
        2 => {
            let node = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
            Ok(XmlNodeDiff::Replace { node })
        }
        other => Err(format!("xml node diff binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attrs_diff_bin(diff: &XmlAttributesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for name in &diff.removed {
        write_str_lp(out, name);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        write_str_lp(out, &entry.name);
        write_str_lp(out, &entry.value);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        write_str_lp(out, &entry.name);
        write_str_lp(out, &entry.value);
    }
    store::pack_rt::write_varint_u64(out, diff.order.len() as u64);
    for name in &diff.order {
        write_str_lp(out, name);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attrs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlAttributesDiff, String> {
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
        modified.push(XmlAttrModified { name, value });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let name = read_str_lp(reader)?;
        let value = read_str_lp(reader)?;
        added.push(XmlAttrAdded { name, value });
    }
    let order_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut order = Vec::with_capacity(order_count as usize);
    for _ in 0..order_count {
        order.push(read_str_lp(reader)?);
    }
    Ok(XmlAttributesDiff { removed, modified, added, order })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_children_diff_bin(diff: &XmlChildrenDiff, out: &mut Vec<u8>) {
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
        enc_xml_node_bin(&entry.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_children_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlChildrenDiff, String> {
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
        modified.push(XmlChildModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_xml_node_bin(reader)?;
        added.push(XmlChildAdded { index, item });
    }
    Ok(XmlChildrenDiff { removed, modified, added })
}

impl protocol::DiffBinary for XmlDiff {
/// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags u8 | [declaration][doctype][root]`),
/// matching `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload
/// bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100%
/// of stdio's `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` bits
/// 0/1/2 mark `declaration`/`doctype`/`root` presence; each present field's own tri-state/
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
    if let Some(prolog) = &self.prolog {
        enc_prolog_bin(prolog, &mut out);
    }
    if let Some(epilog) = &self.epilog {
        enc_prolog_bin(epilog, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
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
    let prolog = if flags & 0b1000 != 0 { Some(dec_prolog_bin(&mut reader).map_err(|e| malformed("diff prolog", reader.position(), e))?) } else { None };
    let epilog = if flags & 0b1_0000 != 0 { Some(dec_prolog_bin(&mut reader).map_err(|e| malformed("diff epilog", reader.position(), e))?) } else { None };
    Ok(XmlDiff { prolog, epilog, declaration, doctype, root })
}
}
}
pub use diff_codec::*;

impl crate::schema::diff::XmlChildrenDiff {
    pub fn encode_binary(&self, output: &mut Vec<u8>) {
        enc_children_diff_bin(self, output);
    }
}

impl crate::schema::diff::XmlChildrenDiff {
    pub fn decode_binary(reader: &mut store::ByteReader<'_>) -> Result<Self, String> {
        dec_children_diff_bin(reader)
    }
}
