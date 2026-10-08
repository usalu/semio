//! 🔺️ SemioFlowDiff — handcrafted sparse diff over `SemioFlowSnapshot`
//! (`nodes: Vec<FlowNode>` + `edges: Vec<FlowEdge>`, both id-keyed). No
//! `snapshot: Option<SemioFlowSnapshot>` full-replace slot — even `SetSnapshot`'s diff is the
//! sparse field-by-field `SemioFlowDiff::between(base, next)`.
//!
//! Built directly on the shared `engine::triples::NamedTripleDiff<K,D,T>` (per
//! w1b-type-ownership.md: "this is what every W2 subset's real sparse diff... should be built
//! on"), reused THREE times — top-level `nodes`/`edges` (both id-keyed) and each node's own
//! nested `params` (key-keyed) — via one small set of generic `between_named`/`apply_named`/
//! `inverse_named`/`absorb_named` helpers, the same generalization docx/xlsx/bcf independently
//! converged on for their own name-keyed collections.
//!
//! 🧪️ Not attempting `#[derive(dsl::DslDiff)]` here: `position: Option<SemioPoint2>` on
//! `FlowNodeDiff` and `from`/`to: Option<PortRef>` on `FlowEdgeDiff` are whole-value-
//! replace `Option<T>` over a named struct `T` that is not itself `#[derive(dsl::DslRecord)]`, and
//! `NamedTripleDiff<K,D,T>: DslField` has no generic bridge in the `dsl` crate (f6-final-summary.md
//! §4.4, hit by 5 independent artifacts — the most-hit gap of the whole F6 program). Hand-rolled
//! per this ticket's explicit instruction ("hand-roll all diff/op codecs — do not fight the
//! derive").

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff, NamedAdded};



use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️CollectionDiffTypes
pub type FlowParamsDiff = NamedTripleDiff<String, FlowParamDiff, NamedAdded<FlowParam>>;
pub type FlowNodesDiff = NamedTripleDiff<String, FlowNodeDiff, NamedAdded<FlowNode>>;
pub type FlowEdgesDiff = NamedTripleDiff<String, FlowEdgeDiff, NamedAdded<FlowEdge>>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowParamDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 🌳️ `position`/weak value structs replace whole-value per the recipe ("Weak entities = value
/// structs — whole-value replaced in diffs, never sub-diffed").
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowNodeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<FlowParamsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<SemioPoint2>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowEdgeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<PortRef>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<PortRef>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
//#endregion 🔖️CollectionDiffTypes

//#region 🔖️Diff
/// 🔺️ Diff for `s.stdio.semio.flow`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.flow.diff")]
pub struct SemioFlowDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<FlowNodesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<FlowEdgesDiff>,
}

