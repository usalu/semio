//! 🔺️ SemioValueTreeDiff — recursive, handcrafted diff mirroring `SemioValue`'s shape. `List` gets an
//! index-keyed triple, `Map` gets a name-keyed triple, the top-level `nodes` graph gets an
//! id-keyed triple — all THREE built directly on the shared
//! `crate::artifacts::semio::standards::v1::subsets::any::schema::triples` codec (`IndexedTripleDiff`/
//! `NamedTripleDiff` + their `enc_*`/`dec_*` bridge functions) per this ticket's explicit
//! instruction to reuse it rather than reinvent it a 14th time (bcf/docx and now `json` each
//! rolled their own copy before this shared engine existed). No `snapshot: Option<SemioValueSnapshot>`
//! full-replace slot anywhere — `SetSnapshot`'s own diff is the sparse `between(base, next)` just
//! like every other mutation. Structural template (Replace-on-kind-change fallback, recursive
//! between/apply/absorb) copied from `json`'s own `JsonDiff` (this subset's informing source).

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{
    dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple, split_top_level, strip_brackets, IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff,
};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, ValueId};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️NamedAdded
/// 🧷 Position-carrying "added" wrapper for name/id-keyed collections — the recipe's own
/// normative shape (`🧬️schema-design.md`'s `CAdded { pub index: usize, pub item: C }`, "full
/// payload + final position") mirrors `engine::triples::IndexAdded<T>` exactly, but the shared
/// `engine::triples::NamedTripleDiff<K,D,T>` only provides that for INDEXED collections
/// (`IndexAdded<T>`) — its named-triple counterpart's `added: Vec<T>` carries no position,
/// confirmed by direct inspection of `engine/🧰️triples/🦀️component.rs`. Without a position, a
/// re-added interior member (e.g. `between(b, a)` re-adding a member `a` has but `b` doesn't) can
/// only be appended at the END, which breaks `between_roundtrip_law` in the REVERSE direction the
/// moment the member's real position isn't already last (caught live by this subset's own
/// standalone algorithm-verification harness before this fix landed). `json`'s own
/// `JsonObjectAdded{index,key,item}` independently carries the identical index field for the
/// identical reason — this is this subset's local instantiation of that same normative shape,
/// supplied as `T` for `Map`/`nodes`' own `NamedTripleDiff<K,D,T>` rather than editing the
/// shared engine file (out of scope — see this subset's own report's "shared infra gaps").
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedAdded<T> {
    pub index: usize,
    pub item: T,
}
//#endregion 🔖️NamedAdded

//#region 🔖️SemioValueDiff
/// 🔺️ Recursive diff mirroring [`SemioValue`]'s shape. `Replace` is the fallback used whenever
/// the node's KIND changes between base and next (e.g. a value goes from `Int` to `Str`); the
/// other variants are direct/structural diffs used whenever the kind is stable. `List`/`Map` wrap
/// the SHARED `engine::triples` generic collection diffs directly (no local reimplementation).
/// 🧪️ `#[derive(dsl::DslDiff)]` is unusable here for the same structural reason confirmed live by
/// the F6 recon pilot on `SvgDiff`/`GifDiff` and independently by `json`'s own `JsonValueDiff`
/// (f6-recon-report.md §3a, this file's own informing source's doc comment): this is a genuine
/// data-carrying enum, and `DslField` has no impl for any data-carrying enum. `DiffCodec` is
/// hand-rolled below (§🔖️HandcraftedDiffCodec), grammar template copied from `JsonDiff`'s.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SemioValueDiff {
    /// 🔁️ Whole-node replace — the node's KIND changed, or a mutation explicitly overwrites it.
    Replace {
        value: SemioValue,
    },
    Bool {
        value: bool,
    },
    Int {
        lexeme: String,
    },
    Float {
        lexeme: String,
    },
    Str {
        value: String,
    },
    Bytes {
        value: Vec<u8>,
    },
    List {
        diff: IndexedTripleDiff<SemioValueDiff, SemioValue>,
    },
    Map {
        diff: NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>,
    },
    Ref {
        id: ValueId,
    },
}

/// 🩹 Never constructed as a "real" empty diff (there is no meaningful empty `SemioValueDiff` —
/// `SemioValueTreeDiff.root` is `Option<SemioValueDiff>` precisely so `None` carries that meaning).
/// Required ONLY because the shared `engine::triples::{IndexedTripleDiff,NamedTripleDiff}`'s
/// `Deserialize` derive needs `D: Default` — a `#[serde(default)]`-triggered bound-inference
/// quirk on their OWN generic fields (`removed`/`modified`/`added`), confirmed independently by
/// the sibling `presentation` subset hitting the identical `SlideShapeDiff: Default` requirement
/// for the same reason. No enum variant here is fieldless, so `#[derive(Default)]` (which requires
/// a unit `#[default]` variant) is not usable — hand-rolled instead.
impl Default for SemioValueDiff {
    fn default() -> Self {
        SemioValueDiff::Replace { value: SemioValue::default() }
    }
}
//#endregion 🔖️SemioValueDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.semio.value`. `schema` is an identity field and is never diffed. `nodes`
/// is the id-keyed value-GRAPH triple (see the snapshot module's doc comment) — a second,
/// top-level collection sibling to `root`'s own recursive tree, per the recipe's "strong-like
/// entities in ordered collections" rule.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.value.diff")]
pub struct SemioValueTreeDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<SemioValueDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<NamedTripleDiff<ValueId, SemioValueDiff, NamedAdded<SemioValueNode>>>,
}

impl MutationDiff<SemioValueSnapshot> for SemioValueTreeDiff {
    fn apply(&self, base: &SemioValueSnapshot) -> protocol::MutationApplyResult<SemioValueSnapshot> {
        let mut next = base.clone();
        if let Some(diff) = &self.root {
            validate_value_diff(diff, &base.root, vec!["root".to_string()])?;
            next.root = apply_value_diff(diff, &base.root);
        }
        if let Some(diff) = &self.nodes {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&base.nodes, diff, |node| node.id.clone(), |added| added.item.id.clone(), ["nodes"])?;
            validate_added_positions(diff.added.iter().map(|added| added.index), base.nodes.len() - diff.removed.len(), ["nodes"])?;
            for modified in &diff.modified {
                let node = base.nodes.iter().find(|node| node.id == modified.key).ok_or_else(|| semio_framework_plugin::resolve_ready(protocol::MutationApplyError::new("mutation.apply.missing-node", format!("node {:?} is absent", modified.key))).at(["nodes"]))?;
                validate_value_diff(&modified.diff, &node.value, vec!["nodes".to_string(), format!("{:?}", modified.key)])?;
            }
            next.nodes = apply_nodes_diff(diff, &base.nodes);
        }
        Ok(next)
    }

    /// ➕️ Structural, total, base-free, sequential-coalesce absorb — same shape `json`'s `JsonDiff`
    /// uses: a composed diff that ends up structurally empty (e.g. an insert immediately cancelled
    /// by a matching remove) collapses back to `None` rather than surviving as a no-op wrapper.
    fn absorb(&mut self, other: Self) {
        self.root = match (self.root.take(), other.root) {
            (None, None) => None,
            (Some(d1), None) => Some(d1),
            (None, Some(d2)) => Some(d2),
            (Some(d1), Some(d2)) => {
                let combined = absorb_value_diff(d1, d2);
                if is_value_diff_effectively_empty(&combined) {
                    None
                } else {
                    Some(combined)
                }
            }
        };
        self.nodes = match (self.nodes.take(), other.nodes) {
            (None, None) => None,
            (Some(d1), None) => Some(d1),
            (None, Some(d2)) => Some(d2),
            (Some(d1), Some(d2)) => {
                let combined = absorb_named(d1, d2, &|n: &NamedAdded<SemioValueNode>| n.item.id.clone(), &absorb_value_diff, &apply_value_diff_to_named_node, &is_value_diff_effectively_empty);
                if is_named_empty(&combined) {
                    None
                } else {
                    Some(combined)
                }
            }
        };
    }
}

impl DiffAlgebra<SemioValueSnapshot> for SemioValueTreeDiff {
    /// 🔁️ Diff-level undo, derived generically from `between`: `mid = self.apply(base)`, then
    /// `between(mid, base)` is exactly the diff that restores `base` when applied to `mid`.
    fn inverse(&self, base: &SemioValueSnapshot) -> Self {
        let mid = self.apply(base).unwrap();
        Self::between(&mid, base)
    }

    fn between(base: &SemioValueSnapshot, other: &SemioValueSnapshot) -> Self {
        let root = value_diff_between(&base.root, &other.root);
        let nodes_diff = nodes_diff_between(&base.nodes, &other.nodes);
        let nodes = if is_named_empty(&nodes_diff) { None } else { Some(nodes_diff) };
        SemioValueTreeDiff { root, nodes }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none() && self.nodes.is_none()
    }
}

