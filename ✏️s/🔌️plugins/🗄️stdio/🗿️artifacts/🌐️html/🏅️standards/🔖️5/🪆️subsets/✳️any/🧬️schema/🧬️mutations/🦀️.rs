//! 🧬️ HtmlMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, path/key-aware. Structural
//! pattern borrowed from `🎨️svg`'s `SvgMutation` (own types throughout).
//!
//! Leaf-per-variant shape mirrored from `🖼️tiff`'s `TiffBaselineMutation` (ticket
//! `26/08/29/S-END-TO-END`): `NoMutation` was dropped (`#[derive(dsl::Mutations)]` requires every
//! variant to wrap exactly one leaf payload and a unit variant wraps none; `no` is not an approved
//! semantic verb either), and every remaining variant now wraps its own `dsl::MutationLeaf` struct
//! instead of carrying its fields as a struct-literal directly. `#[value(tag = "mutation", ...)]`
//! is kept (unlike tiff, which has none) so the wire shape this artifact's committed fixtures and
//! `OpText`/`OpBinary` already depend on stays byte-for-byte identical — serde's internally-tagged
//! representation supports a newtype variant wrapping a plain struct.

use crate::standards::v5::subsets::any::schema::diff::{dec_html_node, dec_str, decode_option, enc_html_node, enc_str, encode_option, split_top_level, strip_brackets};
use crate::standards::v5::subsets::any::schema::diff::{diff_at_path, diff_set_snapshot, HtmlAttrAdded, HtmlAttrModified, HtmlAttributesDiff, HtmlChildAdded, HtmlChildrenDiff, HtmlDiff, HtmlElementDiff, HtmlNodeDiff};
use crate::standards::v5::subsets::any::schema::snapshot::{element_attr, node_at, HtmlNode, HtmlSnapshot, NodePath};
use protocol::OpBinary;
use protocol::{Mutation, OpText};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.html`. Beyond the baseline `SetSnapshot`, this addresses
/// nodes in the persisted `HtmlSnapshot.root` tree by `NodePath` (child-index chain from the root
/// element). `InsertNode`/`RemoveNode`'s `parent` addresses the PARENT element (`index` is the
/// position among the parent's children); every other path-carrying variant's `path` addresses the
/// target node itself.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "📜set-doctype/🦀️.rs"]
pub mod set_doctype;
#[path = "➕insert-node/🦀️.rs"]
pub mod insert_node;
#[path = "➖remove-node/🦀️.rs"]
pub mod remove_node;
#[path = "🏷️set-element-name/🦀️.rs"]
pub mod set_element_name;
#[path = "🔖set-attribute/🦀️.rs"]
pub mod set_attribute;
#[path = "✍️set-text/🦀️.rs"]
pub mod set_text;
#[path = "💬set-comment/🦀️.rs"]
pub mod set_comment;
#[path = "⌨️set-raw-text/🦀️.rs"]
pub mod set_raw_text;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = HtmlSnapshot, diff = HtmlDiff, schema = "HtmlMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum HtmlMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// 📜️ Sets (or, if `None`, clears) the document's raw `<!DOCTYPE ...>` declaration content.
    SetDoctype(set_doctype::SetDoctype),
    /// ➕️ Inserts `node` as child `index` of the element at `parent`.
    InsertNode(insert_node::InsertNode),
    /// ➖️ Removes child `index` of the element at `parent`.
    RemoveNode(remove_node::RemoveNode),
    /// 🏷️ Renames the element at `path`.
    SetElementName(set_element_name::SetElementName),
    /// 🏷️ Sets attribute `name` on the element at `path`. Tri-state `value`: `None` = remove the
    /// attribute entirely, `Some(None)` = set/keep it VALUELESS (e.g. `disabled`), `Some(Some(v))`
    /// = set it to `v`.
    SetAttribute(set_attribute::SetAttribute),
    /// ✍️ Replaces the literal text of the `Text` node at `path`.
    SetText(set_text::SetText),
    /// 💬️ Replaces the literal text of the `Comment` node at `path`.
    SetComment(set_comment::SetComment),
    /// 📄️ Replaces the literal text of the `RawText` node at `path` (its `parent_kind` — whether
    /// it belongs to a `<script>` or `<style>` element — is left unchanged).
    SetRawText(set_raw_text::SetRawText),
}

/// 📇️ Kebab-case spelling of every `HtmlMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../🔣️oracle.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["set-snapshot", "set-doctype", "insert-node", "remove-node", "set-element-name", "set-attribute", "set-text", "set-comment", "set-raw-text"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_html_mutation(snapshot: &mut HtmlSnapshot, mutation: &HtmlMutation) -> protocol::MutationOutcome<HtmlDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️AttributeHelper
/// 🏷️ Shared diff-construction for `SetAttribute`. Resolves the PRIOR tri-state of `name` on the
/// element addressed by `path` against `base`, then builds the exact `HtmlAttributesDiff` entry
/// the transition requires, lowered through `diff_at_path`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attribute_diff_at_path(base: &HtmlSnapshot, path: &[usize], name: &str, value: Option<Option<String>>) -> HtmlDiff {
    let target = node_at(base, path).ok();
    let existing: Option<&Option<String>> = target.and_then(|n| element_attr(n, name));
    let attrs_diff = match (existing, value) {
        (Some(_), None) => HtmlAttributesDiff { removed: vec![name.to_string()], modified: Vec::new(), added: Vec::new() },
        (Some(_), Some(v)) => HtmlAttributesDiff { removed: Vec::new(), modified: vec![HtmlAttrModified { name: name.to_string(), value: v }], added: Vec::new() },
        (None, Some(v)) => {
            let next_index = match target {
                Some(HtmlNode::Element { attributes, .. }) => attributes.len(),
                _ => 0,
            };
            HtmlAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![HtmlAttrAdded { index: next_index, name: name.to_string(), value: v }] }
        }
        (None, None) => HtmlAttributesDiff::default(),
    };
    diff_at_path(path, HtmlNodeDiff::Element(HtmlElementDiff { name: None, attributes: Some(attrs_diff), children: None }))
}

