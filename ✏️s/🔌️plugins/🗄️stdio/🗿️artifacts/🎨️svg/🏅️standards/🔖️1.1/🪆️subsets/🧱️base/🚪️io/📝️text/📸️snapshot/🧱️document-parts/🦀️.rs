//! 🧱️ The text codecs of the document parts (nodes, doctype, declaration, prolog) that the snapshot and the diff codecs share.

use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlQuote, XmlDeclaration};
use crate::schema::snapshot::{SvgAttr, SvgNode};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
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
pub fn enc_prolog(prolog: &[SvgNode]) -> String {
    format!("[{}]", prolog.iter().map(enc_svg_node).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_prolog(s: &str) -> Result<Vec<SvgNode>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().map(dec_svg_node).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_declaration(d: &XmlDeclaration) -> String {
    format!(
        "[{},{},{},{}]",
        enc_str(&d.version),
        encode_option(&d.encoding, |v| enc_str(v)),
        encode_option(&d.standalone, |v| if *v { "1".to_string() } else { "0".to_string() }),
        enc_quote(d.quote),
    )
}

/// 🗣️ `XmlQuote` as the 1/0 flag the declaration frame carries (`1` = the `'` spelling), the same
/// shape `standalone` already uses one slot earlier — restated here exactly as `📰️xml`'s own
/// sibling codec spells it (svg reuses xml's `XmlDeclaration` type but owns its own wire).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_quote(quote: XmlQuote) -> String {
    if quote.is_double() {
        "0".to_string()
    } else {
        "1".to_string()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_quote(s: &str) -> Result<XmlQuote, String> {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_svg_node(s: &str) -> Result<SvgNode, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "E" => {
            let parts = split_top_level(inner, ',');
            let [name, attrs, children] = parts.as_slice() else { return Err(format!("element: expected 3 fields, got {}", parts.len())) };
            let attrs = split_top_level(strip_brackets(attrs)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_attr).collect::<Result<Vec<_>, String>>()?;
            let children = split_top_level(strip_brackets(children)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_svg_node).collect::<Result<Vec<_>, String>>()?;
            Ok(SvgNode::Element { name: dec_str(name)?, attrs, children })
        }
        "T" => Ok(SvgNode::Text { text: dec_str(inner)? }),
        "D" => Ok(SvgNode::CData { text: dec_str(inner)? }),
        "M" => Ok(SvgNode::Comment { text: dec_str(inner)? }),
        "P" => {
            let parts = split_top_level(inner, ',');
            let [target, data] = parts.as_slice() else { return Err(format!("PI: expected 2 fields, got {}", parts.len())) };
            Ok(SvgNode::ProcessingInstruction { target: dec_str(target)?, data: dec_str(data)? })
        }
        other => Err(format!("xml node: unknown tag {other:?}")),
    }
}

pub fn enc_svg_node(node:&crate::schema::snapshot::SvgNode)->String {semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::diff::enc_xml_node(&crate::standards::v1_1::subsets::base::io::text::snapshot::native_svg_node(node))}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_attr(s: &str) -> Result<SvgAttr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, value] = parts.as_slice() else { return Err(format!("attr: expected 2 fields, got {}", parts.len())) };
    {let name=dec_str(name)?;Ok(SvgAttr { value:crate::standards::v1_1::subsets::base::io::text::snapshot::bind_svg_attribute(&name,&dec_str(value)?)?,name })}
}
