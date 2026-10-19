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
impl pack::snapshot::ArtifactDsl for XmlSnapshot {
    const EXTENSION: &'static str = "xml";
    fn envelope_id() -> &'static str {
        "stdio.xml"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        match store::semio_format::split_text_preamble(text) {
            Ok((_, body)) => crate::standards::v1_0::subsets::base::io::text::snapshot::decode_snapshot(body.trim()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("xml state parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
            Err(_) => Self::import_utf8(text.as_bytes()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("xml parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
        }
    }
    fn print_dsl(&self) -> String {
        let body = crate::standards::v1_0::subsets::base::io::text::snapshot::encode_snapshot(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::v1_0::subsets::base::schema::snapshot::*;
use crate::STDIO_XML_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🏳️ Parses the leading `<?xml version="1.0" encoding="..." standalone="..."?>` declaration, if
/// present. Per the XML 1.0 spec the declaration (when present at all) MUST be the very first
/// thing in the document -- unlike ordinary processing instructions it is not represented as an
/// `XmlNode::ProcessingInstruction` and is only ever looked for here, at the very start.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_xml_declaration_prolog(s: &str, pos: &mut usize) -> Result<Option<XmlDeclaration>, String> {
    if !s[*pos..].starts_with("<?xml") {
        return Ok(None);
    }
    let after = s[*pos + "<?xml".len()..].chars().next();
    match after {
        Some(c) if c.is_ascii_whitespace() || c == '?' => {}
        _ => return Ok(None),
    }
    *pos += "<?xml".len();
    let mut version = None;
    let mut encoding = None;
    let mut standalone = None;
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

use super::node_text_codec::{enc_str, dec_str, enc_prolog, dec_prolog, split_top_level, strip_brackets, encode_option, decode_option, enc_declaration, dec_declaration, enc_doctype, dec_doctype, enc_xml_node, dec_xml_node};

pub fn encode_snapshot(snapshot: &XmlSnapshot) -> String {
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

pub fn decode_snapshot(value: &str) -> Result<XmlSnapshot, String> {
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

#[allow(unused_imports)]
mod node_text_codec {
use crate::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_prolog(prolog: &[XmlNode]) -> String {
    format!("[{}]", prolog.iter().map(enc_xml_node).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_prolog(s: &str) -> Result<Vec<XmlNode>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().map(dec_xml_node).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr(a: &XmlAttr) -> String {
    format!("[{},{}]", enc_str(&a.name), enc_str(&a.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attr(s: &str) -> Result<XmlAttr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, value] = parts.as_slice() else { return Err(format!("attr: expected 2 fields, got {}", parts.len())) };
    Ok(XmlAttr { name: dec_str(name)?, value: dec_str(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_declaration(d: &XmlDeclaration) -> String {
    format!("[{},{},{},{}]", enc_str(&d.version), encode_option(&d.encoding, |v| enc_str(v)), encode_option(&d.standalone, |v| if *v { "1".to_string() } else { "0".to_string() }), enc_quote(d.quote),)
}

/// 🗣️ `XmlQuote` as the 1/0 flag the declaration frame carries (`1` = the `'` spelling), the same
/// shape `standalone` already uses one slot earlier.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_quote(quote: XmlQuote) -> String {
    if quote.is_double() {
        "0".to_string()
    } else {
        "1".to_string()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_quote(s: &str) -> Result<XmlQuote, String> {
    match s {
        "0" => Ok(XmlQuote::Double),
        "1" => Ok(XmlQuote::Single),
        other => Err(format!("declaration quote: expected 0 or 1, got {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_declaration(s: &str) -> Result<XmlDeclaration, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [version, encoding, standalone, quote] = parts.as_slice() else { return Err(format!("declaration: expected 4 fields, got {}", parts.len())) };
    Ok(XmlDeclaration { version: dec_str(version)?, encoding: decode_option(encoding, dec_str)?, standalone: decode_option(standalone, |v| Ok(v == "1"))?, quote: dec_quote(quote)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_doctype(doctype: &XmlDoctype) -> String {
    let external = encode_option(&doctype.external_id, |external| match external {
        XmlExternalId::System { system_id } => format!("S[{}]", enc_str(system_id)),
        XmlExternalId::Public { public_id, system_id } => {
            format!("P[{},{}]", enc_str(public_id), enc_str(system_id))
        }
    });
    let declarations = doctype
        .declarations
        .iter()
        .map(|declaration| match declaration {
            XmlDtdDeclaration::Entity { parameter, name, value } => format!("E[{},{},{}]", if *parameter { "1" } else { "0" }, enc_str(name), enc_str(value)),
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{},{},{},[{}]]", doctype.prolog_position, enc_str(&doctype.name), external, declarations)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_doctype(s: &str) -> Result<XmlDoctype, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [prolog_position, name, external, declarations] = parts.as_slice() else {
        return Err(format!("doctype: expected 4 fields, got {}", parts.len()));
    };
    let external_id = decode_option(external, |value| {
        let (tag, rest) = value.split_at(1);
        let fields = split_top_level(strip_brackets(rest)?, ',');
        match (tag, fields.as_slice()) {
            ("S", [system_id]) => Ok(XmlExternalId::System { system_id: dec_str(system_id)? }),
            ("P", [public_id, system_id]) => Ok(XmlExternalId::Public { public_id: dec_str(public_id)?, system_id: dec_str(system_id)? }),
            _ => Err(format!("doctype external id: bad shape {value:?}")),
        }
    })?;
    let declarations = split_top_level(strip_brackets(declarations)?, ',')
        .into_iter()
        .filter(|value| !value.is_empty())
        .map(|value| {
            let (tag, rest) = value.split_at(1);
            let fields = split_top_level(strip_brackets(rest)?, ',');
            match (tag, fields.as_slice()) {
                ("E", [parameter, name, value]) => Ok(XmlDtdDeclaration::Entity { parameter: *parameter == "1", name: dec_str(name)?, value: dec_str(value)? }),
                _ => Err(format!("doctype declaration: bad shape {value:?}")),
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(XmlDoctype { prolog_position: prolog_position.parse::<u64>().map_err(|error|error.to_string())?, name: dec_str(name)?, external_id, declarations })
}

/// 🌳 Recursive: `E[name,[attrs],[children]]` / `T[text]` / `D[text]` (CData) / `M[text]` (comment)
/// / `P[target,data]` (processing instruction) — single-letter tag prefix, no ambiguity with the
/// hex payload since hex never starts with an uppercase letter.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_xml_node(node: &XmlNode) -> String {
    enum Action<'a> { Node(&'a XmlNode), Literal(&'static str) }
    let mut out = String::new(); let mut stack = vec![Action::Node(node)];
    while let Some(action) = stack.pop() {
        match action {
            Action::Literal(value) => out.push_str(value),
            Action::Node(node) => match node {
                XmlNode::Element { name, attrs, children } => {
                    out.push_str("E["); out.push_str(&enc_str(name)); out.push_str(",[");
                    for (index, attr) in attrs.iter().enumerate() { if index > 0 { out.push(','); } out.push_str(&enc_attr(attr)); }
                    out.push_str("],["); stack.push(Action::Literal("]]"));
                    for (index, child) in children.iter().enumerate().rev() { stack.push(Action::Node(child)); if index > 0 { stack.push(Action::Literal(",")); } }
                },
                XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => {
                    out.push_str(match node { XmlNode::Text { .. } => "T[", XmlNode::CData { .. } => "D[", _ => "M[" }); out.push_str(&enc_str(text)); out.push(']');
                },
                XmlNode::ProcessingInstruction { target, data } => { out.push_str("P["); out.push_str(&enc_str(target)); out.push(','); out.push_str(&enc_str(data)); out.push(']'); },
            },
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_xml_node(s: &str) -> Result<XmlNode, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "E" => {
            let parts = split_top_level(inner, ',');
            let [name, attrs, children] = parts.as_slice() else { return Err(format!("element: expected 3 fields, got {}", parts.len())) };
            let attrs = split_top_level(strip_brackets(attrs)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_attr).collect::<Result<Vec<_>, String>>()?;
            let children = split_top_level(strip_brackets(children)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_xml_node).collect::<Result<Vec<_>, String>>()?;
            Ok(XmlNode::Element { name: dec_str(name)?, attrs, children })
        }
        "T" => Ok(XmlNode::Text { text: dec_str(inner)? }),
        "D" => Ok(XmlNode::CData { text: dec_str(inner)? }),
        "M" => Ok(XmlNode::Comment { text: dec_str(inner)? }),
        "P" => {
            let parts = split_top_level(inner, ',');
            let [target, data] = parts.as_slice() else { return Err(format!("PI: expected 2 fields, got {}", parts.len())) };
            Ok(XmlNode::ProcessingInstruction { target: dec_str(target)?, data: dec_str(data)? })
        }
        other => Err(format!("xml node: unknown tag {other:?}")),
    }
}
}
pub use node_text_codec::*;

#[allow(unused_imports)]
mod native_xml_codec {
use crate::schema::snapshot::*;
use super::*;
use crate::STDIO_XML_DOCUMENT_SCHEMA;
impl XmlSnapshot {
    /// 📥️ Parses XML into its lossless logical model.
    pub fn import_utf8(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: xml_document_from_text(text)? })
    }

    /// 📤️ Deterministically materializes XML from the logical model.
    pub fn export_utf8(&self) -> Result<Vec<u8>, String> {
        Ok(xml_document_to_text_checked(&self.doc)?.into_bytes())
    }
}
impl From<&str> for XmlDoctype {
    fn from(value: &str) -> Self {
        crate::standards::v1_0::subsets::base::io::text::snapshot::parse_doctype(value).expect("valid XML document type literal")
    }
}
//#region 🔖️XmlTextCodec
/// 🔤 Escapes character data for text-node content. Per XML 1.0 §2.11, only the two-character
/// sequence `#xD #xA` and any lone `#xD` are normalized (to `#xA`) on the NEXT parse -- a literal
/// tab or `\n` is legal, untouched, and round-trips as-is, so only `\r` needs re-escaping here
/// (as `&#13;`) to survive; escaping `\n` too would be a needless (though harmless) divergence
/// from what the spec actually requires. This is deliberately narrower than [`xml_escape_attr`] --
/// see that function's doc for why attribute values need a wider set of characters escaped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xml_escape_text(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '\r' => out.push_str("&#13;"),
            _ => out.push(ch),
        }
    }
    out
}

/// 🔤 Escapes character data for a double-quoted attribute value. Per XML 1.0 §3.3.3, attribute
/// value normalization replaces every literal tab/`\n`/`\r` with a single space on the NEXT parse
/// (after line-break normalization already folds `\r`/`\r\n` to `\n`) -- but a character reference
/// like `&#9;`/`&#10;`/`&#13;` is exempt from that step and survives verbatim. So a value decoded
/// from such a reference (real example: the folded base64 `xlink:href` in the committed
/// `qr-code.svg` fixture, which carries dozens of `&#10;`) MUST be re-escaped as a reference on
/// write, or the byte written is a literal newline that silently collapses to a space next parse,
/// changing the value's meaning. This is deliberately wider than [`xml_escape_text`], whose text
/// content has no such normalization step for `\t`/`\n`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xml_escape_attr(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '"' => out.push_str("&quot;"),
            '\t' => out.push_str("&#9;"),
            '\n' => out.push_str("&#10;"),
            '\r' => out.push_str("&#13;"),
            _ => out.push(ch),
        }
    }
    out
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xml_document_to_text(doc: &XmlDocument) -> String {
    xml_document_to_text_checked(doc).expect("valid XML document boundaries")
}

/// 📤 Validates and deterministically materializes one logical XML document.
pub fn xml_document_to_text_checked(doc: &XmlDocument) -> Result<String, String> {
    validate_xml_document_boundaries(doc)?;
    let mut out = String::new();
    if let Some(decl) = &doc.declaration {
        let quote = decl.quote.as_char();
        out.push_str("<?xml version=");
        out.push(quote);
        out.push_str(&decl.version);
        out.push(quote);
        if let Some(encoding) = &decl.encoding {
            out.push_str(" encoding=");
            out.push(quote);
            out.push_str(encoding);
            out.push(quote);
        }
        if let Some(standalone) = decl.standalone {
            out.push_str(" standalone=");
            out.push(quote);
            out.push_str(if standalone { "yes" } else { "no" });
            out.push(quote);
        }
        out.push_str("?>\n");
    }
    for (index, node) in doc.prolog.iter().enumerate() {
        if doc.doctype.as_ref().is_some_and(|doctype| doctype.prolog_position == index as u64) {
            xml_doctype_to_text(doc.doctype.as_ref().expect("checked doctype"), &mut out);
            out.push('\n');
        }
        xml_node_to_text(node, 0, &mut out);
        out.push('\n');
    }
    if let Some(doctype) = doc.doctype.as_ref().filter(|doctype| doctype.prolog_position == doc.prolog.len() as u64) {
        xml_doctype_to_text(doctype, &mut out);
        out.push('\n');
    }
    if let Some(node) = &doc.root {
        xml_node_to_text(node, 0, &mut out);
    }
    for node in &doc.epilog {
        if !out.is_empty() {
            out.push('\n');
        }
        xml_node_to_text(node, 0, &mut out);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_doctype_to_text(doctype: &XmlDoctype, out: &mut String) {
    out.push_str("<!DOCTYPE ");
    out.push_str(&doctype.name);
    if let Some(external_id) = &doctype.external_id {
        match external_id {
            XmlExternalId::System { system_id } => {
                out.push_str(" SYSTEM \"");
                out.push_str(&xml_escape_attr(system_id));
                out.push('\"');
            }
            XmlExternalId::Public { public_id, system_id } => {
                out.push_str(" PUBLIC \"");
                out.push_str(&xml_escape_attr(public_id));
                out.push_str("\" \"");
                out.push_str(&xml_escape_attr(system_id));
                out.push('\"');
            }
        }
    }
    if !doctype.declarations.is_empty() {
        out.push_str(" [");
        for declaration in &doctype.declarations {
            match declaration {
                XmlDtdDeclaration::Entity { parameter, name, value } => {
                    out.push_str("<!ENTITY ");
                    if *parameter {
                        out.push_str("% ");
                    }
                    out.push_str(name);
                    out.push_str(" \"");
                    out.push_str(&xml_escape_attr(value));
                    out.push_str("\">");
                }
            }
        }
        out.push(']');
    }
    out.push('>');
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_current_column(out: &str) -> usize {
    out.rsplit_once('\n').map_or(out.len(), |(_, line)| line.len())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_node_to_text(node: &XmlNode, depth: usize, out: &mut String) {
    match node {
        XmlNode::Text { text } => out.push_str(&xml_escape_text(text)),
        XmlNode::CData { text } => {
            out.push_str("<![CDATA[");
            out.push_str(text);
            out.push_str("]]>");
        }
        XmlNode::Comment { text } => {
            out.push_str("<!--");
            out.push_str(text);
            out.push_str("-->");
        }
        XmlNode::ProcessingInstruction { target, data } => {
            out.push_str("<?");
            out.push_str(target);
            if !data.is_empty() {
                out.push(' ');
                out.push_str(data);
            }
            out.push_str("?>");
        }
        XmlNode::Element { name, attrs, children } => {
            out.push('<');
            out.push_str(name);
            for (index, attr) in attrs.iter().enumerate() {
                let rendered = format!("{}=\"{}\"", attr.name, xml_escape_attr(&attr.value));
                let closing_width = if index + 1 == attrs.len() {
                    match children.as_slice() {
                        [] => 2,
                        [XmlNode::Text { text }] => xml_escape_text(text).chars().count() + name.len() + 4,
                        _ => 1,
                    }
                } else {
                    0
                };
                if xml_current_column(out) + 1 + rendered.len() + closing_width > 120 {
                    out.push('\n');
                    out.push_str(&" ".repeat((depth + 1) * 4));
                } else {
                    out.push(' ');
                }
                out.push_str(&rendered);
            }
            if children.is_empty() {
                out.push_str("/>");
                return;
            }
            out.push('>');
            for child in children {
                xml_node_to_text(child, depth + 1, out);
            }
            out.push_str("</");
            out.push_str(name);
            out.push('>');
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xml_document_from_text(text: &str) -> Result<XmlDocument, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(XmlDocument::default());
    }
    let mut pos = 0;
    let declaration = crate::standards::v1_0::subsets::base::io::text::snapshot::parse_xml_declaration_prolog(trimmed, &mut pos)?;
    let (doctype, prolog) = crate::standards::v1_0::subsets::base::io::text::snapshot::parse_misc(trimmed, &mut pos, true)?;
    let root = crate::standards::v1_0::subsets::base::io::text::snapshot::parse_node(trimmed, &mut pos)?;
    let (_, epilog) = crate::standards::v1_0::subsets::base::io::text::snapshot::parse_misc(trimmed, &mut pos, false)?;
    if pos < trimmed.len() {
        return Err("trailing content after root element".into());
    }
    let document = XmlDocument { root: Some(root), doctype, declaration, prolog, epilog };
    validate_xml_document_boundaries(&document)?;
    Ok(document)
}






















//#endregion 🔖️XmlTextCodec
}
pub use native_xml_codec::*;
