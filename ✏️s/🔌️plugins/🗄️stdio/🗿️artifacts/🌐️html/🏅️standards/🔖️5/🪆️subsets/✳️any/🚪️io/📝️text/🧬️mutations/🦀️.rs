//! 📝️ Text representation codec surface for `stdio.html` (mutations). The REAL codec is the
//! hand-rolled `print_op`/`parse_op` in the sibling `🦀️.rs` two levels up.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v5::subsets::any::schema::mutations::*;
use crate::standards::v5::subsets::any::io::text::diff::{dec_html_node};
use crate::standards::v5::subsets::any::io::text::diff::{enc_html_node};
use crate::standards::v5::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::v5::subsets::any::io::text::diff::{split_top_level};
use crate::standards::v5::subsets::any::io::text::diff::{decode_option};
use crate::standards::v5::subsets::any::io::text::diff::{encode_option};
use crate::standards::v5::subsets::any::io::text::diff::{dec_str};
use crate::standards::v5::subsets::any::io::text::diff::{enc_str};
use crate::standards::v5::subsets::any::schema::diff::{diff_at_path, diff_set_snapshot, HtmlAttrAdded, HtmlAttrModified, HtmlAttributesDiff, HtmlChildAdded, HtmlChildrenDiff, HtmlDiff, HtmlElementDiff, HtmlNodeDiff};
use crate::standards::v5::subsets::any::schema::snapshot::{element_attr, node_at, HtmlNode, HtmlSnapshot, NodePath};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use semio_s_artifact_stdio_contract::deserialize_double_option;

/// 🧪️ Hand-rolled `OpText`/`OpBinary` for `HtmlMutation` (same blocker as `HtmlDiff`'s hand-rolled
/// `DiffCodec`: `#[derive(dsl::DslOps)]` requires `DslField` on every reachable type, which no
/// data-carrying enum implements) — reuses the diff module's `pub(crate)` grammar primitives.
/// Grammar: `keyword arg=value ...` (space-separated), one match arm per variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_path(p: &NodePath) -> String {
    format!("[{}]", p.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_path(s: &str) -> Result<NodePath, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| s.parse().map_err(|e: std::num::ParseIntError| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_html_snapshot(s: &HtmlSnapshot) -> String {
    format!("[{},{},{}]", enc_str(&s.schema), encode_option(&s.doctype, |v| enc_str(v)), enc_html_node(&s.root))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_html_snapshot(s: &str) -> Result<HtmlSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, doctype, root] = parts.as_slice() else { return Err(format!("html snapshot: expected 3 fields, got {}", parts.len())) };
    Ok(HtmlSnapshot { schema: dec_str(schema)?, doctype: decode_option(doctype, dec_str)?, root: dec_html_node(root)? })
}

/// 🏳️ Tri-state attribute value: `[0]` = remove, `[1,[0]]` = valueless, `[1,[1,hex]]` = set value.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr_value_tristate(v: &Option<Option<String>>) -> String {
    encode_option(v, |inner: &Option<String>| encode_option(inner, |s| enc_str(s)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attr_value_tristate(s: &str) -> Result<Option<Option<String>>, String> {
    decode_option(s, |inner| decode_option(inner, dec_str))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_html_mutation(m: &HtmlMutation) -> String {
    match m {
        HtmlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_html_snapshot(snapshot)),
        HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype }) => format!("set-doctype doctype={}", encode_option(doctype, |v| enc_str(v))),
        HtmlMutation::InsertNode(insert_node::InsertNode { parent, index, node }) => format!("insert-node parent={} index={index} node={}", enc_node_path(parent), enc_html_node(node)),
        HtmlMutation::RemoveNode(remove_node::RemoveNode { parent, index }) => format!("remove-node parent={} index={index}", enc_node_path(parent)),
        HtmlMutation::SetElementName(set_element_name::SetElementName { path, name }) => format!("set-element-name path={} name={}", enc_node_path(path), enc_str(name)),
        HtmlMutation::SetAttribute(set_attribute::SetAttribute { path, name, value }) => format!("set-attribute path={} name={} value={}", enc_node_path(path), enc_str(name), enc_attr_value_tristate(value)),
        HtmlMutation::SetText(set_text::SetText { path, text }) => format!("set-text path={} text={}", enc_node_path(path), enc_str(text)),
        HtmlMutation::SetComment(set_comment::SetComment { path, text }) => format!("set-comment path={} text={}", enc_node_path(path), enc_str(text)),
        HtmlMutation::SetRawText(set_raw_text::SetRawText { path, text }) => format!("set-raw-text path={} text={}", enc_node_path(path), enc_str(text)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_html_mutation(line: &str) -> Result<HtmlMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("html mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("html mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(HtmlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_html_snapshot(arg("snapshot")?)? })),
        "set-doctype" => Ok(HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype: decode_option(arg("doctype")?, dec_str)? })),
        "insert-node" => Ok(HtmlMutation::InsertNode(insert_node::InsertNode { parent: dec_node_path(arg("parent")?)?, index: usize_arg("index")?, node: dec_html_node(arg("node")?)? })),
        "remove-node" => Ok(HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: dec_node_path(arg("parent")?)?, index: usize_arg("index")? })),
        "set-element-name" => Ok(HtmlMutation::SetElementName(set_element_name::SetElementName { path: dec_node_path(arg("path")?)?, name: dec_str(arg("name")?)? })),
        "set-attribute" => Ok(HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: dec_node_path(arg("path")?)?, name: dec_str(arg("name")?)?, value: dec_attr_value_tristate(arg("value")?)? })),
        "set-text" => Ok(HtmlMutation::SetText(set_text::SetText { path: dec_node_path(arg("path")?)?, text: dec_str(arg("text")?)? })),
        "set-comment" => Ok(HtmlMutation::SetComment(set_comment::SetComment { path: dec_node_path(arg("path")?)?, text: dec_str(arg("text")?)? })),
        "set-raw-text" => Ok(HtmlMutation::SetRawText(set_raw_text::SetRawText { path: dec_node_path(arg("path")?)?, text: dec_str(arg("text")?)? })),
        other => Err(format!("html mutation: unknown keyword {other:?}")),
    }
}

impl OpText for HtmlMutation {
    fn print_op(&self) -> String {
        print_html_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_html_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
