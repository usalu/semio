//! 🧱️ binary rep for stdio.xml 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1_0::subsets::base::schema::snapshot::*;
use crate::STDIO_XML_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🧩️ Typed XML components for enclosing owned native documents.

/// 🧪️ P2-FG1: `stdio.xml` is TEXT-NATIVE (per the W0 census row) — there is no "binary XML"; the
/// pack container is the SEMIO envelope wrapping the artifact's own REAL wire text
/// (`xml_document_to_text`/`xml_document_from_text`) verbatim, same treatment json's own
/// `ArtifactPack` gives its RFC8259 text (`🔣️json/…/📸️snapshot/🦀️.rs`'s
/// `write_json_text(&self.value).into_bytes()`). Replaces the previous `serde_json::to_vec`/
/// `from_slice` placeholder, which satisfied the trait but was a literal-JSON-payload-disguised-as-
/// binary violation of `POLICY_STDIO_JSON_TRANSFER_BAN` (flagged by name in the P2-W0 recon report,
/// `xml` row, "Yes — in scope").
impl pack::snapshot::ArtifactPack for XmlSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let mut raw = vec![1];
        crate::standards::v1_0::subsets::base::io::binary::snapshot::encode_snapshot_binary(self, &mut raw);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        let mut reader = store::ByteReader::new(&inner);
        let version = reader.read_u8().map_err(|e| store::PackError::from(e))?;
        if version != 1 {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("unsupported xml snapshot state version {version}"))));
        }
        crate::standards::v1_0::subsets::base::io::binary::snapshot::decode_snapshot_binary(&mut reader).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for XmlSnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
}

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::standards::v1_0::subsets::base::schema::mutation_support::*;
use crate::schema::snapshot::{XmlDocument, XmlNode};
use crate::XmlSnapshot;

use super::node_wire_codec::{enc_xml_node_bin, write_str_lp, enc_declaration_bin, enc_doctype_bin, enc_prolog_bin, dec_xml_node_bin, read_str_lp, dec_declaration_bin, dec_doctype_bin, dec_prolog_bin};

pub fn encode_snapshot_binary(snapshot: &XmlSnapshot, output: &mut Vec<u8>) {
    write_str_lp(output, &snapshot.schema);
    output.push(u8::from(snapshot.doc.root.is_some()));
    if let Some(root) = &snapshot.doc.root {
        enc_xml_node_bin(root, output);
    }
    output.push(u8::from(snapshot.doc.doctype.is_some()));
    if let Some(doctype) = &snapshot.doc.doctype {
        enc_doctype_bin(doctype, output);
    }
    output.push(u8::from(snapshot.doc.declaration.is_some()));
    if let Some(declaration) = &snapshot.doc.declaration {
        enc_declaration_bin(declaration, output);
    }
    enc_prolog_bin(&snapshot.doc.prolog, output);
    enc_prolog_bin(&snapshot.doc.epilog, output);
}

