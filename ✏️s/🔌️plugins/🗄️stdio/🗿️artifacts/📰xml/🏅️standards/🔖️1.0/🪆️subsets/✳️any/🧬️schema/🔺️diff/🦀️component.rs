//! 🔺️ XmlDiff — handcrafted recursive tree diff. `declaration`/`doctype` are tri-state top-level
//! scalars (`Some(None)` = cleared); `root` nests the recursive `XmlNodeDiff` tree, itself shaped
//! like the `XmlNode` it targets (`XmlNode::Element` <-> `XmlElementDiff`, `XmlNode::Text` <->
//! `Text{text}`, everything else -- CData/Comment/ProcessingInstruction, plus any node-KIND change
//! -- via the `Replace` fallback). Origin of the xml/svg node-diff pattern (`.🦑️repo/🎫️tickets/
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
//! for `📰xml`'s own crate-visibility scope).

use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode};
use crate::artifacts::xml::XmlSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.xml`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml.diff")]
pub struct XmlDiff {
    /// 🧭 Logical document-prolog nodes.
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prolog: Option<Vec<XmlNode>>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = declaration removed, `Some(Some(d))` = set.
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declaration: Option<Option<XmlDeclaration>>,
    /// 📜️ Tri-state: `None` = unchanged, `Some(None)` = doctype removed, `Some(Some(s))` = set.
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<Option<XmlDoctype>>,
    /// 🌳 `None` = root subtree unchanged; `Some(diff)` = the root changed (recursive, possibly
    /// down to a deeply nested leaf via `diff_at_path`, or a wholesale `Replace` incl. root
    /// presence/absence itself).
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<XmlNodeDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️NodeDiff
/// 🌳 Recursive per-node diff, shaped like the `XmlNode` it targets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum XmlNodeDiff {
    Element(XmlElementDiff),
    Text {
        #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlElementDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<XmlAttributesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<XmlChildrenDiff>,
}

/// 🏷️ Name-keyed, ORDER-preserving attribute triple. Deliberately a Vec-based triple (not a
/// `HashMap`) -- XML attribute order is not semantically meaningful per the spec but IS
/// significant for byte-preserving round-trips, so it must survive the diff/apply cycle.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlAttributesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<XmlAttrModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<XmlAttrAdded>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlAttrModified {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlAttrAdded {
    pub index: usize,
    pub name: String,
    pub value: String,
}

/// 🌳 Index-keyed, recursive children triple. `removed`/`modified` indices refer to BASE state
/// (descending removal order on apply); `added` indices refer to FINAL state (ascending insert).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlChildrenDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<XmlChildModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<XmlChildAdded>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlChildModified {
    pub index: usize,
    pub diff: XmlNodeDiff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlChildAdded {
    pub index: usize,
    pub item: XmlNode,
}
//#endregion 🔖️NodeDiff

//#region 🔖️DiffAtPath
/// 🧭️ Lowers a `leaf` diff targeting the node addressed by `path` (a chain of child indices from
/// the document root -- mirrors `crate::artifacts::xml::schema::mutations::XmlNodePath`, kept as a
/// bare `&[usize]` here so this module never needs to depend on the mutations module) into a full
/// `XmlDiff` by nesting it through `XmlChildModified` entries from the root down to that depth.
/// `path == []` addresses the root itself, so `leaf` becomes `XmlDiff.root` directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_at_path(path: &[usize], leaf: XmlNodeDiff) -> XmlDiff {
    let mut node_diff = leaf;
    for &index in path.iter().rev() {
        node_diff = XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: Vec::new(), modified: vec![XmlChildModified { index, diff: node_diff }], added: Vec::new() }) });
    }
    XmlDiff { prolog: None, declaration: None, doctype: None, root: Some(node_diff) }
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
        if let Some(declaration) = &self.declaration {
            next.doc.declaration = declaration.clone();
        }
        if let Some(doctype) = &self.doctype {
            next.doc.doctype = doctype.clone();
        }
        if let Some(node_diff) = &self.root {
            next.doc.root = apply_root_diff(next.doc.root.as_ref(), node_diff);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.prolog.is_some() {
            self.prolog = other.prolog;
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
        if added.index > final_len
            || diff.added[..position].iter().any(|candidate| candidate.index == added.index)
            || base.iter().any(|attr| attr.name == added.name)
            || diff.removed.contains(&added.name)
            || diff.modified.iter().any(|candidate| candidate.name == added.name)
        {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "attribute addition target is invalid or conflicting"));
        }
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
        validate_xml_node(Some(&base[entry.index]), &entry.diff).map_err(|error| error.under(vec!["modified".to_string(), entry.index.to_string()]))?;
    }
    let final_len = base.len() - removed.len() + diff.added.len();
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !added.insert(entry.index) {
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
                    Some(children_diff) => Box::pin(apply_children_diff(children, children_diff)),
                    None => children.clone(),
                },
            },
            other => other.clone(),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_attrs_diff(attrs: &[XmlAttr], diff: &XmlAttributesDiff) -> Vec<XmlAttr> {
    let mut out: Vec<XmlAttr> = attrs
        .iter()
        .filter(|a| !diff.removed.contains(&a.name))
        .map(|a| match diff.modified.iter().find(|m| m.name == a.name) {
            Some(m) => XmlAttr { name: a.name.clone(), value: m.value.clone() },
            None => a.clone(),
        })
        .collect();
    let mut additions: Vec<&XmlAttrAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, XmlAttr { name: add.name.clone(), value: add.value.clone() });
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_children_diff(children: &[XmlNode], diff: &XmlChildrenDiff) -> Vec<XmlNode> {
    let mut slots: Vec<Option<XmlNode>> = children.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(node)) = slots.get(m.index) {
            let patched = apply_node_diff(node, &m.diff);
            slots[m.index] = Some(Box::pin(patched));
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
            declaration: self.declaration.as_ref().map(|_| base.doc.declaration.clone()),
            doctype: self.doctype.as_ref().map(|_| base.doc.doctype.clone()),
            root: self.root.as_ref().map(|d| inverse_node_diff(base.doc.root.as_ref(), d)),
        }
    }

    fn between(base: &XmlSnapshot, other: &XmlSnapshot) -> Self {
        XmlDiff {
            prolog: if base.doc.prolog != other.doc.prolog { Some(other.doc.prolog.clone()) } else { None },
            declaration: if base.doc.declaration != other.doc.declaration { Some(other.doc.declaration.clone()) } else { None },
            doctype: if base.doc.doctype != other.doc.doctype { Some(other.doc.doctype.clone()) } else { None },
            root: between_root(base.doc.root.as_ref(), other.doc.root.as_ref()),
        }
    }

    fn is_empty(&self) -> bool {
        self.prolog.is_none() && self.declaration.is_none() && self.doctype.is_none() && self.root.is_none()
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
            added.push(XmlAttrAdded { index: idx, name: original.name.clone(), value: original.value.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    XmlAttributesDiff { removed, modified, added }
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
            let children = Box::pin(between_children(bc, oc));
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
    for (i, o) in other.iter().enumerate() {
        if !base.iter().any(|b| b.name == o.name) {
            added.push(XmlAttrAdded { index: i, name: o.name.clone(), value: o.value.clone() });
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(XmlAttributesDiff { removed, modified, added })
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
            if let Some(d) = Box::pin(between_node(&base[i], &other[i])) {
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
        (XmlNodeDiff::Element(ea), XmlNodeDiff::Element(eb)) => XmlNodeDiff::Element(Box::pin(absorb_element_diff(ea, eb))),
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
        (Some(ad), Some(bd)) => Some(absorb_attrs_diff(ad, bd)),
    };
    a.children = match (a.children.take(), b.children) {
        (None, x) => x,
        (x, None) => x,
        (Some(ad), Some(bd)) => Some(Box::pin(absorb_children_diff(ad, bd))),
    };
    a
}

/// 🏷️ Name-keyed absorb -- simpler than the index-keyed children case since attribute NAME (not
/// position) is the stable identity; only `added.index` needs any position bookkeeping at all,
/// approximated (not fully index-transported like children) since attribute order carries no
/// spec-mandated meaning, only round-trip fidelity.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_attrs_diff(mut a: XmlAttributesDiff, b: XmlAttributesDiff) -> XmlAttributesDiff {
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
                existing.index = ba.index;
            }
            None => added.push(ba.clone()),
        }
    }
    added.sort_by_key(|x| x.index);
    XmlAttributesDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_children_diff(d1: XmlChildrenDiff, d2: XmlChildrenDiff) -> XmlChildrenDiff {
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map(|m| m + 1).unwrap_or(0);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map(|m| m + 1).unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified.clone();
    let mut working_added = d1.added.clone();
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
                    Some(existing) => existing.diff = Box::pin(absorb_node_diff(existing.diff.clone(), m2.diff.clone())),
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
/// copies are `pub(crate)` to svg's own crate-visibility scope, not reachable from `📰xml`) but
/// marked `pub(crate)` in THIS file so `📰xml`'s own `🧬️mutations/component.rs` can reuse them for
/// its hand-rolled `OpText`/`OpBinary` (same intra-artifact reuse pattern svg uses).
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
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
pub(crate) fn enc_prolog(prolog: &Vec<XmlNode>) -> String {
    format!("[{}]", prolog.iter().map(enc_xml_node).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_prolog(s: &str) -> Result<Vec<XmlNode>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().map(dec_xml_node).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_prolog_bin(prolog: &Vec<XmlNode>, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, prolog.len() as u64);
    for node in prolog {
        enc_xml_node_bin(node, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_prolog_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<XmlNode>, String> {
    let count = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    (0..count).map(|_| dec_xml_node_bin(reader)).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_usize(s: &str) -> Result<usize, String> {
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

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8) backing the
/// upgraded `OpBinary`/`DiffCodec` frames below (and, via re-export, `../🧬️mutations/🦀️component.rs`'s
/// own upgraded `OpBinary`) -- reuses `store::pack_rt::write_varint_u64`/`store::ByteReader` rather
/// than reinventing varint encode/decode, same shape json's own `write_str_lp`/`read_str_lp` uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
//#endregion 🔖️BinaryPrimitives
//#endregion 🔖️Primitives

//#region 🔖️XmlValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_attr(a: &XmlAttr) -> String {
    format!("[{},{}]", enc_str(&a.name), enc_str(&a.value))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_attr(s: &str) -> Result<XmlAttr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, value] = parts.as_slice() else { return Err(format!("attr: expected 2 fields, got {}", parts.len())) };
    Ok(XmlAttr { name: dec_str(name)?, value: dec_str(value)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_declaration(d: &XmlDeclaration) -> String {
    format!("[{},{},{}]", enc_str(&d.version), encode_option(&d.encoding, |v| enc_str(v)), encode_option(&d.standalone, |v| if *v { "1".to_string() } else { "0".to_string() }),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_declaration(s: &str) -> Result<XmlDeclaration, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [version, encoding, standalone] = parts.as_slice() else { return Err(format!("declaration: expected 3 fields, got {}", parts.len())) };
    Ok(XmlDeclaration { version: dec_str(version)?, encoding: decode_option(encoding, dec_str)?, standalone: decode_option(standalone, |v| Ok(v == "1"))? })
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
    format!("[{},{},[{}]]", enc_str(&doctype.name), external, declarations)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_doctype(s: &str) -> Result<XmlDoctype, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, external, declarations] = parts.as_slice() else {
        return Err(format!("doctype: expected 3 fields, got {}", parts.len()));
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
    Ok(XmlDoctype { name: dec_str(name)?, external_id, declarations })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_doctype_bin(doctype: &XmlDoctype, out: &mut Vec<u8>) {
    write_str_lp(out, &doctype.name);
    match &doctype.external_id {
        None => out.push(0),
        Some(XmlExternalId::System { system_id }) => {
            out.push(1);
            write_str_lp(out, system_id);
        }
        Some(XmlExternalId::Public { public_id, system_id }) => {
            out.push(2);
            write_str_lp(out, public_id);
            write_str_lp(out, system_id);
        }
    }
    store::pack_rt::write_varint_u64(out, doctype.declarations.len() as u64);
    for declaration in &doctype.declarations {
        match declaration {
            XmlDtdDeclaration::Entity { parameter, name, value } => {
                out.push(1);
                out.push(u8::from(*parameter));
                write_str_lp(out, name);
                write_str_lp(out, value);
            }
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_doctype_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlDoctype, String> {
    let name = read_str_lp(reader)?;
    let external_id = match reader.read_u8().map_err(|error| error.to_string())? {
        0 => None,
        1 => Some(XmlExternalId::System { system_id: read_str_lp(reader)? }),
        2 => Some(XmlExternalId::Public { public_id: read_str_lp(reader)?, system_id: read_str_lp(reader)? }),
        tag => return Err(format!("unknown XML external id tag {tag}")),
    };
    let count = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    let mut declarations = Vec::with_capacity(count);
    for _ in 0..count {
        match reader.read_u8().map_err(|error| error.to_string())? {
            1 => declarations.push(XmlDtdDeclaration::Entity { parameter: reader.read_u8().map_err(|error| error.to_string())? != 0, name: read_str_lp(reader)?, value: read_str_lp(reader)? }),
            tag => return Err(format!("unknown XML DTD declaration tag {tag}")),
        }
    }
    Ok(XmlDoctype { name, external_id, declarations })
}
/// 🌳 Recursive: `E[name,[attrs],[children]]` / `T[text]` / `D[text]` (CData) / `M[text]` (comment)
/// / `P[target,data]` (processing instruction) — single-letter tag prefix, no ambiguity with the
/// hex payload since hex never starts with an uppercase letter.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xml_node(n: &XmlNode) -> String {
    match n {
        XmlNode::Element { name, attrs, children } => {
            let attrs = attrs.iter().map(enc_attr).collect::<Vec<_>>().join(",");
            let children = children.iter().map(enc_xml_node).collect::<Vec<_>>().join(",");
            format!("E[{},[{}],[{}]]", enc_str(name), attrs, children)
        }
        XmlNode::Text { text } => format!("T[{}]", enc_str(text)),
        XmlNode::CData { text } => format!("D[{}]", enc_str(text)),
        XmlNode::Comment { text } => format!("M[{}]", enc_str(text)),
        XmlNode::ProcessingInstruction { target, data } => format!("P[{},{}]", enc_str(target), enc_str(data)),
    }
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

//#region 🔖️XmlValueBinaryCodecs
/// 🧪️ P2-FG1: real recursive binary twins of [`enc_xml_node`]/[`dec_xml_node`] and
/// [`enc_declaration`]/[`dec_declaration`] above -- a 1-byte kind tag (`0`=Element/`1`=Text/
/// `2`=CData/`3`=Comment/`4`=ProcessingInstruction, distinct numbering from the text codec's letter
/// tags) followed by the real payload (length-prefixed strings for scalars, a varint COUNT then
/// that many recursively-encoded elements for `Element`'s attrs/children -- genuinely recursive,
/// not text-as-bytes). Backs the upgraded `OpBinary` frame (`../🧬️mutations/🦀️component.rs`) and
/// the `Replace`/added-item payloads inside [`enc_node_diff_bin`] below. `pub(crate)` so the
/// sibling `../🧬️mutations/🦀️component.rs` (same artifact, different facet module) can reuse these
/// rather than duplicating them a second time, matching this file's own existing text-codec reuse
/// convention.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr_bin(a: &XmlAttr, out: &mut Vec<u8>) {
    write_str_lp(out, &a.name);
    write_str_lp(out, &a.value);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attr_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlAttr, String> {
    let name = read_str_lp(reader)?;
    let value = read_str_lp(reader)?;
    Ok(XmlAttr { name, value })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_declaration_bin(d: &XmlDeclaration, out: &mut Vec<u8>) {
    write_str_lp(out, &d.version);
    out.push(if d.encoding.is_some() { 1 } else { 0 });
    if let Some(encoding) = &d.encoding {
        write_str_lp(out, encoding);
    }
    out.push(if d.standalone.is_some() { 1 } else { 0 });
    if let Some(standalone) = d.standalone {
        out.push(if standalone { 1 } else { 0 });
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_declaration_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlDeclaration, String> {
    let version = read_str_lp(reader)?;
    let encoding = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let standalone = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_u8().map_err(|e| e.to_string())? != 0) } else { None };
    Ok(XmlDeclaration { version, encoding, standalone })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xml_node_bin(node: &XmlNode, out: &mut Vec<u8>) {
    match node {
        XmlNode::Element { name, attrs, children } => {
            out.push(0);
            write_str_lp(out, name);
            store::pack_rt::write_varint_u64(out, attrs.len() as u64);
            for attr in attrs {
                enc_attr_bin(attr, out);
            }
            store::pack_rt::write_varint_u64(out, children.len() as u64);
            for child in children {
                enc_xml_node_bin(child, out);
            }
        }
        XmlNode::Text { text } => {
            out.push(1);
            write_str_lp(out, text);
        }
        XmlNode::CData { text } => {
            out.push(2);
            write_str_lp(out, text);
        }
        XmlNode::Comment { text } => {
            out.push(3);
            write_str_lp(out, text);
        }
        XmlNode::ProcessingInstruction { target, data } => {
            out.push(4);
            write_str_lp(out, target);
            write_str_lp(out, data);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xml_node_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlNode, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = read_str_lp(reader)?;
            let attr_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut attrs = Vec::with_capacity(attr_count as usize);
            for _ in 0..attr_count {
                attrs.push(dec_attr_bin(reader)?);
            }
            let child_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut children = Vec::with_capacity(child_count as usize);
            for _ in 0..child_count {
                children.push(Box::pin(dec_xml_node_bin(reader))?);
            }
            Ok(XmlNode::Element { name, attrs, children })
        }
        1 => Ok(XmlNode::Text { text: read_str_lp(reader)? }),
        2 => Ok(XmlNode::CData { text: read_str_lp(reader)? }),
        3 => Ok(XmlNode::Comment { text: read_str_lp(reader)? }),
        4 => {
            let target = read_str_lp(reader)?;
            let data = read_str_lp(reader)?;
            Ok(XmlNode::ProcessingInstruction { target, data })
        }
        other => Err(format!("xml node binary: unknown tag {other}")),
    }
}
//#endregion 🔖️XmlValueBinaryCodecs
//#endregion 🔖️XmlValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_attrs_diff(d: &XmlAttributesDiff) -> String {
    let removed = d.removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.name), enc_str(&m.value))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.index, enc_str(&a.name), enc_str(&a.value))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_attrs_diff(body: &str) -> Result<XmlAttributesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("attrs diff: expected 3 sections, got {}", three.len())) };
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
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
            let (name, value) = rest.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
            Ok(XmlAttrAdded { index: parse_usize(idx)?, name: dec_str(name)?, value: dec_str(value)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(XmlAttributesDiff { removed, modified, added })
}

/// 🌳 Recursive: `XmlNodeDiff` itself needs a tag (`E`=Element, `T`=Text, `R`=Replace) since,
/// unlike `XmlNode`, it appears standalone (not always inside a bracketed container) at the `root=`
/// top-level token position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_node_diff(d: &XmlNodeDiff) -> String {
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
        XmlNodeDiff::Replace { node } => format!("R[{}]", encode_option(node, |v| enc_xml_node(v))),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_node_diff(s: &str) -> Result<XmlNodeDiff, String> {
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
fn enc_children_diff(d: &XmlChildrenDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_node_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_xml_node(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_children_diff(body: &str) -> Result<XmlChildrenDiff, String> {
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

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-FG1: real recursive binary twin of [`enc_node_diff`]/[`dec_node_diff`] -- same 1-byte tag
/// numbering scheme as [`enc_xml_node_bin`] (`0`=Element/`1`=Text) plus `2`=`Replace` (needs its own
/// arm since `Replace` wraps a whole [`XmlNode`], not a bare scalar payload). `attrs`/`children`
/// collection triples encode as three varint-counted, recursively-encoded lists (removed/modified/
/// added) -- genuinely structured binary, backing the upgraded `DiffCodec::encode_diff`/
/// `decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_node_diff_bin(diff: &XmlNodeDiff, out: &mut Vec<u8>) {
    match diff {
        XmlNodeDiff::Element(e) => {
            out.push(0);
            out.push(if e.name.is_some() { 1 } else { 0 });
            if let Some(name) = &e.name {
                write_str_lp(out, name);
            }
            out.push(if e.attributes.is_some() { 1 } else { 0 });
            if let Some(attrs) = &e.attributes {
                enc_attrs_diff_bin(attrs, out);
            }
            out.push(if e.children.is_some() { 1 } else { 0 });
            if let Some(children) = &e.children {
                enc_children_diff_bin(children, out);
            }
        }
        XmlNodeDiff::Text { text } => {
            out.push(1);
            out.push(if text.is_some() { 1 } else { 0 });
            if let Some(text) = text {
                write_str_lp(out, text);
            }
        }
        XmlNodeDiff::Replace { node } => {
            out.push(2);
            out.push(if node.is_some() { 1 } else { 0 });
            if let Some(node) = node {
                enc_xml_node_bin(node, out);
            }
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_node_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlNodeDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            let attributes = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_attrs_diff_bin(reader)?) } else { None };
            let children = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(Box::pin(dec_children_diff_bin(reader))?) } else { None };
            Ok(XmlNodeDiff::Element(XmlElementDiff { name, attributes, children }))
        }
        1 => {
            let text = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            Ok(XmlNodeDiff::Text { text })
        }
        2 => {
            let node = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
            Ok(XmlNodeDiff::Replace { node })
        }
        other => Err(format!("xml node diff binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_attrs_diff_bin(diff: &XmlAttributesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for name in &diff.removed {
        write_str_lp(out, name);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        write_str_lp(out, &entry.name);
        write_str_lp(out, &entry.value);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        write_str_lp(out, &entry.name);
        write_str_lp(out, &entry.value);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_attrs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlAttributesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(read_str_lp(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let name = read_str_lp(reader)?;
        let value = read_str_lp(reader)?;
        modified.push(XmlAttrModified { name, value });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let name = read_str_lp(reader)?;
        let value = read_str_lp(reader)?;
        added.push(XmlAttrAdded { index, name, value });
    }
    Ok(XmlAttributesDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_children_diff_bin(diff: &XmlChildrenDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for index in &diff.removed {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_node_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_xml_node_bin(&entry.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_children_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlChildrenDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = Box::pin(dec_node_diff_bin(reader))?;
        modified.push(XmlChildModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_xml_node_bin(reader)?;
        added.push(XmlChildAdded { index, item });
    }
    Ok(XmlChildrenDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_xml_diff(d: &XmlDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.prolog {
        tokens.push(format!("prolog={}", enc_prolog(v)));
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
fn parse_xml_diff(line: &str) -> Result<XmlDiff, String> {
    let mut d = XmlDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("prolog=") {
            d.prolog = Some(dec_prolog(rest)?);
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

impl protocol::DiffCodec for XmlDiff {
    fn print_diff(&self) -> String {
        print_xml_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_xml_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags u8 | [declaration][doctype][root]`),
    /// matching `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload
    /// bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100%
    /// of stdio's `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` bits
    /// 0/1/2 mark `declaration`/`doctype`/`root` presence; each present field's own tri-state/
    /// recursive payload follows in that fixed order.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u8 = 0;
        if self.declaration.is_some() {
            flags |= 0b001;
        }
        if self.doctype.is_some() {
            flags |= 0b010;
        }
        if self.root.is_some() {
            flags |= 0b100;
        }
        if self.prolog.is_some() {
            flags |= 0b1000;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(declaration) = &self.declaration {
            out.push(if declaration.is_some() { 1 } else { 0 });
            if let Some(declaration) = declaration {
                enc_declaration_bin(declaration, &mut out);
            }
        }
        if let Some(doctype) = &self.doctype {
            out.push(if doctype.is_some() { 1 } else { 0 });
            if let Some(doctype) = doctype {
                enc_doctype_bin(doctype, &mut out);
            }
        }
        if let Some(root) = &self.root {
            enc_node_diff_bin(root, &mut out);
        }
        if let Some(prolog) = &self.prolog {
            enc_prolog_bin(prolog, &mut out);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        let declaration = if flags & 0b001 != 0 {
            let has = reader.read_u8().map_err(|e| malformed("diff declaration presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
            Some(if has != 0 { Some(dec_declaration_bin(&mut reader).map_err(|e| malformed("diff declaration", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None })
        } else {
            None
        };
        let doctype = if flags & 0b010 != 0 {
            let has = reader.read_u8().map_err(|e| malformed("diff doctype presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
            Some(if has != 0 { Some(dec_doctype_bin(&mut reader).map_err(|e| malformed("diff doctype", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None })
        } else {
            None
        };
        let root = if flags & 0b100 != 0 { Some(dec_node_diff_bin(&mut reader).map_err(|e| malformed("diff root", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let prolog = if flags & 0b1000 != 0 { Some(dec_prolog_bin(&mut reader).map_err(|e| malformed("diff prolog", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        Ok(XmlDiff { prolog, declaration, doctype, root })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `XmlDiff` values (both top-level tri-states, the recursive
/// `Element`/`Text`/`Replace` `XmlNodeDiff` tree, attribute add/remove/modify, nested child
/// add/remove/modify) — the single prolog of truth reused by `diff_codec_text_binary_roundtrip_law`
/// below AND by `⚙️engine/🦀️component.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<XmlDiff> {
    use crate::artifacts::xml::schema::snapshot::XmlDocument;

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
        declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }),
        prolog: Vec::new(),
    });
    let b = snapshot(XmlDocument { root: Some(elem("root", vec![("width", "20"), ("height", "30")], vec![elem("other", vec![("r", "5")], vec![]), XmlNode::Text { text: "hi".into() }])), doctype: None, declaration: None, prolog: Vec::new() });
    let c = snapshot(XmlDocument { root: None, doctype: None, declaration: None, prolog: Vec::new() });

    vec![XmlDiff::default(), XmlDiff::between(&a, &b), XmlDiff::between(&b, &a), XmlDiff::between(&a, &c), XmlDiff::between(&c, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    /// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `XmlDiff` grammar — exercises the
    /// recursive enum tree (`Element`/`Text`/`Replace` `XmlNodeDiff` variants), both top-level
    /// tri-states, attribute add/remove/modify, and nested child add/remove/modify. Reuses
    /// `demo_diff_cases()` (the single prolog of truth also consumed by
    /// `⚙️engine/🦀️component.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`).
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = XmlDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = XmlDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
