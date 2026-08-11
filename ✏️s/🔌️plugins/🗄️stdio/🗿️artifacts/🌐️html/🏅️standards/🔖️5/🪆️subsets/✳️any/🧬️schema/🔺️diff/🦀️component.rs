//! 🔺️ HtmlDiff — handcrafted recursive tree diff over `HtmlSnapshot.root` (an `HtmlNode`).
//! `doctype` is a tri-state top-level scalar (`Some(None)` = cleared); `root` nests the recursive
//! `HtmlNodeDiff` tree, itself shaped like the `HtmlNode` it targets (`Element` <-> `HtmlElementDiff`,
//! `Text`/`Comment`/`RawText` <-> their own direct field diffs, any node-KIND change via the
//! `Replace` fallback). Structural pattern borrowed from `🎨️svg`'s own `SvgDiff`/`SvgNodeDiff`
//! (index-keyed children triple, name-keyed attribute triple, symbolic-position absorb) — own
//! types throughout, per the ticket's "HTML is not XML" instruction.
//! 🧪️ `#[derive(dsl::DslDiff)]` is unusable here for the identical structural reason already on
//! record for `SvgDiff`/`JsonValueDiff` (see those files' doc comments): `HtmlNodeDiff` is a
//! genuine data-carrying enum, and `DslField` has no impl for any data-carrying enum (only
//! `DslRecord`-derived structs and `DslScalar`-derived UNIT-only enums implement it). `DiffCodec`
//! is hand-rolled below, grammar template copied from `SvgDiff`'s.

