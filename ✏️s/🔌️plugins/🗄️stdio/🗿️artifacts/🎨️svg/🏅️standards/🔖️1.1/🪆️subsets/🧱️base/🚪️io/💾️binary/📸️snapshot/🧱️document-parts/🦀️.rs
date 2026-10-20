//! 🧱️ The binary codecs of the document parts (nodes, doctype, declaration, prolog) that the snapshot and the diff codecs share.

use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlQuote, XmlDeclaration};
use crate::schema::snapshot::{SvgAttr, SvgNode};

/// 🧪️ P2-FG3: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8) backing
/// the upgraded `DiffCodec` frame below (and, via re-export, `../🧬️mutations/🦀️.rs`'s own
/// upgraded `OpBinary`) — reuses `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than
/// reinventing varint encode/decode, same shape `📰️xml`'s own sibling `XmlDiff` codec uses (svg's
/// copies stay `pub(crate)` to this artifact's own crate-visibility scope, not reachable from
/// `📰️xml`, matching that file's own established duplication convention).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
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
pub fn enc_prolog_bin(prolog: &Vec<SvgNode>, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, prolog.len() as u64);
    for node in prolog {
        enc_svg_node_bin(node, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_prolog_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<SvgNode>, String> {
    let count = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    (0..count).map(|_| dec_svg_node_bin(reader)).collect()
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_attr_bin(reader: &mut store::ByteReader<'_>) -> Result<SvgAttr, String> {
    let name = read_str_lp(reader)?;
    let value = read_str_lp(reader)?;
    Ok(SvgAttr { value: crate::standards::v1_1::subsets::base::io::text::snapshot::attributes::bind_svg_attribute(&name, &value)?, name })
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_svg_node_bin(reader: &mut store::ByteReader<'_>) -> Result<SvgNode, String> {
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
                children.push(dec_svg_node_bin(reader)?);
            }
            Ok(SvgNode::Element { name, attrs, children })
        }
        1 => Ok(SvgNode::Text { text: read_str_lp(reader)? }),
        2 => Ok(SvgNode::CData { text: read_str_lp(reader)? }),
        3 => Ok(SvgNode::Comment { text: read_str_lp(reader)? }),
        4 => {
            let target = read_str_lp(reader)?;
            let data = read_str_lp(reader)?;
            Ok(SvgNode::ProcessingInstruction { target, data })
        }
        other => Err(format!("xml node binary: unknown tag {other}")),
    }
}

pub fn enc_svg_node_bin(node:&crate::schema::snapshot::SvgNode,out:&mut Vec<u8>) {semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::binary::snapshot::enc_xml_node_bin(&crate::standards::v1_1::subsets::base::io::text::snapshot::native_svg_node(node),out)}