/// 🧮️ Whether the sparse triple can reproduce `other`'s ORDER. [`apply_named`] keeps every surviving
/// member where it already stood and pushes `added` onto the tail, so the key sequence it produces is
/// exactly `survivors(base order) ++ added(other order)`. When `other` orders its members any other
/// way — a member that survives but moved, or a new member that belongs before an old one — the
/// sparse triple is not a faithful description of the transition and `between` degrades to a full
/// replacement instead, which the same `apply_named` reproduces exactly.
///
/// 🐛️ This is the fix for a real defect the `🌊️mutate-semio-flow` differential caught: `set-snapshot`
/// on the Nakagin capsule network, undone by its own inverse, restored one node's `params` as
/// `[rotation, name, composeGuid]` instead of the committed `[name, composeGuid, rotation]`, because
/// `rotation` was the one key the replacement snapshot shared with the original and therefore kept
/// its position while the other two were appended behind it. `set-snapshot` means the snapshot
/// BECOMES the named one, order included, and now it does.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reproduces_order<K, T>(base: &[T], other: &[T], removed: &[K], added: &[NamedAdded<T>], key_of: &impl Fn(&T) -> K) -> bool
where
    K: PartialEq,
{
    let mut keys: Vec<K> = base.iter().map(key_of).filter(|k| !removed.contains(k)).collect();
    let mut ascending: Vec<&NamedAdded<T>> = added.iter().collect();
    ascending.sort_by_key(|a| a.index);
    for a in ascending {
        let at = a.index.min(keys.len());
        keys.insert(at, key_of(&a.item));
    }
    keys.len() == other.len() && keys.iter().zip(other.iter().map(key_of)).all(|(produced, expected)| *produced == expected)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) {
            apply_item(item, &m.diff);
        }
    }
    let mut ascending: Vec<&NamedAdded<T>> = diff.added.iter().collect();
    ascending.sort_by_key(|a| a.index);
    for a in ascending {
        let at = a.index.min(items.len());
        items.insert(at, a.item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, NamedAdded<T>>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(|a| key_of(&a.item)).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added: Vec<NamedAdded<T>> = diff.removed.iter().filter_map(|k| base_items.iter().position(|i| &key_of(i) == k).map(|index| NamedAdded { index, item: base_items[index].clone() })).collect();
    added.sort_by_key(|a| a.index);
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Name-keyed absorb (recipe's normative absorb, key-identity variant — no index transport
/// needed since identity IS the key): a `d2`-removal of a `d1`-added key annihilates the add; a
/// `d2`-modify of a `d1`-added key patches into the carried payload; everything else composes
/// directly on the shared key space.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, NamedAdded<T>>, d2: &NamedTripleDiff<K, D, NamedAdded<T>>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, NamedAdded<T>>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(|a| key_of(&a.item)).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<NamedAdded<T>> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(&a.item))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(&a.item) == m2.key) {
            apply_item(&mut added.item, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(&a2.item);
        match working_added.iter_mut().find(|a| key_of(&a.item) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️ParamLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_param(old: &FlowParam, new: &FlowParam) -> Option<FlowParamDiff> {
    if old == new {
        return None;
    }
    Some(FlowParamDiff { value: (old.value != new.value).then(|| new.value.clone()) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_param(param: &mut FlowParam, diff: &FlowParamDiff) {
    if let Some(v) = &diff.value {
        param.value = v.clone();
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_param(base: &FlowParam, diff: &FlowParamDiff) -> FlowParamDiff {
    FlowParamDiff { value: diff.value.as_ref().map(|_| base.value.clone()) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_param_diff(mut a: FlowParamDiff, b: FlowParamDiff) -> FlowParamDiff {
    if b.value.is_some() {
        a.value = b.value;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_node(node: &mut FlowNode, diff: &FlowNodeDiff) {
    if let Some(v) = &diff.kind {
        node.kind = v.clone();
    }
    if let Some(v) = &diff.label {
        node.label = v.clone();
    }
    if let Some(pd) = &diff.params {
        apply_named(&mut node.params, pd, |p| p.key.clone(), apply_param);
    }
    if let Some(v) = diff.position {
        node.position = v;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_node(base: &FlowNode, diff: &FlowNodeDiff) -> FlowNodeDiff {
    FlowNodeDiff {
        kind: diff.kind.as_ref().map(|_| base.kind.clone()),
        label: diff.label.as_ref().map(|_| base.label.clone()),
        params: diff.params.as_ref().map(|pd| inverse_named(&base.params, pd, |p| p.key.clone(), inverse_param)),
        position: diff.position.map(|_| base.position),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_node_diff(mut a: FlowNodeDiff, b: FlowNodeDiff) -> FlowNodeDiff {
    if b.kind.is_some() {
        a.kind = b.kind;
    }
    if b.label.is_some() {
        a.label = b.label;
    }
    if b.position.is_some() {
        a.position = b.position;
    }
    a.params = match (a.params.take(), b.params) {
        (None, x) => x,
        (x, None) => x,
        (Some(pa), Some(pb)) => Some(absorb_named(pa, &pb, |p| p.key.clone(), absorb_param_diff, apply_param)),
    };
    a
}

//#endregion 🔖️NodeLogic

//#region 🔖️EdgeLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_edge(old: &FlowEdge, new: &FlowEdge) -> Option<FlowEdgeDiff> {
    if old == new {
        return None;
    }
    let from = (old.from != new.from).then(|| new.from.clone());
    let to = (old.to != new.to).then(|| new.to.clone());
    let kind = (old.kind != new.kind).then(|| new.kind.clone());
    if from.is_none() && to.is_none() && kind.is_none() {
        None
    } else {
        Some(FlowEdgeDiff { from, to, kind })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_edge(edge: &mut FlowEdge, diff: &FlowEdgeDiff) {
    if let Some(v) = &diff.from {
        edge.from = v.clone();
    }
    if let Some(v) = &diff.to {
        edge.to = v.clone();
    }
    if let Some(v) = &diff.kind {
        edge.kind = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_edge(base: &FlowEdge, diff: &FlowEdgeDiff) -> FlowEdgeDiff {
    FlowEdgeDiff { from: diff.from.as_ref().map(|_| base.from.clone()), to: diff.to.as_ref().map(|_| base.to.clone()), kind: diff.kind.as_ref().map(|_| base.kind.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_edge_diff(mut a: FlowEdgeDiff, b: FlowEdgeDiff) -> FlowEdgeDiff {
    if b.from.is_some() {
        a.from = b.from;
    }
    if b.to.is_some() {
        a.to = b.to;
    }
    if b.kind.is_some() {
        a.kind = b.kind;
    }
    a
}

//#endregion 🔖️EdgeLogic

//#region 🔖️Apply
impl MutationDiff<SemioFlowSnapshot> for SemioFlowDiff {
    fn apply(&self, base: &SemioFlowSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioFlowSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.nodes {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.nodes, d, |item| item.id.clone(), |added| added.item.id.clone(), ["nodes"])?;
            apply_named(&mut next.nodes, d, |n| n.id.clone(), apply_node);
        }
        if let Some(d) = &self.edges {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.edges, d, |item| item.id.clone(), |added| added.item.id.clone(), ["edges"])?;
            apply_named(&mut next.edges, d, |e| e.id.clone(), apply_edge);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.nodes = match (self.nodes.take(), other.nodes) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |n| n.id.clone(), absorb_node_diff, apply_node)),
        };
        self.edges = match (self.edges.take(), other.edges) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |e| e.id.clone(), absorb_edge_diff, apply_edge)),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioFlowSnapshot> for SemioFlowDiff {
    fn inverse(&self, base: &SemioFlowSnapshot) -> Self {
        SemioFlowDiff { nodes: self.nodes.as_ref().map(|d| inverse_named(&base.nodes, d, |n| n.id.clone(), inverse_node)), edges: self.edges.as_ref().map(|d| inverse_named(&base.edges, d, |e| e.id.clone(), inverse_edge)) }
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_none() && self.edges.is_none()
    }
}
//#endregion 🔖️DiffAlgebra



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_node(base: &SemioFlowSnapshot, node: FlowNode, at: Option<usize>) -> SemioFlowDiff {
    let index = at.map_or(base.nodes.len(), |at| at.min(base.nodes.len()));
    SemioFlowDiff { nodes: Some(FlowNodesDiff { added: vec![NamedAdded { index, item: node }], ..Default::default() }), edges: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_node(id: &str) -> SemioFlowDiff {
    SemioFlowDiff { nodes: Some(FlowNodesDiff { removed: vec![id.to_string()], ..Default::default() }), edges: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_node_kind(id: &str, kind: &str) -> SemioFlowDiff {
    let d = FlowNodeDiff { kind: Some(kind.to_string()), ..Default::default() };
    SemioFlowDiff { nodes: Some(FlowNodesDiff { modified: vec![NamedModified { key: id.to_string(), diff: d }], ..Default::default() }), edges: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_node_label(id: &str, label: &str) -> SemioFlowDiff {
    let d = FlowNodeDiff { label: Some(label.to_string()), ..Default::default() };
    SemioFlowDiff { nodes: Some(FlowNodesDiff { modified: vec![NamedModified { key: id.to_string(), diff: d }], ..Default::default() }), edges: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_node_position(id: &str, position: SemioPoint2) -> SemioFlowDiff {
    let d = FlowNodeDiff { position: Some(position), ..Default::default() };
    SemioFlowDiff { nodes: Some(FlowNodesDiff { modified: vec![NamedModified { key: id.to_string(), diff: d }], ..Default::default() }), edges: None }
}
/// 🧩 Upserts one param on node `id` — a `FlowParamsDiff` `modified` entry if `key` already
/// exists on that node, an `added` entry (full `FlowParam`) otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_node_param(base: &SemioFlowSnapshot, id: &str, key: &str, value: &str, at: Option<usize>) -> SemioFlowDiff {
    let Some(node) = base.nodes.iter().find(|n| n.id == id) else { return SemioFlowDiff::default() };
    let params_diff = match node.params.iter().find(|p| p.key == key) {
        Some(existing) if existing.value == value => return SemioFlowDiff::default(),
        Some(_) => FlowParamsDiff { modified: vec![NamedModified { key: key.to_string(), diff: FlowParamDiff { value: Some(value.to_string()) } }], ..Default::default() },
        None => FlowParamsDiff { added: vec![NamedAdded { index: at.map_or(node.params.len(), |at| at.min(node.params.len())), item: FlowParam { key: key.to_string(), value: value.to_string() } }], ..Default::default() },
    };
    let node_diff = FlowNodeDiff { params: Some(params_diff), ..Default::default() };
    SemioFlowDiff { nodes: Some(FlowNodesDiff { modified: vec![NamedModified { key: id.to_string(), diff: node_diff }], ..Default::default() }), edges: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_node_param(id: &str, key: &str) -> SemioFlowDiff {
    let params_diff = FlowParamsDiff { removed: vec![key.to_string()], ..Default::default() };
    let node_diff = FlowNodeDiff { params: Some(params_diff), ..Default::default() };
    SemioFlowDiff { nodes: Some(FlowNodesDiff { modified: vec![NamedModified { key: id.to_string(), diff: node_diff }], ..Default::default() }), edges: None }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_edge(base: &SemioFlowSnapshot, edge: FlowEdge, at: Option<usize>) -> SemioFlowDiff {
    let index = at.map_or(base.edges.len(), |at| at.min(base.edges.len()));
    SemioFlowDiff { nodes: None, edges: Some(FlowEdgesDiff { added: vec![NamedAdded { index, item: edge }], ..Default::default() }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_edge(id: &str) -> SemioFlowDiff {
    SemioFlowDiff { nodes: None, edges: Some(FlowEdgesDiff { removed: vec![id.to_string()], ..Default::default() }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_edge_endpoints(id: &str, from: PortRef, to: PortRef) -> SemioFlowDiff {
    let d = FlowEdgeDiff { from: Some(from), to: Some(to), kind: None };
    SemioFlowDiff { nodes: None, edges: Some(FlowEdgesDiff { modified: vec![NamedModified { key: id.to_string(), diff: d }], ..Default::default() }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_edge_kind(id: &str, kind: &str) -> SemioFlowDiff {
    let d = FlowEdgeDiff { from: None, to: None, kind: Some(kind.to_string()) };
    SemioFlowDiff { nodes: None, edges: Some(FlowEdgesDiff { modified: vec![NamedModified { key: id.to_string(), diff: d }], ..Default::default() }) }
}
//#region 🔖️Demo
/// 🌱 Representative `SemioFlowDiff` cases built declaratively (empty/no-op and an empty-but-present node and edge row triple) — single source of truth for `diff_grammar_conformance_law`/`protocol_walk_law` in
/// `🎹️composer/🦀️.rs`.
#[cfg(all(test, feature = "conversion-flow"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioFlowDiff> {
    vec![SemioFlowDiff::default(), SemioFlowDiff { nodes: Some(Default::default()), edges: Some(Default::default()) }]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
