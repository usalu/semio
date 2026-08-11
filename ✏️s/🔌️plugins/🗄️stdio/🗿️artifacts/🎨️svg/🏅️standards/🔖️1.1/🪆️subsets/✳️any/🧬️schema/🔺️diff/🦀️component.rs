//! 🔺️ SvgDiff — handcrafted recursive tree diff over `SvgSnapshot.doc` (an `XmlDocument`).
//! `declaration`/`doctype` are tri-state top-level scalars (`Some(None)` = cleared); `root` nests
//! the recursive `SvgNodeDiff` tree, itself shaped like the `XmlNode` it targets
//! (`XmlNode::Element` <-> `SvgElementDiff`, `XmlNode::Text` <-> `Text{text}`, everything else --
//! CData/Comment/ProcessingInstruction, plus any node-KIND change -- via the `Replace` fallback).
//! Builds on the xml/svg node-diff pattern originated by `📰xml`'s own `XmlDiff`
//! (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/
//! 🧬️schema-design.md`) but declares its OWN diff types (per the spec-mandated-reuse rule: svg
//! embeds xml's *node* model, never xml's *diff* model).

use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlNode};
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.svg`. No `snapshot: Option<SvgSnapshot>` full-replace slot -- even
/// `SetSnapshot`'s diff is the sparse field-by-field `SvgDiff::between(base, next)`.
/// 🧪️ F6-PILOT CONFIRMED: `#[derive(dsl::DslDiff)]` on this struct fails to compile with TWO
/// independent, simultaneous reasons (both captured verbatim, see `f6-recon-report.md`): (1)
/// `root: Option<SvgNodeDiff>` — `SvgNodeDiff` is a genuine data-carrying enum (`Element`/`Text`/
/// `Replace`), and `DslField` has no impl for it (only `DslRecord`-derived structs and
/// `DslScalar`-derived UNIT-only enums implement `DslField`); (2) `declaration`/`doctype` are
/// tri-state `Option<Option<T>>` fields — same blocker as `GifDiff` (see that file). `DiffCodec`
/// is hand-rolled below.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg.diff")]
pub struct SvgDiff {
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = declaration removed, `Some(Some(d))` = set.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declaration: Option<Option<XmlDeclaration>>,
    /// 📜️ Tri-state: `None` = unchanged, `Some(None)` = doctype removed, `Some(Some(s))` = set.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<Option<String>>,
    /// 🌳 `None` = root subtree unchanged; `Some(diff)` = the root changed (recursive, possibly
    /// down to a deeply nested leaf via `diff_at_path`, or a wholesale `Replace` incl. root
    /// presence/absence itself).
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<SvgNodeDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️NodeDiff
/// 🌳 Recursive per-node diff, shaped like the `XmlNode` it targets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SvgNodeDiff {
    Element(SvgElementDiff),
    Text {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    /// 🔁 Wholesale node replace -- node-KIND changes (e.g. `Text` -> `Element`, or either endpoint
    /// is `CData`/`Comment`/`ProcessingInstruction`) and, uniquely at the document ROOT, root
    /// presence/absence itself (`node: None` = root removed).
    Replace {
        node: Option<XmlNode>,
    },
}

/// 🏷️ Per-element diff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgElementDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SvgAttributesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<SvgChildrenDiff>,
}

/// 🏷️ Name-keyed, ORDER-preserving attribute triple. Deliberately a Vec-based triple (not a
/// `HashMap`) -- attribute order carries no SVG/XML-spec meaning but IS significant for
/// byte-preserving round-trips.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgAttributesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<SvgAttrModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<SvgAttrAdded>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgAttrModified {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgAttrAdded {
    pub index: usize,
    pub name: String,
    pub value: String,
}

