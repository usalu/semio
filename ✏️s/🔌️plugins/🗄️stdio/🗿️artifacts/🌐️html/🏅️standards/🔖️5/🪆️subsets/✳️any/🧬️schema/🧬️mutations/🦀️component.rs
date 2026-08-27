//! 🧬️ HtmlMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, path/key-aware. Structural
//! pattern borrowed from `🎨️svg`'s `SvgMutation` (own types throughout).

use crate::artifacts::html::standards::v5::subsets::any::schema::diff::{dec_html_node, dec_str, decode_option, enc_html_node, enc_str, encode_option, split_top_level, strip_brackets};
use crate::artifacts::html::standards::v5::subsets::any::schema::diff::{diff_at_path, diff_set_snapshot, HtmlAttrAdded, HtmlAttrModified, HtmlAttributesDiff, HtmlChildAdded, HtmlChildrenDiff, HtmlDiff, HtmlElementDiff, HtmlNodeDiff};
use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::{element_attr, node_at, HtmlNode, HtmlSnapshot, NodePath};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.html`. Beyond the baseline `{NoMutation, SetSnapshot}`,
/// this addresses nodes in the persisted `HtmlSnapshot.root` tree by `NodePath` (child-index chain
/// from the root element). `InsertNode`/`RemoveNode`'s `parent` addresses the PARENT element
/// (`index` is the position among the parent's children); every other path-carrying variant's
/// `path` addresses the target node itself.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum HtmlMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: HtmlSnapshot,
    },
    /// 📜️ Sets (or, if `None`, clears) the document's raw `<!DOCTYPE ...>` declaration content.
    SetDoctype {
        doctype: Option<String>,
    },
    /// ➕️ Inserts `node` as child `index` of the element at `parent`.
    InsertNode {
        parent: NodePath,
        index: usize,
        node: HtmlNode,
    },
    /// ➖️ Removes child `index` of the element at `parent`.
    RemoveNode {
        parent: NodePath,
        index: usize,
    },
    /// 🏷️ Renames the element at `path`.
    SetElementName {
        path: NodePath,
        name: String,
    },
    /// 🏷️ Sets attribute `name` on the element at `path`. Tri-state `value`: `None` = remove the
    /// attribute entirely, `Some(None)` = set/keep it VALUELESS (e.g. `disabled`), `Some(Some(v))`
    /// = set it to `v`.
    SetAttribute {
        path: NodePath,
        name: String,
        value: Option<Option<String>>,
    },
    /// ✍️ Replaces the literal text of the `Text` node at `path`.
    SetText {
        path: NodePath,
        text: String,
    },
    /// 💬️ Replaces the literal text of the `Comment` node at `path`.
    SetComment {
        path: NodePath,
        text: String,
    },
    /// 📄️ Replaces the literal text of the `RawText` node at `path` (its `parent_kind` — whether
    /// it belongs to a `<script>` or `<style>` element — is left unchanged).
    SetRawText {
        path: NodePath,
        text: String,
    },
}

/// 📇️ Kebab-case spelling of every `HtmlMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../🧪️oracle/🔣️.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-doctype", "insert-node", "remove-node", "set-element-name", "set-attribute", "set-text", "set-comment", "set-raw-text"];
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
impl Mutation<HtmlSnapshot> for HtmlMutation {
    type Diff = HtmlDiff;

    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            HtmlMutation::NoMutation => HtmlDiff::default(),
            HtmlMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            HtmlMutation::SetDoctype { doctype } => HtmlDiff { doctype: Some(doctype.clone()), root: None },
            HtmlMutation::InsertNode { parent, index, node } => diff_at_path(
                parent,
                HtmlNodeDiff::Element(HtmlElementDiff { name: None, attributes: None, children: Some(HtmlChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![HtmlChildAdded { index: *index, item: node.clone() }] }) }),
            ),
            HtmlMutation::RemoveNode { parent, index } => {
                diff_at_path(parent, HtmlNodeDiff::Element(HtmlElementDiff { name: None, attributes: None, children: Some(HtmlChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) }))
            }
            HtmlMutation::SetElementName { path, name } => diff_at_path(path, HtmlNodeDiff::Element(HtmlElementDiff { name: Some(name.clone()), attributes: None, children: None })),
            HtmlMutation::SetAttribute { path, name, value } => attribute_diff_at_path(base, path, name, value.clone()),
            HtmlMutation::SetText { path, text } => diff_at_path(path, HtmlNodeDiff::Text { text: Some(text.clone()) }),
            HtmlMutation::SetComment { path, text } => diff_at_path(path, HtmlNodeDiff::Comment { text: Some(text.clone()) }),
            HtmlMutation::SetRawText { path, text } => diff_at_path(path, HtmlNodeDiff::RawText { parent_kind: None, text: Some(text.clone()) }),
        })
    }

    fn inverse(&self, base: &HtmlSnapshot) -> Vec<Self> {
        match self {
            HtmlMutation::NoMutation => vec![HtmlMutation::NoMutation],
            HtmlMutation::SetSnapshot { .. } => vec![HtmlMutation::SetSnapshot { snapshot: base.clone() }],
            HtmlMutation::SetDoctype { .. } => vec![HtmlMutation::SetDoctype { doctype: base.doctype.clone() }],
            HtmlMutation::InsertNode { parent, index, .. } => vec![HtmlMutation::RemoveNode { parent: parent.clone(), index: *index }],
            HtmlMutation::RemoveNode { parent, index } => match node_at(base, parent) {
                Ok(HtmlNode::Element { children, .. }) => match children.get(*index) {
                    Some(node) => vec![HtmlMutation::InsertNode { parent: parent.clone(), index: *index, node: node.clone() }],
                    None => vec![HtmlMutation::NoMutation],
                },
                _ => vec![HtmlMutation::NoMutation],
            },
            HtmlMutation::SetElementName { path, .. } => {
                let prior = match node_at(base, path) {
                    Ok(HtmlNode::Element { name, .. }) => name.clone(),
                    _ => return vec![HtmlMutation::NoMutation],
                };
                vec![HtmlMutation::SetElementName { path: path.clone(), name: prior }]
            }
            HtmlMutation::SetAttribute { path, name, .. } => {
                vec![HtmlMutation::SetAttribute { path: path.clone(), name: name.clone(), value: prior_attribute(base, path, name) }]
            }
            HtmlMutation::SetText { path, .. } => {
                let old = match node_at(base, path) {
                    Ok(HtmlNode::Text { text }) => text.clone(),
                    _ => String::new(),
                };
                vec![HtmlMutation::SetText { path: path.clone(), text: old }]
            }
            HtmlMutation::SetComment { path, .. } => {
                let old = match node_at(base, path) {
                    Ok(HtmlNode::Comment { text }) => text.clone(),
                    _ => String::new(),
                };
                vec![HtmlMutation::SetComment { path: path.clone(), text: old }]
            }
            HtmlMutation::SetRawText { path, .. } => {
                let old = match node_at(base, path) {
                    Ok(HtmlNode::RawText { text, .. }) => text.clone(),
                    _ => String::new(),
                };
                vec![HtmlMutation::SetRawText { path: path.clone(), text: old }]
            }
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
        HtmlMutation::NoMutation => "no-mutation".to_string(),
        HtmlMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_html_snapshot(snapshot)),
        HtmlMutation::SetDoctype { doctype } => format!("set-doctype doctype={}", encode_option(doctype, |v| enc_str(v))),
        HtmlMutation::InsertNode { parent, index, node } => format!("insert-node parent={} index={index} node={}", enc_node_path(parent), enc_html_node(node)),
        HtmlMutation::RemoveNode { parent, index } => format!("remove-node parent={} index={index}", enc_node_path(parent)),
        HtmlMutation::SetElementName { path, name } => format!("set-element-name path={} name={}", enc_node_path(path), enc_str(name)),
        HtmlMutation::SetAttribute { path, name, value } => format!("set-attribute path={} name={} value={}", enc_node_path(path), enc_str(name), enc_attr_value_tristate(value)),
        HtmlMutation::SetText { path, text } => format!("set-text path={} text={}", enc_node_path(path), enc_str(text)),
        HtmlMutation::SetComment { path, text } => format!("set-comment path={} text={}", enc_node_path(path), enc_str(text)),
        HtmlMutation::SetRawText { path, text } => format!("set-raw-text path={} text={}", enc_node_path(path), enc_str(text)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_html_mutation(line: &str) -> Result<HtmlMutation, String> {
    if line == "no-mutation" {
        return Ok(HtmlMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("html mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("html mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(HtmlMutation::SetSnapshot { snapshot: dec_html_snapshot(arg("snapshot")?)? }),
        "set-doctype" => Ok(HtmlMutation::SetDoctype { doctype: decode_option(arg("doctype")?, dec_str)? }),
        "insert-node" => Ok(HtmlMutation::InsertNode { parent: dec_node_path(arg("parent")?)?, index: usize_arg("index")?, node: dec_html_node(arg("node")?)? }),
        "remove-node" => Ok(HtmlMutation::RemoveNode { parent: dec_node_path(arg("parent")?)?, index: usize_arg("index")? }),
        "set-element-name" => Ok(HtmlMutation::SetElementName { path: dec_node_path(arg("path")?)?, name: dec_str(arg("name")?)? }),
        "set-attribute" => Ok(HtmlMutation::SetAttribute { path: dec_node_path(arg("path")?)?, name: dec_str(arg("name")?)?, value: dec_attr_value_tristate(arg("value")?)? }),
        "set-text" => Ok(HtmlMutation::SetText { path: dec_node_path(arg("path")?)?, text: dec_str(arg("text")?)? }),
        "set-comment" => Ok(HtmlMutation::SetComment { path: dec_node_path(arg("path")?)?, text: dec_str(arg("text")?)? }),
        "set-raw-text" => Ok(HtmlMutation::SetRawText { path: dec_node_path(arg("path")?)?, text: dec_str(arg("text")?)? }),
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
mod tests {
    use super::*;
    use crate::artifacts::html::standards::v5::subsets::any::schema::diff::{HtmlChildAdded as HtmlChildAddedT, HtmlNodeDiff as HtmlNodeDiffT};
    use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::{write_html_document, HtmlAttr, STDIO_HTML_DOCUMENT_SCHEMA};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn el(name: &str, attrs: Vec<HtmlAttr>, children: Vec<HtmlNode>) -> HtmlNode {
        HtmlNode::Element { name: name.into(), attributes: attrs, children }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fixture() -> HtmlSnapshot {
        <HtmlSnapshot as store::ArtifactDsl>::parse_dsl("<!DOCTYPE html>\n<html><body><p id=\"x\" width=\"5\">hi</p></body></html>\n").unwrap()
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_then_remove_node_apply_and_inverse() {
        let base = fixture();
        let insert = HtmlMutation::InsertNode { parent: vec![0], index: 1, node: el("span", vec![HtmlAttr::new("class", "x")], vec![]) };
        let mut after = base.clone();
        apply_html_mutation(&mut after, &insert);
        match node_at(&after, &[0]).unwrap() {
            HtmlNode::Element { children, .. } => assert_eq!(children.len(), 2),
            other => panic!("unexpected node {other:?}"),
        }
        let inverses = Mutation::inverse(&insert, &base);
        let mut restored = after.clone();
        for inv in &inverses {
            apply_html_mutation(&mut restored, inv);
        }
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_attribute_tristate_apply_and_inverse_round_trip() {
        let base = fixture();
        // Some(Some(v)): modify existing value.
        let m1 = HtmlMutation::SetAttribute { path: vec![0, 0], name: "width".into(), value: Some(Some("99".into())) };
        let d1 = Mutation::diff(&m1, &base);
        let after1 = <HtmlDiff as MutationDiff<HtmlSnapshot>>::apply(d1.diff(), &base).unwrap();
        assert_eq!(element_attr(node_at(&after1, &[0, 0]).unwrap(), "width"), Some(&Some("99".to_string())));
        let mut restored1 = after1.clone();
        for inv in Mutation::inverse(&m1, &base) {
            apply_html_mutation(&mut restored1, &inv);
        }
        assert_eq!(write_html_document(&restored1), write_html_document(&base));

        // Some(None): make valueless.
        let m2 = HtmlMutation::SetAttribute { path: vec![0, 0], name: "width".into(), value: Some(None) };
        let mut after2 = base.clone();
        apply_html_mutation(&mut after2, &m2);
        assert_eq!(element_attr(node_at(&after2, &[0, 0]).unwrap(), "width"), Some(&None));
        for inv in Mutation::inverse(&m2, &base) {
            apply_html_mutation(&mut after2, &inv);
        }
        assert_eq!(write_html_document(&after2), write_html_document(&base));

        // None: remove entirely.
        let m3 = HtmlMutation::SetAttribute { path: vec![0, 0], name: "width".into(), value: None };
        let mut after3 = base.clone();
        apply_html_mutation(&mut after3, &m3);
        assert_eq!(element_attr(node_at(&after3, &[0, 0]).unwrap(), "width"), None);
        for inv in Mutation::inverse(&m3, &base) {
            apply_html_mutation(&mut after3, &inv);
        }
        assert_eq!(write_html_document(&after3), write_html_document(&base));

        // None -> Some: add a brand new attribute.
        let m4 = HtmlMutation::SetAttribute { path: vec![0, 0], name: "hidden".into(), value: Some(None) };
        let mut after4 = base.clone();
        apply_html_mutation(&mut after4, &m4);
        assert_eq!(element_attr(node_at(&after4, &[0, 0]).unwrap(), "hidden"), Some(&None));
        for inv in Mutation::inverse(&m4, &base) {
            apply_html_mutation(&mut after4, &inv);
        }
        assert_eq!(write_html_document(&after4), write_html_document(&base));
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_node_inverse_restores_removed_node() {
        let base = fixture();
        let remove = HtmlMutation::RemoveNode { parent: vec![0], index: 0 };
        let mut after = base.clone();
        apply_html_mutation(&mut after, &remove);
        match node_at(&after, &[0]).unwrap() {
            HtmlNode::Element { children, .. } => assert!(children.is_empty()),
            other => panic!("unexpected node {other:?}"),
        }
        for inv in Mutation::inverse(&remove, &base) {
            apply_html_mutation(&mut after, &inv);
        }
        assert_eq!(write_html_document(&after), write_html_document(&base));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_element_name_apply_and_inverse() {
        let base = fixture();
        let mutation = HtmlMutation::SetElementName { path: vec![0, 0], name: "div".into() };
        let mut after = base.clone();
        apply_html_mutation(&mut after, &mutation);
        match node_at(&after, &[0, 0]).unwrap() {
            HtmlNode::Element { name, .. } => assert_eq!(name, "div"),
            other => panic!("unexpected node {other:?}"),
        }
        for inv in Mutation::inverse(&mutation, &base) {
            apply_html_mutation(&mut after, &inv);
        }
        assert_eq!(write_html_document(&after), write_html_document(&base));
    }

    //#region 🔖️Fixtures
    /// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field. `doctype` goes `Some(x) -> None`
    /// (tri-state `Some(None)`). `root`'s attrs (name-keyed) exercise removed+modified+added
    /// simultaneously. The naive positional `children_diff_between` can only ever show ONE of
    /// {removed-tail, added-tail} per instance, so `removed` is exercised at the top-level
    /// children triple and `added` at the nested triple inside the modified child, while that
    /// modified child's OWN diff (name+attributes+children all `Some`) is the
    /// "modified-in-every-field" collection entry.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> HtmlSnapshot {
        HtmlSnapshot {
            schema: STDIO_HTML_DOCUMENT_SCHEMA.into(),
            doctype: Some("DOCTYPE html".into()),
            root: el(
                "html",
                vec![HtmlAttr::new("keep", "k"), HtmlAttr::new("toRemove", "r"), HtmlAttr::new("toModify", "old")],
                vec![el("g", vec![HtmlAttr::new("x", "1")], vec![el("rect", vec![], vec![])]), HtmlNode::Text { text: "stay".into() }, el("toDrop", vec![], vec![])],
            ),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> HtmlSnapshot {
        HtmlSnapshot {
            schema: STDIO_HTML_DOCUMENT_SCHEMA.into(),
            doctype: None,
            root: el(
                "htmlRenamed",
                vec![HtmlAttr::new("keep", "k"), HtmlAttr::new("toModify", "new"), HtmlAttr::boolean("added")],
                vec![el("gModified", vec![HtmlAttr::new("x", "2"), HtmlAttr::new("y", "3")], vec![el("rect", vec![], vec![]), el("circle", vec![], vec![])]), HtmlNode::Text { text: "stay".into() }],
            ),
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_mutations() -> Vec<HtmlMutation> {
        vec![
            HtmlMutation::NoMutation,
            HtmlMutation::SetSnapshot { snapshot: sweep_b() },
            HtmlMutation::SetDoctype { doctype: Some("DOCTYPE html PUBLIC".into()) },
            HtmlMutation::SetDoctype { doctype: None },
            HtmlMutation::InsertNode { parent: vec![0], index: 1, node: el("span", vec![], vec![]) },
            HtmlMutation::RemoveNode { parent: vec![0], index: 0 },
            HtmlMutation::SetElementName { path: vec![0, 0], name: "div".into() },
            HtmlMutation::SetAttribute { path: vec![0, 0], name: "width".into(), value: Some(Some("99".into())) },
            HtmlMutation::SetAttribute { path: vec![0, 0], name: "width".into(), value: None },
            HtmlMutation::SetText { path: vec![0, 0, 0], text: "hi".into() },
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).unwrap();

            let mut via_apply = base.clone();
            let diff_from_apply = apply_html_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        for mutation in sample_mutations() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_html_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <HtmlMutation as Mutation<HtmlSnapshot>>::inverse(&mutation, &base) {
                apply_html_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(diff.diff(), &base).unwrap();
            let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
            let restored = MutationDiff::apply(&inverse_diff, &next).unwrap();
            assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn two_child_root(a_name: &str, b_name: &str) -> HtmlSnapshot {
        HtmlSnapshot { schema: STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype: None, root: el("html", vec![], vec![el(a_name, vec![], vec![]), el(b_name, vec![], vec![])]) }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_absorb_matches_sequential(base: &HtmlSnapshot, d1: &HtmlDiff, d2: &HtmlDiff) -> HtmlDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base).unwrap()).unwrap();
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base).unwrap(), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn root_children_diff(diff: &HtmlDiff) -> &HtmlChildrenDiff {
        match diff.root.as_ref().expect("root diff present") {
            HtmlNodeDiffT::Element(e) => e.children.as_ref().expect("children diff present"),
            other => panic!("expected element diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&HtmlMutation::InsertNode { parent: vec![], index: 2, node: el("f", vec![], vec![]) }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&HtmlMutation::RemoveNode { parent: vec![], index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert_eq!(triple.removed, vec![0]);
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].index, 1);
            let HtmlNode::Element { name, .. } = &triple.added[0].item else { panic!("expected element") };
            assert_eq!(name, "f");
        }
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&HtmlMutation::InsertNode { parent: vec![], index: 2, node: el("f", vec![], vec![]) }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&HtmlMutation::InsertNode { parent: vec![], index: 2, node: el("g", vec![], vec![]) }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
            let names: Vec<&str> = triple
                .added
                .iter()
                .map(|a| match &a.item {
                    HtmlNode::Element { name, .. } => name.as_str(),
                    _ => "",
                })
                .collect();
            assert!(names.contains(&"f"));
            assert!(names.contains(&"g"));
        }
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&HtmlMutation::InsertNode { parent: vec![], index: 1, node: el("f", vec![], vec![]) }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&HtmlMutation::SetAttribute { path: vec![1], name: "k".into(), value: Some(Some("v".into())) }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            let HtmlNode::Element { attributes, .. } = &triple.added[0].item else { panic!("expected element") };
            assert!(attributes.iter().any(|a| a.name == "k" && a.value.as_deref() == Some("v")));
        }
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&HtmlMutation::SetAttribute { path: vec![1], name: "k".into(), value: Some(Some("v".into())) }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&HtmlMutation::RemoveNode { parent: vec![], index: 1 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&HtmlMutation::InsertNode { parent: vec![], index: 2, node: el("f", vec![], vec![]) }, &base);
            let mid1 = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&HtmlMutation::InsertNode { parent: vec![], index: 2, node: el("g", vec![], vec![]) }, &mid1);
            let mid2 = MutationDiff::apply(d2.diff(), &mid1).unwrap();
            let d3 = Mutation::diff(&HtmlMutation::RemoveNode { parent: vec![], index: 0 }, &mid2);
            let sequential = MutationDiff::apply(d3.diff(), &mid2).unwrap();

            let mut left = d1.diff().clone();
            MutationDiff::absorb(&mut left, d2.diff().clone());
            MutationDiff::absorb(&mut left, d3.diff().clone());

            let mut d2_then_d3 = d2.diff().clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
            let mut right = d1.diff().clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base).unwrap(), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base).unwrap(), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&a, &b), &a).unwrap(), b);
        assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&b, &a), &b).unwrap(), a);

        let sample = fixture();
        assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&sample, &sample), &sample).unwrap(), sample);

        let real = <HtmlSnapshot as store::ArtifactDsl>::parse_dsl("<!DOCTYPE html>\n<html><body><div id=\"layer1\"><p>a</p><span>b</span></div></body></html>\n").unwrap();
        let mut mutated = real.clone();
        apply_html_mutation(&mut mutated, &HtmlMutation::SetAttribute { path: vec![0, 0], name: "id".into(), value: Some(Some("root".into())) });
        assert_ne!(real, mutated);
        assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&real, &mutated), &real).unwrap(), mutated);
        assert_eq!(MutationDiff::apply(&<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&mutated, &real), &mutated).unwrap(), real);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️FieldSweep
    #[semio_framework_async_macros::async_test]
    async fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a).unwrap(), b);
        let diff_ba = <HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b).unwrap(), a);
        assert!(<HtmlDiff as DiffAlgebra<HtmlSnapshot>>::between(&a, &a).is_empty());

        assert_eq!(diff_ab.doctype, Some(None));
        assert!(diff_ab.root.is_some());

        let HtmlNodeDiffT::Element(root_diff) = diff_ab.root.as_ref().unwrap() else { panic!("expected element diff") };
        assert!(root_diff.name.is_some());
        let attrs_diff = root_diff.attributes.as_ref().expect("attrs diff present");
        assert!(!attrs_diff.removed.is_empty(), "attrs: removed not exercised");
        assert!(!attrs_diff.modified.is_empty(), "attrs: modified not exercised");
        assert!(!attrs_diff.added.is_empty(), "attrs: added not exercised");

        let children_diff = root_diff.children.as_ref().expect("children diff present");
        assert!(!children_diff.removed.is_empty(), "children: removed not exercised");
        assert_eq!(children_diff.modified.len(), 1);
        let modified_entry = &children_diff.modified[0];
        let HtmlNodeDiffT::Element(modified_element) = &modified_entry.diff else { panic!("expected element diff") };
        assert!(modified_element.name.is_some(), "modified child: name not exercised");
        assert!(modified_element.attributes.is_some(), "modified child: attributes not exercised");
        let nested_children = modified_element.children.as_ref().expect("nested children diff present");
        let nested_added: &Vec<HtmlChildAddedT> = &nested_children.added;
        assert!(!nested_added.is_empty(), "children: added (nested) not exercised");
    }
    //#endregion 🔖️FieldSweep

    /// 🧪️ op_text_binary_roundtrip_law: round-trip laws for the hand-rolled `HtmlMutation` grammar
    /// — exercises every variant incl. `InsertNode`'s bare `HtmlNode` payload and `SetAttribute`'s
    /// tri-state value.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = fixture();
        let mutations = vec![
            HtmlMutation::NoMutation,
            HtmlMutation::SetSnapshot { snapshot: base.clone() },
            HtmlMutation::SetDoctype { doctype: Some("DOCTYPE html".into()) },
            HtmlMutation::SetDoctype { doctype: None },
            HtmlMutation::InsertNode { parent: vec![0], index: 1, node: el("span", vec![HtmlAttr::new("r", "1")], vec![]) },
            HtmlMutation::RemoveNode { parent: vec![0], index: 2 },
            HtmlMutation::SetElementName { path: vec![0], name: "g".into() },
            HtmlMutation::SetAttribute { path: vec![0], name: "width".into(), value: Some(Some("99".into())) },
            HtmlMutation::SetAttribute { path: vec![0], name: "width".into(), value: Some(None) },
            HtmlMutation::SetAttribute { path: vec![0], name: "width".into(), value: None },
            HtmlMutation::SetText { path: vec![0, 1], text: "hello world".into() },
            HtmlMutation::SetComment { path: vec![0, 1], text: " comment ".into() },
            HtmlMutation::SetRawText { path: vec![0, 1], text: "console.log(1);".into() },
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = HtmlMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = HtmlMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }

    //#region kinds_law
    /// 🎯️ kinds_const_matches_enum_variants_in_declaration_order: `KINDS` is the ONLY thing the
    /// wave 7 catalog contract (`../🧪️oracle/🔣️.json`'s `kinds` array) is checked against,
    /// so it must genuinely enumerate every variant, in order, with the exact `OpText` keyword the
    /// case's adapter registers `mutate-<kind>`/`inverse-<kind>` scenario ids under.
    #[semio_framework_async_macros::async_test]
    async fn kinds_const_matches_enum_variants_in_declaration_order() {
        let base = fixture();
        let one_per_variant = vec![
            HtmlMutation::NoMutation,
            HtmlMutation::SetSnapshot { snapshot: base.clone() },
            HtmlMutation::SetDoctype { doctype: Some("DOCTYPE html".into()) },
            HtmlMutation::InsertNode { parent: vec![0], index: 0, node: el("span", vec![], vec![]) },
            HtmlMutation::RemoveNode { parent: vec![0], index: 0 },
            HtmlMutation::SetElementName { path: vec![0], name: "div".into() },
            HtmlMutation::SetAttribute { path: vec![0], name: "id".into(), value: Some(Some("x".into())) },
            HtmlMutation::SetText { path: vec![0, 0], text: "x".into() },
            HtmlMutation::SetComment { path: vec![0, 0], text: "x".into() },
            HtmlMutation::SetRawText { path: vec![0, 0], text: "x".into() },
        ];
        assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            let printed = mutation.print_op();
            let keyword = printed.split(' ').next().unwrap_or(&printed);
            assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
        }
    }
    //#endregion kinds_law
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/declares-the-document-language-on-the-root-html-element/🦀️component.rs"]
    mod tests_set_snapshot_declares_the_document_language_on_the_root_html_element;
}
//#endregion 🧪️FixtureTests
