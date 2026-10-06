//! 📝️ Text representation codec surface for `stdio.xml` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1_0::subsets::base::schema::snapshot::*;
use crate::STDIO_XML_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🧩️ Typed XML components for enclosing owned native documents.
use native_encoding::{XmlNativeEmission,emit_xml_native_document,emit_xml_native_node,emit_xml_native_snapshot_fields};
use native_decoding::{XmlNativeInput,read_xml_native_document,read_xml_native_node,read_xml_native_snapshot_fields};

impl store::ArtifactDsl for XmlSnapshot {
    const EXTENSION: &'static str = "xml";
    fn envelope_id() -> &'static str {
        "stdio.xml"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        match store::semio_format::split_text_preamble(text) {
            Ok((_, body)) => crate::schema::mutation_support::decode_snapshot(body.trim()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("xml state parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
            Err(_) => Self::import_utf8(text.as_bytes()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("xml parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
        }
    }
    fn print_dsl(&self) -> String {
        let body = crate::schema::mutation_support::encode_snapshot(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1_0::subsets::base::schema::snapshot::*;
use crate::STDIO_XML_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🧩️ Typed XML components for enclosing owned native documents.
use native_encoding::{XmlNativeEmission,emit_xml_native_document,emit_xml_native_node,emit_xml_native_snapshot_fields};
use native_decoding::{XmlNativeInput,read_xml_native_document,read_xml_native_node,read_xml_native_snapshot_fields};
























}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::v1_0::subsets::base::schema::snapshot::*;
use crate::STDIO_XML_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🧩️ Typed XML components for enclosing owned native documents.
use native_encoding::{XmlNativeEmission,emit_xml_native_document,emit_xml_native_node,emit_xml_native_snapshot_fields};
use native_decoding::{XmlNativeInput,read_xml_native_document,read_xml_native_node,read_xml_native_snapshot_fields};

/// 🏳️ Parses the leading `<?xml version="1.0" encoding="..." standalone="..."?>` declaration, if
/// present. Per the XML 1.0 spec the declaration (when present at all) MUST be the very first
/// thing in the document -- unlike ordinary processing instructions it is not represented as an
/// `XmlNode::ProcessingInstruction` and is only ever looked for here, at the very start.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_xml_declaration_prolog(s: &str, pos: &mut usize) -> Result<Option<XmlDeclaration>, String> {
    if !s[*pos..].starts_with("<?xml") {
        return Ok(None);
    }
    // Distinguish the reserved `<?xml ...?>` declaration target from an ordinary PI whose target
    // merely starts with the same four letters (e.g. `<?xml-stylesheet ...?>`).
    let after = s[*pos + "<?xml".len()..].chars().next();
    match after {
        Some(c) if c.is_ascii_whitespace() || c == '?' => {}
        _ => return Ok(None),
    }
    *pos += "<?xml".len();
    let mut version = None;
    let mut encoding = None;
    let mut standalone = None;
    // 🗣️ `version` is mandatory and always first, so its delimiter is the declaration's own
    // (see `XmlQuote`); a declaration whose pseudo-attributes disagree normalizes to that one.
    let mut quote = XmlQuote::default();
    loop {
        skip_ws(s, pos);
        if s[*pos..].starts_with("?>") {
            break;
        }
        let name = parse_name(s, pos)?;
        skip_ws(s, pos);
        if !s[*pos..].starts_with('=') {
            return Err("expected = in xml declaration".into());
        }
        *pos += 1;
        let (raw, delimiter) = parse_attr_value_quoted(s, pos)?;
        let value = xml_unescape_text(&raw)?;
        match name.as_str() {
            "version" => {
                version = Some(value);
                quote = delimiter;
            }
            "encoding" => encoding = Some(value),
            "standalone" if value == "yes" => standalone = Some(true),
            "standalone" if value == "no" => standalone = Some(false),
            "standalone" => return Err("xml declaration standalone must be yes or no".into()),
            other => return Err(format!("unknown xml declaration attribute {other}")),
        }
    }
    *pos += 2;
    Ok(Some(XmlDeclaration { version: version.ok_or("xml declaration missing version")?, encoding, standalone, quote }))
}

/// 🚧️ Parses prolog processing instructions, comments, and a typed document declaration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_misc(s: &str, pos: &mut usize, allow_doctype: bool) -> Result<(Option<XmlDoctype>, Vec<XmlNode>), String> {
    let mut doctype = None;
    let mut nodes = Vec::new();
    loop {
        skip_ws(s, pos);
        if s[*pos..].starts_with("<?") {
            *pos += 2;
            let target = parse_name(s, pos)?;
            skip_ws(s, pos);
            let end = s[*pos..].find("?>").ok_or("unclosed processing instruction")?;
            let data = s[*pos..*pos + end].to_string();
            *pos += end + 2;
            nodes.push(XmlNode::ProcessingInstruction { target, data });
            continue;
        }
        if s[*pos..].starts_with("<!--") {
            *pos += 4;
            let end = s[*pos..].find("-->").ok_or("unclosed comment")?;
            nodes.push(XmlNode::Comment { text: s[*pos..*pos + end].to_string() });
            *pos += end + 3;
            continue;
        }
        if s[*pos..].starts_with("<!DOCTYPE") || s[*pos..].starts_with("<!doctype") {
            if !allow_doctype {
                return Err("DOCTYPE declaration cannot appear after root element".into());
            }
            if doctype.is_some() {
                return Err("duplicate DOCTYPE declaration".into());
            }
            let start = *pos;
            *pos += "<!DOCTYPE".len();
            let mut depth = 0i32;
            let mut quote = None;
            loop {
                if *pos >= s.len() {
                    return Err("unclosed DOCTYPE declaration".into());
                }
                let byte = s.as_bytes()[*pos];
                if let Some(delimiter) = quote {
                    if byte == delimiter {
                        quote = None;
                    }
                    *pos += 1;
                    continue;
                }
                match byte {
                    b'\'' | b'"' => quote = Some(byte),
                    b'[' => depth += 1,
                    b']' if depth > 0 => depth -= 1,
                    b'>' if depth <= 0 => {
                        *pos += 1;
                        break;
                    }
                    _ => {}
                }
                *pos += 1;
            }
            let mut parsed = parse_doctype(&s[start..*pos])?;
            parsed.prolog_position = nodes.len() as u64;
            doctype = Some(parsed);
            continue;
        }
        break;
    }
    Ok((doctype, nodes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_doctype(text: &str) -> Result<XmlDoctype, String> {
    let mut pos = "<!DOCTYPE".len();
    skip_ws(text, &mut pos);
    let name = parse_name(text, &mut pos)?;
    skip_ws(text, &mut pos);
    let external_id = if text[pos..].starts_with("SYSTEM") {
        pos += "SYSTEM".len();
        Some(XmlExternalId::System { system_id: xml_unescape_text(&parse_attr_value(text, &mut pos)?)? })
    } else if text[pos..].starts_with("PUBLIC") {
        pos += "PUBLIC".len();
        let public_id = xml_unescape_text(&parse_attr_value(text, &mut pos)?)?;
        let system_id = xml_unescape_text(&parse_attr_value(text, &mut pos)?)?;
        Some(XmlExternalId::Public { public_id, system_id })
    } else {
        None
    };
    skip_ws(text, &mut pos);
    let mut declarations = Vec::new();
    if text[pos..].starts_with('[') {
        pos += 1;
        loop {
            skip_ws(text, &mut pos);
            if text[pos..].starts_with(']') {
                pos += 1;
                break;
            }
            if !text[pos..].starts_with("<!ENTITY") {
                return Err("unsupported XML DTD declaration; only typed ENTITY declarations are modeled".into());
            }
            pos += "<!ENTITY".len();
            skip_ws(text, &mut pos);
            let parameter = text[pos..].starts_with('%');
            if parameter {
                pos += 1;
                skip_ws(text, &mut pos);
            }
            let entity_name = parse_name(text, &mut pos)?;
            let value = xml_unescape_text(&parse_attr_value(text, &mut pos)?)?;
            skip_ws(text, &mut pos);
            if !text[pos..].starts_with('>') {
                return Err("expected > after XML entity declaration".into());
            }
            pos += 1;
            declarations.push(XmlDtdDeclaration::Entity { parameter, name: entity_name, value });
        }
    }
    skip_ws(text, &mut pos);
    if !text[pos..].starts_with('>') {
        return Err("expected > after XML document type declaration".into());
    }
    pos += 1;
    if pos != text.len() {
        return Err("trailing content in XML document type declaration".into());
    }
    Ok(XmlDoctype { prolog_position: 0, name, external_id, declarations })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_node(s: &str, pos: &mut usize) -> Result<XmlNode, String> {
    if *pos >= s.len() || !s[*pos..].starts_with('<') {
        return Err("expected element start".into());
    }
    if s[*pos..].starts_with("</") {
        return Err("unexpected closing tag".into());
    }
    *pos += 1;
    let name = parse_name(s, pos)?;
    let attrs = parse_attrs(s, pos)?;
    skip_ws(s, pos);
    if s[*pos..].starts_with("/>") {
        *pos += 2;
        return Ok(XmlNode::Element { name, attrs, children: vec![] });
    }
    if !s[*pos..].starts_with('>') {
        return Err("expected > or />".into());
    }
    *pos += 1;
    let mut children = Vec::new();
    loop {
        if *pos >= s.len() {
            return Err(format!("unclosed element <{name}>: unexpected end of input"));
        }
        if s[*pos..].starts_with("</") {
            *pos += 2;
            let close = parse_name(s, pos)?;
            skip_ws(s, pos);
            if !s[*pos..].starts_with('>') {
                return Err("expected > on closing tag".into());
            }
            *pos += 1;
            if close != name {
                return Err(format!("closing tag mismatch: expected </{}>, got </{}>", name, close));
            }
            break;
        }
        if s[*pos..].starts_with("<![CDATA[") {
            *pos += "<![CDATA[".len();
            let end = s[*pos..].find("]]>").ok_or("unclosed CDATA section")?;
            let text = s[*pos..*pos + end].to_string();
            *pos += end + 3;
            children.push(XmlNode::CData { text });
            continue;
        }
        if s[*pos..].starts_with("<!--") {
            *pos += 4;
            let end = s[*pos..].find("-->").ok_or("unclosed comment")?;
            let text = s[*pos..*pos + end].to_string();
            *pos += end + 3;
            children.push(XmlNode::Comment { text });
            continue;
        }
        if s[*pos..].starts_with("<?") {
            *pos += 2;
            let target = parse_name(s, pos)?;
            skip_ws(s, pos);
            let end = s[*pos..].find("?>").ok_or("unclosed processing instruction")?;
            let data = s[*pos..*pos + end].to_string();
            *pos += end + 2;
            children.push(XmlNode::ProcessingInstruction { target, data });
            continue;
        }
        if s[*pos..].starts_with('<') {
            children.push(parse_node(s, pos)?);
            continue;
        }
        let start = *pos;
        while *pos < s.len() && !s[*pos..].starts_with('<') {
            *pos += s[*pos..].chars().next().unwrap().len_utf8();
        }
        let raw = &s[start..*pos];
        if !raw.is_empty() {
            let text = xml_unescape_text(raw)?;
            children.push(XmlNode::Text { text });
        }
    }
    Ok(XmlNode::Element { name, attrs, children })
}

/// 🔓️ Decode the five predefined XML entities plus numeric character references (`&#NNN;`,
/// `&#xHHHH;`/`&#XHHHH;`) into their literal characters. This is the read-side half that was
/// entirely missing before: without it, `&amp;` in a source document is kept as the 5 literal
/// characters `&`,`a`,`m`,`p`,`;` in the `Text` node, and the NEXT write-side escape turns that
/// lone `&` into `&amp;` again -- so every decode->encode cycle grows `&amp;` into `&amp;amp;`
/// into `&amp;amp;amp;`, permanently corrupting the document. An unrecognized/malformed entity is
/// a hard parse error (never silently dropped or passed through raw).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xml_unescape_text(s: &str) -> Result<String, String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.char_indices().peekable();
    while let Some((i, ch)) = chars.next() {
        if ch != '&' {
            out.push(ch);
            continue;
        }
        let rest = &s[i..];
        let end = rest.find(';').ok_or_else(|| format!("unterminated entity reference at byte {i}"))?;
        let entity = &rest[1..end];
        let decoded = if let Some(numeric) = entity.strip_prefix('#') {
            let code = if let Some(hex) = numeric.strip_prefix('x').or_else(|| numeric.strip_prefix('X')) {
                u32::from_str_radix(hex, 16).map_err(|_| format!("invalid hex character reference &{entity};"))?
            } else {
                numeric.parse::<u32>().map_err(|_| format!("invalid decimal character reference &{entity};"))?
            };
            char::from_u32(code).ok_or_else(|| format!("invalid unicode scalar &{entity};"))?
        } else {
            match entity {
                "amp" => '&',
                "lt" => '<',
                "gt" => '>',
                "quot" => '"',
                "apos" => '\'',
                other => return Err(format!("unknown entity &{other};")),
            }
        };
        out.push(decoded);
        // Advance the char iterator past the consumed entity (end index is relative to `rest`,
        // i.e. `i`-relative; convert to an absolute byte offset and skip to just past the `;`).
        let consume_to = i + end + 1;
        while let Some(&(j, _)) = chars.peek() {
            if j < consume_to {
                chars.next();
            } else {
                break;
            }
        }
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn skip_ws(s: &str, pos: &mut usize) {
    while *pos < s.len() && s.as_bytes()[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_name(s: &str, pos: &mut usize) -> Result<String, String> {
    let start = *pos;
    if *pos >= s.len() || !is_name_start(s[*pos..].chars().next().unwrap()) {
        return Err("expected XML name".into());
    }
    *pos += 1;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if is_name_char(ch) {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
    Ok(s[start..*pos].to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_attr_value(s: &str, pos: &mut usize) -> Result<String, String> {
    parse_attr_value_quoted(s, pos).map(|(value, _)| value)
}

/// 🗣️ `parse_attr_value` plus the delimiter it consumed — the XML declaration is the one place
/// whose quoting is modeled state ([`XmlQuote`]), so only that caller needs the second half.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_attr_value_quoted(s: &str, pos: &mut usize) -> Result<(String, XmlQuote), String> {
    skip_ws(s, pos);
    let quote = s[*pos..].chars().next().ok_or("expected attribute value")?;
    if quote != '"' && quote != '\'' {
        return Err("attribute value must be quoted".into());
    }
    *pos += 1;
    let start = *pos;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if ch == quote {
            let value = s[start..*pos].to_string();
            *pos += 1;
            return Ok((value, XmlQuote::from_char(quote)));
        }
        *pos += ch.len_utf8();
    }
    Err("unclosed attribute value".into())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_attrs(s: &str, pos: &mut usize) -> Result<Vec<XmlAttr>, String> {
    let mut attrs = Vec::new();
    loop {
        skip_ws(s, pos);
        if *pos >= s.len() || s[*pos..].starts_with(">") || s[*pos..].starts_with("/>") {
            break;
        }
        let name = parse_name(s, pos)?;
        skip_ws(s, pos);
        if !s[*pos..].starts_with('=') {
            return Err("expected = in attribute".into());
        }
        *pos += 1;
        let value = xml_unescape_text(&parse_attr_value(s, pos)?)?;
        attrs.push(XmlAttr { name, value });
    }
    Ok(attrs)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn is_name_start(ch: char) -> bool {
    ch == ':' || ch.is_ascii_alphabetic() || ch == '_'
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn is_name_char(ch: char) -> bool {
    is_name_start(ch) || ch.is_ascii_digit() || ch == '-' || ch == '.'
}
}
pub use snapshot_wire2_codec::*;

#[allow(unused_imports)]
mod snapshot_wire3_codec {
use crate::standards::v1_0::subsets::base::schema::mutation_support::*;
use crate::schema::snapshot::{XmlDocument, XmlNode};
use crate::XmlSnapshot;

pub(crate) fn encode_snapshot(snapshot: &XmlSnapshot) -> String {
    use crate::standards::v1_0::subsets::base::io::text::diff::{enc_xml_node};
    use crate::standards::v1_0::subsets::base::io::text::diff::{enc_doctype};
    use crate::standards::v1_0::subsets::base::io::text::diff::{enc_declaration};
    use crate::standards::v1_0::subsets::base::io::text::diff::{enc_prolog};
    use crate::standards::v1_0::subsets::base::io::text::diff::{encode_option};
    use crate::standards::v1_0::subsets::base::io::text::diff::{enc_str};
    format!(
        "[{},{},{},{},{},{}]",
        enc_str(&snapshot.schema),
        encode_option(&snapshot.doc.root, enc_xml_node),
        encode_option(&snapshot.doc.doctype, enc_doctype),
        encode_option(&snapshot.doc.declaration, enc_declaration),
        enc_prolog(&snapshot.doc.prolog),
        enc_prolog(&snapshot.doc.epilog)
    )
}

pub(crate) fn decode_snapshot(value: &str) -> Result<XmlSnapshot, String> {
    use crate::standards::v1_0::subsets::base::io::text::diff::{dec_xml_node};
    use crate::standards::v1_0::subsets::base::io::text::diff::{dec_doctype};
    use crate::standards::v1_0::subsets::base::io::text::diff::{dec_declaration};
    use crate::standards::v1_0::subsets::base::io::text::diff::{dec_prolog};
    use crate::standards::v1_0::subsets::base::io::text::diff::{decode_option};
    use crate::standards::v1_0::subsets::base::io::text::diff::{strip_brackets};
    use crate::standards::v1_0::subsets::base::io::text::diff::{split_top_level};
    use crate::standards::v1_0::subsets::base::io::text::diff::{dec_str};
    let parts = split_top_level(strip_brackets(value)?, ',');
    let [schema, root, doctype, declaration, prolog, epilog] = parts.as_slice() else {
        return Err(format!("xml snapshot: expected 6 fields, got {}", parts.len()));
    };
    let snapshot = XmlSnapshot {
        schema: dec_str(schema)?,
        doc: XmlDocument { root: decode_option(root, dec_xml_node)?, doctype: decode_option(doctype, dec_doctype)?, declaration: decode_option(declaration, dec_declaration)?, prolog: dec_prolog(prolog)?, epilog: dec_prolog(epilog)? },
    };
    Ok(snapshot)
}
}
pub use snapshot_wire3_codec::*;