use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, HtmlNode, HtmlSnapshot, RawTextKind};
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.html`. No `snapshot: Option<HtmlSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is the sparse field-by-field `HtmlDiff::between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.html.diff")]
pub struct HtmlDiff {
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = doctype removed, `Some(Some(s))` = set.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<Option<String>>,
    /// 🌳 `None` = root subtree unchanged; `Some(diff)` = the root changed (recursive, possibly
    /// down to a deeply nested leaf via `diff_at_path`, or a wholesale `Replace`). The root
    /// `HtmlNode` itself is never optional (unlike xml/svg's `Option<XmlNode>` root), so there is
    /// no "root removed" state — only "root replaced" via `HtmlNodeDiff::Replace`.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<HtmlNodeDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️NodeDiff
/// 🌳 Recursive per-node diff, shaped like the `HtmlNode` it targets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum HtmlNodeDiff {
    Element(HtmlElementDiff),
    Text {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    Comment {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    RawText {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parent_kind: Option<RawTextKind>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    /// 🔁 Wholesale node replace — node-KIND changes between base/next, and (uniquely at the
    /// document root, since `root` is never optional) any change to the root at all can always
    /// fall back to this.
    Replace {
        node: HtmlNode,
    },
}

/// 🏷️ Per-element diff.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlElementDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HtmlAttributesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<HtmlChildrenDiff>,
}

/// 🏷️ Name-keyed, ORDER-preserving attribute triple. Deliberately a Vec-based triple (not a
/// `HashMap`) — attribute order carries no HTML-spec meaning but IS significant for byte-preserving
/// round-trips.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlAttributesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<HtmlAttrModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<HtmlAttrAdded>,
}

/// 🏷️ `value: None` sets/keeps the attribute VALUELESS (e.g. `disabled`) — distinct from removal,
/// which is tracked separately via `HtmlAttributesDiff::removed`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlAttrModified {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlAttrAdded {
    pub index: usize,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 🌳 Index-keyed, recursive children triple. `removed`/`modified` indices refer to BASE state
/// (descending removal order on apply); `added` indices refer to FINAL state (ascending insert).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlChildrenDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<HtmlChildModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<HtmlChildAdded>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlChildModified {
    pub index: usize,
    pub diff: HtmlNodeDiff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlChildAdded {
    pub index: usize,
    pub item: HtmlNode,
}
//#endregion 🔖️NodeDiff

//#region 🔖️DiffAtPath
/// 🧭️ Lowers a `leaf` diff targeting the node addressed by `path` (a chain of child indices from
/// the document root — mirrors `crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::NodePath`,
/// kept as a bare `&[usize]` here so this module never needs to depend on the mutations module)
/// into a full `HtmlDiff` by nesting it through `HtmlChildModified` entries from the root down to
/// that depth. `path == []` addresses the root itself, so `leaf` becomes `HtmlDiff.root` directly.
pub fn diff_at_path(path: &[usize], leaf: HtmlNodeDiff) -> HtmlDiff {
    let mut node_diff = leaf;
    for &index in path.iter().rev() {
        node_diff = HtmlNodeDiff::Element(HtmlElementDiff {
            name: None,
            attributes: None,
            children: Some(HtmlChildrenDiff { removed: Vec::new(), modified: vec![HtmlChildModified { index, diff: node_diff }], added: Vec::new() }),
        });
    }
    HtmlDiff { doctype: None, root: Some(node_diff) }
}
//#endregion 🔖️DiffAtPath

//#region 🔖️Apply
impl MutationDiff<HtmlSnapshot> for HtmlDiff {
    fn apply(&self, base: &HtmlSnapshot) -> HtmlSnapshot {
        let mut next = base.clone();
        if let Some(doctype) = &self.doctype {
            next.doctype = doctype.clone();
        }
        if let Some(node_diff) = &self.root {
            next.root = apply_node_diff(&base.root, node_diff);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
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

fn apply_node_diff(node: &HtmlNode, diff: &HtmlNodeDiff) -> HtmlNode {
    match diff {
        HtmlNodeDiff::Replace { node: replacement } => replacement.clone(),
        HtmlNodeDiff::Text { text } => match node {
            HtmlNode::Text { text: current } => HtmlNode::Text { text: text.clone().unwrap_or_else(|| current.clone()) },
            other => other.clone(),
        },
        HtmlNodeDiff::Comment { text } => match node {
            HtmlNode::Comment { text: current } => HtmlNode::Comment { text: text.clone().unwrap_or_else(|| current.clone()) },
            other => other.clone(),
        },
        HtmlNodeDiff::RawText { parent_kind, text } => match node {
            HtmlNode::RawText { parent_kind: current_kind, text: current_text } => {
                HtmlNode::RawText { parent_kind: parent_kind.unwrap_or(*current_kind), text: text.clone().unwrap_or_else(|| current_text.clone()) }
            }
            other => other.clone(),
        },
        HtmlNodeDiff::Element(element_diff) => match node {
            HtmlNode::Element { name, attributes, children } => HtmlNode::Element {
                name: element_diff.name.clone().unwrap_or_else(|| name.clone()),
                attributes: match &element_diff.attributes {
                    Some(attrs_diff) => apply_attrs_diff(attributes, attrs_diff),
                    None => attributes.clone(),
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

fn apply_attrs_diff(attrs: &[HtmlAttr], diff: &HtmlAttributesDiff) -> Vec<HtmlAttr> {
    let mut out: Vec<HtmlAttr> = attrs
        .iter()
        .filter(|a| !diff.removed.contains(&a.name))
        .map(|a| match diff.modified.iter().find(|m| m.name == a.name) {
            Some(m) => HtmlAttr { name: a.name.clone(), value: m.value.clone() },
            None => a.clone(),
        })
        .collect();
    let mut additions: Vec<&HtmlAttrAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, HtmlAttr { name: add.name.clone(), value: add.value.clone() });
    }
    out
}

fn apply_children_diff(children: &[HtmlNode], diff: &HtmlChildrenDiff) -> Vec<HtmlNode> {
    let mut slots: Vec<Option<HtmlNode>> = children.iter().cloned().map(Some).collect();
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
    let mut out: Vec<HtmlNode> = slots.into_iter().flatten().collect();
    let mut additions: Vec<&HtmlChildAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, add.item.clone());
    }
    out
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<HtmlSnapshot> for HtmlDiff {
    fn inverse(&self, base: &HtmlSnapshot) -> Self {
        HtmlDiff {
            doctype: self.doctype.as_ref().map(|_| base.doctype.clone()),
            root: self.root.as_ref().map(|d| inverse_node_diff(&base.root, d)),
        }
    }

    fn between(base: &HtmlSnapshot, other: &HtmlSnapshot) -> Self {
        HtmlDiff {
            doctype: if base.doctype != other.doctype { Some(other.doctype.clone()) } else { None },
            root: node_diff_between(&base.root, &other.root),
        }
    }

    fn is_empty(&self) -> bool {
        self.doctype.is_none() && self.root.is_none()
    }
}

fn inverse_node_diff(current: &HtmlNode, diff: &HtmlNodeDiff) -> HtmlNodeDiff {
    match diff {
        HtmlNodeDiff::Replace { .. } => HtmlNodeDiff::Replace { node: current.clone() },
        HtmlNodeDiff::Text { .. } => match current {
            HtmlNode::Text { text } => HtmlNodeDiff::Text { text: Some(text.clone()) },
            other => HtmlNodeDiff::Replace { node: other.clone() },
        },
        HtmlNodeDiff::Comment { .. } => match current {
            HtmlNode::Comment { text } => HtmlNodeDiff::Comment { text: Some(text.clone()) },
            other => HtmlNodeDiff::Replace { node: other.clone() },
        },
        HtmlNodeDiff::RawText { .. } => match current {
            HtmlNode::RawText { parent_kind, text } => HtmlNodeDiff::RawText { parent_kind: Some(*parent_kind), text: Some(text.clone()) },
            other => HtmlNodeDiff::Replace { node: other.clone() },
        },
        HtmlNodeDiff::Element(element_diff) => match current {
            HtmlNode::Element { name, attributes, children } => HtmlNodeDiff::Element(HtmlElementDiff {
                name: element_diff.name.as_ref().map(|_| name.clone()),
                attributes: element_diff.attributes.as_ref().map(|ad| inverse_attrs_diff(attributes, ad)),
                children: element_diff.children.as_ref().map(|cd| inverse_children_diff(children, cd)),
            }),
            other => HtmlNodeDiff::Replace { node: other.clone() },
        },
    }
}

fn inverse_attrs_diff(base_attrs: &[HtmlAttr], diff: &HtmlAttributesDiff) -> HtmlAttributesDiff {
    let removed: Vec<String> = diff.added.iter().map(|a| a.name.clone()).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_attrs.iter().find(|a| a.name == m.name) {
            modified.push(HtmlAttrModified { name: original.name.clone(), value: original.value.clone() });
        }
    }
    let mut added = Vec::new();
    for name in &diff.removed {
        if let Some(idx) = base_attrs.iter().position(|a| &a.name == name) {
            let original = &base_attrs[idx];
            added.push(HtmlAttrAdded { index: idx, name: original.name.clone(), value: original.value.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    HtmlAttributesDiff { removed, modified, added }
}

fn inverse_children_diff(base_children: &[HtmlNode], diff: &HtmlChildrenDiff) -> HtmlChildrenDiff {
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_children.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &diff.added);
            modified.push(HtmlChildModified { index: next_index, diff: inverse_node_diff(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_children.get(idx) {
            added.push(HtmlChildAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    HtmlChildrenDiff { removed, modified, added }
}

fn node_diff_between(base: &HtmlNode, other: &HtmlNode) -> Option<HtmlNodeDiff> {
    if base == other {
        return None;
    }
    match (base, other) {
        (HtmlNode::Text { .. }, HtmlNode::Text { text: ot }) => Some(HtmlNodeDiff::Text { text: Some(ot.clone()) }),
        (HtmlNode::Comment { .. }, HtmlNode::Comment { text: ot }) => Some(HtmlNodeDiff::Comment { text: Some(ot.clone()) }),
        (HtmlNode::RawText { parent_kind: bk, text: bt }, HtmlNode::RawText { parent_kind: ok, text: ot }) => Some(HtmlNodeDiff::RawText {
            parent_kind: if bk != ok { Some(*ok) } else { None },
            text: if bt != ot { Some(ot.clone()) } else { None },
        }),
        (HtmlNode::Element { name: bn, attributes: ba, children: bc }, HtmlNode::Element { name: on, attributes: oa, children: oc }) => {
            let name = if bn != on { Some(on.clone()) } else { None };
            let attributes = attrs_diff_between(ba, oa);
            let children = children_diff_between(bc, oc);
            if name.is_none() && attributes.is_none() && children.is_none() {
                None
            } else {
                Some(HtmlNodeDiff::Element(HtmlElementDiff { name, attributes, children }))
            }
        }
        _ => Some(HtmlNodeDiff::Replace { node: other.clone() }),
    }
}

fn attrs_diff_between(base: &[HtmlAttr], other: &[HtmlAttr]) -> Option<HtmlAttributesDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        match other.iter().find(|o| o.name == b.name) {
            Some(o) if o.value != b.value => modified.push(HtmlAttrModified { name: b.name.clone(), value: o.value.clone() }),
            Some(_) => {}
            None => removed.push(b.name.clone()),
        }
    }
    let mut added = Vec::new();
    for (i, o) in other.iter().enumerate() {
        if !base.iter().any(|b| b.name == o.name) {
            added.push(HtmlAttrAdded { index: i, name: o.name.clone(), value: o.value.clone() });
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(HtmlAttributesDiff { removed, modified, added }) }
}

/// 🧮️ Naive positional child diff per the recipe's "between matching" rule for index-keyed
/// collections: pairwise-compare `0..min(base.len(), other.len())` as `modified`, the base tail
/// as `removed`, the other tail as `added`. Not an LCS-based diff (no move/reorder detection).
fn children_diff_between(base: &[HtmlNode], other: &[HtmlNode]) -> Option<HtmlChildrenDiff> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = node_diff_between(&base[i], &other[i]) {
                modified.push(HtmlChildModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<HtmlChildAdded> = (min_len..other.len()).map(|i| HtmlChildAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() { None } else { Some(HtmlChildrenDiff { removed, modified, added }) }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️Absorb
/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (base-free index-transport
/// over `d1`'s removed/added), ported from `SvgDiff`'s own `absorb_children_diff`/`transform_index`
/// (identical algorithm, own types).
fn transform_index(idx: usize, removed: &[usize], added: &[HtmlChildAdded]) -> usize {
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
/// from — built by `simulate_mid_origins` so `absorb_children_diff` can classify `d2`'s indices.
enum ChildOrigin {
    Base(usize),
    Added(usize),
}

/// 🧱️ Materializes a synthetic mid-array (base -> after `d1`) large enough to answer every index
/// `d1`/`d2` actually reference. Absorb is base-free (no real snapshot access), so `base_len` is
/// the SMALLEST synthetic length that avoids clamping any referenced position.
fn simulate_mid_origins(base_len: usize, removed: &[usize], added: &[HtmlChildAdded]) -> Vec<ChildOrigin> {
    let mut mid: Vec<ChildOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ChildOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ChildOrigin::Added(k));
    }
    mid
}

fn absorb_node_diff(a: HtmlNodeDiff, b: HtmlNodeDiff) -> HtmlNodeDiff {
    match (a, b) {
        (_, HtmlNodeDiff::Replace { node }) => HtmlNodeDiff::Replace { node },
        (HtmlNodeDiff::Replace { node }, b) => HtmlNodeDiff::Replace { node: apply_node_diff(&node, &b) },
        (HtmlNodeDiff::Text { text: ta }, HtmlNodeDiff::Text { text: tb }) => HtmlNodeDiff::Text { text: tb.or(ta) },
        (HtmlNodeDiff::Comment { text: ta }, HtmlNodeDiff::Comment { text: tb }) => HtmlNodeDiff::Comment { text: tb.or(ta) },
        (HtmlNodeDiff::RawText { parent_kind: ka, text: ta }, HtmlNodeDiff::RawText { parent_kind: kb, text: tb }) => {
            HtmlNodeDiff::RawText { parent_kind: kb.or(ka), text: tb.or(ta) }
        }
        (HtmlNodeDiff::Element(ea), HtmlNodeDiff::Element(eb)) => HtmlNodeDiff::Element(absorb_element_diff(ea, eb)),
        (_, b) => b,
    }
}

fn absorb_element_diff(mut a: HtmlElementDiff, b: HtmlElementDiff) -> HtmlElementDiff {
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

/// 🏷️ Name-keyed absorb — attribute NAME (not position) is the stable identity; only
/// `added.index` needs any position bookkeeping, approximated (not fully index-transported like
/// children) since attribute order carries no spec-mandated meaning, only round-trip fidelity.
fn absorb_attrs_diff(mut a: HtmlAttributesDiff, b: HtmlAttributesDiff) -> HtmlAttributesDiff {
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
    let mut modified: Vec<HtmlAttrModified> = a.modified.into_iter().filter(|m| !removed.contains(&m.name)).collect();
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
    HtmlAttributesDiff { removed, modified, added }
}

fn absorb_children_diff(d1: HtmlChildrenDiff, d2: HtmlChildrenDiff) -> HtmlChildrenDiff {
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
                    None => modified.push(HtmlChildModified { index: *bi, diff: m2.diff.clone() }),
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
        added.push(HtmlChildAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    HtmlChildrenDiff { removed, modified, added }
}
//#endregion 🔖️Absorb

//#region 🔖️SetSnapshot
/// 🧩️ Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<HtmlSnapshot>` full-replace slot — this IS `HtmlDiff::between`.
pub fn diff_set_snapshot(base: &HtmlSnapshot, next: &HtmlSnapshot) -> HtmlDiff {
    HtmlDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` for `HtmlDiff`, following `SvgDiff`'s template (bracket
/// -depth-aware split, hex for strings, `[0]`/`[1,x]` for `Option<T>`). Self-contained (own copies
/// of the small primitive set — no shared "hand-roll helpers" module exists yet, same rationale
/// `SvgDiff`'s file documents).
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
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
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
//#endregion 🔖️Primitives

//#region 🔖️HtmlValueCodecs
fn enc_html_attr(a: &HtmlAttr) -> String {
    format!("[{},{}]", enc_str(&a.name), encode_option(&a.value, |v| enc_str(v)))
}
fn dec_html_attr(s: &str) -> Result<HtmlAttr, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, value] = parts.as_slice() else { return Err(format!("attr: expected 2 fields, got {}", parts.len())) };
    Ok(HtmlAttr { name: dec_str(name)?, value: decode_option(value, dec_str)? })
}
pub(crate) fn enc_raw_kind(k: RawTextKind) -> &'static str {
    match k {
        RawTextKind::Script => "0",
        RawTextKind::Style => "1",
    }
}
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
//#endregion 🔖️HtmlValueCodecs

//#region 🔖️DiffValueCodecs
fn enc_attrs_diff(d: &HtmlAttributesDiff) -> String {
    let removed = d.removed.iter().map(|n| enc_str(n)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.name), encode_option(&m.value, |v| enc_str(v)))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.index, enc_str(&a.name), encode_option(&a.value, |v| enc_str(v)))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_attrs_diff(body: &str) -> Result<HtmlAttributesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("attrs diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (name, value) = entry.split_once(':').ok_or_else(|| format!("attr modified: bad entry {entry:?}"))?;
        Ok(HtmlAttrModified { name: dec_str(name)?, value: decode_option(value, dec_str)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
        let (name, value) = rest.split_once(':').ok_or_else(|| format!("attr added: bad entry {entry:?}"))?;
        Ok(HtmlAttrAdded { index: parse_usize(idx)?, name: dec_str(name)?, value: decode_option(value, dec_str)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(HtmlAttributesDiff { removed, modified, added })
}

/// 🌳 Recursive: `HtmlNodeDiff` itself needs a tag (`E`=Element, `T`=Text, `M`=Comment, `W`=RawText,
/// `R`=Replace) since, unlike `HtmlNode`, it appears standalone (not always inside a bracketed
/// container) at the `root=` top-level token position.
fn enc_node_diff(d: &HtmlNodeDiff) -> String {
    match d {
        HtmlNodeDiff::Element(e) => format!(
            "E[{},{},{}]",
            encode_option(&e.name, |v| enc_str(v)),
            match &e.attributes { Some(a) => format!("[1,{}]", enc_attrs_diff(a)), None => "[0]".to_string() },
            match &e.children { Some(c) => format!("[1,{}]", enc_children_diff(c)), None => "[0]".to_string() },
        ),
        HtmlNodeDiff::Text { text } => format!("T[{}]", encode_option(text, |v| enc_str(v))),
        HtmlNodeDiff::Comment { text } => format!("M[{}]", encode_option(text, |v| enc_str(v))),
        HtmlNodeDiff::RawText { parent_kind, text } => {
            format!("W[{},{}]", encode_option(parent_kind, |k| enc_raw_kind(*k).to_string()), encode_option(text, |v| enc_str(v)))
        }
        HtmlNodeDiff::Replace { node } => format!("R[{}]", enc_html_node(node)),
    }
}
fn dec_node_diff(s: &str) -> Result<HtmlNodeDiff, String> {
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
fn enc_children_diff(d: &HtmlChildrenDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_node_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_html_node(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
fn dec_children_diff(body: &str) -> Result<HtmlChildrenDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("children diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child modified: bad entry {entry:?}"))?;
        Ok(HtmlChildModified { index: parse_usize(idx)?, diff: dec_node_diff(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("child added: bad entry {entry:?}"))?;
        Ok(HtmlChildAdded { index: parse_usize(idx)?, item: dec_html_node(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(HtmlChildrenDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_html_diff(d: &HtmlDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.doctype { tokens.push(format!("doctype={}", encode_option(v, |v| enc_str(v)))); }
    if let Some(v) = &d.root { tokens.push(format!("root={}", enc_node_diff(v))); }
    tokens.join(" ")
}
fn parse_html_diff(line: &str) -> Result<HtmlDiff, String> {
    let mut d = HtmlDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("doctype=") { d.doctype = Some(decode_option(rest, dec_str)?); }
        else if let Some(rest) = token.strip_prefix("root=") { d.root = Some(dec_node_diff(rest)?); }
        else { return Err(format!("html diff: unknown token {token:?}")); }
    }
    Ok(d)
}

impl protocol::DiffCodec for HtmlDiff {
    fn print_diff(&self) -> String {
        print_html_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_html_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim, same simplification `SvgDiff`/`JsonDiff` (and the
    /// repo's only other hand-rolled `DiffCodec`s) use — satisfies every `DiffCodec` law without
    /// inventing a second wire format.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    fn elem(name: &str, attrs: Vec<(&str, Option<&str>)>, children: Vec<HtmlNode>) -> HtmlNode {
        HtmlNode::Element {
            name: name.to_string(),
            attributes: attrs.into_iter().map(|(n, v)| HtmlAttr { name: n.to_string(), value: v.map(|s| s.to_string()) }).collect(),
            children,
        }
    }

    fn snapshot(doctype: Option<&str>, root: HtmlNode) -> HtmlSnapshot {
        HtmlSnapshot { schema: crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype: doctype.map(|s| s.to_string()), root }
    }

    /// 🧪️ diff_codec_text_binary_roundtrip_law: exercises the recursive enum tree (`Element`/
    /// `Text`/`Comment`/`RawText`/`Replace` `HtmlNodeDiff` variants), the top-level tri-state, and
    /// nested attribute/child add/remove/modify.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = snapshot(
            Some("DOCTYPE html"),
            elem("html", vec![("lang", Some("en"))], vec![elem("p", vec![("id", Some("x")), ("disabled", None)], vec![])]),
        );
        let b = snapshot(
            None,
            elem(
                "html",
                vec![("lang", Some("de")), ("data-x", None)],
                vec![
                    elem("div", vec![], vec![HtmlNode::Text { text: "hi".into() }, HtmlNode::Comment { text: " c ".into() }]),
                    HtmlNode::Element { name: "script".into(), attributes: vec![], children: vec![HtmlNode::RawText { parent_kind: RawTextKind::Script, text: "1+1;".into() }] },
                ],
            ),
        );
        let c = snapshot(None, HtmlNode::Text { text: "root-replaced".into() });

        let cases = vec![
            HtmlDiff::default(),
            HtmlDiff::between(&a, &b),
            HtmlDiff::between(&b, &a),
            HtmlDiff::between(&a, &c),
            HtmlDiff::between(&c, &a),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = HtmlDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = HtmlDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