pub fn decode_snapshot_binary(reader: &mut store::ByteReader<'_>) -> Result<XmlSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let root = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
    let doctype = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_doctype_bin(reader)?) } else { None };
    let declaration = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_declaration_bin(reader)?) } else { None };
    let prolog = dec_prolog_bin(reader)?;
    let epilog = dec_prolog_bin(reader)?;
    let snapshot = XmlSnapshot { schema, doc: XmlDocument { root, doctype, declaration, prolog, epilog } };
    Ok(snapshot)
}
}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod node_wire_codec {
use crate::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_prolog_bin(prolog: &Vec<XmlNode>, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, prolog.len() as u64);
    for node in prolog {
        enc_xml_node_bin(node, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_prolog_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<XmlNode>, String> {
    let count = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    (0..count).map(|_| dec_xml_node_bin(reader)).collect()
}

/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8) backing the
/// upgraded `OpBinary`/`DiffCodec` frames below (and, via re-export, `../🧬️mutations/🦀️.rs`'s
/// own upgraded `OpBinary`) -- reuses `store::pack_rt::write_varint_u64`/`store::ByteReader` rather
/// than reinventing varint encode/decode, same shape json's own `write_str_lp`/`read_str_lp` uses.
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
pub fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_doctype_bin(doctype: &XmlDoctype, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, doctype.prolog_position);
    write_str_lp(out, &doctype.name);
    match &doctype.external_id {
        None => out.push(0),
        Some(XmlExternalId::System { system_id }) => {
            out.push(1);
            write_str_lp(out, system_id);
        }
        Some(XmlExternalId::Public { public_id, system_id }) => {
            out.push(2);
            write_str_lp(out, public_id);
            write_str_lp(out, system_id);
        }
    }
    store::pack_rt::write_varint_u64(out, doctype.declarations.len() as u64);
    for declaration in &doctype.declarations {
        match declaration {
            XmlDtdDeclaration::Entity { parameter, name, value } => {
                out.push(1);
                out.push(u8::from(*parameter));
                write_str_lp(out, name);
                write_str_lp(out, value);
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_doctype_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlDoctype, String> {
    let prolog_position = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let name = read_str_lp(reader)?;
    let external_id = match reader.read_u8().map_err(|error| error.to_string())? {
        0 => None,
        1 => Some(XmlExternalId::System { system_id: read_str_lp(reader)? }),
        2 => Some(XmlExternalId::Public { public_id: read_str_lp(reader)?, system_id: read_str_lp(reader)? }),
        tag => return Err(format!("unknown XML external id tag {tag}")),
    };
    let count = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    let mut declarations = Vec::with_capacity(count);
    for _ in 0..count {
        match reader.read_u8().map_err(|error| error.to_string())? {
            1 => declarations.push(XmlDtdDeclaration::Entity { parameter: reader.read_u8().map_err(|error| error.to_string())? != 0, name: read_str_lp(reader)?, value: read_str_lp(reader)? }),
            tag => return Err(format!("unknown XML DTD declaration tag {tag}")),
        }
    }
    Ok(XmlDoctype { prolog_position, name, external_id, declarations })
}

/// 🧪️ P2-FG1: real recursive binary twins of [`enc_xml_node`]/[`dec_xml_node`] and
/// [`enc_declaration`]/[`dec_declaration`] above -- a 1-byte kind tag (`0`=Element/`1`=Text/
/// `2`=CData/`3`=Comment/`4`=ProcessingInstruction, distinct numbering from the text codec's letter
/// tags) followed by the real payload (length-prefixed strings for scalars, a varint COUNT then
/// that many recursively-encoded elements for `Element`'s attrs/children -- genuinely recursive,
/// not text-as-bytes). Backs the upgraded `OpBinary` frame (`../🧬️mutations/🦀️.rs`) and
/// the `Replace`/added-item payloads inside [`enc_node_diff_bin`] below. `pub(crate)` so the
/// sibling `../🧬️mutations/🦀️.rs` (same artifact, different facet module) can reuse these
/// rather than duplicating them a second time, matching this file's own existing text-codec reuse
/// convention.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr_bin(a: &XmlAttr, out: &mut Vec<u8>) {
    write_str_lp(out, &a.name);
    write_str_lp(out, &a.value);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attr_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlAttr, String> {
    let name = read_str_lp(reader)?;
    let value = read_str_lp(reader)?;
    Ok(XmlAttr { name, value })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_declaration_bin(d: &XmlDeclaration, out: &mut Vec<u8>) {
    write_str_lp(out, &d.version);
    out.push(if d.encoding.is_some() { 1 } else { 0 });
    if let Some(encoding) = &d.encoding {
        write_str_lp(out, encoding);
    }
    out.push(if d.standalone.is_some() { 1 } else { 0 });
    if let Some(standalone) = d.standalone {
        out.push(if standalone { 1 } else { 0 });
    }
    out.push(u8::from(!d.quote.is_double()));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_declaration_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlDeclaration, String> {
    let version = read_str_lp(reader)?;
    let encoding = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let standalone = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_u8().map_err(|e| e.to_string())? != 0) } else { None };
    let quote = if reader.read_u8().map_err(|e| e.to_string())? != 0 { XmlQuote::Single } else { XmlQuote::Double };
    Ok(XmlDeclaration { version, encoding, standalone, quote })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_xml_node_bin(node: &XmlNode, out: &mut Vec<u8>) {
    let mut stack = vec![node];
    while let Some(node) = stack.pop() {
        match node {
            XmlNode::Element { name, attrs, children } => {
                out.push(0); write_str_lp(out, name); store::pack_rt::write_varint_u64(out, attrs.len() as u64);
                for attr in attrs { enc_attr_bin(attr, out); }
                store::pack_rt::write_varint_u64(out, children.len() as u64); stack.extend(children.iter().rev());
            },
            XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => { out.push(match node { XmlNode::Text { .. } => 1, XmlNode::CData { .. } => 2, _ => 3 }); write_str_lp(out, text); },
            XmlNode::ProcessingInstruction { target, data } => { out.push(4); write_str_lp(out, target); write_str_lp(out, data); },
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_xml_node_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlNode, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = read_str_lp(reader)?;
            let attr_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut attrs = Vec::with_capacity(attr_count as usize);
            for _ in 0..attr_count {
                attrs.push(dec_attr_bin(reader)?);
            }
            let child_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut children = Vec::with_capacity(child_count as usize);
            for _ in 0..child_count {
                children.push(dec_xml_node_bin(reader)?);
            }
            Ok(XmlNode::Element { name, attrs, children })
        }
        1 => Ok(XmlNode::Text { text: read_str_lp(reader)? }),
        2 => Ok(XmlNode::CData { text: read_str_lp(reader)? }),
        3 => Ok(XmlNode::Comment { text: read_str_lp(reader)? }),
        4 => {
            let target = read_str_lp(reader)?;
            let data = read_str_lp(reader)?;
            Ok(XmlNode::ProcessingInstruction { target, data })
        }
        other => Err(format!("xml node binary: unknown tag {other}")),
    }
}
}
pub use node_wire_codec::*;
