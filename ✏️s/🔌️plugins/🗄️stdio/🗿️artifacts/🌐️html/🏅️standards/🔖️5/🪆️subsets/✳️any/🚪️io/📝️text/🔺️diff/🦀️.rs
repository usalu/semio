//! 📝️ Text representation codec surface for `stdio.html` (diff). The REAL codec is the
//! hand-rolled `print_diff`/`parse_diff` in the sibling `🦀️.rs` two levels up.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v5::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, HtmlNode, HtmlSnapshot, RawTextKind};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

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
pub(crate) fn enc_html_attr(a: &HtmlAttr) -> String {
    format!("[{},{}]", enc_str(&a.name), encode_option(&a.value, |v| enc_str(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_html_attr(s: &str) -> Result<HtmlAttr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, value] = parts.as_slice() else { return Err(format!("attr: expected 2 fields, got {}", parts.len())) };
    Ok(HtmlAttr { name: dec_str(name)?, value: decode_option(value, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_raw_kind(k: RawTextKind) -> &'static str {
    match k {
        RawTextKind::Script => "0",
        RawTextKind::Style => "1",
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_raw_kind(s: &str) -> Result<RawTextKind, String> {
    match s {
        "0" => Ok(RawTextKind::Script),
        "1" => Ok(RawTextKind::Style),
        other => Err(format!("raw text kind: unknown tag {other:?}")),
    }
}

/// 🌳 Recursive: `E[name,[attrs],[children]]` / `T[text]` (Text) / `C[text]` (Comment) /
/// `W[kind,text]` (RawText) — single-letter tag prefix, no ambiguity with the hex payload since
/// hex never starts with an uppercase letter.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_html_node(n: &HtmlNode) -> String {
    match n {
        HtmlNode::Element { name, attributes, children } => {
            let attrs = attributes.iter().map(enc_html_attr).collect::<Vec<_>>().join(",");
            let children = children.iter().map(enc_html_node).collect::<Vec<_>>().join(",");
            format!("E[{},[{}],[{}]]", enc_str(name), attrs, children)
        }
        HtmlNode::Text { text } => format!("T[{}]", enc_str(text)),
        HtmlNode::Comment { text } => format!("C[{}]", enc_str(text)),
        HtmlNode::RawText { parent_kind, text } => format!("W[{},{}]", enc_raw_kind(*parent_kind), enc_str(text)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_html_node(s: &str) -> Result<HtmlNode, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "E" => {
            let parts = split_top_level(inner, ',');
            let [name, attrs, children] = parts.as_slice() else { return Err(format!("element: expected 3 fields, got {}", parts.len())) };
            let attributes = split_top_level(strip_brackets(attrs)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_html_attr).collect::<Result<Vec<_>, String>>()?;
            let children = split_top_level(strip_brackets(children)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_html_node).collect::<Result<Vec<_>, String>>()?;
            Ok(HtmlNode::Element { name: dec_str(name)?, attributes, children })
        }
        "T" => Ok(HtmlNode::Text { text: dec_str(inner)? }),
        "C" => Ok(HtmlNode::Comment { text: dec_str(inner)? }),
        "W" => {
            let parts = split_top_level(inner, ',');
            let [kind, text] = parts.as_slice() else { return Err(format!("raw text: expected 2 fields, got {}", parts.len())) };
            Ok(HtmlNode::RawText { parent_kind: dec_raw_kind(kind)?, text: dec_str(text)? })
        }
        other => Err(format!("html node: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attrs_diff(d: &HtmlAttributesDiff) -> String {
    let removed = d.removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.name), encode_option(&m.value, |v| enc_str(v)))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.index, enc_str(&a.name), encode_option(&a.value, |v| enc_str(v)))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attrs_diff(body: &str) -> Result<HtmlAttributesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("attrs diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (name, value) = entry.split_once(':').ok_or_else(|| format!("attr modified: bad entry {entry:?}"))?;
            Ok(HtmlAttrModified { name: dec_str(name)?, value: decode_option(value, dec_str)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
            let (name, value) = rest.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
            Ok(HtmlAttrAdded { index: parse_usize(idx)?, name: dec_str(name)?, value: decode_option(value, dec_str)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(HtmlAttributesDiff { removed, modified, added })
}

/// 🌳 Recursive: `HtmlNodeDiff` itself needs a tag (`E`=Element, `T`=Text, `M`=Comment, `W`=RawText,
/// `R`=Replace) since, unlike `HtmlNode`, it appears standalone (not always inside a bracketed
/// container) at the `root=` top-level token position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff(d: &HtmlNodeDiff) -> String {
    match d {
        HtmlNodeDiff::Element(e) => format!(
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
        HtmlNodeDiff::Text { text } => format!("T[{}]", encode_option(text, |v| enc_str(v))),
        HtmlNodeDiff::Comment { text } => format!("M[{}]", encode_option(text, |v| enc_str(v))),
        HtmlNodeDiff::RawText { parent_kind, text } => {
            format!("W[{},{}]", encode_option(parent_kind, |k| enc_raw_kind(*k).to_string()), encode_option(text, |v| enc_str(v)))
        }
        HtmlNodeDiff::Replace { node } => format!("R[{}]", enc_html_node(node)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff(s: &str) -> Result<HtmlNodeDiff, String> {
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
            Ok(HtmlNodeDiff::Element(HtmlElementDiff { name: decode_option(name, dec_str)?, attributes, children }))
        }
        "T" => Ok(HtmlNodeDiff::Text { text: decode_option(inner, dec_str)? }),
        "M" => Ok(HtmlNodeDiff::Comment { text: decode_option(inner, dec_str)? }),
        "W" => {
            let parts = split_top_level(inner, ',');
            let [kind, text] = parts.as_slice() else { return Err(format!("raw text diff: expected 2 fields, got {}", parts.len())) };
            Ok(HtmlNodeDiff::RawText { parent_kind: decode_option(kind, dec_raw_kind)?, text: decode_option(text, dec_str)? })
        }
        "R" => Ok(HtmlNodeDiff::Replace { node: dec_html_node(inner)? }),
        other => Err(format!("node diff: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_children_diff(d: &HtmlChildrenDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_node_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_html_node(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_children_diff(body: &str) -> Result<HtmlChildrenDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("children diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child modified: bad entry {entry:?}"))?;
            Ok(HtmlChildModified { index: parse_usize(idx)?, diff: dec_node_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child added: bad entry {entry:?}"))?;
            Ok(HtmlChildAdded { index: parse_usize(idx)?, item: dec_html_node(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(HtmlChildrenDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_html_diff(d: &HtmlDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.doctype {
        tokens.push(format!("doctype={}", encode_option(v, |v| enc_str(v))));
    }
    if let Some(v) = &d.root {
        tokens.push(format!("root={}", enc_node_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_html_diff(line: &str) -> Result<HtmlDiff, String> {
    let mut d = HtmlDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("doctype=") {
            d.doctype = Some(decode_option(rest, dec_str)?);
        } else if let Some(rest) = token.strip_prefix("root=") {
            d.root = Some(dec_node_diff(rest)?);
        } else {
            return Err(format!("html diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for HtmlDiff {
fn print_diff(&self) -> String {
    print_html_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_html_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