/// 🔎 Reads the PRIOR tri-state of attribute `name` on the element addressed by `path` in `base`:
/// `None` = attribute absent, `Some(None)` = present and valueless, `Some(Some(v))` = present with
/// value `v`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn prior_attribute(base: &HtmlSnapshot, path: &[usize], name: &str) -> Option<Option<String>> {
    node_at(base, path).ok().and_then(|n| element_attr(n, name)).cloned()
}
//#endregion 🔖️AttributeHelper

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &HtmlMutation, base: &HtmlSnapshot) -> protocol::MutationOutcome<HtmlDiff> {
    protocol::MutationOutcome::new(match this {
        HtmlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype }) => HtmlDiff { doctype: Some(doctype.clone()), root: None },
        HtmlMutation::InsertNode(insert_node::InsertNode { parent, index, node }) => diff_at_path(
            parent,
            HtmlNodeDiff::Element(HtmlElementDiff { name: None, attributes: None, children: Some(HtmlChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![HtmlChildAdded { index: *index, item: node.clone() }] }) }),
        ),
        HtmlMutation::RemoveNode(remove_node::RemoveNode { parent, index }) => {
            diff_at_path(parent, HtmlNodeDiff::Element(HtmlElementDiff { name: None, attributes: None, children: Some(HtmlChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) }))
        }
        HtmlMutation::SetElementName(set_element_name::SetElementName { path, name }) => diff_at_path(path, HtmlNodeDiff::Element(HtmlElementDiff { name: Some(name.clone()), attributes: None, children: None })),
        HtmlMutation::SetAttribute(set_attribute::SetAttribute { path, name, value }) => attribute_diff_at_path(base, path, name, value.clone()),
        HtmlMutation::SetText(set_text::SetText { path, text }) => diff_at_path(path, HtmlNodeDiff::Text { text: Some(text.clone()) }),
        HtmlMutation::SetComment(set_comment::SetComment { path, text }) => diff_at_path(path, HtmlNodeDiff::Comment { text: Some(text.clone()) }),
        HtmlMutation::SetRawText(set_raw_text::SetRawText { path, text }) => diff_at_path(path, HtmlNodeDiff::RawText { parent_kind: None, text: Some(text.clone()) }),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &HtmlMutation, base: &HtmlSnapshot) -> Vec<HtmlMutation> {
    match this {
        HtmlMutation::SetSnapshot(_) => vec![HtmlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        HtmlMutation::SetDoctype(_) => vec![HtmlMutation::SetDoctype(set_doctype::SetDoctype { doctype: base.doctype.clone() })],
        HtmlMutation::InsertNode(insert_node::InsertNode { parent, index, .. }) => vec![HtmlMutation::RemoveNode(remove_node::RemoveNode { parent: parent.clone(), index: *index })],
        HtmlMutation::RemoveNode(remove_node::RemoveNode { parent, index }) => match node_at(base, parent) {
            Ok(HtmlNode::Element { children, .. }) => match children.get(*index) {
                Some(node) => vec![HtmlMutation::InsertNode(insert_node::InsertNode { parent: parent.clone(), index: *index, node: node.clone() })],
                None => Vec::new(),
            },
            _ => Vec::new(),
        },
        HtmlMutation::SetElementName(set_element_name::SetElementName { path, .. }) => {
            let prior = match node_at(base, path) {
                Ok(HtmlNode::Element { name, .. }) => name.clone(),
                _ => return Vec::new(),
            };
            vec![HtmlMutation::SetElementName(set_element_name::SetElementName { path: path.clone(), name: prior })]
        }
        HtmlMutation::SetAttribute(set_attribute::SetAttribute { path, name, .. }) => {
            vec![HtmlMutation::SetAttribute(set_attribute::SetAttribute { path: path.clone(), name: name.clone(), value: prior_attribute(base, path, name) })]
        }
        HtmlMutation::SetText(set_text::SetText { path, .. }) => {
            let old = match node_at(base, path) {
                Ok(HtmlNode::Text { text }) => text.clone(),
                _ => String::new(),
            };
            vec![HtmlMutation::SetText(set_text::SetText { path: path.clone(), text: old })]
        }
        HtmlMutation::SetComment(set_comment::SetComment { path, .. }) => {
            let old = match node_at(base, path) {
                Ok(HtmlNode::Comment { text }) => text.clone(),
                _ => String::new(),
            };
            vec![HtmlMutation::SetComment(set_comment::SetComment { path: path.clone(), text: old })]
        }
        HtmlMutation::SetRawText(set_raw_text::SetRawText { path, .. }) => {
            let old = match node_at(base, path) {
                Ok(HtmlNode::RawText { text, .. }) => text.clone(),
                _ => String::new(),
            };
            vec![HtmlMutation::SetRawText(set_raw_text::SetRawText { path: path.clone(), text: old })]
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ Hand-rolled `OpText`/`OpBinary` for `HtmlMutation` (same blocker as `HtmlDiff`'s hand-rolled
/// `DiffCodec`: `#[derive(dsl::DslOps)]` requires `DslField` on every reachable type, which no
/// data-carrying enum implements) — reuses the diff module's `pub(crate)` grammar primitives.
/// Grammar: `keyword arg=value ...` (space-separated), one match arm per variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_node_path(p: &NodePath) -> String {
    format!("[{}]", p.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_node_path(s: &str) -> Result<NodePath, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| s.parse().map_err(|e: std::num::ParseIntError| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_html_snapshot(s: &HtmlSnapshot) -> String {
    format!("[{},{},{}]", enc_str(&s.schema), encode_option(&s.doctype, |v| enc_str(v)), enc_html_node(&s.root))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_html_snapshot(s: &str) -> Result<HtmlSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, doctype, root] = parts.as_slice() else { return Err(format!("html snapshot: expected 3 fields, got {}", parts.len())) };
    Ok(HtmlSnapshot { schema: dec_str(schema)?, doctype: decode_option(doctype, dec_str)?, root: dec_html_node(root)? })
}
/// 🏳️ Tri-state attribute value: `[0]` = remove, `[1,[0]]` = valueless, `[1,[1,hex]]` = set value.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_attr_value_tristate(v: &Option<Option<String>>) -> String {
    encode_option(v, |inner: &Option<String>| encode_option(inner, |s| enc_str(s)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_attr_value_tristate(s: &str) -> Result<Option<Option<String>>, String> {
    decode_option(s, |inner| decode_option(inner, dec_str))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_html_mutation(m: &HtmlMutation) -> String {
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
fn parse_html_mutation(line: &str) -> Result<HtmlMutation, String> {
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_html_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification as `HtmlDiff`'s hand-rolled codec.
impl OpBinary for HtmlMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