/// 🧩 Builds the sparse `between(base, next)` diff for a `SetSnapshot` mutation — NOT a full
/// `snapshot: Option<SemioValueSnapshot>` replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &SemioValueSnapshot, next: &SemioValueSnapshot) -> SemioValueTreeDiff {
    SemioValueTreeDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️Apply
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_added_positions(indices: impl IntoIterator<Item = usize>, mut length: usize, target: impl IntoIterator<Item = impl Into<String>>) -> protocol::MutationApplyResult<()> {
    let target: Vec<String> = target.into_iter().map(Into::into).collect();
    let mut indices: Vec<usize> = indices.into_iter().collect();
    indices.sort_unstable();
    let mut previous = None;
    for index in indices {
        if index > length || previous == Some(index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("add index {index} is out of range or duplicated")).at(target.clone()));
        }
        previous = Some(index);
        length += 1;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_value_diff(diff: &SemioValueDiff, base: &SemioValue, target: Vec<String>) -> protocol::MutationApplyResult<()> {
    let kind_matches = matches!(
        (diff, base),
        (SemioValueDiff::Replace { .. }, _)
            | (SemioValueDiff::Bool { .. }, SemioValue::Bool { .. })
            | (SemioValueDiff::Int { .. }, SemioValue::Int { .. })
            | (SemioValueDiff::Float { .. }, SemioValue::Float { .. })
            | (SemioValueDiff::Str { .. }, SemioValue::Str { .. })
            | (SemioValueDiff::Bytes { .. }, SemioValue::Bytes { .. })
            | (SemioValueDiff::List { .. }, SemioValue::List { .. })
            | (SemioValueDiff::Map { .. }, SemioValue::Map { .. })
            | (SemioValueDiff::Ref { .. }, SemioValue::Ref { .. })
    );
    if !kind_matches {
        return Err(protocol::MutationApplyError::new("mutation.apply.value-kind-mismatch", "Semio value diff kind does not match the base value kind").at(target));
    }
    match (diff, base) {
        (SemioValueDiff::List { diff }, SemioValue::List { items }) => {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_indexed_triple(diff, items.len(), target.clone())?;
            for modified in &diff.modified {
                let mut nested = target.clone();
                nested.push(modified.index.to_string());
                validate_value_diff(&modified.diff, &items[modified.index], nested)?;
            }
        }
        (SemioValueDiff::Map { diff }, SemioValue::Map { entries }) => {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(entries, diff, |entry| entry.key.clone(), |added| added.item.key.clone(), target.clone())?;
            validate_added_positions(diff.added.iter().map(|added| added.index), entries.len() - diff.removed.len(), target.clone())?;
            for modified in &diff.modified {
                let entry = entries.iter().find(|entry| entry.key == modified.key).ok_or_else(|| semio_framework_plugin::resolve_ready(protocol::MutationApplyError::new("mutation.apply.missing-map-entry", format!("map entry {:?} is absent", modified.key))).at(target.clone()))?;
                let mut nested = target.clone();
                nested.push(modified.key.clone());
                validate_value_diff(&modified.diff, &entry.value, nested)?;
            }
        }
        _ => {}
    }
    Ok(())
}
/// ▶️ Applies a [`SemioValueDiff`] against the corresponding base node.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_value_diff(diff: &SemioValueDiff, base: &SemioValue) -> SemioValue {
    match diff {
        SemioValueDiff::Replace { value } => value.clone(),
        SemioValueDiff::Bool { value } => SemioValue::Bool { value: *value },
        SemioValueDiff::Int { lexeme } => SemioValue::Int { lexeme: lexeme.clone() },
        SemioValueDiff::Float { lexeme } => SemioValue::Float { lexeme: lexeme.clone() },
        SemioValueDiff::Str { value } => SemioValue::Str { value: value.clone() },
        SemioValueDiff::Bytes { value } => SemioValue::Bytes { value: value.clone() },
        SemioValueDiff::List { diff } => {
            let items: &[SemioValue] = match base {
                SemioValue::List { items } => items.as_slice(),
                _ => &[],
            };
            SemioValue::List { items: Box::pin(apply_list_diff(diff, items)) }
        }
        SemioValueDiff::Map { diff } => {
            let entries: &[SemioValueEntry] = match base {
                SemioValue::Map { entries } => entries.as_slice(),
                _ => &[],
            };
            SemioValue::Map { entries: Box::pin(apply_map_diff(diff, entries)) }
        }
        SemioValueDiff::Ref { id } => SemioValue::Ref { id: id.clone() },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_value_diff_to_named_node(diff: &SemioValueDiff, node: &NamedAdded<SemioValueNode>) -> NamedAdded<SemioValueNode> {
    NamedAdded { index: node.index, item: SemioValueNode { id: node.item.id.clone(), value: apply_value_diff(diff, &node.item.value) } }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_value_diff_to_named_entry(diff: &SemioValueDiff, entry: &NamedAdded<SemioValueEntry>) -> NamedAdded<SemioValueEntry> {
    NamedAdded { index: entry.index, item: SemioValueEntry { key: entry.item.key.clone(), value: apply_value_diff(diff, &entry.item.value) } }
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert at
/// `min(index, len)`). Out-of-range indices are graceful no-ops.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_list_diff(diff: &IndexedTripleDiff<SemioValueDiff, SemioValue>, base: &[SemioValue]) -> Vec<SemioValue> {
    let mut items: Vec<SemioValue> = base.to_vec();
    for m in &diff.modified {
        if let Some(old) = base.get(m.index) {
            if let Some(slot) = items.get_mut(m.index) {
                *slot = Box::pin(apply_value_diff(&m.diff, old));
            }
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.dedup();
    for idx in removed_sorted.into_iter().rev() {
        if idx < items.len() {
            items.remove(idx);
        }
    }
    let mut added_sorted = diff.added.clone();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        let pos = a.index.min(items.len());
        items.insert(pos, a.item);
    }
    items
}

/// ▶️ Same normative apply semantics as [`apply_list_diff`], keyed by member name instead of
/// position — `added` entries carry their own target position (see [`NamedAdded`]'s doc comment
/// for why the shared engine's generic `T` alone can't) and are inserted at `min(index, len)`,
/// ascending, exactly mirroring [`apply_list_diff`]'s own index-added handling.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_map_diff(diff: &NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>, base: &[SemioValueEntry]) -> Vec<SemioValueEntry> {
    let mut entries: Vec<SemioValueEntry> = base.to_vec();
    for m in &diff.modified {
        if let Some(pos) = entries.iter().position(|e| e.key == m.key) {
            let old = entries[pos].value.clone();
            entries[pos].value = Box::pin(apply_value_diff(&m.diff, &old));
        }
    }
    for key in &diff.removed {
        if let Some(pos) = entries.iter().position(|e| &e.key == key) {
            entries.remove(pos);
        }
    }
    let mut added_sorted = diff.added.clone();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        let pos = a.index.min(entries.len());
        entries.insert(pos, a.item);
    }
    entries
}

/// ▶️ Same shape as [`apply_map_diff`] but keyed by [`ValueId`] over the top-level value graph.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_nodes_diff(diff: &NamedTripleDiff<ValueId, SemioValueDiff, NamedAdded<SemioValueNode>>, base: &[SemioValueNode]) -> Vec<SemioValueNode> {
    let mut nodes: Vec<SemioValueNode> = base.to_vec();
    for m in &diff.modified {
        if let Some(pos) = nodes.iter().position(|n| n.id == m.key) {
            let old = nodes[pos].value.clone();
            nodes[pos].value = apply_value_diff(&m.diff, &old);
        }
    }
    for id in &diff.removed {
        if let Some(pos) = nodes.iter().position(|n| &n.id == id) {
            nodes.remove(pos);
        }
    }
    let mut added_sorted = diff.added.clone();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        let pos = a.index.min(nodes.len());
        nodes.insert(pos, a.item);
    }
    nodes
}
//#endregion 🔖️Apply

//#region 🔖️Between
/// 🧭️ State-delta construction: `None` when nodes are equal; a direct field diff when the KIND is
/// stable; `Replace` when it changed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn value_diff_between(a: &SemioValue, b: &SemioValue) -> Option<SemioValueDiff> {
    if a == b {
        return None;
    }
    match (a, b) {
        (SemioValue::Bool { .. }, SemioValue::Bool { value }) => Some(SemioValueDiff::Bool { value: *value }),
        (SemioValue::Int { .. }, SemioValue::Int { lexeme }) => Some(SemioValueDiff::Int { lexeme: lexeme.clone() }),
        (SemioValue::Float { .. }, SemioValue::Float { lexeme }) => Some(SemioValueDiff::Float { lexeme: lexeme.clone() }),
        (SemioValue::Str { .. }, SemioValue::Str { value }) => Some(SemioValueDiff::Str { value: value.clone() }),
        (SemioValue::Bytes { .. }, SemioValue::Bytes { value }) => Some(SemioValueDiff::Bytes { value: value.clone() }),
        (SemioValue::Ref { .. }, SemioValue::Ref { id }) => Some(SemioValueDiff::Ref { id: id.clone() }),
        (SemioValue::List { items: av }, SemioValue::List { items: bv }) => {
            let diff = list_diff_between(av, bv);
            if is_indexed_empty(&diff) {
                None
            } else {
                Some(SemioValueDiff::List { diff })
            }
        }
        (SemioValue::Map { entries: am }, SemioValue::Map { entries: bm }) => {
            let diff = map_diff_between(am, bm);
            if is_named_empty(&diff) {
                None
            } else {
                Some(SemioValueDiff::Map { diff })
            }
        }
        _ => Some(SemioValueDiff::Replace { value: b.clone() }),
    }
}

