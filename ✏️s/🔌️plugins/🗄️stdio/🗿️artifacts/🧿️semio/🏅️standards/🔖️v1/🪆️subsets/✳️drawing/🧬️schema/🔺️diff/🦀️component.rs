//! 🔺️ SemioDrawingDiff — handcrafted recursive sparse diff over `SemioDrawingSnapshot`
//! (canvas/styles/layers, each layer a recursive `DrawNode` tree). No `snapshot:
//! Option<SemioDrawingSnapshot>` full-replace slot — even a whole-document swap's diff is the
//! sparse field-by-field `SemioDrawingDiff::between(base, next)`. `styles` is name-keyed
//! (`engine::triples::NamedTripleDiff`), `layers` and every `Group.children` are index-keyed
//! (`engine::triples::IndexedTripleDiff`) — both reused from the shared engine rather than
//! reinvented, per this ticket's brief. Built directly off svg's own `SvgNodeDiff` recursive-diff
//! template (`🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`) — same
//! apply/between/inverse/absorb shape, generalized here into `*_indexed`/`*_named` helpers so
//! both the `layers` collection and every nested `Group.children` collection share one
//! implementation instead of two near-duplicates.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{
    dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple, split_top_level, strip_brackets, IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff,
};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    dec_layer, dec_list, dec_node, dec_path_segment, dec_point2, dec_rgba, dec_style, dec_transform, enc_layer, enc_list, enc_node, enc_path_segment, enc_point2, enc_rgba, enc_style, enc_transform, DrawCanvas, DrawLayer, DrawNode, DrawStyle,
    PathSegment, SemioDrawingSnapshot,
};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.drawing.diff")]
pub struct SemioDrawingDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canvas: Option<DrawCanvasDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<NamedTripleDiff<String, DrawStyleDiff, DrawStyle>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layers: Option<IndexedTripleDiff<DrawLayerDiff, DrawLayer>>,
}
//#endregion 🔖️Diff

//#region 🔖️CanvasDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawCanvasDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = background cleared, `Some(Some(v))` = set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<Option<SemioRgba>>,
}
//#endregion 🔖️CanvasDiff

//#region 🔖️StyleDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawStyleDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<Option<SemioRgba>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Option<SemioRgba>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<Option<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<Option<f32>>,
}
//#endregion 🔖️StyleDiff

//#region 🔖️LayerDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<DrawNodeDiff>,
}
//#endregion 🔖️LayerDiff

//#region 🔖️NodeDiff
/// 🌳 Recursive per-node diff, shaped like `DrawNode` -- `Group.children` is a recursive
/// `IndexedTripleDiff<DrawNodeDiff, DrawNode>` (shared with `layers`' own triple shape). Node-KIND
/// change (e.g. `Path` -> `Text`) goes through `Replace`, matching `SvgNodeDiff::Replace`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DrawNodeDiff {
    Path(DrawPathDiff),
    Text(DrawTextDiff),
    Group(DrawGroupDiff),
    Image(DrawImageDiff),
    Replace { node: DrawNode },
}

