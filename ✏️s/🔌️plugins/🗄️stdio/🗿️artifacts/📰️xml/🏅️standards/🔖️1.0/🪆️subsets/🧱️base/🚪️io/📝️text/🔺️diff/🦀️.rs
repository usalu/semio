//! 📝️ Text representation codec surface for `stdio.xml` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_0::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::{validate_xml_document_boundaries, XmlAttr, XmlDeclaration, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote};
use crate::XmlSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::deserialize_double_option;

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
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_prolog(prolog: &[XmlNode]) -> String {
    format!("[{}]", prolog.iter().map(enc_xml_node).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_prolog(s: &str) -> Result<Vec<XmlNode>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().map(dec_xml_node).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

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
pub(crate) fn enc_declaration(d: &XmlDeclaration) -> String {
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
pub(crate) fn dec_declaration(s: &str) -> Result<XmlDeclaration, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [version, encoding, standalone, quote] = parts.as_slice() else { return Err(format!("declaration: expected 4 fields, got {}", parts.len())) };
    Ok(XmlDeclaration { version: dec_str(version)?, encoding: decode_option(encoding, dec_str)?, standalone: decode_option(standalone, |v| Ok(v == "1"))?, quote: dec_quote(quote)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_doctype(doctype: &XmlDoctype) -> String {
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
pub(crate) fn dec_doctype(s: &str) -> Result<XmlDoctype, String> {
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
pub(crate) fn dec_xml_node(s: &str) -> Result<XmlNode, String> {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attrs_diff(d: &XmlAttributesDiff) -> String {
    let removed = d.removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.name), enc_str(&m.value))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", enc_str(&a.name), enc_str(&a.value))).collect::<Vec<_>>().join(",");
    let order = d.order.iter().map(|name| enc_str(name)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}];[{order}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attrs_diff(body: &str) -> Result<XmlAttributesDiff, String> {
    let sections = split_top_level(body, ';');
    let [removed_s, modified_s, added_s, order_s] = sections.as_slice() else { return Err(format!("attrs diff: expected 4 sections, got {}", sections.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name, value) = entry.split_once(':').ok_or_else(|| format!("attr modified: bad entry {entry:?}"))?;
            Ok(XmlAttrModified { name: dec_str(name)?, value: dec_str(value)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name, value) = entry.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
            Ok(XmlAttrAdded { name: dec_str(name)?, value: dec_str(value)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let order = split_top_level(strip_brackets(order_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    Ok(XmlAttributesDiff { removed, modified, added, order })
}

/// 🌳 Recursive: `XmlNodeDiff` itself needs a tag (`E`=Element, `T`=Text, `R`=Replace) since,
/// unlike `XmlNode`, it appears standalone (not always inside a bracketed container) at the `root=`
/// top-level token position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff(d: &XmlNodeDiff) -> String {
    match d {
        XmlNodeDiff::Element(e) => format!(
            "E[{},{},{}]",
            encode_option(&e.name, |v| enc_str(v)),
            match &e.attributes {
                Some(a) => format!("[1,{}]", enc_attrs_diff(a)),
                None => "[0]".to_string(),
            },
            match &e.children {
                Some(c) => format!("[1,{}]", enc_children_diff(c)),
                None => "[0]".to_string(),
            },
        ),
        XmlNodeDiff::Text { text } => format!("T[{}]", encode_option(text, |v| enc_str(v))),
        XmlNodeDiff::Replace { node } => format!("R[{}]", encode_option(node, enc_xml_node)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff(s: &str) -> Result<XmlNodeDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "E" => {
            let parts = split_top_level(inner, ',');
            let [name, attributes, children] = parts.as_slice() else { return Err(format!("node diff element: expected 3 fields, got {}", parts.len())) };
            let attributes = match split_top_level(strip_brackets(attributes)?, ',').as_slice() {
                ["0"] => None,
                [tag, rest @ ..] if *tag == "1" => Some(dec_attrs_diff(&rest.join(","))?),
                other => return Err(format!("node diff element attrs: bad shape {other:?}")),
            };
            let children = match split_top_level(strip_brackets(children)?, ',').as_slice() {
                ["0"] => None,
                [tag, rest @ ..] if *tag == "1" => Some(dec_children_diff(&rest.join(","))?),
                other => return Err(format!("node diff element children: bad shape {other:?}")),
            };
            Ok(XmlNodeDiff::Element(XmlElementDiff { name: decode_option(name, dec_str)?, attributes, children }))
        }
        "T" => Ok(XmlNodeDiff::Text { text: decode_option(inner, dec_str)? }),
        "R" => Ok(XmlNodeDiff::Replace { node: decode_option(inner, dec_xml_node)? }),
        other => Err(format!("node diff: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_children_diff(d: &XmlChildrenDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_node_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_xml_node(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_children_diff(body: &str) -> Result<XmlChildrenDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("children diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child modified: bad entry {entry:?}"))?;
            Ok(XmlChildModified { index: parse_usize(idx)?, diff: dec_node_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child added: bad entry {entry:?}"))?;
            Ok(XmlChildAdded { index: parse_usize(idx)?, item: dec_xml_node(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(XmlChildrenDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_xml_diff(d: &XmlDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.prolog {
        tokens.push(format!("prolog={}", enc_prolog(v)));
    }
    if let Some(v) = &d.epilog {
        tokens.push(format!("epilog={}", enc_prolog(v)));
    }
    if let Some(v) = &d.declaration {
        tokens.push(format!("declaration={}", encode_option(v, enc_declaration)));
    }
    if let Some(v) = &d.doctype {
        tokens.push(format!("doctype={}", encode_option(v, enc_doctype)));
    }
    if let Some(v) = &d.root {
        tokens.push(format!("root={}", enc_node_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_xml_diff(line: &str) -> Result<XmlDiff, String> {
    let mut d = XmlDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("prolog=") {
            d.prolog = Some(dec_prolog(rest)?);
        } else if let Some(rest) = token.strip_prefix("epilog=") {
            d.epilog = Some(dec_prolog(rest)?);
        } else if let Some(rest) = token.strip_prefix("declaration=") {
            d.declaration = Some(decode_option(rest, dec_declaration)?);
        } else if let Some(rest) = token.strip_prefix("doctype=") {
            d.doctype = Some(decode_option(rest, dec_doctype)?);
        } else if let Some(rest) = token.strip_prefix("root=") {
            d.root = Some(dec_node_diff(rest)?);
        } else {
            return Err(format!("xml diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for XmlDiff {
fn print_diff(&self) -> String {
    print_xml_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_xml_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;

impl crate::schema::diff::XmlChildrenDiff {
    pub fn encode_text(&self) -> String {
        crate::standards::v1_0::subsets::base::io::text::diff::enc_children_diff(self)
    }
}

impl crate::schema::diff::XmlChildrenDiff {
    pub fn decode_text(text: &str) -> Result<Self, String> {
        crate::standards::v1_0::subsets::base::io::text::diff::dec_children_diff(text)
    }
}
