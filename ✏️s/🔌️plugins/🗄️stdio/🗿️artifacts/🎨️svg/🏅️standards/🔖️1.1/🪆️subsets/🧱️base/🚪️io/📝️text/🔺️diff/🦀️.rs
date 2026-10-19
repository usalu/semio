//! 📝️ Text representation codec surface for `stdio.svg` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_1::subsets::base::io::text::snapshot::document_parts::{dec_attr, enc_str, dec_str, split_top_level, strip_brackets, encode_option, decode_option, enc_prolog, dec_prolog, enc_declaration, dec_declaration, enc_doctype, dec_doctype, dec_svg_node, enc_svg_node};
use crate::standards::v1_1::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};

use crate::standards::v1_1::subsets::base::io::binary::snapshot::document_parts::enc_svg_node_bin;
use crate::SvgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlQuote};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::SvgAttr;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration;
use crate::schema::snapshot::SvgNode;





// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}















// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attrs_diff(d: &SvgAttributesDiff) -> String {
    let removed = d.removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.name), enc_str(&crate::standards::v1_1::subsets::base::io::text::snapshot::print_svg_attribute(&m.value)))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.index, enc_str(&a.name), enc_str(&crate::standards::v1_1::subsets::base::io::text::snapshot::print_svg_attribute(&a.value)))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attrs_diff(body: &str) -> Result<SvgAttributesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("attrs diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name, value) = entry.split_once(':').ok_or_else(|| format!("attr modified: bad entry {entry:?}"))?;
            {let name=dec_str(name)?;Ok(SvgAttrModified {value:crate::standards::v1_1::subsets::base::io::text::snapshot::bind_svg_attribute(&name,&dec_str(value)?)?,name})}
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
            let (name, value) = rest.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
            {let name=dec_str(name)?;Ok(SvgAttrAdded {index:parse_usize(idx)?,value:crate::standards::v1_1::subsets::base::io::text::snapshot::bind_svg_attribute(&name,&dec_str(value)?)?,name})}
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(SvgAttributesDiff { removed, modified, added })
}

/// 🌳 Recursive: `SvgNodeDiff` itself needs a tag (`E`=Element, `T`=Text, `R`=Replace) since,
/// unlike `SvgNode`, it appears standalone (not always inside a bracketed container) at the `root=`
/// top-level token position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff(d: &SvgNodeDiff) -> String {
    match d {
        SvgNodeDiff::Element(e) => format!(
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
        SvgNodeDiff::Text { text } => format!("T[{}]", encode_option(text, |v| enc_str(v))),
        SvgNodeDiff::Replace { node } => format!("R[{}]", encode_option(node, enc_svg_node)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff(s: &str) -> Result<SvgNodeDiff, String> {
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
            Ok(SvgNodeDiff::Element(SvgElementDiff { name: decode_option(name, dec_str)?, attributes, children }))
        }
        "T" => Ok(SvgNodeDiff::Text { text: decode_option(inner, dec_str)? }),
        "R" => Ok(SvgNodeDiff::Replace { node: decode_option(inner, dec_svg_node)? }),
        other => Err(format!("node diff: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_children_diff(d: &SvgChildrenDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_node_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_svg_node(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_children_diff(body: &str) -> Result<SvgChildrenDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("children diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child modified: bad entry {entry:?}"))?;
            Ok(SvgChildModified { index: parse_usize(idx)?, diff: dec_node_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child added: bad entry {entry:?}"))?;
            Ok(SvgChildAdded { index: parse_usize(idx)?, item: dec_svg_node(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(SvgChildrenDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_svg_diff(d: &SvgDiff) -> String {
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
pub(crate) fn parse_svg_diff(line: &str) -> Result<SvgDiff, String> {
    let mut d = SvgDiff::default();
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
            return Err(format!("svg diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SvgDiff {
fn print_diff(&self) -> String {
    print_svg_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_svg_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}

}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v1_1::subsets::base::io::text::snapshot::document_parts::{dec_attr, enc_str, dec_str, split_top_level, strip_brackets, encode_option, decode_option, enc_prolog, dec_prolog, enc_declaration, dec_declaration, enc_doctype, dec_doctype, dec_svg_node, enc_svg_node};
use crate::standards::v1_1::subsets::base::schema::diff::*;

use crate::standards::v1_1::subsets::base::io::binary::snapshot::document_parts::enc_svg_node_bin;
use crate::SvgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlQuote};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::SvgAttr;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration;
use crate::schema::snapshot::SvgNode;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr(a: &SvgAttr) -> String {
    format!("[{},{}]", enc_str(&a.name), enc_str(&crate::standards::v1_1::subsets::base::io::text::snapshot::print_svg_attribute(&a.value)))
}
}