/// 🧭️ Index-pairwise: `modified` compares `0..min(len)`, `removed` is the base tail, `added` is
/// the other tail (final-state indices, per the normative apply contract).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn list_diff_between(a: &[SemioValue], b: &[SemioValue]) -> IndexedTripleDiff<SemioValueDiff, SemioValue> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if let Some(diff) = value_diff_between(&a[i], &b[i]) {
            modified.push(IndexModified { index: i, diff });
        }
    }
    let removed: Vec<usize> = if a.len() > b.len() { (b.len()..a.len()).collect() } else { Vec::new() };
    let added: Vec<IndexAdded<SemioValue>> = if b.len() > a.len() { (a.len()..b.len()).map(|i| IndexAdded { index: i, item: b[i].clone() }).collect() } else { Vec::new() };
    IndexedTripleDiff { removed, modified, added }
}

/// 🧭️ Name-keyed: base members missing from `b` are `removed`; members present in both with a
/// changed value are `modified`; members only in `b` are `added` AT THEIR `b`-POSITION (see
/// [`NamedAdded`]'s doc comment — renames are documented as `removed`+`added` — no rename
/// detection, matching `json`'s own `value_diff_between`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn map_diff_between(a: &[SemioValueEntry], b: &[SemioValueEntry]) -> NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for ae in a {
        match b.iter().find(|be| be.key == ae.key) {
            Some(be) => {
                if let Some(diff) = value_diff_between(&ae.value, &be.value) {
                    modified.push(NamedModified { key: ae.key.clone(), diff });
                }
            }
            None => removed.push(ae.key.clone()),
        }
    }
    let mut added = Vec::new();
    for (i, be) in b.iter().enumerate() {
        if !a.iter().any(|ae| ae.key == be.key) {
            added.push(NamedAdded { index: i, item: be.clone() });
        }
    }
    NamedTripleDiff { removed, modified, added }
}

/// 🧭️ Same shape as [`map_diff_between`], keyed by [`ValueId`] over the top-level value graph.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nodes_diff_between(a: &[SemioValueNode], b: &[SemioValueNode]) -> NamedTripleDiff<ValueId, SemioValueDiff, NamedAdded<SemioValueNode>> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for an in a {
        match b.iter().find(|bn| bn.id == an.id) {
            Some(bn) => {
                if let Some(diff) = value_diff_between(&an.value, &bn.value) {
                    modified.push(NamedModified { key: an.id.clone(), diff });
                }
            }
            None => removed.push(an.id.clone()),
        }
    }
    let mut added = Vec::new();
    for (i, bn) in b.iter().enumerate() {
        if !a.iter().any(|an| an.id == bn.id) {
            added.push(NamedAdded { index: i, item: bn.clone() });
        }
    }
    NamedTripleDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_indexed_empty<D, T>(d: &IndexedTripleDiff<D, T>) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_named_empty<K, D, T>(d: &NamedTripleDiff<K, D, T>) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

/// 🕳️ Whether a (possibly freshly-absorbed) node diff represents no actual change. Scalar
/// replace/field diffs are never "empty" in isolation, but a collection diff with nothing
/// removed/modified/added genuinely changes nothing and should collapse away rather than survive
/// as a no-op wrapper (same rationale `json`'s `is_value_diff_effectively_empty` documents).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_value_diff_effectively_empty(d: &SemioValueDiff) -> bool {
    match d {
        SemioValueDiff::List { diff } => is_indexed_empty(diff),
        SemioValueDiff::Map { diff } => is_named_empty(diff),
        _ => false,
    }
}
//#endregion 🔖️Between

//#region 🔖️Absorb
/// ➕️ Diff-level absorb (base→mid composed with mid→after). `d2` always wins on a full `Replace`;
/// a `Replace` in `d1` gets `d2` baked into its known literal value via `apply_value_diff`;
/// otherwise both sides share the same node KIND (guaranteed by construction against the real
/// intervening `mid` state) and compose per-kind, recursing into collections.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_value_diff(d1: SemioValueDiff, d2: SemioValueDiff) -> SemioValueDiff {
    if matches!(d2, SemioValueDiff::Replace { .. }) {
        return d2;
    }
    if let SemioValueDiff::Replace { value } = d1 {
        let merged = apply_value_diff(&d2, &value);
        return SemioValueDiff::Replace { value: merged };
    }
    match (d1, d2) {
        (SemioValueDiff::Bool { .. }, SemioValueDiff::Bool { value }) => SemioValueDiff::Bool { value },
        (SemioValueDiff::Int { .. }, SemioValueDiff::Int { lexeme }) => SemioValueDiff::Int { lexeme },
        (SemioValueDiff::Float { .. }, SemioValueDiff::Float { lexeme }) => SemioValueDiff::Float { lexeme },
        (SemioValueDiff::Str { .. }, SemioValueDiff::Str { value }) => SemioValueDiff::Str { value },
        (SemioValueDiff::Bytes { .. }, SemioValueDiff::Bytes { value }) => SemioValueDiff::Bytes { value },
        (SemioValueDiff::Ref { .. }, SemioValueDiff::Ref { id }) => SemioValueDiff::Ref { id },
        (SemioValueDiff::List { diff: a1 }, SemioValueDiff::List { diff: a2 }) => SemioValueDiff::List { diff: absorb_indexed(a1, a2, &absorb_value_diff, &apply_value_diff, &is_value_diff_effectively_empty) },
        (SemioValueDiff::Map { diff: o1 }, SemioValueDiff::Map { diff: o2 }) => {
            SemioValueDiff::Map { diff: absorb_named(o1, o2, &|e: &NamedAdded<SemioValueEntry>| e.item.key.clone(), &absorb_value_diff, &apply_value_diff_to_named_entry, &is_value_diff_effectively_empty) }
        }
        // Defensive: a kind mismatch that isn't a Replace shouldn't arise from two diffs produced
        // by real sequential application against the same intervening state — fall back to d2
        // (last-write-wins) rather than panicking.
        (_, other) => other,
    }
}

/// ➕️ Index-keyed absorb via symbolic position simulation, generic over the collection's item/diff
/// types — the SAME token-replay algorithm `json`'s `absorb_array_diff` uses (see that module's
/// doc comment for the full case-by-case citation), generalized so `List` is the only instantiation
/// site needed at THIS level (`nodes`/`Map` reuse [`absorb_named`] below instead).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed<D: Clone, T: Clone>(d1: IndexedTripleDiff<D, T>, d2: IndexedTripleDiff<D, T>, absorb_d: &impl Fn(D, D) -> D, apply_d_to_t: &impl Fn(&D, &T) -> T, is_d_empty: &impl Fn(&D) -> bool) -> IndexedTripleDiff<D, T> {
    #[derive(Clone, Copy)]
    enum Origin {
        Base(usize),
        D1Added(usize),
    }
    enum AfterSlot<D, T> {
        Base { orig: usize, diff: Option<D> },
        D1Added { tag: usize, patch: Option<D> },
        D2Added(T),
    }

    let max_ref = d1
        .removed
        .iter()
        .copied()
        .chain(d1.modified.iter().map(|m| m.index))
        .chain(d1.added.iter().map(|a| a.index))
        .chain(d2.removed.iter().copied())
        .chain(d2.modified.iter().map(|m| m.index))
        .chain(d2.added.iter().map(|a| a.index))
        .max()
        .unwrap_or(0);
    let n = max_ref + d1.removed.len() + d2.removed.len() + 64;

    // Step A: base -> mid.
    let mut mid: Vec<Origin> = (0..n).map(Origin::Base).collect();
    let mut d1_removed_sorted = d1.removed.clone();
    d1_removed_sorted.sort_unstable();
    d1_removed_sorted.dedup();
    for idx in d1_removed_sorted.iter().rev() {
        if *idx < mid.len() {
            mid.remove(*idx);
        }
    }
    let mut d1_added_order: Vec<usize> = (0..d1.added.len()).collect();
    d1_added_order.sort_by_key(|&tag| d1.added[tag].index);
    for tag in d1_added_order {
        let pos = d1.added[tag].index.min(mid.len());
        mid.insert(pos, Origin::D1Added(tag));
    }
    let d1_modified: std::collections::HashMap<usize, D> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();

    // Step B: mid -> after.
    let mut after: Vec<AfterSlot<D, T>> = mid
        .iter()
        .map(|origin| match origin {
            Origin::Base(orig) => AfterSlot::Base { orig: *orig, diff: d1_modified.get(orig).cloned() },
            Origin::D1Added(tag) => AfterSlot::D1Added { tag: *tag, patch: None },
        })
        .collect();

    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut d2_removed_sorted = d2.removed.clone();
    d2_removed_sorted.sort_unstable();
    d2_removed_sorted.dedup();
    for idx in d2_removed_sorted.iter().rev() {
        if *idx < after.len() {
            match after.remove(*idx) {
                AfterSlot::Base { orig, .. } => final_removed.push(orig),
                AfterSlot::D1Added { .. } => {} // cancels the add: no removed entry, no added entry
                AfterSlot::D2Added(_) => {}
            }
        }
    }
    for m in &d2.modified {
        if let Some(slot) = after.get_mut(m.index) {
            match slot {
                AfterSlot::Base { diff, .. } => {
                    let combined = match diff.take() {
                        Some(existing) => absorb_d(existing, m.diff.clone()),
                        None => m.diff.clone(),
                    };
                    *diff = if is_d_empty(&combined) { None } else { Some(combined) };
                }
                AfterSlot::D1Added { patch, .. } => {
                    let combined = match patch.take() {
                        Some(existing) => absorb_d(existing, m.diff.clone()),
                        None => m.diff.clone(),
                    };
                    *patch = if is_d_empty(&combined) { None } else { Some(combined) };
                }
                AfterSlot::D2Added(_) => {}
            }
        }
    }
    let mut d2_added_order: Vec<usize> = (0..d2.added.len()).collect();
    d2_added_order.sort_by_key(|&tag| d2.added[tag].index);
    for tag in d2_added_order {
        let pos = d2.added[tag].index.min(after.len());
        after.insert(pos, AfterSlot::D2Added(d2.added[tag].item.clone()));
    }

    // Step C: walk `after`, emitting the combined triple.
    let mut modified = Vec::new();
    let mut added = Vec::new();
    for (pos, slot) in after.into_iter().enumerate() {
        match slot {
            AfterSlot::Base { orig, diff: Some(diff) } => modified.push(IndexModified { index: orig, diff }),
            AfterSlot::Base { .. } => {}
            AfterSlot::D1Added { tag, patch } => {
                let mut item = d1.added[tag].item.clone();
                if let Some(patch) = patch {
                    item = apply_d_to_t(&patch, &item);
                }
                added.push(IndexAdded { index: pos, item });
            }
            AfterSlot::D2Added(item) => added.push(IndexAdded { index: pos, item }),
        }
    }
    final_removed.sort_unstable();
    final_removed.dedup();
    IndexedTripleDiff { removed: final_removed, modified, added }
}

