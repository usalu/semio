//! 🔺️ XmlDiff — handcrafted recursive tree diff. `declaration`/`doctype` are tri-state top-level
//! scalars (`Some(None)` = cleared); `root` nests the recursive `XmlNodeDiff` tree, itself shaped
//! like the `XmlNode` it targets (`XmlNode::Element` <-> `XmlElementDiff`, `XmlNode::Text` <->
//! `Text{text}`, everything else -- CData/Comment/ProcessingInstruction, plus any node-KIND change
//! -- via the `Replace` fallback). Origin of the xml/svg node-diff pattern (`.🧬semio/🦑️repo/🎫️tickets/
//! 🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/🧬️schema-design.md`):
//! svg's own diff types build on this shape but are declared separately in svg's own facet dir.
//!
//! 🧪️ F6 CONFIRMED (real `cargo check -p semio-s-plugin-stdio --lib`, not guessed): adding
//! `#[derive(dsl::DslDiff)]` to `XmlDiff` fails with `error[E0277]: the trait bound
//! ...::XmlNodeDiff: DslField is not satisfied` (root: Option<XmlNodeDiff> — `XmlNodeDiff` is a
//! genuine data-carrying enum, and `DslField` has no impl for it, only `DslRecord`-derived structs
//! and `DslScalar`-derived UNIT-only enums implement `DslField`). A second, independent blocker is
//! also present even without the enum: `declaration`/`doctype` are tri-state `Option<Option<T>>`
//! fields — same blocker as `GifFrameDiff`/`SvgDiff` (`classify_field` peels exactly one `Option`
//! layer, and no `impl<T: DslField> DslField for Option<T>` exists). `DiffCodec` is hand-rolled
//! below, adapting svg's own hand-rolled `enc_xml_node`/`dec_xml_node` primitives (svg embeds this
//! same `XmlNode` type, so its encoding logic applies verbatim -- svg's copies stay `pub(crate)` to
//! svg, not importable from here across the artifact boundary, so this file declares its own copies
//! for `📰️xml`'s own crate-visibility scope).

