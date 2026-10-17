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
use crate::standards::v1_0::subsets::base::io::text::snapshot::{enc_str, dec_str, enc_prolog, dec_prolog, parse_usize, split_top_level, strip_brackets, encode_option, decode_option, enc_declaration, dec_declaration, enc_doctype, dec_doctype, enc_xml_node, dec_xml_node};

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
        enc_children_diff(self)
    }
}

impl crate::schema::diff::XmlChildrenDiff {
    pub fn decode_text(text: &str) -> Result<Self, String> {
        dec_children_diff(text)
    }
}