/// ➕️ Name/id-keyed absorb, generic over the key type `K` (`String` for `Map`, [`ValueId`] for
/// `nodes`) via an explicit `key_of` extractor — resolution of WHICH entry a `d2` op refers to
/// is exact (key/id identity), but a surviving `d1`-added entry's relative position among OTHER
/// entries is not renegotiated by unrelated `d2` removals elsewhere (name/id identity carries no
/// positional information base-free, unlike list indices) — exact for every realistic mutation
/// pattern (new entries always appended, see `SetMapEntry`/`SetNode`'s own diff construction)
/// and every canonical `absorb_law` case tested below, same documented shape `json`'s own
/// `absorb_value_diff` carries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K: Clone + PartialEq, D, T: Clone>(
    d1: NamedTripleDiff<K, D, T>,
    d2: NamedTripleDiff<K, D, T>,
    key_of: &impl Fn(&T) -> K,
    absorb_d: &impl Fn(D, D) -> D,
    apply_d_to_t: &impl Fn(&D, &T) -> T,
    is_d_empty: &impl Fn(&D) -> bool,
) -> NamedTripleDiff<K, D, T> {
    let mut removed: Vec<K> = d1.removed;
    let mut modified: Vec<NamedModified<K, D>> = d1.modified;
    let mut added: Vec<T> = d1.added;
    let mut merged_removed: Vec<K> = Vec::new();

    for key in d2.removed {
        if let Some(pos) = added.iter().position(|t| key_of(t) == key) {
            added.remove(pos);
        } else if let Some(pos) = modified.iter().position(|m| m.key == key) {
            modified.remove(pos);
            if !merged_removed.contains(&key) {
                merged_removed.push(key.clone());
                removed.push(key);
            }
        } else if !merged_removed.contains(&key) {
            merged_removed.push(key.clone());
            removed.push(key);
        }
    }
    for m in d2.modified {
        if let Some(t) = added.iter_mut().find(|t| key_of(t) == m.key) {
            let updated = apply_d_to_t(&m.diff, t);
            *t = updated;
        } else if let Some(pos) = modified.iter().position(|e| e.key == m.key) {
            let existing = modified.remove(pos);
            let combined = absorb_d(existing.diff, m.diff);
            if !is_d_empty(&combined) {
                modified.push(NamedModified { key: m.key, diff: combined });
            }
        } else {
            modified.push(NamedModified { key: m.key, diff: m.diff });
        }
    }
    for a in d2.added {
        added.push(a);
    }
    NamedTripleDiff { removed, modified, added }
}
//#endregion 🔖️Absorb

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` for `SemioValueTreeDiff` — grammar template copied from
/// `JsonDiff`'s (this subset's own informing source).
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
pub(crate) fn enc_value_id(id: &ValueId) -> String {
    enc_str(&id.value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value_id(s: &str) -> Result<ValueId, String> {
    Ok(ValueId::new(dec_str(s)?))
}
//#endregion 🔖️Primitives

//#region 🔖️SemioValueCodecs
/// 🌳 Tag-prefixed like `json`'s `enc_json_value`: `Z` (null, no payload, no brackets) / `B[0|1]`
/// / `I[hex(lexeme)]` / `F[hex(lexeme)]` / `S[hex(value)]` / `Y[hex(bytes)]` / `L[v1,v2,...]` /
/// `M[hexkey1:v1,hexkey2:v2,...]` / `R[hex(id)]` — member insertion order preserved by
/// construction (a list, never re-sorted).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value(v: &SemioValue) -> String {
    match v {
        SemioValue::Null => "Z".to_string(),
        SemioValue::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        SemioValue::Int { lexeme } => format!("I[{}]", enc_str(lexeme)),
        SemioValue::Float { lexeme } => format!("F[{}]", enc_str(lexeme)),
        SemioValue::Str { value } => format!("S[{}]", enc_str(value)),
        SemioValue::Bytes { value } => format!("Y[{}]", hex_encode(value)),
        SemioValue::List { items } => format!("L[{}]", items.iter().map(enc_semio_value).collect::<Vec<_>>().join(",")),
        SemioValue::Map { entries } => format!("M[{}]", entries.iter().map(|e| format!("{}:{}", enc_str(&e.key), enc_semio_value(&e.value))).collect::<Vec<_>>().join(",")),
        SemioValue::Ref { id } => format!("R[{}]", enc_value_id(id)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value(s: &str) -> Result<SemioValue, String> {
    if s == "Z" {
        return Ok(SemioValue::Null);
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "B" => Ok(SemioValue::Bool { value: inner == "1" }),
        "I" => Ok(SemioValue::Int { lexeme: dec_str(inner)? }),
        "F" => Ok(SemioValue::Float { lexeme: dec_str(inner)? }),
        "S" => Ok(SemioValue::Str { value: dec_str(inner)? }),
        "Y" => Ok(SemioValue::Bytes { value: hex_decode(inner)? }),
        "L" => Ok(SemioValue::List { items: split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_semio_value).collect::<Result<Vec<_>, String>>()? }),
        "M" => {
            let entries = split_top_level(inner, ',')
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|entry| {
                    let (key, value) = entry.split_once(':').ok_or_else(|| format!("map entry: bad entry {entry:?}"))?;
                    Ok(SemioValueEntry { key: dec_str(key)?, value: dec_semio_value(value)? })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(SemioValue::Map { entries })
        }
        "R" => Ok(SemioValue::Ref { id: dec_value_id(inner)? }),
        other => Err(format!("semio value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value_entry(e: &SemioValueEntry) -> String {
    format!("{}:{}", enc_str(&e.key), enc_semio_value(&e.value))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value_entry(s: &str) -> Result<SemioValueEntry, String> {
    let (key, value) = s.split_once(':').ok_or_else(|| format!("value entry: bad entry {s:?}"))?;
    Ok(SemioValueEntry { key: dec_str(key)?, value: dec_semio_value(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value_node(n: &SemioValueNode) -> String {
    format!("{}:{}", enc_value_id(&n.id), enc_semio_value(&n.value))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value_node(s: &str) -> Result<SemioValueNode, String> {
    let (id, value) = s.split_once(':').ok_or_else(|| format!("value node: bad entry {s:?}"))?;
    Ok(SemioValueNode { id: dec_value_id(id)?, value: dec_semio_value(value)? })
}

/// 🧷 `NamedAdded<T>`-wrapping variants of the two encoders above — `index:` prefixed — used ONLY
/// for a diff's own `added` list (see [`NamedAdded`]'s doc comment); the plain (unwrapped)
/// encoders above stay the ones `🧬️mutations`' snapshot-level `nodes` list encoding uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_entry(a: &NamedAdded<SemioValueEntry>) -> String {
    format!("{}:{}", a.index, enc_semio_value_entry(&a.item))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_entry(s: &str) -> Result<NamedAdded<SemioValueEntry>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added entry: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_semio_value_entry(rest)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_node(a: &NamedAdded<SemioValueNode>) -> String {
    format!("{}:{}", a.index, enc_semio_value_node(&a.item))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_node(s: &str) -> Result<NamedAdded<SemioValueNode>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added node: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_semio_value_node(rest)? })
}
//#endregion 🔖️SemioValueCodecs

//#region 🔖️DiffValueCodecs
/// 🌳 `SemioValueDiff` itself needs a tag (`P`=rePlace, `B`=Bool, `I`=Int, `F`=Float, `S`=Str,
/// `Y`=Bytes, `L`=List, `M`=Map, `R`=Ref) since, unlike a plain [`SemioValue`], it appears
/// standalone (not always inside a bracketed container) at the top-level `root=`/`nodes=` token
/// position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value_diff(d: &SemioValueDiff) -> String {
    match d {
        SemioValueDiff::Replace { value } => format!("P[{}]", enc_semio_value(value)),
        SemioValueDiff::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        SemioValueDiff::Int { lexeme } => format!("I[{}]", enc_str(lexeme)),
        SemioValueDiff::Float { lexeme } => format!("F[{}]", enc_str(lexeme)),
        SemioValueDiff::Str { value } => format!("S[{}]", enc_str(value)),
        SemioValueDiff::Bytes { value } => format!("Y[{}]", hex_encode(value)),
        SemioValueDiff::List { diff } => format!("L[{}.await]", enc_indexed_triple(diff, enc_value_diff, enc_semio_value)),
        SemioValueDiff::Map { diff } => format!("M[{}.await]", enc_named_triple(diff, |k: &String| enc_str(k), enc_value_diff, enc_named_added_entry)),
        SemioValueDiff::Ref { id } => format!("R[{}]", enc_value_id(id)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value_diff(s: &str) -> Result<SemioValueDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "P" => Ok(SemioValueDiff::Replace { value: dec_semio_value(inner)? }),
        "B" => Ok(SemioValueDiff::Bool { value: inner == "1" }),
        "I" => Ok(SemioValueDiff::Int { lexeme: dec_str(inner)? }),
        "F" => Ok(SemioValueDiff::Float { lexeme: dec_str(inner)? }),
        "S" => Ok(SemioValueDiff::Str { value: dec_str(inner)? }),
        "Y" => Ok(SemioValueDiff::Bytes { value: hex_decode(inner)? }),
        "L" => Ok(SemioValueDiff::List { diff: dec_indexed_triple(inner, dec_value_diff, dec_semio_value)? }),
        "M" => Ok(SemioValueDiff::Map { diff: dec_named_triple(inner, dec_str, dec_value_diff, dec_named_added_entry)? }),
        "R" => Ok(SemioValueDiff::Ref { id: dec_value_id(inner)? }),
        other => Err(format!("semio value diff: unknown tag {other:?}")),
    }
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️BinaryPrimitives
/// 🧪️ Real length-prefixed binary primitives (`store::pack_rt::write_varint_u64`/
/// `store::ByteReader`), the genuinely-recursive twin of `hex_encode`/`hex_decode` above —
/// template copied verbatim from `json`'s own `write_bytes_lp`/`read_bytes_lp`/`write_str_lp`/
/// `read_str_lp` (`🔣️json/…/🔺️diff/🦀️component.rs`).
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

//#region 🔖️SemioValueBinaryCodecs
/// 🌳️ Real recursive binary twin of [`enc_semio_value`]/[`dec_semio_value`] — a 1-byte kind tag
/// (`0`=Null/`1`=Bool/`2`=Int/`3`=Float/`4`=Str/`5`=Bytes/`6`=List/`7`=Map/`8`=Ref) followed by the
/// real payload (length-prefixed bytes for scalars, a varint COUNT then that many recursively
/// encoded elements for `List`/`Map` — genuinely recursive, not text-as-bytes). Template copied
/// from json's `enc_json_value_bin`/`dec_json_value_bin`. Backs the upgraded `DiffCodec`/`OpBinary`
/// frames (this file's own `encode_diff`/`decode_diff`, and the sibling `🧬️mutations/🦀️component.rs`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value_bin(value: &SemioValue, out: &mut Vec<u8>) {
    match value {
        SemioValue::Null => out.push(0),
        SemioValue::Bool { value } => {
            out.push(1);
            out.push(if *value { 1 } else { 0 });
        }
        SemioValue::Int { lexeme } => {
            out.push(2);
            write_str_lp(out, lexeme);
        }
        SemioValue::Float { lexeme } => {
            out.push(3);
            write_str_lp(out, lexeme);
        }
        SemioValue::Str { value } => {
            out.push(4);
            write_str_lp(out, value);
        }
        SemioValue::Bytes { value } => {
            out.push(5);
            write_bytes_lp(out, value);
        }
        SemioValue::List { items } => {
            out.push(6);
            store::pack_rt::write_varint_u64(out, items.len() as u64);
            for item in items {
                enc_semio_value_bin(item, out);
            }
        }
        SemioValue::Map { entries } => {
            out.push(7);
            store::pack_rt::write_varint_u64(out, entries.len() as u64);
            for entry in entries {
                write_str_lp(out, &entry.key);
                enc_semio_value_bin(&entry.value, out);
            }
        }
        SemioValue::Ref { id } => {
            out.push(8);
            write_str_lp(out, &id.value);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(SemioValue::Null),
        1 => Ok(SemioValue::Bool { value: reader.read_u8().map_err(|e| e.to_string())? != 0 }),
        2 => Ok(SemioValue::Int { lexeme: read_str_lp(reader)? }),
        3 => Ok(SemioValue::Float { lexeme: read_str_lp(reader)? }),
        4 => Ok(SemioValue::Str { value: read_str_lp(reader)? }),
        5 => Ok(SemioValue::Bytes { value: read_bytes_lp(reader)? }),
        6 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut items = Vec::with_capacity(count as usize);
            for _ in 0..count {
                items.push(Box::pin(dec_semio_value_bin(reader))?);
            }
            Ok(SemioValue::List { items })
        }
        7 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut entries = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let key = read_str_lp(reader)?;
                let value = Box::pin(dec_semio_value_bin(reader))?;
                entries.push(SemioValueEntry { key, value });
            }
            Ok(SemioValue::Map { entries })
        }
        8 => Ok(SemioValue::Ref { id: ValueId::new(read_str_lp(reader)?) }),
        other => Err(format!("semio value binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value_node_bin(node: &SemioValueNode, out: &mut Vec<u8>) {
    write_str_lp(out, &node.id.value);
    enc_semio_value_bin(&node.value, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value_node_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioValueNode, String> {
    let id = ValueId::new(read_str_lp(reader)?);
    let value = dec_semio_value_bin(reader)?;
    Ok(SemioValueNode { id, value })
}
//#endregion 🔖️SemioValueBinaryCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🌳️ Real recursive binary twin of [`enc_value_diff`]/[`dec_value_diff`] — tag numbering distinct
/// from the text codec's letter tags (`0`=Replace/`1`=Bool/`2`=Int/`3`=Float/`4`=Str/`5`=Bytes/
/// `6`=List/`7`=Map/`8`=Ref), same shape json's `enc_value_diff_bin`/`dec_value_diff_bin` uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value_diff_bin(d: &SemioValueDiff, out: &mut Vec<u8>) {
    match d {
        SemioValueDiff::Replace { value } => {
            out.push(0);
            enc_semio_value_bin(value, out);
        }
        SemioValueDiff::Bool { value } => {
            out.push(1);
            out.push(if *value { 1 } else { 0 });
        }
        SemioValueDiff::Int { lexeme } => {
            out.push(2);
            write_str_lp(out, lexeme);
        }
        SemioValueDiff::Float { lexeme } => {
            out.push(3);
            write_str_lp(out, lexeme);
        }
        SemioValueDiff::Str { value } => {
            out.push(4);
            write_str_lp(out, value);
        }
        SemioValueDiff::Bytes { value } => {
            out.push(5);
            write_bytes_lp(out, value);
        }
        SemioValueDiff::List { diff } => {
            out.push(6);
            enc_indexed_diff_bin(diff, out);
        }
        SemioValueDiff::Map { diff } => {
            out.push(7);
            enc_map_diff_bin(diff, out);
        }
        SemioValueDiff::Ref { id } => {
            out.push(8);
            write_str_lp(out, &id.value);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioValueDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(SemioValueDiff::Replace { value: dec_semio_value_bin(reader)? }),
        1 => Ok(SemioValueDiff::Bool { value: reader.read_u8().map_err(|e| e.to_string())? != 0 }),
        2 => Ok(SemioValueDiff::Int { lexeme: read_str_lp(reader)? }),
        3 => Ok(SemioValueDiff::Float { lexeme: read_str_lp(reader)? }),
        4 => Ok(SemioValueDiff::Str { value: read_str_lp(reader)? }),
        5 => Ok(SemioValueDiff::Bytes { value: read_bytes_lp(reader)? }),
        6 => Ok(SemioValueDiff::List { diff: Box::pin(dec_indexed_diff_bin(reader))? }),
        7 => Ok(SemioValueDiff::Map { diff: Box::pin(dec_map_diff_bin(reader))? }),
        8 => Ok(SemioValueDiff::Ref { id: ValueId::new(read_str_lp(reader)?) }),
        other => Err(format!("semio value diff binary: unknown tag {other}")),
    }
}

/// 🌳️ `List`'s `IndexedTripleDiff<SemioValueDiff, SemioValue>` — varint COUNT then that many
/// entries per section, same shape json's `enc_array_diff_bin`/`dec_array_diff_bin` uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_indexed_diff_bin(diff: &IndexedTripleDiff<SemioValueDiff, SemioValue>, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for index in &diff.removed {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_value_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_semio_value_bin(&entry.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_indexed_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<IndexedTripleDiff<SemioValueDiff, SemioValue>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = Box::pin(dec_value_diff_bin(reader))?;
        modified.push(IndexModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_semio_value_bin(reader)?;
        added.push(IndexAdded { index, item });
    }
    Ok(IndexedTripleDiff { removed, modified, added })
}

/// 🌳️ `Map`'s `NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>` — `added`
/// entries carry their own `index` (see [`NamedAdded`]'s doc comment), same shape json's
/// `enc_value_diff_bin`/`dec_value_diff_bin` uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_map_diff_bin(diff: &NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for key in &diff.removed {
        write_str_lp(out, key);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        write_str_lp(out, &entry.key);
        enc_value_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        write_str_lp(out, &entry.item.key);
        enc_semio_value_bin(&entry.item.value, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_map_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(read_str_lp(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = read_str_lp(reader)?;
        let diff = Box::pin(dec_value_diff_bin(reader))?;
        modified.push(NamedModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let key = read_str_lp(reader)?;
        let value = dec_semio_value_bin(reader)?;
        added.push(NamedAdded { index, item: SemioValueEntry { key, value } });
    }
    Ok(NamedTripleDiff { removed, modified, added })
}

/// 🌳️ The top-level `nodes` GRAPH's `NamedTripleDiff<ValueId, SemioValueDiff,
/// NamedAdded<SemioValueNode>>` — same shape as [`enc_map_diff_bin`]/[`dec_map_diff_bin`], keyed
/// by [`ValueId`] instead of a plain map-entry key.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_nodes_diff_bin(diff: &NamedTripleDiff<ValueId, SemioValueDiff, NamedAdded<SemioValueNode>>, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for id in &diff.removed {
        write_str_lp(out, &id.value);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        write_str_lp(out, &entry.key.value);
        enc_value_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_semio_value_node_bin(&entry.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_nodes_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<NamedTripleDiff<ValueId, SemioValueDiff, NamedAdded<SemioValueNode>>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(ValueId::new(read_str_lp(reader)?));
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = ValueId::new(read_str_lp(reader)?);
        let diff = dec_value_diff_bin(reader)?;
        modified.push(NamedModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_semio_value_node_bin(reader)?;
        added.push(NamedAdded { index, item });
    }
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel
/// 🧭️ Two-field top level (`root=<enc>` / `nodes=<enc>`, either absent = unchanged).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_value_tree_diff(d: &SemioValueTreeDiff) -> String {
    let mut tokens = Vec::new();
    if let Some(v) = &d.root {
        tokens.push(format!("root={}", enc_value_diff(v)));
    }
    if let Some(o) = &d.nodes {
        tokens.push(format!("nodes={}", enc_named_triple(o, enc_value_id, enc_value_diff, enc_named_added_node)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_value_tree_diff(line: &str) -> Result<SemioValueTreeDiff, String> {
    let mut d = SemioValueTreeDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("root=") {
            d.root = Some(dec_value_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("nodes=") {
            d.nodes = Some(dec_named_triple(rest, dec_value_id, dec_value_diff, dec_named_added_node)?);
        } else {
            return Err(format!("semio value diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for SemioValueTreeDiff {
    fn print_diff(&self) -> String {
        print_value_tree_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_value_tree_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ Real binary frame (`format u8 | presence u8 | root? | nodes?`), matching
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
    /// upgraded from the `print_diff().into_bytes()` text-as-binary shortcut this facet started
    /// with. `presence` bit0=`root` present, bit1=`nodes` present (both real, individually
    /// protocol-walkable fixed fields); the recursive `SemioValueDiff`/`NamedTripleDiff` payloads
    /// are real LEB128-varint-framed binary ([`enc_value_diff_bin`]/[`enc_nodes_diff_bin`]
    /// above), honestly opaque only at the PROTOCOL-DESCRIPTION layer (`Prim::Ref` can't recurse —
    /// recipe §5's `protocol-prim-ref-recursion` gap), genuinely structured and round-trip tested
    /// at the Rust layer — same treatment json's own `JsonDiff::encode_diff` uses.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let presence: u8 = (if self.root.is_some() { 1 } else { 0 }) | (if self.nodes.is_some() { 2 } else { 0 });
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, presence];
        if let Some(root) = &self.root {
            enc_value_diff_bin(root, &mut out);
        }
        if let Some(nodes) = &self.nodes {
            enc_nodes_diff_bin(nodes, &mut out);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let presence = reader.read_u8().map_err(|e| malformed("diff presence", 1, e.to_string()))?;
        let root = if presence & 1 != 0 { Some(dec_value_diff_bin(&mut reader).map_err(|e| malformed("diff root", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let nodes = if presence & 2 != 0 { Some(dec_nodes_diff_bin(&mut reader).map_err(|e| malformed("diff nodes", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        Ok(SemioValueTreeDiff { root, nodes })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ Representative `SemioValueTreeDiff` values (every `SemioValueDiff` variant incl. the `Replace`
/// kind-change fallback, nested list/map/nodes-graph collection triples, and the empty/`None`
/// diff) — the single source of truth reused by `diff_codec_text_binary_roundtrip_law` below AND by
/// `🎹️composer/🦀️component.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests, same convention json's own `demo_diff_cases` uses.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioValueTreeDiff> {
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snap(root: SemioValue, nodes: Vec<SemioValueNode>) -> SemioValueSnapshot {
        SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn listv(items: Vec<SemioValue>) -> SemioValue {
        SemioValue::List { items }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn mapv(pairs: Vec<(&str, SemioValue)>) -> SemioValue {
        SemioValue::Map { entries: pairs.into_iter().map(|(k, v)| SemioValueEntry { key: k.into(), value: v }).collect() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn intv(lexeme: &str) -> SemioValue {
        SemioValue::Int { lexeme: lexeme.into() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn strv(s: &str) -> SemioValue {
        SemioValue::Str { value: s.into() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn node(id: &str, value: SemioValue) -> SemioValueNode {
        SemioValueNode { id: ValueId::new(id), value }
    }

    let a = snap(mapv(vec![("keepInt", intv("1")), ("kindChange", intv("1")), ("keepBytes", SemioValue::Bytes { value: vec![1, 2, 3] })]), vec![node("n1", strv("kept")), node("n2", strv("removed-node"))]);
    let b = snap(mapv(vec![("keepInt", intv("2")), ("kindChange", strv("now a string")), ("keepBytes", SemioValue::Bytes { value: vec![4, 5] })]), vec![node("n1", strv("kept")), node("n3", strv("added-node"))]);
    let nested = mapv(vec![("tags", listv(vec![strv("x"), strv("y"), strv("z")])), ("meta", mapv(vec![("a", intv("1")), ("b", SemioValue::Null)]))]);
    let nested2 = mapv(vec![("tags", listv(vec![strv("x"), strv("w")])), ("meta", mapv(vec![("a", intv("9")), ("c", strv("new"))])), ("extra", SemioValue::Bool { value: true })]);

    vec![
        SemioValueTreeDiff::default(),
        SemioValueTreeDiff::between(&a, &b),
        SemioValueTreeDiff::between(&b, &a),
        SemioValueTreeDiff::between(&snap(nested.clone(), vec![]), &snap(nested2.clone(), vec![])),
        SemioValueTreeDiff::between(&snap(nested2, vec![]), &snap(nested, vec![])),
        SemioValueTreeDiff::between(&snap(intv("1"), vec![]), &snap(strv("1"), vec![])),
        SemioValueTreeDiff::between(&snap(SemioValue::Null, vec![]), &snap(listv(vec![intv("1"), intv("2")]), vec![])),
        SemioValueTreeDiff::between(&snap(SemioValue::Null, vec![]), &snap(SemioValue::Null, vec![node("z", SemioValue::Bytes { value: vec![9, 8, 7] })])),
        SemioValueTreeDiff::between(&snap(SemioValue::Ref { id: ValueId::new("a") }, vec![]), &snap(SemioValue::Ref { id: ValueId::new("b") }, vec![])),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
    use std::collections::HashMap;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snap(root: SemioValue, nodes: Vec<SemioValueNode>) -> SemioValueSnapshot {
        SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn listv(items: Vec<SemioValue>) -> SemioValue {
        SemioValue::List { items }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn mapv(pairs: Vec<(&str, SemioValue)>) -> SemioValue {
        SemioValue::Map { entries: pairs.into_iter().map(|(k, v)| SemioValueEntry { key: k.into(), value: v }).collect() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn intv(lexeme: &str) -> SemioValue {
        SemioValue::Int { lexeme: lexeme.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn floatv(lexeme: &str) -> SemioValue {
        SemioValue::Float { lexeme: lexeme.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn strv(s: &str) -> SemioValue {
        SemioValue::Str { value: s.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn refv(id: &str) -> SemioValue {
        SemioValue::Ref { id: ValueId::new(id) }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn node(id: &str, value: SemioValue) -> SemioValueNode {
        SemioValueNode { id: ValueId::new(id), value }
    }

    //#region between_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law_scalars_and_kind_change() {
        let cases = [
            (SemioValue::Null, SemioValue::Bool { value: true }),
            (SemioValue::Bool { value: true }, SemioValue::Bool { value: false }),
            (intv("1"), intv("2")),
            (floatv("1.0"), floatv("2.5e10")),
            (strv("a"), strv("b")),
            (SemioValue::Bytes { value: vec![1, 2] }, SemioValue::Bytes { value: vec![3] }),
            (refv("a"), refv("b")),
            (intv("1"), strv("1")),
        ];
        for (a, b) in cases {
            let (sa, sb) = (snap(a.clone(), vec![]), snap(b.clone(), vec![]));
            assert_eq!(SemioValueTreeDiff::between(&sa, &sb).apply(&sa).expect("apply must succeed for a well-formed fixture"), sb, "a={a:?} b={b:?}");
            assert_eq!(SemioValueTreeDiff::between(&sb, &sa).apply(&sb).expect("apply must succeed for a well-formed fixture"), sa);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law_nested_collections_and_graph() {
        let a = snap(mapv(vec![("tags", listv(vec![strv("x"), strv("y")])), ("n", intv("1"))]), vec![node("n1", strv("hello"))]);
        let b = snap(mapv(vec![("tags", listv(vec![strv("x"), strv("z"), strv("w")])), ("n", intv("2")), ("extra", refv("n1"))]), vec![node("n1", strv("world")), node("n2", intv("9"))]);
        assert_eq!(SemioValueTreeDiff::between(&a, &b).apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        assert_eq!(SemioValueTreeDiff::between(&b, &a).apply(&b).expect("apply must succeed for a well-formed fixture"), a);
    }

    #[semio_framework_async_macros::async_test]
    async fn between_self_is_empty() {
        let a = snap(mapv(vec![("x", intv("1"))]), vec![node("n1", strv("v"))]);
        assert!(SemioValueTreeDiff::between(&a, &a).is_empty());
    }
    //#endregion between_roundtrip_law

    //#region inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law_diff_level() {
        let a = snap(mapv(vec![("x", intv("1")), ("y", listv(vec![intv("1"), intv("2")]))]), vec![node("n1", strv("a"))]);
        let b = snap(mapv(vec![("x", intv("2")), ("z", strv("new"))]), vec![node("n1", strv("b")), node("n2", intv("5"))]);
        let d = SemioValueTreeDiff::between(&a, &b);
        let mid = d.apply(&a).expect("apply must succeed for a well-formed fixture");
        assert_eq!(mid, b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&mid).expect("apply must succeed for a well-formed fixture"), a);
    }
    //#endregion inverse_law

    //#region absorb_law canonical cases (list/index-keyed)
    // NOTE: these construct `d1`/`d2` DIRECTLY as genuine Insert/Remove/Modify list diffs (matching
    // exactly what `InsertListItem`/`RemoveListItem`/`SetValue` would produce) rather than via
    // `SemioValueTreeDiff::between(base, next)` — same rationale `json`'s own absorb tests document.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn list_diff(d: IndexedTripleDiff<SemioValueDiff, SemioValue>) -> SemioValueTreeDiff {
        SemioValueTreeDiff { root: Some(SemioValueDiff::List { diff: d }), nodes: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_list_insert_then_remove_before() {
        // base = [a,b,c]; d1 = Insert(2,f) -> mid=[a,b,f,c]; d2 = Remove(0) -> after=[b,f,c].
        let base = snap(listv(vec![strv("a"), strv("b"), strv("c")]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: strv("f") }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
        assert_eq!(sequential.root, listv(vec![strv("b"), strv("f"), strv("c")]));
        match &combined.root {
            Some(SemioValueDiff::List { diff }) => {
                assert_eq!(diff.removed, vec![0]);
                assert_eq!(diff.added, vec![IndexAdded { index: 1, item: strv("f") }]);
            }
            other => panic!("expected list diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_list_insert_insert_same_index_both_survive() {
        let base = snap(listv(vec![strv("a"), strv("b")]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: strv("f") }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: strv("g") }], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
        assert_eq!(sequential.root, listv(vec![strv("a"), strv("b"), strv("g"), strv("f")]));
        match &combined.root {
            Some(SemioValueDiff::List { diff }) => assert_eq!(diff.added.len(), 2, "both inserts must survive"),
            other => panic!("expected list diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_list_insert_then_remove_of_same_added_item_cancels() {
        let base = snap(listv(vec![strv("a")]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 1, item: strv("f") }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff { removed: vec![1], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
        assert_eq!(sequential, base);
        assert!(combined.is_empty(), "cancelling insert+remove must coalesce to an empty diff");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_list_add_then_setfield_patches_added_payload() {
        let base = snap(listv(vec![]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 0, item: mapv(vec![("x", intv("1"))]) }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff {
            modified: vec![IndexModified { index: 0, diff: SemioValueDiff::Map { diff: NamedTripleDiff { added: vec![NamedAdded { index: 1, item: SemioValueEntry { key: "y".into(), value: intv("2") } }], ..Default::default() } } }],
            ..Default::default()
        });
        let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
        assert_eq!(sequential.root, listv(vec![mapv(vec![("x", intv("1")), ("y", intv("2"))])]));
        match &combined.root {
            Some(SemioValueDiff::List { diff }) => {
                assert!(diff.modified.is_empty(), "the patch must land INSIDE the carried added payload, not as a separate modified entry");
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].item, mapv(vec![("x", intv("1")), ("y", intv("2"))]));
            }
            other => panic!("expected list diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_list_modify_then_remove_drops_pending_patch() {
        let base = snap(listv(vec![intv("1"), intv("2")]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { modified: vec![IndexModified { index: 0, diff: SemioValueDiff::Int { lexeme: "9".into() } }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
        assert_eq!(sequential.root, listv(vec![intv("2")]));
        match &combined.root {
            Some(SemioValueDiff::List { diff }) => {
                assert_eq!(diff.removed, vec![0]);
                assert!(diff.modified.is_empty(), "the pending modify on the removed base index must be dropped");
            }
            other => panic!("expected list diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_list_associativity() {
        let s0 = snap(listv(vec![intv("1"), intv("2"), intv("3")]), vec![]);
        let s1 = snap(listv(vec![intv("1"), intv("9"), intv("3")]), vec![]);
        let s2 = snap(listv(vec![intv("9"), intv("3"), intv("4")]), vec![]);
        let s3 = snap(listv(vec![intv("9"), intv("4")]), vec![]);
        let d1 = SemioValueTreeDiff::between(&s0, &s1);
        let d2 = SemioValueTreeDiff::between(&s1, &s2);
        let d3 = SemioValueTreeDiff::between(&s2, &s3);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);

        assert_eq!(left.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
        assert_eq!(right.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (list/index-keyed)

    //#region absorb_law canonical cases (map/name-keyed)
    #[semio_framework_async_macros::async_test]
    async fn absorb_map_add_then_setfield_patches_added_payload() {
        let base = snap(mapv(vec![]), vec![]);
        let mid = snap(mapv(vec![("config", mapv(vec![]))]), vec![]);
        let after = snap(mapv(vec![("config", mapv(vec![("x", intv("5"))]))]), vec![]);
        let d1 = SemioValueTreeDiff::between(&base, &mid);
        let d2 = SemioValueTreeDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
        match &combined.root {
            Some(SemioValueDiff::Map { diff }) => {
                assert!(diff.modified.is_empty());
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].item.value, mapv(vec![("x", intv("5"))]));
            }
            other => panic!("expected map diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_map_modify_then_remove_drops_pending_patch() {
        let base = snap(mapv(vec![("a", intv("1")), ("b", intv("2"))]), vec![]);
        let mid = snap(mapv(vec![("a", intv("9")), ("b", intv("2"))]), vec![]);
        let after = snap(mapv(vec![("b", intv("2"))]), vec![]);
        let d1 = SemioValueTreeDiff::between(&base, &mid);
        let d2 = SemioValueTreeDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
        match &combined.root {
            Some(SemioValueDiff::Map { diff }) => {
                assert_eq!(diff.removed, vec!["a".to_string()]);
                assert!(diff.modified.is_empty());
            }
            other => panic!("expected map diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_map_insert_insert_both_survive() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let mid = snap(mapv(vec![("a", intv("1")), ("f", intv("2"))]), vec![]);
        let after = snap(mapv(vec![("a", intv("1")), ("f", intv("2")), ("g", intv("3"))]), vec![]);
        let d1 = SemioValueTreeDiff::between(&base, &mid);
        let d2 = SemioValueTreeDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
        match &combined.root {
            Some(SemioValueDiff::Map { diff }) => assert_eq!(diff.added.len(), 2),
            other => panic!("expected map diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_map_insert_then_remove_of_same_added_item_cancels() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let mid = snap(mapv(vec![("a", intv("1")), ("f", intv("2"))]), vec![]);
        let after = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let d1 = SemioValueTreeDiff::between(&base, &mid);
        let d2 = SemioValueTreeDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), base);
        assert!(combined.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_map_associativity() {
        let s0 = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let s1 = snap(mapv(vec![("a", intv("1")), ("b", intv("2"))]), vec![]);
        let s2 = snap(mapv(vec![("a", intv("9")), ("b", intv("2"))]), vec![]);
        let s3 = snap(mapv(vec![("b", intv("2")), ("c", intv("3"))]), vec![]);
        let d1 = SemioValueTreeDiff::between(&s0, &s1);
        let d2 = SemioValueTreeDiff::between(&s1, &s2);
        let d3 = SemioValueTreeDiff::between(&s2, &s3);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);

        assert_eq!(left.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
        assert_eq!(right.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (map/name-keyed)

    //#region absorb_law canonical cases (nodes graph / id-keyed)
    #[semio_framework_async_macros::async_test]
    async fn absorb_nodes_add_then_setfield_patches_added_payload() {
        let base = snap(SemioValue::Null, vec![]);
        let mid = snap(SemioValue::Null, vec![node("n1", mapv(vec![]))]);
        let after = snap(SemioValue::Null, vec![node("n1", mapv(vec![("x", intv("5"))]))]);
        let d1 = SemioValueTreeDiff::between(&base, &mid);
        let d2 = SemioValueTreeDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
        match &combined.nodes {
            Some(diff) => {
                assert!(diff.modified.is_empty());
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].item.value, mapv(vec![("x", intv("5"))]));
            }
            None => panic!("expected an nodes diff"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_nodes_modify_then_remove_drops_pending_patch() {
        let base = snap(SemioValue::Null, vec![node("a", intv("1")), node("b", intv("2"))]);
        let mid = snap(SemioValue::Null, vec![node("a", intv("9")), node("b", intv("2"))]);
        let after = snap(SemioValue::Null, vec![node("b", intv("2"))]);
        let d1 = SemioValueTreeDiff::between(&base, &mid);
        let d2 = SemioValueTreeDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
        match &combined.nodes {
            Some(diff) => {
                assert_eq!(diff.removed, vec![ValueId::new("a")]);
                assert!(diff.modified.is_empty());
            }
            None => panic!("expected an nodes diff"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_nodes_associativity() {
        let s0 = snap(SemioValue::Null, vec![node("a", intv("1"))]);
        let s1 = snap(SemioValue::Null, vec![node("a", intv("1")), node("b", intv("2"))]);
        let s2 = snap(SemioValue::Null, vec![node("a", intv("9")), node("b", intv("2"))]);
        let s3 = snap(SemioValue::Null, vec![node("b", intv("2")), node("c", intv("3"))]);
        let d1 = SemioValueTreeDiff::between(&s0, &s1);
        let d2 = SemioValueTreeDiff::between(&s1, &s2);
        let d3 = SemioValueTreeDiff::between(&s2, &s3);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);

        assert_eq!(left.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
        assert_eq!(right.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (nodes graph / id-keyed)

    //#region field_sweep
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> SemioValueSnapshot {
        snap(
            mapv(vec![
                ("keepBool", SemioValue::Bool { value: true }),
                ("keepInt", intv("1")),
                ("keepFloat", floatv("1.5")),
                ("keepStr", strv("base")),
                ("keepBytes", SemioValue::Bytes { value: vec![1, 2, 3] }),
                ("keepRef", refv("n1")),
                ("kindChange", intv("1")),
                ("nullToValue", SemioValue::Null),
                ("removedMember", strv("gone")),
                ("modifiedMember", intv("10")),
                ("nestedList", listv(vec![intv("1"), intv("2"), intv("3")])),
                ("nestedMap", mapv(vec![("inner", strv("x"))])),
            ]),
            vec![node("n1", strv("kept")), node("n2", strv("removed-node")), node("n3", intv("10"))],
        )
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> SemioValueSnapshot {
        snap(
            mapv(vec![
                ("keepBool", SemioValue::Bool { value: false }),
                ("keepInt", intv("2")),
                ("keepFloat", floatv("2.75e3")),
                ("keepStr", strv("changed")),
                ("keepBytes", SemioValue::Bytes { value: vec![4, 5] }),
                ("keepRef", refv("n3")),
                ("kindChange", strv("now a string")),
                ("nullToValue", SemioValue::Bool { value: true }),
                ("modifiedMember", intv("99")),
                ("nestedList", listv(vec![intv("1"), intv("20"), intv("30"), intv("4")])),
                ("nestedMap", mapv(vec![("inner", strv("y")), ("extra", SemioValue::Bool { value: true })])),
                ("addedMember", strv("new")),
            ]),
            vec![node("n1", strv("kept")), node("n3", intv("99")), node("n4", strv("added-node"))],
        )
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_between_roundtrips_both_directions() {
        let (a, b) = (sweep_a(), sweep_b());
        assert_eq!(SemioValueTreeDiff::between(&a, &b).apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        assert_eq!(SemioValueTreeDiff::between(&b, &a).apply(&b).expect("apply must succeed for a well-formed fixture"), a);
        assert!(SemioValueTreeDiff::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_every_field_present_in_diff() {
        let (a, b) = (sweep_a(), sweep_b());
        let diff = SemioValueTreeDiff::between(&a, &b);

        let map_diff = match &diff.root {
            Some(SemioValueDiff::Map { diff }) => diff,
            other => panic!("expected a top-level map diff, got {other:?}"),
        };
        assert_eq!(map_diff.removed, vec!["removedMember".to_string()]);
        assert_eq!(map_diff.added.len(), 1);
        assert_eq!(map_diff.added[0].item.key, "addedMember");

        let by_key: HashMap<&str, &SemioValueDiff> = map_diff.modified.iter().map(|m| (m.key.as_str(), &m.diff)).collect();
        for key in ["keepBool", "keepInt", "keepFloat", "keepStr", "keepBytes", "keepRef", "kindChange", "nullToValue", "modifiedMember", "nestedList", "nestedMap"] {
            assert!(by_key.contains_key(key), "expected a modified entry for `{key}`");
        }
        assert!(matches!(by_key["kindChange"], SemioValueDiff::Replace { .. }), "Int->Str must fall back to Replace");
        assert!(matches!(by_key["nullToValue"], SemioValueDiff::Replace { .. }), "Null->Bool must fall back to Replace");
        assert!(matches!(by_key["keepBool"], SemioValueDiff::Bool { .. }));
        assert!(matches!(by_key["keepInt"], SemioValueDiff::Int { .. }));
        assert!(matches!(by_key["keepFloat"], SemioValueDiff::Float { .. }));
        assert!(matches!(by_key["keepStr"], SemioValueDiff::Str { .. }));
        assert!(matches!(by_key["keepBytes"], SemioValueDiff::Bytes { .. }));
        assert!(matches!(by_key["keepRef"], SemioValueDiff::Ref { .. }));
        match by_key["nestedList"] {
            SemioValueDiff::List { diff } => {
                assert!(!diff.modified.is_empty());
                assert!(!diff.added.is_empty());
            }
            other => panic!("expected list diff, got {other:?}"),
        }
        match by_key["nestedMap"] {
            SemioValueDiff::Map { diff } => {
                assert!(!diff.modified.is_empty());
                assert!(!diff.added.is_empty());
            }
            other => panic!("expected map diff, got {other:?}"),
        }

        let nodes_diff = diff.nodes.as_ref().expect("expected an nodes graph diff");
        assert_eq!(nodes_diff.removed, vec![ValueId::new("n2")]);
        assert_eq!(nodes_diff.added.len(), 1);
        assert_eq!(nodes_diff.added[0].item.id, ValueId::new("n4"));
        assert_eq!(nodes_diff.modified.len(), 1);
        assert_eq!(nodes_diff.modified[0].key, ValueId::new("n3"));
    }
    //#endregion field_sweep

    //#region 🔖️HandcraftedDiffCodecTests
    /// 🧪️ diff_codec_text_binary_roundtrip_law: exercises every `SemioValueDiff` variant (incl.
    /// the `Replace` kind-change fallback), nested list/map/nodes-graph collection triples, and
    /// the empty (`None`/`None`) diff.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;

        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioValueTreeDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioValueTreeDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion 🔖️HandcraftedDiffCodecTests
}
//#endregion 🧪️Tests