/// 🌳 Index-keyed, recursive children triple. `removed`/`modified` indices refer to BASE state
/// (descending removal order on apply); `added` indices refer to FINAL state (ascending insert).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgChildrenDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<SvgChildModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<SvgChildAdded>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgChildModified {
    pub index: usize,
    pub diff: SvgNodeDiff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SvgChildAdded {
    pub index: usize,
    pub item: XmlNode,
}
//#endregion 🔖️NodeDiff

//#region 🔖️DiffAtPath
/// 🧭️ Lowers a `leaf` diff targeting the node addressed by `path` (a chain of child indices from
/// the document root -- mirrors `crate::artifacts::svg::schema::mutations::NodePath`, kept as a
/// bare `&[usize]` here so this module never needs to depend on the mutations module) into a full
/// `SvgDiff` by nesting it through `SvgChildModified` entries from the root down to that depth.
/// `path == []` addresses the root itself, so `leaf` becomes `SvgDiff.root` directly.
pub fn diff_at_path(path: &[usize], leaf: SvgNodeDiff) -> SvgDiff {
    let mut node_diff = leaf;
    for &index in path.iter().rev() {
        node_diff = SvgNodeDiff::Element(SvgElementDiff {
            name: None,
            attributes: None,
            children: Some(SvgChildrenDiff {
                removed: Vec::new(),
                modified: vec![SvgChildModified { index, diff: node_diff }],
                added: Vec::new(),
            }),
        });
    }
    SvgDiff { declaration: None, doctype: None, root: Some(node_diff) }
}
//#endregion 🔖️DiffAtPath