/// 🩹 Manual `Default` (not derivable on a data-carrying enum without `#[default]`) — needed
/// transitively as the `D` of `triples::IndexedTripleDiff<DrawNodeDiff, DrawNode>`'s generated
/// `Deserialize` impl (see `DrawStyle`'s doc comment in the snapshot facet for why).
impl Default for DrawNodeDiff {
    fn default() -> Self {
        DrawNodeDiff::Group(DrawGroupDiff::default())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawPathDiff {
    /// 📐️ Weak/value-list per the recipe -- segments are whole-list replaced, never sub-diffed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<PathSegment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<Option<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawTextDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<SemioPoint2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<Option<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawGroupDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<SemioTransform>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<IndexedTripleDiff<DrawNodeDiff, DrawNode>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawImageDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<SemioPoint2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}
//#endregion 🔖️NodeDiff

//#region 🔖️IndexedHelpers
/// 🧮️ Generic `IndexedTripleDiff<D,T>` apply/between/inverse/absorb -- shared by BOTH `layers`
/// (index-keyed z-order) and every `Group.children` (recursive) so the two collections don't need
/// two near-duplicate implementations. Mirrors svg's `apply_children_diff`/`between_children`/
/// `inverse_children_diff`/`absorb_children_diff`, generalized over `T`/`D`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_indexed<T: Clone, D>(items: &[T], diff: &IndexedTripleDiff<D, T>, apply_item: impl Fn(&T, &D) -> T) -> Vec<T> {
    let mut slots: Vec<Option<T>> = items.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(it)) = slots.get(m.index) {
            let patched = apply_item(it, &m.diff);
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
    let mut out: Vec<T> = slots.into_iter().flatten().collect();
    let mut additions: Vec<&IndexAdded<T>> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, add.item.clone());
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_indexed<T: Clone + PartialEq, D>(base: &[T], other: &[T], between_item: impl Fn(&T, &T) -> Option<D>) -> Option<IndexedTripleDiff<D, T>> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = between_item(&base[i], &other[i]) {
                modified.push(IndexModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<IndexAdded<T>> = (min_len..other.len()).map(|i| IndexAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() {
        None
    } else {
        Some(IndexedTripleDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn added_indices<T>(added: &[IndexAdded<T>]) -> Vec<usize> {
    added.iter().map(|a| a.index).collect()
}

/// 📐️ Maps a BASE-side index through a diff's own removed/added into the position it ends up at
/// once that diff has been applied (used to build the inverse diff's own indices, which target
/// the AFTER array).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transform_index(idx: usize, removed: &[usize], added_idx: &[usize]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order = added_idx.to_vec();
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_indexed<T: Clone, D>(base: &[T], diff: &IndexedTripleDiff<D, T>, inverse_item: impl Fn(&T, &D) -> D) -> IndexedTripleDiff<D, T> {
    let added_idx = added_indices(&diff.added);
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &added_idx);
            modified.push(IndexModified { index: next_index, diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base.get(idx) {
            added.push(IndexAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    IndexedTripleDiff { removed, modified, added }
}

enum ItemOrigin {
    Base(usize),
    Added(usize),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_mid_origins(base_len: usize, removed: &[usize], added_idx: &[usize]) -> Vec<ItemOrigin> {
    let mut mid: Vec<ItemOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ItemOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added_idx.iter().enumerate().map(|(k, &idx)| (idx, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ItemOrigin::Added(k));
    }
    mid
}

/// 🧮️ Sequential-coalesce absorb (base-free index-transport over `d1`'s own removed/added),
/// mirroring svg's `absorb_children_diff` generically over `T`/`D`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed<T: Clone, D: Clone>(d1: IndexedTripleDiff<D, T>, d2: IndexedTripleDiff<D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T) -> IndexedTripleDiff<D, T> {
    let d1_added_idx = added_indices(&d1.added);
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map(|m| m + 1).unwrap_or(0);
    let mid_len_needed_by_d1 = d1_added_idx.iter().map(|&i| i + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map(|m| m + 1).unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1_added_idx);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified.clone();
    let mut working_added = d1.added.clone();
    let mut annihilated: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(ItemOrigin::Base(bi)) => {
                if !removed.contains(bi) {
                    removed.push(*bi);
                }
                modified.retain(|m| &m.index != bi);
            }
            Some(ItemOrigin::Added(k)) => {
                annihilated.insert(*k);
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(ItemOrigin::Base(bi)) => {
                if removed.contains(bi) {
                    continue;
                }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(IndexModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(ItemOrigin::Added(k)) => {
                if annihilated.contains(k) {
                    continue;
                }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_item(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let d2_added_idx = added_indices(&d2.added);
    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) {
            continue;
        }
        let final_index = transform_index(add.index, &d2.removed, &d2_added_idx);
        added.push(IndexAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    IndexedTripleDiff { removed, modified, added }
}
//#endregion 🔖️IndexedHelpers

//#region 🔖️NamedHelpers
/// 🏷️ Generic `NamedTripleDiff<K,D,T>` apply/between/inverse/absorb for `styles` -- key (not
/// position) is the stable identity, mirroring svg's name-keyed attribute-triple absorb.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named<K: PartialEq, D, T: Clone>(items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> &K, apply_item: impl Fn(&T, &D) -> T) -> Vec<T> {
    let mut out: Vec<T> = items
        .iter()
        .filter(|it| !diff.removed.iter().any(|k| k == key_of(it)))
        .map(|it| match diff.modified.iter().find(|m| &m.key == key_of(it)) {
            Some(m) => apply_item(it, &m.diff),
            None => it.clone(),
        })
        .collect();
    out.extend(diff.added.iter().cloned());
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_named<K: PartialEq + Clone, D, T: Clone + PartialEq>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, between_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            Some(o) => {
                if let Some(d) = between_item(b, o) {
                    modified.push(NamedModified { key: bk, diff: d });
                }
            }
            None => removed.push(bk),
        }
    }
    let mut added = Vec::new();
    for o in other {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(o.clone());
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_named<K: PartialEq + Clone, D, T: Clone>(base: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T> {
    let removed: Vec<K> = diff.added.iter().map(|t| key_of(t)).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(orig) = base.iter().find(|b| key_of(b) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(orig, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for k in &diff.removed {
        if let Some(orig) = base.iter().find(|b| &key_of(b) == k) {
            added.push(orig.clone());
        }
    }
    NamedTripleDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, D, T>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&T, &D) -> T, key_of: impl Fn(&T) -> K) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone + std::hash::Hash + Eq,
    D: Clone,
    T: Clone,
{
    let a1_added_keys: std::collections::HashSet<K> = d1.added.iter().map(|t| key_of(t)).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: std::collections::HashSet<K> = std::collections::HashSet::new();
    for k in &d2.removed {
        if a1_added_keys.contains(k) {
            annihilated.insert(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut added: Vec<T> = d1.added.into_iter().filter(|t| !annihilated.contains(&key_of(t))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in d2.modified {
        if let Some(t) = added.iter_mut().find(|t| key_of(t) == m2.key) {
            *t = apply_item(t, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(m2),
        }
    }
    for a2 in d2.added {
        match added.iter_mut().find(|t| key_of(t) == key_of(&a2)) {
            Some(slot) => *slot = a2,
            None => added.push(a2),
        }
    }
    NamedTripleDiff { removed, modified, added }
}
//#endregion 🔖️NamedHelpers

//#region 🔖️NodeAlgebra
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_node_diff(node: &DrawNode, diff: &DrawNodeDiff) -> DrawNode {
    match diff {
        DrawNodeDiff::Replace { node: replacement } => replacement.clone(),
        DrawNodeDiff::Path(pd) => match node {
            DrawNode::Path { segments, style } => DrawNode::Path { segments: pd.segments.clone().unwrap_or_else(|| segments.clone()), style: pd.style.clone().unwrap_or_else(|| style.clone()) },
            other => other.clone(),
        },
        DrawNodeDiff::Text(td) => match node {
            DrawNode::Text { value, at, style } => DrawNode::Text { value: td.value.clone().unwrap_or_else(|| value.clone()), at: td.at.unwrap_or(*at), style: td.style.clone().unwrap_or_else(|| style.clone()) },
            other => other.clone(),
        },
        DrawNodeDiff::Group(gd) => match node {
            DrawNode::Group { transform, children } => DrawNode::Group {
                transform: gd.transform.unwrap_or(*transform),
                children: match &gd.children {
                    Some(cd) => apply_indexed(children, cd, apply_node_diff),
                    None => children.clone(),
                },
            },
            other => other.clone(),
        },
        DrawNodeDiff::Image(id) => match node {
            DrawNode::Image { at, width, height, mime, bytes } => {
                DrawNode::Image { at: id.at.unwrap_or(*at), width: id.width.unwrap_or(*width), height: id.height.unwrap_or(*height), mime: id.mime.clone().unwrap_or_else(|| mime.clone()), bytes: id.bytes.clone().unwrap_or_else(|| bytes.clone()) }
            }
            other => other.clone(),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_node(base: &DrawNode, other: &DrawNode) -> Option<DrawNodeDiff> {
    if base == other {
        return None;
    }
    match (base, other) {
        (DrawNode::Path { segments: bs, style: bst }, DrawNode::Path { segments: os, style: ost }) => {
            let segments = if bs != os { Some(os.clone()) } else { None };
            let style = if bst != ost { Some(ost.clone()) } else { None };
            if segments.is_none() && style.is_none() {
                None
            } else {
                Some(DrawNodeDiff::Path(DrawPathDiff { segments, style }))
            }
        }
        (DrawNode::Text { value: bv, at: ba, style: bst }, DrawNode::Text { value: ov, at: oa, style: ost }) => {
            let value = if bv != ov { Some(ov.clone()) } else { None };
            let at = if ba != oa { Some(*oa) } else { None };
            let style = if bst != ost { Some(ost.clone()) } else { None };
            if value.is_none() && at.is_none() && style.is_none() {
                None
            } else {
                Some(DrawNodeDiff::Text(DrawTextDiff { value, at, style }))
            }
        }
        (DrawNode::Group { transform: bt, children: bc }, DrawNode::Group { transform: ot, children: oc }) => {
            let transform = if bt != ot { Some(*ot) } else { None };
            let children = between_indexed(bc, oc, between_node);
            if transform.is_none() && children.is_none() {
                None
            } else {
                Some(DrawNodeDiff::Group(DrawGroupDiff { transform, children }))
            }
        }
        (DrawNode::Image { at: ba, width: bw, height: bh, mime: bm, bytes: bb }, DrawNode::Image { at: oa, width: ow, height: oh, mime: om, bytes: ob }) => {
            let at = if ba != oa { Some(*oa) } else { None };
            let width = if bw != ow { Some(*ow) } else { None };
            let height = if bh != oh { Some(*oh) } else { None };
            let mime = if bm != om { Some(om.clone()) } else { None };
            let bytes = if bb != ob { Some(ob.clone()) } else { None };
            if at.is_none() && width.is_none() && height.is_none() && mime.is_none() && bytes.is_none() {
                None
            } else {
                Some(DrawNodeDiff::Image(DrawImageDiff { at, width, height, mime, bytes }))
            }
        }
        _ => Some(DrawNodeDiff::Replace { node: other.clone() }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_node_diff(current: &DrawNode, diff: &DrawNodeDiff) -> DrawNodeDiff {
    match diff {
        DrawNodeDiff::Replace { .. } => DrawNodeDiff::Replace { node: current.clone() },
        DrawNodeDiff::Path(pd) => match current {
            DrawNode::Path { segments, style } => DrawNodeDiff::Path(DrawPathDiff { segments: pd.segments.as_ref().map(|_| segments.clone()), style: pd.style.as_ref().map(|_| style.clone()) }),
            other => DrawNodeDiff::Replace { node: other.clone() },
        },
        DrawNodeDiff::Text(td) => match current {
            DrawNode::Text { value, at, style } => DrawNodeDiff::Text(DrawTextDiff { value: td.value.as_ref().map(|_| value.clone()), at: td.at.as_ref().map(|_| *at), style: td.style.as_ref().map(|_| style.clone()) }),
            other => DrawNodeDiff::Replace { node: other.clone() },
        },
        DrawNodeDiff::Group(gd) => match current {
            DrawNode::Group { transform, children } => {
                DrawNodeDiff::Group(DrawGroupDiff { transform: gd.transform.as_ref().map(|_| *transform), children: gd.children.as_ref().map(|cd| inverse_indexed(children, cd, |c, d| inverse_node_diff(c, d))) })
            }
            other => DrawNodeDiff::Replace { node: other.clone() },
        },
        DrawNodeDiff::Image(id) => match current {
            DrawNode::Image { at, width, height, mime, bytes } => DrawNodeDiff::Image(DrawImageDiff {
                at: id.at.as_ref().map(|_| *at),
                width: id.width.as_ref().map(|_| *width),
                height: id.height.as_ref().map(|_| *height),
                mime: id.mime.as_ref().map(|_| mime.clone()),
                bytes: id.bytes.as_ref().map(|_| bytes.clone()),
            }),
            other => DrawNodeDiff::Replace { node: other.clone() },
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_node_diff(a: DrawNodeDiff, b: DrawNodeDiff) -> DrawNodeDiff {
    match (a, b) {
        (_, DrawNodeDiff::Replace { node }) => DrawNodeDiff::Replace { node },
        (DrawNodeDiff::Replace { node }, b) => DrawNodeDiff::Replace { node: apply_node_diff(&node, &b) },
        (DrawNodeDiff::Path(pa), DrawNodeDiff::Path(pb)) => DrawNodeDiff::Path(DrawPathDiff { segments: pb.segments.or(pa.segments), style: pb.style.or(pa.style) }),
        (DrawNodeDiff::Text(ta), DrawNodeDiff::Text(tb)) => DrawNodeDiff::Text(DrawTextDiff { value: tb.value.or(ta.value), at: tb.at.or(ta.at), style: tb.style.or(ta.style) }),
        (DrawNodeDiff::Group(ga), DrawNodeDiff::Group(gb)) => DrawNodeDiff::Group(DrawGroupDiff {
            transform: gb.transform.or(ga.transform),
            children: match (ga.children, gb.children) {
                (None, x) => x,
                (x, None) => x,
                (Some(ac), Some(bc)) => Some(absorb_indexed(ac, bc, absorb_node_diff, apply_node_diff)),
            },
        }),
        (DrawNodeDiff::Image(ia), DrawNodeDiff::Image(ib)) => DrawNodeDiff::Image(DrawImageDiff { at: ib.at.or(ia.at), width: ib.width.or(ia.width), height: ib.height.or(ia.height), mime: ib.mime.or(ia.mime), bytes: ib.bytes.or(ia.bytes) }),
        // 🩹 Kind-mismatched non-Replace pair should not arise from real `between`/mutation output
        // (both always tag-match the current node or emit `Replace`) — total fallback: keep `b`.
        (_, b) => b,
    }
}
//#endregion 🔖️NodeAlgebra

//#region 🔖️ScalarAlgebra
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_canvas_diff(canvas: &DrawCanvas, diff: &DrawCanvasDiff) -> DrawCanvas {
    DrawCanvas { width: diff.width.unwrap_or(canvas.width), height: diff.height.unwrap_or(canvas.height), background: diff.background.clone().unwrap_or(canvas.background) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_canvas_diff(base: &DrawCanvas, other: &DrawCanvas) -> Option<DrawCanvasDiff> {
    let width = if base.width != other.width { Some(other.width) } else { None };
    let height = if base.height != other.height { Some(other.height) } else { None };
    let background = if base.background != other.background { Some(other.background) } else { None };
    if width.is_none() && height.is_none() && background.is_none() {
        None
    } else {
        Some(DrawCanvasDiff { width, height, background })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_canvas_diff(base: &DrawCanvas, diff: &DrawCanvasDiff) -> DrawCanvasDiff {
    DrawCanvasDiff { width: diff.width.map(|_| base.width), height: diff.height.map(|_| base.height), background: diff.background.as_ref().map(|_| base.background) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_canvas_diff(a: DrawCanvasDiff, b: DrawCanvasDiff) -> DrawCanvasDiff {
    DrawCanvasDiff { width: b.width.or(a.width), height: b.height.or(a.height), background: b.background.or(a.background) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_style_diff(style: &DrawStyle, diff: &DrawStyleDiff) -> DrawStyle {
    DrawStyle {
        name: style.name.clone(),
        fill: diff.fill.clone().unwrap_or(style.fill),
        stroke: diff.stroke.clone().unwrap_or(style.stroke),
        stroke_width: diff.stroke_width.unwrap_or(style.stroke_width),
        opacity: diff.opacity.unwrap_or(style.opacity),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_style_diff(base: &DrawStyle, other: &DrawStyle) -> Option<DrawStyleDiff> {
    let fill = if base.fill != other.fill { Some(other.fill) } else { None };
    let stroke = if base.stroke != other.stroke { Some(other.stroke) } else { None };
    let stroke_width = if base.stroke_width != other.stroke_width { Some(other.stroke_width) } else { None };
    let opacity = if base.opacity != other.opacity { Some(other.opacity) } else { None };
    if fill.is_none() && stroke.is_none() && stroke_width.is_none() && opacity.is_none() {
        None
    } else {
        Some(DrawStyleDiff { fill, stroke, stroke_width, opacity })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_style_diff(base: &DrawStyle, diff: &DrawStyleDiff) -> DrawStyleDiff {
    DrawStyleDiff { fill: diff.fill.map(|_| base.fill), stroke: diff.stroke.map(|_| base.stroke), stroke_width: diff.stroke_width.map(|_| base.stroke_width), opacity: diff.opacity.map(|_| base.opacity) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_style_diff(a: DrawStyleDiff, b: DrawStyleDiff) -> DrawStyleDiff {
    DrawStyleDiff { fill: b.fill.or(a.fill), stroke: b.stroke.or(a.stroke), stroke_width: b.stroke_width.or(a.stroke_width), opacity: b.opacity.or(a.opacity) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_layer_diff(layer: &DrawLayer, diff: &DrawLayerDiff) -> DrawLayer {
    DrawLayer {
        id: diff.id.clone().unwrap_or_else(|| layer.id.clone()),
        name: diff.name.clone().unwrap_or_else(|| layer.name.clone()),
        visible: diff.visible.unwrap_or(layer.visible),
        root: match &diff.root {
            Some(rd) => apply_node_diff(&layer.root, rd),
            None => layer.root.clone(),
        },
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_layer_diff(base: &DrawLayer, other: &DrawLayer) -> Option<DrawLayerDiff> {
    let id = if base.id != other.id { Some(other.id.clone()) } else { None };
    let name = if base.name != other.name { Some(other.name.clone()) } else { None };
    let visible = if base.visible != other.visible { Some(other.visible) } else { None };
    let root = between_node(&base.root, &other.root);
    if id.is_none() && name.is_none() && visible.is_none() && root.is_none() {
        None
    } else {
        Some(DrawLayerDiff { id, name, visible, root })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_layer_diff(base: &DrawLayer, diff: &DrawLayerDiff) -> DrawLayerDiff {
    DrawLayerDiff { id: diff.id.as_ref().map(|_| base.id.clone()), name: diff.name.as_ref().map(|_| base.name.clone()), visible: diff.visible.as_ref().map(|_| base.visible), root: diff.root.as_ref().map(|rd| inverse_node_diff(&base.root, rd)) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_layer_diff(a: DrawLayerDiff, b: DrawLayerDiff) -> DrawLayerDiff {
    DrawLayerDiff {
        id: b.id.or(a.id),
        name: b.name.or(a.name),
        visible: b.visible.or(a.visible),
        root: match (a.root, b.root) {
            (None, x) => x,
            (x, None) => x,
            (Some(ar), Some(br)) => Some(absorb_node_diff(ar, br)),
        },
    }
}
//#endregion 🔖️ScalarAlgebra

//#region 🔖️Apply
impl MutationDiff<SemioDrawingSnapshot> for SemioDrawingDiff {
    fn apply(&self, base: &SemioDrawingSnapshot) -> protocol::MutationApplyResult<SemioDrawingSnapshot> {
        let mut next = base.clone();
        if let Some(cd) = &self.canvas {
            next.canvas = apply_canvas_diff(&next.canvas, cd);
        }
        if let Some(sd) = &self.styles {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.styles, sd, |item| item.name.clone(), |item| item.name.clone(), ["styles"])?;
            next.styles = apply_named(&next.styles, sd, |s| &s.name, apply_style_diff);
        }
        if let Some(ld) = &self.layers {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_indexed_triple(ld, next.layers.len(), ["layers"])?;
            next.layers = apply_indexed(&next.layers, ld, apply_layer_diff);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.canvas = match (self.canvas.take(), other.canvas) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_canvas_diff(a, b)),
        };
        self.styles = match (self.styles.take(), other.styles) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_named(a, b, absorb_style_diff, apply_style_diff, |s: &DrawStyle| s.name.clone())),
        };
        self.layers = match (self.layers.take(), other.layers) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_indexed(a, b, absorb_layer_diff, apply_layer_diff)),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioDrawingSnapshot> for SemioDrawingDiff {
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Self {
        SemioDrawingDiff {
            canvas: self.canvas.as_ref().map(|cd| inverse_canvas_diff(&base.canvas, cd)),
            styles: self.styles.as_ref().map(|sd| inverse_named(&base.styles, sd, |s: &DrawStyle| s.name.clone(), inverse_style_diff)),
            layers: self.layers.as_ref().map(|ld| inverse_indexed(&base.layers, ld, inverse_layer_diff)),
        }
    }

    fn between(base: &SemioDrawingSnapshot, other: &SemioDrawingSnapshot) -> Self {
        SemioDrawingDiff {
            canvas: between_canvas_diff(&base.canvas, &other.canvas),
            styles: between_named(&base.styles, &other.styles, |s: &DrawStyle| s.name.clone(), between_style_diff),
            layers: between_indexed(&base.layers, &other.layers, between_layer_diff),
        }
    }

    fn is_empty(&self) -> bool {
        self.canvas.is_none() && self.styles.is_none() && self.layers.is_none()
    }
}

//#endregion 🔖️DiffAlgebra

//#region 🔖️NodePath
/// 🧭️ Mutation-level-only node address: `layer` selects `SemioDrawingSnapshot.layers[layer]`,
/// `path` is a chain of child indices from that layer's `root` (`path == []` addresses the root
/// itself). Kept out of the diff facet (svg precedent) -- `diff_at_path` lowers it into a nested
/// `SemioDrawingDiff` via `Group.children` triple entries down to the addressed depth.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePath {
    pub layer: usize,
    #[serde(default)]
    pub path: Vec<usize>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_at_path(np: &NodePath, leaf: DrawNodeDiff) -> SemioDrawingDiff {
    let mut node_diff = leaf;
    for &index in np.path.iter().rev() {
        node_diff = DrawNodeDiff::Group(DrawGroupDiff { transform: None, children: Some(IndexedTripleDiff { removed: Vec::new(), modified: vec![IndexModified { index, diff: node_diff }], added: Vec::new() }) });
    }
    let layer_diff = DrawLayerDiff { id: None, name: None, visible: None, root: Some(node_diff) };
    SemioDrawingDiff { canvas: None, styles: None, layers: Some(IndexedTripleDiff { removed: Vec::new(), modified: vec![IndexModified { index: np.layer, diff: layer_diff }], added: Vec::new() }) }
}

/// 🔎️ Reads the node currently addressed by `np`, or `None` if the path is out of range.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn node_at<'a>(snapshot: &'a SemioDrawingSnapshot, np: &NodePath) -> Option<&'a DrawNode> {
    let layer = snapshot.layers.get(np.layer)?;
    let mut current = &layer.root;
    for &idx in &np.path {
        match current {
            DrawNode::Group { children, .. } => current = children.get(idx)?,
            _ => return None,
        }
    }
    Some(current)
}
//#endregion 🔖️NodePath

//#region 🔖️NodeSpatialHelpers
/// 🧭️ Shared by `📍move-node`/`🖐️drag-nodes` (`Group.transform.translation.{x,y}` for a group,
/// `at` for `Text`/`Image`) -- `Path` has no origin field of its own (its geometry lives entirely
/// in `segments`), so it is honestly excluded rather than approximated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn node_origin(snapshot: &SemioDrawingSnapshot, np: &NodePath) -> Option<SemioPoint2> {
    match node_at(snapshot, np)? {
        DrawNode::Group { transform, .. } => Some(SemioPoint2 { x: transform.translation.x, y: transform.translation.y }),
        DrawNode::Text { at, .. } => Some(*at),
        DrawNode::Image { at, .. } => Some(*at),
        DrawNode::Path { .. } => None,
    }
}

/// 📍️ Builds the sparse diff that repositions the node at `np` to `new_origin` -- empty (no-op)
/// diff when the node is absent or is a `Path` (no origin field).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_move_node(snapshot: &SemioDrawingSnapshot, np: &NodePath, new_origin: SemioPoint2) -> SemioDrawingDiff {
    match node_at(snapshot, np) {
        Some(DrawNode::Group { transform, .. }) => {
            let mut next = *transform;
            next.translation.x = new_origin.x;
            next.translation.y = new_origin.y;
            diff_at_path(np, DrawNodeDiff::Group(DrawGroupDiff { transform: Some(next), children: None }))
        }
        Some(DrawNode::Text { .. }) => diff_at_path(np, DrawNodeDiff::Text(DrawTextDiff { value: None, at: Some(new_origin), style: None })),
        Some(DrawNode::Image { .. }) => diff_at_path(np, DrawNodeDiff::Image(DrawImageDiff { at: Some(new_origin), width: None, height: None, mime: None, bytes: None })),
        _ => SemioDrawingDiff::default(),
    }
}

/// 🔄️ Builds the sparse diff that sets a `Group` node's `transform.rotation` -- empty (no-op) for
/// every other node kind (`Path`/`Text`/`Image` carry no rotation field).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_rotate_node(snapshot: &SemioDrawingSnapshot, np: &NodePath, new_rotation: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioQuaternion) -> SemioDrawingDiff {
    match node_at(snapshot, np) {
        Some(DrawNode::Group { transform, .. }) => {
            let next = SemioTransform { translation: transform.translation, rotation: new_rotation, scale: transform.scale };
            diff_at_path(np, DrawNodeDiff::Group(DrawGroupDiff { transform: Some(next), children: None }))
        }
        _ => SemioDrawingDiff::default(),
    }
}

/// 📏️ Builds the sparse diff that sets a `Group` node's `transform.scale` -- empty (no-op) for
/// every other node kind.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_scale_node(snapshot: &SemioDrawingSnapshot, np: &NodePath, new_scale: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3) -> SemioDrawingDiff {
    match node_at(snapshot, np) {
        Some(DrawNode::Group { transform, .. }) => {
            let next = SemioTransform { translation: transform.translation, rotation: transform.rotation, scale: new_scale };
            diff_at_path(np, DrawNodeDiff::Group(DrawGroupDiff { transform: Some(next), children: None }))
        }
        _ => SemioDrawingDiff::default(),
    }
}
//#endregion 🔖️NodeSpatialHelpers

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` -- top-level `canvas=`/`styles=`/`layers=` space-separated
/// tokens (svg `SvgDiff` template); collection substrings reuse `engine::triples`'
/// `enc_indexed_triple`/`enc_named_triple`; leaf VALUES (`SemioRgba`/`SemioPoint2`/
/// `SemioTransform`/`Vec<PathSegment>`/whole `DrawNode`/`DrawLayer`/`DrawStyle` payloads) reuse the
/// REAL hex/bracket-encoded value codecs the sibling `📸️snapshot` facet already established
/// (`enc_rgba`/`enc_point2`/`enc_transform`/`enc_path_segment`/`enc_node`/`enc_layer`/`enc_style`,
/// imported above) -- drawing wave: replaces the old hex-of-`serde_json` shortcut these leaf values
/// were on pre-wave. One source of truth for the entity encoding across `📸️snapshot`/`🔺️diff`/
/// `🧬️mutations`, not three independently-invented copies.
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️NodeValueCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_node_diff(d: &DrawNodeDiff) -> String {
    match d {
        DrawNodeDiff::Path(p) => format!("P[{},{}]", encode_option(&p.segments, |v| enc_list(v, enc_path_segment)), encode_option(&p.style, |v| encode_option(v, |s| enc_str(s)))),
        DrawNodeDiff::Text(t) => format!("T[{},{},{}]", encode_option(&t.value, |v| enc_str(v)), encode_option(&t.at, enc_point2), encode_option(&t.style, |v| encode_option(v, |s| enc_str(s)))),
        DrawNodeDiff::Group(g) => format!(
            "G[{},{}]",
            encode_option(&g.transform, enc_transform),
            match &g.children {
                Some(c) => format!("[1,{}]", enc_indexed_triple(c, enc_node_diff, enc_node)),
                None => "[0]".to_string(),
            }
        ),
        DrawNodeDiff::Image(i) => {
            format!("I[{},{},{},{},{}]", encode_option(&i.at, enc_point2), encode_option(&i.width, |v| v.to_string()), encode_option(&i.height, |v| v.to_string()), encode_option(&i.mime, |v| enc_str(v)), encode_option(&i.bytes, |v| hex_encode(v)),)
        }
        DrawNodeDiff::Replace { node } => format!("R[{}]", enc_node(node)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_node_diff(s: &str) -> Result<DrawNodeDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "P" => {
            let parts = split_top_level(inner, ',');
            let [segments, style] = parts.as_slice() else { return Err(format!("path diff: expected 2 fields, got {}", parts.len())) };
            Ok(DrawNodeDiff::Path(DrawPathDiff { segments: decode_option(segments, |v| dec_list(v, dec_path_segment))?, style: decode_option(style, |v| decode_option(v, dec_str))? }))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [value, at, style] = parts.as_slice() else { return Err(format!("text diff: expected 3 fields, got {}", parts.len())) };
            Ok(DrawNodeDiff::Text(DrawTextDiff { value: decode_option(value, dec_str)?, at: decode_option(at, dec_point2)?, style: decode_option(style, |v| decode_option(v, dec_str))? }))
        }
        "G" => {
            let parts = split_top_level(inner, ',');
            let [transform_s, children_s] = parts.as_slice() else { return Err(format!("group diff: expected 2 fields, got {}", parts.len())) };
            let transform = decode_option(transform_s, dec_transform)?;
            let children = match split_top_level(strip_brackets(children_s)?, ',').as_slice() {
                ["0"] => None,
                [tag, rest @ ..] if *tag == "1" => Some(dec_indexed_triple(&rest.join(","), dec_node_diff, dec_node)?),
                other => return Err(format!("group children: bad shape {other:?}")),
            };
            Ok(DrawNodeDiff::Group(DrawGroupDiff { transform, children }))
        }
        "I" => {
            let parts = split_top_level(inner, ',');
            let [at, width, height, mime, bytes] = parts.as_slice() else { return Err(format!("image diff: expected 5 fields, got {}", parts.len())) };
            Ok(DrawNodeDiff::Image(DrawImageDiff {
                at: decode_option(at, dec_point2)?,
                width: decode_option(width, |v| v.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string()))?,
                height: decode_option(height, |v| v.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string()))?,
                mime: decode_option(mime, dec_str)?,
                bytes: decode_option(bytes, hex_decode)?,
            }))
        }
        "R" => Ok(DrawNodeDiff::Replace { node: dec_node(inner)? }),
        other => Err(format!("node diff: unknown tag {other:?}")),
    }
}
//#endregion 🔖️NodeValueCodec

//#region 🔖️TopLevelCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_canvas(c: &DrawCanvasDiff) -> String {
    format!("[{},{},{}]", encode_option(&c.width, |v| v.to_string()), encode_option(&c.height, |v| v.to_string()), encode_option(&c.background, |v| encode_option(v, enc_rgba)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_canvas(s: &str) -> Result<DrawCanvasDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [width, height, background] = parts.as_slice() else { return Err(format!("canvas diff: expected 3 fields, got {}", parts.len())) };
    Ok(DrawCanvasDiff {
        width: decode_option(width, |v| v.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string()))?,
        height: decode_option(height, |v| v.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string()))?,
        background: decode_option(background, |v| decode_option(v, dec_rgba))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_style_diff(d: &DrawStyleDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.fill, |v| encode_option(v, enc_rgba)),
        encode_option(&d.stroke, |v| encode_option(v, enc_rgba)),
        encode_option(&d.stroke_width, |v| encode_option(v, |x| x.to_string())),
        encode_option(&d.opacity, |v| encode_option(v, |x| x.to_string())),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_style_diff(s: &str) -> Result<DrawStyleDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [fill, stroke, stroke_width, opacity] = parts.as_slice() else { return Err(format!("style diff: expected 4 fields, got {}", parts.len())) };
    Ok(DrawStyleDiff {
        fill: decode_option(fill, |v| decode_option(v, dec_rgba))?,
        stroke: decode_option(stroke, |v| decode_option(v, dec_rgba))?,
        stroke_width: decode_option(stroke_width, |v| decode_option(v, |x| x.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string())))?,
        opacity: decode_option(opacity, |v| decode_option(v, |x| x.parse::<f32>().map_err(|e: std::num::ParseFloatError| e.to_string())))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_layer_diff(d: &DrawLayerDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.id, |v| enc_str(v)), encode_option(&d.name, |v| enc_str(v)), encode_option(&d.visible, |v| if *v { "1".to_string() } else { "0".to_string() }), encode_option(&d.root, enc_node_diff))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_layer_diff(s: &str) -> Result<DrawLayerDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, visible, root] = parts.as_slice() else { return Err(format!("layer diff: expected 4 fields, got {}", parts.len())) };
    Ok(DrawLayerDiff { id: decode_option(id, dec_str)?, name: decode_option(name, dec_str)?, visible: decode_option(visible, |v| Ok(v == "1"))?, root: decode_option(root, dec_node_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_drawing_diff(d: &SemioDrawingDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.canvas {
        tokens.push(format!("canvas={}", enc_canvas(v)));
    }
    if let Some(v) = &d.styles {
        tokens.push(format!("styles={}", enc_named_triple(v, |k: &String| enc_str(k), enc_style_diff, enc_style)));
    }
    if let Some(v) = &d.layers {
        tokens.push(format!("layers={}", enc_indexed_triple(v, enc_layer_diff, enc_layer)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_drawing_diff(line: &str) -> Result<SemioDrawingDiff, String> {
    let mut d = SemioDrawingDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("canvas=") {
            d.canvas = Some(dec_canvas(rest)?);
        } else if let Some(rest) = token.strip_prefix("styles=") {
            d.styles = Some(dec_named_triple(rest, dec_str, dec_style_diff, dec_style)?);
        } else if let Some(rest) = token.strip_prefix("layers=") {
            d.layers = Some(dec_indexed_triple(rest, dec_layer_diff, dec_layer)?);
        } else {
            return Err(format!("drawing diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

/// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
/// shortcut. `format u8` + `presence u8` (bit0=`canvas`, bit1=`styles`, bit2=`layers`) are two REAL
/// fixed header fields; past that, 0-3 varint-length-prefixed opaque blobs follow (one per present
/// collection, reusing the same `enc_canvas`/`enc_named_triple`/`enc_indexed_triple` text this
/// facet's own `print_diff` already emits) -- one opaque blob per present field rather than
/// per-segment `Cond`-guards (`protocol-cond-cannot-chain`: a second `if`-guard on a field that's
/// itself only conditionally decoded hard-errors `eval_cond`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

impl protocol::DiffCodec for SemioDrawingDiff {
    fn print_diff(&self) -> String {
        print_drawing_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_drawing_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence = 0u8;
        if self.canvas.is_some() {
            presence |= 1;
        }
        if self.styles.is_some() {
            presence |= 2;
        }
        if self.layers.is_some() {
            presence |= 4;
        }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(c) = &self.canvas {
            write_bytes_lp(&mut out, enc_canvas(c).as_bytes());
        }
        if let Some(s) = &self.styles {
            write_bytes_lp(&mut out, enc_named_triple(s, |k: &String| enc_str(k), enc_style_diff, enc_style).as_bytes());
        }
        if let Some(l) = &self.layers {
            write_bytes_lp(&mut out, enc_indexed_triple(l, enc_layer_diff, enc_layer).as_bytes());
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut reader = store::ByteReader::new(bytes);
        let format = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: e.to_string() })?;
        if format != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {format}") });
        }
        let presence = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff presence", offset: 1, detail: e.to_string() })?;
        let map_err = |what: &'static str| move |e: String| protocol::ProtocolError::Malformed { what, offset: 2, detail: e };
        let canvas = if presence & 1 != 0 {
            let blob = read_bytes_lp(&mut reader).map_err(map_err("diff canvas blob"))?;
            let text = std::str::from_utf8(&blob).map_err(|e| protocol::ProtocolError::Malformed { what: "diff canvas utf8", offset: 2, detail: e.to_string() })?;
            Some(dec_canvas(text).map_err(map_err("diff canvas"))?)
        } else {
            None
        };
        let styles = if presence & 2 != 0 {
            let blob = read_bytes_lp(&mut reader).map_err(map_err("diff styles blob"))?;
            let text = std::str::from_utf8(&blob).map_err(|e| protocol::ProtocolError::Malformed { what: "diff styles utf8", offset: 2, detail: e.to_string() })?;
            Some(dec_named_triple(text, dec_str, dec_style_diff, dec_style).map_err(map_err("diff styles"))?)
        } else {
            None
        };
        let layers = if presence & 4 != 0 {
            let blob = read_bytes_lp(&mut reader).map_err(map_err("diff layers blob"))?;
            let text = std::str::from_utf8(&blob).map_err(|e| protocol::ProtocolError::Malformed { what: "diff layers utf8", offset: 2, detail: e.to_string() })?;
            Some(dec_indexed_triple(text, dec_layer_diff, dec_layer).map_err(map_err("diff layers"))?)
        } else {
            None
        };
        Ok(SemioDrawingDiff { canvas, styles, layers })
    }
}
//#endregion 🔖️TopLevelCodec
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 `sweep_a`/`sweep_b`, promoted to module scope so both this facet's own tests AND
/// `🎹️composer/🦀️component.rs`'s conformance-law tests can build representative diffs from them
/// (a private item of `#[cfg(test)] mod tests` below is not visible to the sibling `composer`
/// module — same real, first-hit variant of this pattern brep's own report flags).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transform(tx: f64) -> SemioTransform {
    SemioTransform {
        translation: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3 { x: tx, y: 0.0, z: 0.0 },
        rotation: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioQuaternion::default(),
        scale: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 },
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_a() -> SemioDrawingSnapshot {
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA;
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 100.0, height: 50.0, background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles: vec![
            DrawStyle { name: "keep".into(), fill: Some(SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke: None, stroke_width: Some(1.0), opacity: None },
            DrawStyle { name: "gone".into(), fill: None, stroke: Some(SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }), stroke_width: None, opacity: Some(0.5) },
        ],
        layers: vec![
            DrawLayer {
                id: "l0".into(),
                name: "base".into(),
                visible: true,
                root: DrawNode::Group {
                    transform: SemioTransform::identity(),
                    children: vec![
                        DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::Close], style: Some("keep".into()) },
                        DrawNode::Text { value: "old".into(), at: SemioPoint2 { x: 1.0, y: 1.0 }, style: None },
                    ],
                },
            },
            DrawLayer { id: "l1".into(), name: "removed-layer".into(), visible: false, root: DrawNode::default() },
            DrawLayer { id: "l1b".into(), name: "removed-layer-2".into(), visible: false, root: DrawNode::default() },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_b() -> SemioDrawingSnapshot {
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA;
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 200.0, height: 80.0, background: None },
        styles: vec![
            DrawStyle { name: "keep".into(), fill: Some(SemioRgba { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }), stroke: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }), stroke_width: Some(3.0), opacity: Some(0.9) },
            DrawStyle { name: "added".into(), fill: None, stroke: None, stroke_width: None, opacity: None },
        ],
        layers: vec![
            DrawLayer {
                id: "l0".into(),
                name: "base-renamed".into(),
                visible: false,
                root: DrawNode::Group {
                    transform: transform(5.0),
                    children: vec![
                        DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 9.0, y: 9.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 1.0, y: 1.0 } }, PathSegment::Close], style: None },
                        DrawNode::Text { value: "old".into(), at: SemioPoint2 { x: 1.0, y: 1.0 }, style: None },
                        DrawNode::Group { transform: SemioTransform::identity(), children: Vec::new() },
                    ],
                },
            },
            DrawLayer { id: "l2".into(), name: "added-layer".into(), visible: true, root: DrawNode::default() },
        ],
    }
}

/// 🌱 Representative `SemioDrawingDiff` cases (incl. the empty no-op diff), single source of truth
/// for `diff_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioDrawingDiff> {
    use protocol::command::DiffAlgebra;
    let a = sweep_a();
    let b = sweep_b();
    let c = SemioDrawingSnapshot::default();
    vec![SemioDrawingDiff::default(), SemioDrawingDiff::between(&a, &b), SemioDrawingDiff::between(&b, &a), SemioDrawingDiff::between(&a, &c), SemioDrawingDiff::between(&c, &a)]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::DiffCodec;

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_every_field_and_every_collection_shape() {
        let a = sweep_a();
        let b = sweep_b();
        let d = SemioDrawingDiff::between(&a, &b);

        // Every top-level facet changed.
        assert!(d.canvas.is_some());
        assert!(d.styles.is_some());
        assert!(d.layers.is_some());

        let canvas = d.canvas.as_ref().unwrap();
        assert!(canvas.width.is_some() && canvas.height.is_some());
        assert_eq!(canvas.background, Some(None)); // tri-state clear

        let styles = d.styles.as_ref().unwrap();
        assert_eq!(styles.removed, vec!["gone".to_string()]);
        assert_eq!(styles.modified.len(), 1);
        assert_eq!(styles.added.len(), 1);

        // `layers` (3 base -> 2 other): positional pairwise-then-tail gives removed (base tail,
        // `l1b`) + modified (indices 0 and 1) -- `added` is structurally empty here (see the
        // sweep doc comment); `children` below covers the `added` case instead.
        let layers = d.layers.as_ref().unwrap();
        assert_eq!(layers.removed, vec![2usize]);
        assert_eq!(layers.modified.len(), 2);
        assert!(layers.added.is_empty());
        let layer0_diff = &layers.modified[0].diff;
        assert!(layer0_diff.name.is_some() && layer0_diff.visible.is_some() && layer0_diff.root.is_some());
        let DrawNodeDiff::Group(group_diff) = layer0_diff.root.as_ref().unwrap() else { panic!("expected group diff") };
        assert!(group_diff.transform.is_some());
        // `Group.children` (2 base -> 3 other): positional pairwise-then-tail gives modified
        // (index 0, the Path) + added (index 2, the nested Group) -- `removed` is structurally
        // empty here, completing the `layers`/`children` removed+added split the doc comment
        // describes.
        let children = group_diff.children.as_ref().unwrap();
        assert!(children.removed.is_empty());
        assert!(!children.modified.is_empty(), "expected a modified child (the Path)");
        assert!(!children.added.is_empty(), "expected an added child (the nested Group)");
        let DrawNodeDiff::Path(path_diff) = &children.modified[0].diff else { panic!("expected path diff") };
        assert!(path_diff.segments.is_some());
        assert_eq!(path_diff.style, Some(None)); // tri-state clear on a node-level style ref

        assert_eq!(d.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        assert_eq!(<SemioDrawingDiff as DiffAlgebra<SemioDrawingSnapshot>>::between(&b, &a).apply(&b).expect("apply must succeed for a well-formed fixture"), a);
        assert!(<SemioDrawingDiff as DiffAlgebra<SemioDrawingSnapshot>>::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_law_round_trips() {
        let a = sweep_a();
        let b = sweep_b();
        let d = SemioDrawingDiff::between(&a, &b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&d.apply(&a).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture"), a);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_composes_two_sequential_diffs() {
        let a = sweep_a();
        let mid = sweep_b();
        let mut after = sweep_b();
        after.canvas.width = 999.0;
        after.styles.push(DrawStyle { name: "third".into(), fill: None, stroke: None, stroke_width: None, opacity: None });

        let mut d1 = SemioDrawingDiff::between(&a, &mid);
        let d2 = SemioDrawingDiff::between(&mid, &after);
        let applied_before_absorb = d1.apply(&a).expect("apply must succeed for a well-formed fixture");
        d1.absorb(d2.clone());
        assert_eq!(d1.apply(&a).expect("apply must succeed for a well-formed fixture"), d2.apply(&applied_before_absorb).expect("apply must succeed for a well-formed fixture"));
        assert_eq!(d1.apply(&a).expect("apply must succeed for a well-formed fixture"), after);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_then_remove_annihilates_the_add() {
        // 📐️ Canonical correctness case (schema-design.md): Insert(2)+Remove(index-of-that-insert)
        // must annihilate the add entirely, never leave a dangling modified/removed entry.
        let base: Vec<DrawStyle> = vec![DrawStyle { name: "a".into(), fill: None, stroke: None, stroke_width: None, opacity: None }];
        let d1: NamedTripleDiff<String, DrawStyleDiff, DrawStyle> = NamedTripleDiff { removed: vec![], modified: vec![], added: vec![DrawStyle { name: "b".into(), fill: None, stroke: None, stroke_width: None, opacity: None }] };
        let d2: NamedTripleDiff<String, DrawStyleDiff, DrawStyle> = NamedTripleDiff { removed: vec!["b".to_string()], modified: vec![], added: vec![] };
        let absorbed = absorb_named(d1, d2, absorb_style_diff, apply_style_diff, |s: &DrawStyle| s.name.clone());
        assert!(absorbed.added.is_empty());
        assert!(absorbed.removed.is_empty());
        let applied = apply_named(&base, &absorbed, |s| &s.name, apply_style_diff);
        assert_eq!(applied, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        let c = SemioDrawingSnapshot::default();
        let cases = vec![SemioDrawingDiff::default(), SemioDrawingDiff::between(&a, &b), SemioDrawingDiff::between(&b, &a), SemioDrawingDiff::between(&a, &c), SemioDrawingDiff::between(&c, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.await.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioDrawingDiff::parse_diff(&printed).await.unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioDrawingDiff::decode_diff(&encoded).await.unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🔖️Tests