use crate::standards::v1_0::subsets::base::io::binary::diff::dec_children_diff_bin;
use crate::standards::v1_0::subsets::base::io::binary::diff::enc_children_diff_bin;
use crate::schema::snapshot::{validate_xml_document_boundaries, XmlAttr, XmlDeclaration, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote};
use crate::XmlSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::deserialize_double_option;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.xml`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml.diff")]
pub struct XmlDiff {
    /// 🧭 Logical document-prolog nodes.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub prolog: Option<Vec<XmlNode>>,
    /// 🧹 Logical document-epilog nodes.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub epilog: Option<Vec<XmlNode>>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = declaration removed, `Some(Some(d))` = set.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub declaration: Option<Option<XmlDeclaration>>,
    /// 📜️ Tri-state: `None` = unchanged, `Some(None)` = doctype removed, `Some(Some(s))` = set.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub doctype: Option<Option<XmlDoctype>>,
    /// 🌳 `None` = root subtree unchanged; `Some(diff)` = the root changed (recursive, possibly
    /// down to a deeply nested leaf via `diff_at_path`, or a wholesale `Replace` incl. root
    /// presence/absence itself).
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<XmlNodeDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️NodeDiff
/// 🌳 Recursive per-node diff, shaped like the `XmlNode` it targets.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum XmlNodeDiff {
    Element(XmlElementDiff),
    Text {
        #[value(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    /// 🔁 Wholesale node replace -- used when the node's KIND changes (e.g. `Text` -> `Element`,
    /// or either endpoint is `CData`/`Comment`/`ProcessingInstruction`, none of which get their
    /// own structural diff shape since real documents mutate their *content* far more often than
    /// they flip a text node into a comment) and, uniquely at the document ROOT, to express root
    /// presence/absence itself (`node: None` = root removed; deliberately `Option<XmlNode>`
    /// rather than a bare `XmlNode` for exactly this reason -- children never need `None` here
    /// since removing a child goes through the owning `XmlChildrenDiff.removed`, not `Replace`).
    Replace {
        node: Option<XmlNode>,
    },
}

/// 🏷️ Per-element diff.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XmlElementDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<XmlAttributesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<XmlChildrenDiff>,
}

/// 🏷️ Name-keyed, ORDER-preserving attribute triple. Deliberately a Vec-based triple (not a
/// `HashMap`) -- XML attribute order is not semantically meaningful per the spec but IS
/// significant for byte-preserving round-trips; `order` is the sole final placement authority.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XmlAttributesDiff {
    /// 🧭 Complete final attribute identity order for every nonempty edit.
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<XmlAttrModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<XmlAttrAdded>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XmlAttrModified {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XmlAttrAdded {
    pub name: String,
    pub value: String,
}

/// 🌳 Index-keyed, recursive children triple. `removed`/`modified` indices refer to BASE state
/// (descending removal order on apply); `added` indices refer to FINAL state (ascending insert).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XmlChildrenDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<XmlChildModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<XmlChildAdded>,
}

impl XmlChildrenDiff {
    /// 🌱️ Diffs an embedded ordered XML fragment using the XML artifact's recursive algebra.
    pub fn between(base: &[XmlNode], other: &[XmlNode]) -> Option<Self> {
        between_children(base, other)
    }

    /// 🧩️ Validates all addressed children before changing an embedded XML fragment.
    pub fn apply_to(&self, children: &mut Vec<XmlNode>) -> MutationApplyResult<()> {
        validate_xml_children(children, self)?;
        *children = apply_children_diff(children, self);
        Ok(())
    }

    /// ↩️ Restores the exact previous fragment after this diff is applied.
    pub fn inverse(&self, base: &[XmlNode]) -> Self {
        inverse_children_diff(base, self)
    }

    /// 🔗️ Composes the next fragment edit into this edit.
    pub fn absorb(&mut self, next: Self) {
        *self = absorb_children_diff(std::mem::take(self), &next);
    }

    /// 📝️ Encodes a fragment diff using the native XML text grammar.
    pub fn encode_text(&self) -> String {
        enc_children_diff(self)
    }

    /// 📖️ Decodes a fragment diff using the native XML text grammar.
    pub fn decode_text(text: &str) -> Result<Self, String> {
        dec_children_diff(text)
    }

    /// 📦️ Appends a fragment diff using the native XML binary grammar.
    pub fn encode_binary(&self, output: &mut Vec<u8>) {
        enc_children_diff_bin(self, output);
    }

    /// 🔓️ Reads a fragment diff from a caller-owned binary frame.
    pub fn decode_binary(reader: &mut store::ByteReader<'_>) -> Result<Self, String> {
        dec_children_diff_bin(reader)
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XmlChildModified {
    pub index: usize,
    pub diff: XmlNodeDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XmlChildAdded {
    pub index: usize,
    pub item: XmlNode,
}
//#endregion 🔖️NodeDiff

//#region 🔖️DiffAtPath
/// 🧭️ Lowers a `leaf` diff targeting the node addressed by `path` (a chain of child indices from
/// the document root -- mirrors `crate::schema::mutations::XmlNodePath`, kept as a
/// bare `&[usize]` here so this module never needs to depend on the mutations module) into a full
/// `XmlDiff` by nesting it through `XmlChildModified` entries from the root down to that depth.
/// `path == []` addresses the root itself, so `leaf` becomes `XmlDiff.root` directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_at_path(path: &[usize], leaf: XmlNodeDiff) -> XmlDiff {
    let mut node_diff = leaf;
    for &index in path.iter().rev() {
        node_diff = XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: Vec::new(), modified: vec![XmlChildModified { index, diff: node_diff }], added: Vec::new() }) });
    }
    XmlDiff { prolog: None, epilog: None, declaration: None, doctype: None, root: Some(node_diff) }
}
//#endregion 🔖️DiffAtPath

//#region 🔖️Apply
impl MutationDiff<XmlSnapshot> for XmlDiff {
    fn apply(&self, base: &XmlSnapshot) -> MutationApplyResult<XmlSnapshot> {
        if let Some(root) = &self.root {
            validate_xml_node(base.doc.root.as_ref(), root)?;
        }
        let mut next = base.clone();
        if let Some(prolog) = &self.prolog {
            next.doc.prolog = prolog.clone();
        }
        if let Some(epilog) = &self.epilog {
            next.doc.epilog = epilog.clone();
        }
        if let Some(declaration) = &self.declaration {
            next.doc.declaration = declaration.clone();
        }
        if let Some(doctype) = &self.doctype {
            next.doc.doctype = doctype.clone();
        }
        if let Some(node_diff) = &self.root {
            next.doc.root = apply_root_diff(next.doc.root.as_ref(), node_diff);
        }
        validate_xml_document_boundaries(&next.doc).map_err(|detail| MutationApplyError::new("mutation.apply.invalid-document-boundary", detail))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.prolog.is_some() {
            self.prolog = other.prolog;
        }
        if other.epilog.is_some() {
            self.epilog = other.epilog;
        }
        if other.declaration.is_some() {
            self.declaration = other.declaration;
        }
        if other.doctype.is_some() {
            self.doctype = other.doctype;
        }
        self.root = match (self.root.take(), other.root) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_node_diff(a, b)),
        };
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_xml_node(current: Option<&XmlNode>, diff: &XmlNodeDiff) -> MutationApplyResult<()> {
    match diff {
        XmlNodeDiff::Replace { .. } => Ok(()),
        XmlNodeDiff::Text { .. } => match current {
            Some(XmlNode::Text { .. }) => Ok(()),
            Some(_) => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "text diff targets a non-text node")),
            None => Err(MutationApplyError::new("mutation.apply.missing-target", "text diff targets a missing root")),
        },
        XmlNodeDiff::Element(element) => match current {
            Some(XmlNode::Element { attrs, children, .. }) => {
                if let Some(attributes) = &element.attributes {
                    validate_xml_attrs(attrs, attributes)?;
                }
                if let Some(children_diff) = &element.children {
                    validate_xml_children(children, children_diff)?;
                }
                Ok(())
            }
            Some(_) => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "element diff targets a non-element node")),
            None => Err(MutationApplyError::new("mutation.apply.missing-target", "element diff targets a missing root")),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_xml_attrs(base: &[XmlAttr], diff: &XmlAttributesDiff) -> MutationApplyResult<()> {
    if diff.removed.is_empty() && diff.modified.is_empty() && diff.added.is_empty() && diff.order.is_empty() {
        return Ok(());
    }
    for (position, name) in diff.removed.iter().enumerate() {
        if !base.iter().any(|attr| attr.name == *name) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "attribute removal target does not exist"));
        }
        if diff.removed[..position].contains(name) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "attribute removal target is repeated"));
        }
    }
    for (position, modified) in diff.modified.iter().enumerate() {
        if !base.iter().any(|attr| attr.name == modified.name) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "attribute modification target does not exist"));
        }
        if diff.removed.contains(&modified.name) || diff.modified[..position].iter().any(|candidate| candidate.name == modified.name) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "attribute modification target conflicts or repeats"));
        }
    }
    let final_len = base.len() - diff.removed.len() + diff.added.len();
    for (position, added) in diff.added.iter().enumerate() {
        if (base.iter().any(|attr| attr.name == added.name) && !diff.removed.contains(&added.name)) || diff.added[..position].iter().any(|candidate| candidate.name == added.name) || diff.modified.iter().any(|candidate| candidate.name == added.name) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "attribute addition target is invalid or conflicting"));
        }
    }
    let expected: std::collections::HashSet<&str> = base.iter().filter(|attr| !diff.removed.contains(&attr.name)).map(|attr| attr.name.as_str()).chain(diff.added.iter().map(|attr| attr.name.as_str())).collect();
    let ordered: std::collections::HashSet<&str> = diff.order.iter().map(String::as_str).collect();
    if diff.order.len() != final_len || ordered.len() != final_len || ordered != expected {
        return Err(MutationApplyError::new("mutation.apply.invalid-order", "attribute order must name every final identity exactly once"));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_xml_children(base: &[XmlNode], diff: &XmlChildrenDiff) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "child removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "child removal target is repeated"));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "child modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "child modification targets a removed node"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "child modification target is repeated"));
        }
        if matches!(&entry.diff, XmlNodeDiff::Replace { node: None }) {
            return Err(MutationApplyError::new("mutation.apply.invalid-replacement", "child replacement requires a node; removal uses the removed collection"));
        }
        validate_xml_node(Some(&base[entry.index]), &entry.diff).map_err(|error| error.under(vec!["modified".to_string(), entry.index.to_string()]))?;
    }
    let final_len = base.len() - removed.len() + diff.added.len();
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index >= final_len || !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "child addition position is invalid or repeated"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_root_diff(current: Option<&XmlNode>, diff: &XmlNodeDiff) -> Option<XmlNode> {
    match diff {
        XmlNodeDiff::Replace { node } => node.clone(),
        _ => current.map(|n| apply_node_diff(n, diff)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_node_diff(node: &XmlNode, diff: &XmlNodeDiff) -> XmlNode {
    match diff {
        XmlNodeDiff::Replace { node: replacement } => replacement.clone().unwrap_or_else(|| node.clone()),
        XmlNodeDiff::Text { text } => match node {
            XmlNode::Text { text: current } => XmlNode::Text { text: text.clone().unwrap_or_else(|| current.clone()) },
            other => other.clone(),
        },
        XmlNodeDiff::Element(element_diff) => match node {
            XmlNode::Element { name, attrs, children } => XmlNode::Element {
                name: element_diff.name.clone().unwrap_or_else(|| name.clone()),
                attrs: match &element_diff.attributes {
                    Some(attrs_diff) => apply_attrs_diff(attrs, attrs_diff),
                    None => attrs.clone(),
                },
                children: match &element_diff.children {
                    Some(children_diff) => apply_children_diff(children, children_diff),
                    None => children.clone(),
                },
            },
            other => other.clone(),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_attrs_diff(attrs: &[XmlAttr], diff: &XmlAttributesDiff) -> Vec<XmlAttr> {
    if diff.removed.is_empty() && diff.modified.is_empty() && diff.added.is_empty() && diff.order.is_empty() {
        return attrs.to_vec();
    }
    let mut values: std::collections::HashMap<String, String> = attrs.iter().filter(|attr| !diff.removed.contains(&attr.name)).map(|attr| (attr.name.clone(), attr.value.clone())).collect();
    for entry in &diff.modified {
        values.insert(entry.name.clone(), entry.value.clone());
    }
    for entry in &diff.added {
        values.insert(entry.name.clone(), entry.value.clone());
    }
    diff.order.iter().map(|name| XmlAttr { name: name.clone(), value: values.remove(name).expect("validated attribute identity order") }).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_children_diff(children: &[XmlNode], diff: &XmlChildrenDiff) -> Vec<XmlNode> {
    let mut slots: Vec<Option<XmlNode>> = children.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(node)) = slots.get(m.index) {
            let patched = apply_node_diff(node, &m.diff);
            slots[m.index] = Some(patched);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    removed_sorted.dedup();
    for idx in removed_sorted {
        if idx < slots.len() {
            slots.remove(idx);
        }
    }
    let mut out: Vec<XmlNode> = slots.into_iter().flatten().collect();
    let mut additions: Vec<&XmlChildAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, add.item.clone());
    }
    out
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<XmlSnapshot> for XmlDiff {
    fn inverse(&self, base: &XmlSnapshot) -> Self {
        XmlDiff {
            prolog: self.prolog.as_ref().map(|_| base.doc.prolog.clone()),
            epilog: self.epilog.as_ref().map(|_| base.doc.epilog.clone()),
            declaration: self.declaration.as_ref().map(|_| base.doc.declaration.clone()),
            doctype: self.doctype.as_ref().map(|_| base.doc.doctype.clone()),
            root: self.root.as_ref().map(|d| inverse_node_diff(base.doc.root.as_ref(), d)),
        }
    }

    fn between(base: &XmlSnapshot, other: &XmlSnapshot) -> Self {
        XmlDiff {
            prolog: if base.doc.prolog != other.doc.prolog { Some(other.doc.prolog.clone()) } else { None },
            epilog: if base.doc.epilog != other.doc.epilog { Some(other.doc.epilog.clone()) } else { None },
            declaration: if base.doc.declaration != other.doc.declaration { Some(other.doc.declaration.clone()) } else { None },
            doctype: if base.doc.doctype != other.doc.doctype { Some(other.doc.doctype.clone()) } else { None },
            root: between_root(base.doc.root.as_ref(), other.doc.root.as_ref()),
        }
    }

    fn is_empty(&self) -> bool {
        self.prolog.is_none() && self.epilog.is_none() && self.declaration.is_none() && self.doctype.is_none() && self.root.is_none()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_node_diff(current: Option<&XmlNode>, diff: &XmlNodeDiff) -> XmlNodeDiff {
    match diff {
        XmlNodeDiff::Replace { .. } => XmlNodeDiff::Replace { node: current.cloned() },
        XmlNodeDiff::Text { .. } => match current {
            Some(XmlNode::Text { text }) => XmlNodeDiff::Text { text: Some(text.clone()) },
            Some(other) => XmlNodeDiff::Replace { node: Some(other.clone()) },
            None => XmlNodeDiff::Replace { node: None },
        },
        XmlNodeDiff::Element(element_diff) => match current {
            Some(XmlNode::Element { name, attrs, children }) => XmlNodeDiff::Element(XmlElementDiff {
                name: element_diff.name.as_ref().map(|_| name.clone()),
                attributes: element_diff.attributes.as_ref().map(|ad| inverse_attrs_diff(attrs, ad)),
                children: element_diff.children.as_ref().map(|cd| inverse_children_diff(children, cd)),
            }),
            Some(other) => XmlNodeDiff::Replace { node: Some(other.clone()) },
            None => XmlNodeDiff::Replace { node: None },
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_attrs_diff(base_attrs: &[XmlAttr], diff: &XmlAttributesDiff) -> XmlAttributesDiff {
    let removed: Vec<String> = diff.added.iter().map(|a| a.name.clone()).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_attrs.iter().find(|a| a.name == m.name) {
            modified.push(XmlAttrModified { name: original.name.clone(), value: original.value.clone() });
        }
    }
    let mut added = Vec::new();
    for name in &diff.removed {
        if let Some(idx) = base_attrs.iter().position(|a| &a.name == name) {
            let original = &base_attrs[idx];
            added.push(XmlAttrAdded { name: original.name.clone(), value: original.value.clone() });
        }
    }
    XmlAttributesDiff { removed, modified, added, order: base_attrs.iter().map(|attr| attr.name.clone()).collect() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_children_diff(base_children: &[XmlNode], diff: &XmlChildrenDiff) -> XmlChildrenDiff {
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_children.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &diff.added);
            modified.push(XmlChildModified { index: next_index, diff: inverse_node_diff(Some(original), &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_children.get(idx) {
            added.push(XmlChildAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    XmlChildrenDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_root(base: Option<&XmlNode>, other: Option<&XmlNode>) -> Option<XmlNodeDiff> {
    match (base, other) {
        (None, None) => None,
        (None, Some(n)) => Some(XmlNodeDiff::Replace { node: Some(n.clone()) }),
        (Some(_), None) => Some(XmlNodeDiff::Replace { node: None }),
        (Some(b), Some(o)) => between_node(b, o),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_node(base: &XmlNode, other: &XmlNode) -> Option<XmlNodeDiff> {
    if base == other {
        return None;
    }
    match (base, other) {
        (XmlNode::Text { .. }, XmlNode::Text { text: ot }) => Some(XmlNodeDiff::Text { text: Some(ot.clone()) }),
        (XmlNode::Element { name: bn, attrs: ba, children: bc }, XmlNode::Element { name: on, attrs: oa, children: oc }) => {
            let name = if bn != on { Some(on.clone()) } else { None };
            let attributes = between_attrs(ba, oa);
            let children = between_children(bc, oc);
            if name.is_none() && attributes.is_none() && children.is_none() {
                None
            } else {
                Some(XmlNodeDiff::Element(XmlElementDiff { name, attributes, children }))
            }
        }
        _ => Some(XmlNodeDiff::Replace { node: Some(other.clone()) }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_attrs(base: &[XmlAttr], other: &[XmlAttr]) -> Option<XmlAttributesDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        match other.iter().find(|o| o.name == b.name) {
            Some(o) if o.value != b.value => modified.push(XmlAttrModified { name: b.name.clone(), value: o.value.clone() }),
            Some(_) => {}
            None => removed.push(b.name.clone()),
        }
    }
    let mut added = Vec::new();
    for o in other {
        if !base.iter().any(|b| b.name == o.name) {
            added.push(XmlAttrAdded { name: o.name.clone(), value: o.value.clone() });
        }
    }
    if base == other {
        None
    } else {
        Some(XmlAttributesDiff { removed, modified, added, order: other.iter().map(|attr| attr.name.clone()).collect() })
    }
}

/// 🧮️ Naive positional child diff per the recipe's "between matching" rule for index-keyed
/// collections: pairwise-compare `0..min(base.len(), other.len())` as `modified`, the base tail
/// as `removed`, the other tail as `added`. Not an LCS-based diff (no move/reorder detection) --
/// deliberately simple, matching every other stdio artifact's `between` for index-keyed children.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_children(base: &[XmlNode], other: &[XmlNode]) -> Option<XmlChildrenDiff> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = between_node(&base[i], &other[i]) {
                modified.push(XmlChildModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<XmlChildAdded> = (min_len..other.len()).map(|i| XmlChildAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() {
        None
    } else {
        Some(XmlChildrenDiff { removed, modified, added })
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️Absorb
/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (base-free index-transport
/// over `d1`'s removed/added): `transform_index` maps a base-side index through `d1`'s own
/// removed/added to the position it ends up at once `d1` has been applied -- used both to
/// translate `d2`'s references into `d1`'s vocabulary and, in `inverse_children_diff` above, to
/// translate a `modified` entry's base index into its position in the post-apply state.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transform_index(idx: usize, removed: &[usize], added: &[XmlChildAdded]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order: Vec<usize> = added.iter().map(|a| a.index).collect();
    order.sort_unstable();
    let mut shift = 0usize;
    for target in order {
        if target <= pos + shift {
            shift += 1;
        } else {
            break;
        }
    }
    pos + shift
}

/// 🏷️ Which base position (survivor) or which `d1.added` slot a mid-array position originated
/// from -- built by `simulate_mid_origins` so `absorb_children_diff` can classify `d2`'s indices.
enum ChildOrigin {
    Base(usize),
    Added(usize),
}

/// 🧱️ Materializes a synthetic mid-array (base -> after `d1`) large enough to answer every index
/// `d1`/`d2` actually reference. Absorb is base-free (no real snapshot access), so `base_len` is
/// the SMALLEST synthetic length that avoids clamping any referenced position -- untouched
/// synthetic base slots beyond what either diff references are pure placeholders whose exact
/// identity never gets read.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_mid_origins(base_len: usize, removed: &[usize], added: &[XmlChildAdded]) -> Vec<ChildOrigin> {
    let mut mid: Vec<ChildOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ChildOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ChildOrigin::Added(k));
    }
    mid
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_node_diff(a: XmlNodeDiff, b: XmlNodeDiff) -> XmlNodeDiff {
    match (a, b) {
        (_, XmlNodeDiff::Replace { node: Some(n) }) => XmlNodeDiff::Replace { node: Some(n) },
        (XmlNodeDiff::Replace { node: Some(n) }, b) => XmlNodeDiff::Replace { node: Some(apply_node_diff(&n, &b)) },
        (_, XmlNodeDiff::Replace { node: None }) => XmlNodeDiff::Replace { node: None },
        (XmlNodeDiff::Replace { node: None }, _) => XmlNodeDiff::Replace { node: None },
        (XmlNodeDiff::Text { text: ta }, XmlNodeDiff::Text { text: tb }) => XmlNodeDiff::Text { text: tb.or(ta) },
        (XmlNodeDiff::Element(ea), XmlNodeDiff::Element(eb)) => XmlNodeDiff::Element(absorb_element_diff(ea, eb)),
        (_, b) => b,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_element_diff(mut a: XmlElementDiff, b: XmlElementDiff) -> XmlElementDiff {
    if b.name.is_some() {
        a.name = b.name;
    }
    a.attributes = match (a.attributes.take(), b.attributes) {
        (None, x) => x,
        (x, None) => x,
        (Some(ad), Some(bd)) => Some(absorb_attrs_diff(ad, &bd)),
    };
    a.children = match (a.children.take(), b.children) {
        (None, x) => x,
        (x, None) => x,
        (Some(ad), Some(bd)) => Some(absorb_children_diff(ad, &bd)),
    };
    a
}

/// 🔗️ Composes attribute identity changes and retains the last complete final order.
fn absorb_attrs_diff(mut a: XmlAttributesDiff, b: &XmlAttributesDiff) -> XmlAttributesDiff {
    if b.removed.is_empty() && b.modified.is_empty() && b.added.is_empty() && b.order.is_empty() {
        return a;
    }
    let a_added_names: std::collections::HashSet<String> = a.added.iter().map(|x| x.name.clone()).collect();
    let mut removed = a.removed.clone();
    let mut annihilated: Vec<String> = Vec::new();
    for name in &b.removed {
        if a_added_names.contains(name) {
            annihilated.push(name.clone());
        } else if !removed.contains(name) {
            removed.push(name.clone());
        }
    }
    a.added.retain(|x| !annihilated.contains(&x.name));
    let mut modified: Vec<XmlAttrModified> = a.modified.into_iter().filter(|m| !removed.contains(&m.name)).collect();
    for bm in &b.modified {
        if let Some(added) = a.added.iter_mut().find(|x| x.name == bm.name) {
            added.value = bm.value.clone();
            continue;
        }
        if removed.contains(&bm.name) {
            continue;
        }
        match modified.iter_mut().find(|m| m.name == bm.name) {
            Some(existing) => existing.value = bm.value.clone(),
            None => modified.push(bm.clone()),
        }
    }
    let mut added = a.added;
    for ba in &b.added {
        match added.iter_mut().find(|x| x.name == ba.name) {
            Some(existing) => {
                existing.value = ba.value.clone();
            }
            None => added.push(ba.clone()),
        }
    }
    XmlAttributesDiff { removed, modified, added, order: b.order.clone() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_children_diff(d1: XmlChildrenDiff, d2: &XmlChildrenDiff) -> XmlChildrenDiff {
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map_or(0, |m| m + 1);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map_or(0, |m| m + 1);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified.clone();
    let mut working_added = d1.added;
    let mut annihilated: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(ChildOrigin::Base(bi)) => {
                if !removed.contains(bi) {
                    removed.push(*bi);
                }
                modified.retain(|m| &m.index != bi);
            }
            Some(ChildOrigin::Added(k)) => {
                annihilated.insert(*k);
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(ChildOrigin::Base(bi)) => {
                if removed.contains(bi) {
                    continue;
                }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_node_diff(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(XmlChildModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(ChildOrigin::Added(k)) => {
                if annihilated.contains(k) {
                    continue;
                }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_node_diff(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) {
            continue;
        }
        let final_index = transform_index(add.index, &d2.removed, &d2.added);
        added.push(XmlChildAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    XmlChildrenDiff { removed, modified, added }
}
//#endregion 🔖️Absorb

//#region 🔖️SetSnapshot
/// 🧩️ Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<XmlSnapshot>` full-replace slot -- this IS `XmlDiff::between`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &XmlSnapshot, next: &XmlSnapshot) -> XmlDiff {
    XmlDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` for `XmlDiff` — adapted from `🎨️svg`'s own hand-rolled
/// `SvgDiff` codec (F6-PILOT), which itself builds on this file's `XmlNodeDiff` shape. Same grammar
/// style `GifDiff`/`SvgDiff` use (bracket-depth-aware split, hex for strings/bytes, `[0]`/`[1,x]`
/// for `Option<T>`). Primitives duplicated here (not imported from svg — different artifact, svg's
/// copies are `pub(crate)` to svg's own crate-visibility scope, not reachable from `📰️xml`) but
/// marked `pub(crate)` in THIS file so `📰️xml`'s own `🧬️mutations/component.rs` can reuse them for
/// its hand-rolled `OpText`/`OpBinary` (same intra-artifact reuse pattern svg uses).
//#region 🔖️Primitives















//#region 🔖️BinaryPrimitives




//#endregion 🔖️BinaryPrimitives
//#endregion 🔖️Primitives

//#region 🔖️XmlValueCodecs















//#region 🔖️XmlValueBinaryCodecs






//#endregion 🔖️XmlValueBinaryCodecs
//#endregion 🔖️XmlValueCodecs

//#region 🔖️DiffValueCodecs








//#region 🔖️DiffValueBinaryCodecs








//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `XmlDiff` values (both top-level tri-states, the recursive
/// `Element`/`Text`/`Replace` `XmlNodeDiff` tree, attribute add/remove/modify, nested child
/// add/remove/modify) — the single prolog of truth reused by `diff_codec_text_binary_roundtrip_law`
/// below AND by `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<XmlDiff> {
    use crate::schema::snapshot::XmlDocument;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn elem(name: &str, attrs: Vec<(&str, &str)>, children: Vec<XmlNode>) -> XmlNode {
        XmlNode::Element { name: name.to_string(), attrs: attrs.into_iter().map(|(n, v)| XmlAttr { name: n.to_string(), value: v.to_string() }).collect(), children }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot(doc: XmlDocument) -> XmlSnapshot {
        XmlSnapshot { doc, ..Default::default() }
    }

    let a = snapshot(XmlDocument {
        root: Some(elem("root", vec![("width", "10")], vec![elem("child", vec![("x", "0")], vec![])])),
        doctype: Some("<!DOCTYPE root>".into()),
        declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true), quote: XmlQuote::Single }),
        prolog: Vec::new(),
        epilog: Vec::new(),
    });
    let b = snapshot(XmlDocument { root: Some(elem("root", vec![("width", "20"), ("height", "30")], vec![elem("other", vec![("r", "5")], vec![]), XmlNode::Text { text: "hi".into() }])), doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() });
    let c = snapshot(XmlDocument { root: None, doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() });

    vec![XmlDiff::default(), XmlDiff::between(&a, &b), XmlDiff::between(&b, &a), XmlDiff::between(&a, &c), XmlDiff::between(&c, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