//#region 🔖️Apply
impl MutationDiff<SvgSnapshot> for SvgDiff {
    fn apply(&self, base: &SvgSnapshot) -> SvgSnapshot {
        let mut next = base.clone();
        if let Some(declaration) = &self.declaration {
            next.doc.declaration = declaration.clone();
        }
        if let Some(doctype) = &self.doctype {
            next.doc.doctype = doctype.clone();
        }
        if let Some(node_diff) = &self.root {
            next.doc.root = apply_root_diff(next.doc.root.as_ref(), node_diff);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
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

fn apply_root_diff(current: Option<&XmlNode>, diff: &SvgNodeDiff) -> Option<XmlNode> {
    match diff {
        SvgNodeDiff::Replace { node } => node.clone(),
        _ => current.map(|n| apply_node_diff(n, diff)),
    }
}

fn apply_node_diff(node: &XmlNode, diff: &SvgNodeDiff) -> XmlNode {
    match diff {
        SvgNodeDiff::Replace { node: replacement } => replacement.clone().unwrap_or_else(|| node.clone()),
        SvgNodeDiff::Text { text } => match node {
            XmlNode::Text { text: current } => XmlNode::Text { text: text.clone().unwrap_or_else(|| current.clone()) },
            other => other.clone(),
        },
        SvgNodeDiff::Element(element_diff) => match node {
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

fn apply_attrs_diff(attrs: &[XmlAttr], diff: &SvgAttributesDiff) -> Vec<XmlAttr> {
    let mut out: Vec<XmlAttr> = attrs
        .iter()
        .filter(|a| !diff.removed.contains(&a.name))
        .map(|a| match diff.modified.iter().find(|m| m.name == a.name) {
            Some(m) => XmlAttr { name: a.name.clone(), value: m.value.clone() },
            None => a.clone(),
        })
        .collect();
    let mut additions: Vec<&SvgAttrAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, XmlAttr { name: add.name.clone(), value: add.value.clone() });
    }
    out
}

fn apply_children_diff(children: &[XmlNode], diff: &SvgChildrenDiff) -> Vec<XmlNode> {
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
    let mut additions: Vec<&SvgChildAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, add.item.clone());
    }
    out
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SvgSnapshot> for SvgDiff {
    fn inverse(&self, base: &SvgSnapshot) -> Self {
        SvgDiff {
            declaration: self.declaration.as_ref().map(|_| base.doc.declaration.clone()),
            doctype: self.doctype.as_ref().map(|_| base.doc.doctype.clone()),
            root: self.root.as_ref().map(|d| inverse_node_diff(base.doc.root.as_ref(), d)),
        }
    }

    fn between(base: &SvgSnapshot, other: &SvgSnapshot) -> Self {
        SvgDiff {
            declaration: if base.doc.declaration != other.doc.declaration { Some(other.doc.declaration.clone()) } else { None },
            doctype: if base.doc.doctype != other.doc.doctype { Some(other.doc.doctype.clone()) } else { None },
            root: between_root(base.doc.root.as_ref(), other.doc.root.as_ref()),
        }
    }

    fn is_empty(&self) -> bool {
        self.declaration.is_none() && self.doctype.is_none() && self.root.is_none()
    }
}

fn inverse_node_diff(current: Option<&XmlNode>, diff: &SvgNodeDiff) -> SvgNodeDiff {
    match diff {
        SvgNodeDiff::Replace { .. } => SvgNodeDiff::Replace { node: current.cloned() },
        SvgNodeDiff::Text { .. } => match current {
            Some(XmlNode::Text { text }) => SvgNodeDiff::Text { text: Some(text.clone()) },
            Some(other) => SvgNodeDiff::Replace { node: Some(other.clone()) },
            None => SvgNodeDiff::Replace { node: None },
        },
        SvgNodeDiff::Element(element_diff) => match current {
            Some(XmlNode::Element { name, attrs, children }) => SvgNodeDiff::Element(SvgElementDiff {
                name: element_diff.name.as_ref().map(|_| name.clone()),
                attributes: element_diff.attributes.as_ref().map(|ad| inverse_attrs_diff(attrs, ad)),
                children: element_diff.children.as_ref().map(|cd| inverse_children_diff(children, cd)),
            }),
            Some(other) => SvgNodeDiff::Replace { node: Some(other.clone()) },
            None => SvgNodeDiff::Replace { node: None },
        },
    }
}

fn inverse_attrs_diff(base_attrs: &[XmlAttr], diff: &SvgAttributesDiff) -> SvgAttributesDiff {
    let removed: Vec<String> = diff.added.iter().map(|a| a.name.clone()).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_attrs.iter().find(|a| a.name == m.name) {
            modified.push(SvgAttrModified { name: original.name.clone(), value: original.value.clone() });
        }
    }
    let mut added = Vec::new();
    for name in &diff.removed {
        if let Some(idx) = base_attrs.iter().position(|a| &a.name == name) {
            let original = &base_attrs[idx];
            added.push(SvgAttrAdded { index: idx, name: original.name.clone(), value: original.value.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    SvgAttributesDiff { removed, modified, added }
}

fn inverse_children_diff(base_children: &[XmlNode], diff: &SvgChildrenDiff) -> SvgChildrenDiff {
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_children.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &diff.added);
            modified.push(SvgChildModified { index: next_index, diff: inverse_node_diff(Some(original), &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_children.get(idx) {
            added.push(SvgChildAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    SvgChildrenDiff { removed, modified, added }
}

fn between_root(base: Option<&XmlNode>, other: Option<&XmlNode>) -> Option<SvgNodeDiff> {
    match (base, other) {
        (None, None) => None,
        (None, Some(n)) => Some(SvgNodeDiff::Replace { node: Some(n.clone()) }),
        (Some(_), None) => Some(SvgNodeDiff::Replace { node: None }),
        (Some(b), Some(o)) => between_node(b, o),
    }
}

fn between_node(base: &XmlNode, other: &XmlNode) -> Option<SvgNodeDiff> {
    if base == other {
        return None;
    }
    match (base, other) {
        (XmlNode::Text { .. }, XmlNode::Text { text: ot }) => Some(SvgNodeDiff::Text { text: Some(ot.clone()) }),
        (XmlNode::Element { name: bn, attrs: ba, children: bc }, XmlNode::Element { name: on, attrs: oa, children: oc }) => {
            let name = if bn != on { Some(on.clone()) } else { None };
            let attributes = between_attrs(ba, oa);
            let children = between_children(bc, oc);
            if name.is_none() && attributes.is_none() && children.is_none() {
                None
            } else {
                Some(SvgNodeDiff::Element(SvgElementDiff { name, attributes, children }))
            }
        }
        _ => Some(SvgNodeDiff::Replace { node: Some(other.clone()) }),
    }
}

fn between_attrs(base: &[XmlAttr], other: &[XmlAttr]) -> Option<SvgAttributesDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        match other.iter().find(|o| o.name == b.name) {
            Some(o) if o.value != b.value => modified.push(SvgAttrModified { name: b.name.clone(), value: o.value.clone() }),
            Some(_) => {}
            None => removed.push(b.name.clone()),
        }
    }
    let mut added = Vec::new();
    for (i, o) in other.iter().enumerate() {
        if !base.iter().any(|b| b.name == o.name) {
            added.push(SvgAttrAdded { index: i, name: o.name.clone(), value: o.value.clone() });
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(SvgAttributesDiff { removed, modified, added }) }
}

/// 🧮️ Naive positional child diff per the recipe's "between matching" rule for index-keyed
/// collections: pairwise-compare `0..min(base.len(), other.len())` as `modified`, the base tail
/// as `removed`, the other tail as `added`. Not an LCS-based diff (no move/reorder detection).
fn between_children(base: &[XmlNode], other: &[XmlNode]) -> Option<SvgChildrenDiff> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = between_node(&base[i], &other[i]) {
                modified.push(SvgChildModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<SvgChildAdded> = (min_len..other.len()).map(|i| SvgChildAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() { None } else { Some(SvgChildrenDiff { removed, modified, added }) }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️Absorb
/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (base-free index-transport
/// over `d1`'s removed/added): `transform_index` maps a base-side index through `d1`'s own
/// removed/added to the position it ends up at once `d1` has been applied.
fn transform_index(idx: usize, removed: &[usize], added: &[SvgChildAdded]) -> usize {
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
/// the SMALLEST synthetic length that avoids clamping any referenced position.
fn simulate_mid_origins(base_len: usize, removed: &[usize], added: &[SvgChildAdded]) -> Vec<ChildOrigin> {
    let mut mid: Vec<ChildOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ChildOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ChildOrigin::Added(k));
    }
    mid
}

fn absorb_node_diff(a: SvgNodeDiff, b: SvgNodeDiff) -> SvgNodeDiff {
    match (a, b) {
        (_, SvgNodeDiff::Replace { node: Some(n) }) => SvgNodeDiff::Replace { node: Some(n) },
        (SvgNodeDiff::Replace { node: Some(n) }, b) => SvgNodeDiff::Replace { node: Some(apply_node_diff(&n, &b)) },
        (_, SvgNodeDiff::Replace { node: None }) => SvgNodeDiff::Replace { node: None },
        (SvgNodeDiff::Replace { node: None }, _) => SvgNodeDiff::Replace { node: None },
        (SvgNodeDiff::Text { text: ta }, SvgNodeDiff::Text { text: tb }) => SvgNodeDiff::Text { text: tb.or(ta) },
        (SvgNodeDiff::Element(ea), SvgNodeDiff::Element(eb)) => SvgNodeDiff::Element(absorb_element_diff(ea, eb)),
        (_, b) => b,
    }
}

fn absorb_element_diff(mut a: SvgElementDiff, b: SvgElementDiff) -> SvgElementDiff {
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
        (Some(ad), Some(bd)) => Some(absorb_children_diff(ad, bd)),
    };
    a
}

/// 🏷️ Name-keyed absorb -- attribute NAME (not position) is the stable identity; only
/// `added.index` needs any position bookkeeping, approximated (not fully index-transported like
/// children) since attribute order carries no spec-mandated meaning, only round-trip fidelity.
fn absorb_attrs_diff(mut a: SvgAttributesDiff, b: SvgAttributesDiff) -> SvgAttributesDiff {
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
    let mut modified: Vec<SvgAttrModified> = a.modified.into_iter().filter(|m| !removed.contains(&m.name)).collect();
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
    SvgAttributesDiff { removed, modified, added }
}

fn absorb_children_diff(d1: SvgChildrenDiff, d2: SvgChildrenDiff) -> SvgChildrenDiff {
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
                    Some(existing) => existing.diff = absorb_node_diff(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(SvgChildModified { index: *bi, diff: m2.diff.clone() }),
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
        added.push(SvgChildAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    SvgChildrenDiff { removed, modified, added }
}
//#endregion 🔖️Absorb

//#region 🔖️SetSnapshot
/// 🧩️ Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<SvgSnapshot>` full-replace slot -- this IS `SvgDiff::between`.
pub fn diff_set_snapshot(base: &SvgSnapshot, next: &SvgSnapshot) -> SvgDiff {
    SvgDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6-PILOT: **hand-rolled** `protocol::DiffCodec` for `SvgDiff` — the template every other
/// enum-shaped-diff artifact's F6 agent copies (svg is the plan's own named proof-of-concept for
/// this path, `SvgNodeDiff` being a real tagged enum in the tree; xml/json/dxf/pdf/md follow the
/// same shape). Same grammar style `GifDiff`'s hand-rolled codec uses (bracket-depth-aware split,
/// hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`) — see that file's doc comment for the
/// primitive rationale; this file re-derives its own copies of the small helper functions since
/// each hand-rolled codec is self-contained (no shared "hand-roll helpers" module exists yet —
/// flagged as a good future extraction once ≥3 artifacts hand-roll, not worth adding here for one).
//#region 🔖️Primitives
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn parse_usize(s: &str) -> Result<usize, String> { s.parse().map_err(|e: std::num::ParseIntError| e.to_string()) }

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
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG3: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8) backing
/// the upgraded `DiffCodec` frame below (and, via re-export, `../🧬️mutations/🦀️component.rs`'s own
/// upgraded `OpBinary`) — reuses `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than
/// reinventing varint encode/decode, same shape `📰xml`'s own sibling `XmlDiff` codec uses (svg's
/// copies stay `pub(crate)` to this artifact's own crate-visibility scope, not reachable from
/// `📰xml`, matching that file's own established duplication convention).
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
//#endregion 🔖️BinaryPrimitives
//#endregion 🔖️Primitives

//#region 🔖️XmlValueCodecs
fn enc_attr(a: &XmlAttr) -> String {
    format!("[{},{}]", enc_str(&a.name), enc_str(&a.value))
}
fn dec_attr(s: &str) -> Result<XmlAttr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, value] = parts.as_slice() else { return Err(format!("attr: expected 2 fields, got {}", parts.len())) };
    Ok(XmlAttr { name: dec_str(name)?, value: dec_str(value)? })
}
pub(crate) fn enc_declaration(d: &XmlDeclaration) -> String {
    format!(
        "[{},{},{}]",
        enc_str(&d.version),
        encode_option(&d.encoding, |v| enc_str(v)),
        encode_option(&d.standalone, |v| if *v { "1".to_string() } else { "0".to_string() }),
    )
}
pub(crate) fn dec_declaration(s: &str) -> Result<XmlDeclaration, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [version, encoding, standalone] = parts.as_slice() else { return Err(format!("declaration: expected 3 fields, got {}", parts.len())) };
    Ok(XmlDeclaration {
        version: dec_str(version)?,
        encoding: decode_option(encoding, dec_str)?,
        standalone: decode_option(standalone, |v| Ok(v == "1"))?,
    })
}
/// 🌳 Recursive: `E[name,[attrs],[children]]` / `T[text]` / `D[text]` (CData) / `M[text]` (comment)
/// / `P[target,data]` (processing instruction) — single-letter tag prefix, no ambiguity with the
/// hex payload since hex never starts with an uppercase letter.
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
/// 🧪️ P2-FG3: real recursive binary twins of [`enc_xml_node`]/[`dec_xml_node`] and
/// [`enc_declaration`]/[`dec_declaration`] above -- a 1-byte kind tag (`0`=Element/`1`=Text/
/// `2`=CData/`3`=Comment/`4`=ProcessingInstruction, distinct numbering from the text codec's letter
/// tags) followed by the real payload (length-prefixed strings for scalars, a varint COUNT then
/// that many recursively-encoded elements for `Element`'s attrs/children -- genuinely recursive,
/// not text-as-bytes). Backs the upgraded `DiffCodec` frame below and, via `../🧬️mutations/
/// 🦀️component.rs`'s own `pub(crate)` re-export, the upgraded `OpBinary` frame (same intra-artifact
/// reuse convention `📰xml`'s own sibling module already establishes).
pub(crate) fn enc_attr_bin(a: &XmlAttr, out: &mut Vec<u8>) {
    write_str_lp(out, &a.name);
    write_str_lp(out, &a.value);
}
pub(crate) fn dec_attr_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlAttr, String> {
    let name = read_str_lp(reader)?;
    let value = read_str_lp(reader)?;
    Ok(XmlAttr { name, value })
}
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
pub(crate) fn dec_declaration_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlDeclaration, String> {
    let version = read_str_lp(reader)?;
    let encoding = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let standalone = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_u8().map_err(|e| e.to_string())? != 0) } else { None };
    Ok(XmlDeclaration { version, encoding, standalone })
}
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
                children.push(dec_xml_node_bin(reader)?);
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
fn enc_attrs_diff(d: &SvgAttributesDiff) -> String {
    let removed = d.removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.name), enc_str(&m.value))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.index, enc_str(&a.name), enc_str(&a.value))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_attrs_diff(body: &str) -> Result<SvgAttributesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("attrs diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (name, value) = entry.split_once(':').ok_or_else(|| format!("attr modified: bad entry {entry:?}"))?;
        Ok(SvgAttrModified { name: dec_str(name)?, value: dec_str(value)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
        let (name, value) = rest.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
        Ok(SvgAttrAdded { index: parse_usize(idx)?, name: dec_str(name)?, value: dec_str(value)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(SvgAttributesDiff { removed, modified, added })
}

/// 🌳 Recursive: `SvgNodeDiff` itself needs a tag (`E`=Element, `T`=Text, `R`=Replace) since,
/// unlike `XmlNode`, it appears standalone (not always inside a bracketed container) at the `root=`
/// top-level token position.
fn enc_node_diff(d: &SvgNodeDiff) -> String {
    match d {
        SvgNodeDiff::Element(e) => format!(
            "E[{},{},{}]",
            encode_option(&e.name, |v| enc_str(v)),
            match &e.attributes { Some(a) => format!("[1,{}]", enc_attrs_diff(a)), None => "[0]".to_string() },
            match &e.children { Some(c) => format!("[1,{}]", enc_children_diff(c)), None => "[0]".to_string() },
        ),
        SvgNodeDiff::Text { text } => format!("T[{}]", encode_option(text, |v| enc_str(v))),
        SvgNodeDiff::Replace { node } => format!("R[{}]", encode_option(node, |v| enc_xml_node(v))),
    }
}
fn dec_node_diff(s: &str) -> Result<SvgNodeDiff, String> {
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
        "R" => Ok(SvgNodeDiff::Replace { node: decode_option(inner, dec_xml_node)? }),
        other => Err(format!("node diff: unknown tag {other:?}")),
    }
}
fn enc_children_diff(d: &SvgChildrenDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_node_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_xml_node(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_children_diff(body: &str) -> Result<SvgChildrenDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("children diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child modified: bad entry {entry:?}"))?;
        Ok(SvgChildModified { index: parse_usize(idx)?, diff: dec_node_diff(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child added: bad entry {entry:?}"))?;
        Ok(SvgChildAdded { index: parse_usize(idx)?, item: dec_xml_node(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(SvgChildrenDiff { removed, modified, added })
}

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-FG3: real recursive binary twins of [`enc_node_diff`]/[`dec_node_diff`] -- same 1-byte tag
/// numbering scheme as [`enc_xml_node_bin`] (`0`=Element/`1`=Text) plus `2`=`Replace` (needs its own
/// arm since `Replace` wraps a whole [`XmlNode`], not a bare scalar payload). `attrs`/`children`
/// collection triples encode as three varint-counted, recursively-encoded lists (removed/modified/
/// added) -- genuinely structured binary, backing the upgraded `DiffCodec::encode_diff`/
/// `decode_diff` below.
fn enc_node_diff_bin(diff: &SvgNodeDiff, out: &mut Vec<u8>) {
    match diff {
        SvgNodeDiff::Element(e) => {
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
        SvgNodeDiff::Text { text } => {
            out.push(1);
            out.push(if text.is_some() { 1 } else { 0 });
            if let Some(text) = text {
                write_str_lp(out, text);
            }
        }
        SvgNodeDiff::Replace { node } => {
            out.push(2);
            out.push(if node.is_some() { 1 } else { 0 });
            if let Some(node) = node {
                enc_xml_node_bin(node, out);
            }
        }
    }
}
fn dec_node_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<SvgNodeDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            let attributes = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_attrs_diff_bin(reader)?) } else { None };
            let children = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_children_diff_bin(reader)?) } else { None };
            Ok(SvgNodeDiff::Element(SvgElementDiff { name, attributes, children }))
        }
        1 => {
            let text = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            Ok(SvgNodeDiff::Text { text })
        }
        2 => {
            let node = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
            Ok(SvgNodeDiff::Replace { node })
        }
        other => Err(format!("svg node diff binary: unknown tag {other}")),
    }
}

fn enc_attrs_diff_bin(diff: &SvgAttributesDiff, out: &mut Vec<u8>) {
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
fn dec_attrs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<SvgAttributesDiff, String> {
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
        modified.push(SvgAttrModified { name, value });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let name = read_str_lp(reader)?;
        let value = read_str_lp(reader)?;
        added.push(SvgAttrAdded { index, name, value });
    }
    Ok(SvgAttributesDiff { removed, modified, added })
}

fn enc_children_diff_bin(diff: &SvgChildrenDiff, out: &mut Vec<u8>) {
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
fn dec_children_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<SvgChildrenDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_node_diff_bin(reader)?;
        modified.push(SvgChildModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_xml_node_bin(reader)?;
        added.push(SvgChildAdded { index, item });
    }
    Ok(SvgChildrenDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_svg_diff(d: &SvgDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.declaration { tokens.push(format!("declaration={}", encode_option(v, enc_declaration))); }
    if let Some(v) = &d.doctype { tokens.push(format!("doctype={}", encode_option(v, |v| enc_str(v)))); }
    if let Some(v) = &d.root { tokens.push(format!("root={}", enc_node_diff(v))); }
    tokens.join(" ")
}
fn parse_svg_diff(line: &str) -> Result<SvgDiff, String> {
    let mut d = SvgDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("declaration=") { d.declaration = Some(decode_option(rest, dec_declaration)?); }
        else if let Some(rest) = token.strip_prefix("doctype=") { d.doctype = Some(decode_option(rest, dec_str)?); }
        else if let Some(rest) = token.strip_prefix("root=") { d.root = Some(dec_node_diff(rest)?); }
        else { return Err(format!("svg diff: unknown token {token:?}")); }
    }
    Ok(d)
}

impl protocol::DiffCodec for SvgDiff {
    fn print_diff(&self) -> String {
        print_svg_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_svg_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG3: REAL binary frame (`format u8 | flags u8 | [declaration][doctype][root]`),
    /// matching `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload
    /// bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100%
    /// of stdio's `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` bits
    /// 0/1/2 mark `declaration`/`doctype`/`root` presence; each present field's own tri-state/
    /// recursive payload follows in that fixed order.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u8 = 0;
        if self.declaration.is_some() { flags |= 0b001; }
        if self.doctype.is_some() { flags |= 0b010; }
        if self.root.is_some() { flags |= 0b100; }
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
                write_str_lp(&mut out, doctype);
            }
        }
        if let Some(root) = &self.root {
            enc_node_diff_bin(root, &mut out);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        let declaration = if flags & 0b001 != 0 {
            let has = reader.read_u8().map_err(|e| malformed("diff declaration presence", reader.position(), e.to_string()))?;
            Some(if has != 0 { Some(dec_declaration_bin(&mut reader).map_err(|e| malformed("diff declaration", reader.position(), e))?) } else { None })
        } else {
            None
        };
        let doctype = if flags & 0b010 != 0 {
            let has = reader.read_u8().map_err(|e| malformed("diff doctype presence", reader.position(), e.to_string()))?;
            Some(if has != 0 { Some(read_str_lp(&mut reader).map_err(|e| malformed("diff doctype", reader.position(), e))?) } else { None })
        } else {
            None
        };
        let root = if flags & 0b100 != 0 { Some(dec_node_diff_bin(&mut reader).map_err(|e| malformed("diff root", reader.position(), e))?) } else { None };
        Ok(SvgDiff { declaration, doctype, root })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG3: representative `SvgDiff` values (both top-level tri-states, the recursive
/// `Element`/`Text`/`Replace` `SvgNodeDiff` tree, attribute add/remove/modify, nested child
/// add/remove/modify) — the single source of truth reused by `diff_codec_text_binary_roundtrip_law`
/// below AND by `⚙️engine/🦀️component.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests.
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<SvgDiff> {
    use crate::artifacts::xml::schema::snapshot::XmlDocument;

    fn elem(name: &str, attrs: Vec<(&str, &str)>, children: Vec<XmlNode>) -> XmlNode {
        XmlNode::Element {
            name: name.to_string(),
            attrs: attrs.into_iter().map(|(n, v)| XmlAttr { name: n.to_string(), value: v.to_string() }).collect(),
            children,
        }
    }
    fn snapshot(doc: XmlDocument) -> SvgSnapshot {
        SvgSnapshot { doc, ..Default::default() }
    }

    let a = snapshot(XmlDocument {
        root: Some(elem("svg", vec![("width", "10")], vec![elem("rect", vec![("x", "0")], vec![])])),
        doctype: Some("<!DOCTYPE svg>".into()),
        declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }),
    });
    let b = snapshot(XmlDocument {
        root: Some(elem(
            "svg",
            vec![("width", "20"), ("height", "30")],
            vec![elem("circle", vec![("r", "5")], vec![]), XmlNode::Text { text: "hi".into() }],
        )),
        doctype: None,
        declaration: None,
    });
    let c = snapshot(XmlDocument { root: None, doctype: None, declaration: None });

    vec![SvgDiff::default(), SvgDiff::between(&a, &b), SvgDiff::between(&b, &a), SvgDiff::between(&a, &c), SvgDiff::between(&c, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    /// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `SvgDiff` grammar — exercises the
    /// recursive enum tree (`Element`/`Text`/`Replace` `SvgNodeDiff` variants), both top-level
    /// tri-states, attribute add/remove/modify, and nested child add/remove/modify. Reuses
    /// `demo_diff_cases()` (the single source of truth also consumed by `⚙️engine/🦀️component.rs`'s
    /// `diff_grammar_conformance_law`/`protocol_walk_law`).
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SvgDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SvgDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec
