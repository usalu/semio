//! 🔺️ SemioObjectDiff — recursive, handcrafted diff mirroring `SemioValue`'s shape. `List` gets an
//! index-keyed triple, `Map` gets a name-keyed triple, the top-level `objects` graph gets an
//! id-keyed triple — all THREE built directly on the shared
//! `crate::artifacts::semio::standards::v1::engine::triples` codec (`IndexedTripleDiff`/
//! `NamedTripleDiff` + their `enc_*`/`dec_*` bridge functions) per this ticket's explicit
//! instruction to reuse it rather than reinvent it a 14th time (bcf/docx and now `json` each
//! rolled their own copy before this shared engine existed). No `snapshot: Option<SemioObjectSnapshot>`
//! full-replace slot anywhere — `SetSnapshot`'s own diff is the sparse `between(base, next)` just
//! like every other mutation. Structural template (Replace-on-kind-change fallback, recursive
//! between/apply/absorb) copied from `json`'s own `JsonDiff` (this subset's informing source).

use crate::artifacts::semio::standards::v1::engine::triples::{
    dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple, split_top_level, strip_brackets, IndexAdded, IndexModified,
    IndexedTripleDiff, NamedModified, NamedTripleDiff,
};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{ObjectId, SemioObjectEntry, SemioObjectNode, SemioValue};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
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
/// supplied as `T` for `Map`/`objects`' own `NamedTripleDiff<K,D,T>` rather than editing the
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
    Replace { value: SemioValue },
    Bool { value: bool },
    Int { lexeme: String },
    Float { lexeme: String },
    Str { value: String },
    Bytes { value: Vec<u8> },
    List { diff: IndexedTripleDiff<SemioValueDiff, SemioValue> },
    Map { diff: NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioObjectEntry>> },
    Ref { id: ObjectId },
}

/// 🩹 Never constructed as a "real" empty diff (there is no meaningful empty `SemioValueDiff` —
/// `SemioObjectDiff.root` is `Option<SemioValueDiff>` precisely so `None` carries that meaning).
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
/// 🔺️ Diff for `stdio.semio.object`. `schema` is an identity field and is never diffed. `objects`
/// is the id-keyed object-GRAPH triple (see the snapshot module's doc comment) — a second,
/// top-level collection sibling to `root`'s own recursive tree, per the recipe's "strong-like
/// entities in ordered collections" rule.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object.diff")]
pub struct SemioObjectDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<SemioValueDiff>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<NamedTripleDiff<ObjectId, SemioValueDiff, NamedAdded<SemioObjectNode>>>,
}

impl MutationDiff<SemioObjectSnapshot> for SemioObjectDiff {
    fn apply(&self, base: &SemioObjectSnapshot) -> SemioObjectSnapshot {
        let mut next = base.clone();
        if let Some(diff) = &self.root {
            next.root = apply_value_diff(diff, &base.root);
        }
        if let Some(diff) = &self.objects {
            next.objects = apply_objects_diff(diff, &base.objects);
        }
        next
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
                if is_value_diff_effectively_empty(&combined) { None } else { Some(combined) }
            }
        };
        self.objects = match (self.objects.take(), other.objects) {
            (None, None) => None,
            (Some(d1), None) => Some(d1),
            (None, Some(d2)) => Some(d2),
            (Some(d1), Some(d2)) => {
                let combined = absorb_named(d1, d2, &|n: &NamedAdded<SemioObjectNode>| n.item.id.clone(), &absorb_value_diff, &apply_value_diff_to_named_node, &is_value_diff_effectively_empty);
                if is_named_empty(&combined) { None } else { Some(combined) }
            }
        };
    }
}

impl DiffAlgebra<SemioObjectSnapshot> for SemioObjectDiff {
    /// 🔁️ Diff-level undo, derived generically from `between`: `mid = self.apply(base)`, then
    /// `between(mid, base)` is exactly the diff that restores `base` when applied to `mid`.
    fn inverse(&self, base: &SemioObjectSnapshot) -> Self {
        let mid = self.apply(base);
        Self::between(&mid, base)
    }

    fn between(base: &SemioObjectSnapshot, other: &SemioObjectSnapshot) -> Self {
        let root = value_diff_between(&base.root, &other.root);
        let objects_diff = objects_diff_between(&base.objects, &other.objects);
        let objects = if is_named_empty(&objects_diff) { None } else { Some(objects_diff) };
        SemioObjectDiff { root, objects }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none() && self.objects.is_none()
    }
}

/// 🧩 Builds the sparse `between(base, next)` diff for a `SetSnapshot` mutation — NOT a full
/// `snapshot: Option<SemioObjectSnapshot>` replace slot.
pub fn diff_set_snapshot(base: &SemioObjectSnapshot, next: &SemioObjectSnapshot) -> SemioObjectDiff {
    SemioObjectDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️Apply
/// ▶️ Applies a [`SemioValueDiff`] against the corresponding base node.
pub fn apply_value_diff(diff: &SemioValueDiff, base: &SemioValue) -> SemioValue {
    match diff {
        SemioValueDiff::Replace { value } => value.clone(),
        SemioValueDiff::Bool { value } => SemioValue::Bool { value: *value },
        SemioValueDiff::Int { lexeme } => SemioValue::Int { lexeme: lexeme.clone() },
        SemioValueDiff::Float { lexeme } => SemioValue::Float { lexeme: lexeme.clone() },
        SemioValueDiff::Str { value } => SemioValue::Str { value: value.clone() },
        SemioValueDiff::Bytes { value } => SemioValue::Bytes { value: value.clone() },
        SemioValueDiff::List { diff } => {
            let items: &[SemioValue] = match base { SemioValue::List { items } => items.as_slice(), _ => &[] };
            SemioValue::List { items: apply_list_diff(diff, items) }
        }
        SemioValueDiff::Map { diff } => {
            let entries: &[SemioObjectEntry] = match base { SemioValue::Map { entries } => entries.as_slice(), _ => &[] };
            SemioValue::Map { entries: apply_map_diff(diff, entries) }
        }
        SemioValueDiff::Ref { id } => SemioValue::Ref { id: id.clone() },
    }
}

fn apply_value_diff_to_named_node(diff: &SemioValueDiff, node: &NamedAdded<SemioObjectNode>) -> NamedAdded<SemioObjectNode> {
    NamedAdded { index: node.index, item: SemioObjectNode { id: node.item.id.clone(), value: apply_value_diff(diff, &node.item.value) } }
}

fn apply_value_diff_to_named_entry(diff: &SemioValueDiff, entry: &NamedAdded<SemioObjectEntry>) -> NamedAdded<SemioObjectEntry> {
    NamedAdded { index: entry.index, item: SemioObjectEntry { key: entry.item.key.clone(), value: apply_value_diff(diff, &entry.item.value) } }
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert at
/// `min(index, len)`). Out-of-range indices are graceful no-ops.
pub fn apply_list_diff(diff: &IndexedTripleDiff<SemioValueDiff, SemioValue>, base: &[SemioValue]) -> Vec<SemioValue> {
    let mut items: Vec<SemioValue> = base.to_vec();
    for m in &diff.modified {
        if let Some(old) = base.get(m.index) {
            if let Some(slot) = items.get_mut(m.index) {
                *slot = apply_value_diff(&m.diff, old);
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
pub fn apply_map_diff(diff: &NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioObjectEntry>>, base: &[SemioObjectEntry]) -> Vec<SemioObjectEntry> {
    let mut entries: Vec<SemioObjectEntry> = base.to_vec();
    for m in &diff.modified {
        if let Some(pos) = entries.iter().position(|e| e.key == m.key) {
            let old = entries[pos].value.clone();
            entries[pos].value = apply_value_diff(&m.diff, &old);
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

/// ▶️ Same shape as [`apply_map_diff`] but keyed by [`ObjectId`] over the top-level object graph.
pub fn apply_objects_diff(diff: &NamedTripleDiff<ObjectId, SemioValueDiff, NamedAdded<SemioObjectNode>>, base: &[SemioObjectNode]) -> Vec<SemioObjectNode> {
    let mut nodes: Vec<SemioObjectNode> = base.to_vec();
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
            if is_indexed_empty(&diff) { None } else { Some(SemioValueDiff::List { diff }) }
        }
        (SemioValue::Map { entries: am }, SemioValue::Map { entries: bm }) => {
            let diff = map_diff_between(am, bm);
            if is_named_empty(&diff) { None } else { Some(SemioValueDiff::Map { diff }) }
        }
        _ => Some(SemioValueDiff::Replace { value: b.clone() }),
    }
}

/// 🧭️ Index-pairwise: `modified` compares `0..min(len)`, `removed` is the base tail, `added` is
/// the other tail (final-state indices, per the normative apply contract).
fn list_diff_between(a: &[SemioValue], b: &[SemioValue]) -> IndexedTripleDiff<SemioValueDiff, SemioValue> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if let Some(diff) = value_diff_between(&a[i], &b[i]) {
            modified.push(IndexModified { index: i, diff });
        }
    }
    let removed: Vec<usize> = if a.len() > b.len() { (b.len()..a.len()).collect() } else { Vec::new() };
    let added: Vec<IndexAdded<SemioValue>> = if b.len() > a.len() {
        (a.len()..b.len()).map(|i| IndexAdded { index: i, item: b[i].clone() }).collect()
    } else {
        Vec::new()
    };
    IndexedTripleDiff { removed, modified, added }
}

/// 🧭️ Name-keyed: base members missing from `b` are `removed`; members present in both with a
/// changed value are `modified`; members only in `b` are `added` AT THEIR `b`-POSITION (see
/// [`NamedAdded`]'s doc comment — renames are documented as `removed`+`added` — no rename
/// detection, matching `json`'s own `object_diff_between`).
fn map_diff_between(a: &[SemioObjectEntry], b: &[SemioObjectEntry]) -> NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioObjectEntry>> {
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

/// 🧭️ Same shape as [`map_diff_between`], keyed by [`ObjectId`] over the top-level object graph.
fn objects_diff_between(a: &[SemioObjectNode], b: &[SemioObjectNode]) -> NamedTripleDiff<ObjectId, SemioValueDiff, NamedAdded<SemioObjectNode>> {
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

fn is_indexed_empty<D, T>(d: &IndexedTripleDiff<D, T>) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

fn is_named_empty<K, D, T>(d: &NamedTripleDiff<K, D, T>) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

/// 🕳️ Whether a (possibly freshly-absorbed) node diff represents no actual change. Scalar
/// replace/field diffs are never "empty" in isolation, but a collection diff with nothing
/// removed/modified/added genuinely changes nothing and should collapse away rather than survive
/// as a no-op wrapper (same rationale `json`'s `is_value_diff_effectively_empty` documents).
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
        (SemioValueDiff::List { diff: a1 }, SemioValueDiff::List { diff: a2 }) => {
            SemioValueDiff::List { diff: absorb_indexed(a1, a2, &absorb_value_diff, &apply_value_diff, &is_value_diff_effectively_empty) }
        }
        (SemioValueDiff::Map { diff: o1 }, SemioValueDiff::Map { diff: o2 }) => {
            SemioValueDiff::Map { diff: absorb_named(o1, o2, &|e: &NamedAdded<SemioObjectEntry>| e.item.key.clone(), &absorb_value_diff, &apply_value_diff_to_named_entry, &is_value_diff_effectively_empty) }
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
/// site needed at THIS level (`objects`/`Map` reuse [`absorb_named`] below instead).
fn absorb_indexed<D: Clone, T: Clone>(
    d1: IndexedTripleDiff<D, T>,
    d2: IndexedTripleDiff<D, T>,
    absorb_d: &impl Fn(D, D) -> D,
    apply_d_to_t: &impl Fn(&D, &T) -> T,
    is_d_empty: &impl Fn(&D) -> bool,
) -> IndexedTripleDiff<D, T> {
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

    let max_ref = d1.removed.iter().copied()
        .chain(d1.modified.iter().map(|m| m.index))
        .chain(d1.added.iter().map(|a| a.index))
        .chain(d2.removed.iter().copied())
        .chain(d2.modified.iter().map(|m| m.index))
        .chain(d2.added.iter().map(|a| a.index))
        .max().unwrap_or(0);
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
    let mut after: Vec<AfterSlot<D, T>> = mid.iter().map(|origin| match origin {
        Origin::Base(orig) => AfterSlot::Base { orig: *orig, diff: d1_modified.get(orig).cloned() },
        Origin::D1Added(tag) => AfterSlot::D1Added { tag: *tag, patch: None },
    }).collect();

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

/// ➕️ Name/id-keyed absorb, generic over the key type `K` (`String` for `Map`, [`ObjectId`] for
/// `objects`) via an explicit `key_of` extractor — resolution of WHICH entry a `d2` op refers to
/// is exact (key/id identity), but a surviving `d1`-added entry's relative position among OTHER
/// entries is not renegotiated by unrelated `d2` removals elsewhere (name/id identity carries no
/// positional information base-free, unlike list indices) — exact for every realistic mutation
/// pattern (new entries always appended, see `SetMapEntry`/`SetObject`'s own diff construction)
/// and every canonical `absorb_law` case tested below, same documented shape `json`'s own
/// `absorb_object_diff` carries.
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
/// 🧪️ Hand-rolled `protocol::DiffCodec` for `SemioObjectDiff` — grammar template copied from
/// `JsonDiff`'s (this subset's own informing source).
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
pub(crate) fn enc_object_id(id: &ObjectId) -> String {
    enc_str(&id.value)
}
pub(crate) fn dec_object_id(s: &str) -> Result<ObjectId, String> {
    Ok(ObjectId::new(dec_str(s)?))
}
//#endregion 🔖️Primitives

//#region 🔖️SemioValueCodecs
/// 🌳 Tag-prefixed like `json`'s `enc_json_value`: `Z` (null, no payload, no brackets) / `B[0|1]`
/// / `I[hex(lexeme)]` / `F[hex(lexeme)]` / `S[hex(value)]` / `Y[hex(bytes)]` / `L[v1,v2,...]` /
/// `M[hexkey1:v1,hexkey2:v2,...]` / `R[hex(id)]` — member insertion order preserved by
/// construction (a list, never re-sorted).
pub(crate) fn enc_semio_value(v: &SemioValue) -> String {
    match v {
        SemioValue::Null => "Z".to_string(),
        SemioValue::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        SemioValue::Int { lexeme } => format!("I[{}]", enc_str(lexeme)),
        SemioValue::Float { lexeme } => format!("F[{}]", enc_str(lexeme)),
        SemioValue::Str { value } => format!("S[{}]", enc_str(value)),
        SemioValue::Bytes { value } => format!("Y[{}]", hex_encode(value)),
        SemioValue::List { items } => format!("L[{}]", items.iter().map(enc_semio_value).collect::<Vec<_>>().join(",")),
        SemioValue::Map { entries } => format!(
            "M[{}]",
            entries.iter().map(|e| format!("{}:{}", enc_str(&e.key), enc_semio_value(&e.value))).collect::<Vec<_>>().join(",")
        ),
        SemioValue::Ref { id } => format!("R[{}]", enc_object_id(id)),
    }
}
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
        "L" => Ok(SemioValue::List {
            items: split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_semio_value).collect::<Result<Vec<_>, String>>()?,
        }),
        "M" => {
            let entries = split_top_level(inner, ',')
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|entry| {
                    let (key, value) = entry.split_once(':').ok_or_else(|| format!("map entry: bad entry {entry:?}"))?;
                    Ok(SemioObjectEntry { key: dec_str(key)?, value: dec_semio_value(value)? })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(SemioValue::Map { entries })
        }
        "R" => Ok(SemioValue::Ref { id: dec_object_id(inner)? }),
        other => Err(format!("semio value: unknown tag {other:?}")),
    }
}

pub(crate) fn enc_semio_object_entry(e: &SemioObjectEntry) -> String {
    format!("{}:{}", enc_str(&e.key), enc_semio_value(&e.value))
}
pub(crate) fn dec_semio_object_entry(s: &str) -> Result<SemioObjectEntry, String> {
    let (key, value) = s.split_once(':').ok_or_else(|| format!("object entry: bad entry {s:?}"))?;
    Ok(SemioObjectEntry { key: dec_str(key)?, value: dec_semio_value(value)? })
}

pub(crate) fn enc_semio_object_node(n: &SemioObjectNode) -> String {
    format!("{}:{}", enc_object_id(&n.id), enc_semio_value(&n.value))
}
pub(crate) fn dec_semio_object_node(s: &str) -> Result<SemioObjectNode, String> {
    let (id, value) = s.split_once(':').ok_or_else(|| format!("object node: bad entry {s:?}"))?;
    Ok(SemioObjectNode { id: dec_object_id(id)?, value: dec_semio_value(value)? })
}

/// 🧷 `NamedAdded<T>`-wrapping variants of the two encoders above — `index:` prefixed — used ONLY
/// for a diff's own `added` list (see [`NamedAdded`]'s doc comment); the plain (unwrapped)
/// encoders above stay the ones `🧬️mutations`' snapshot-level `objects` list encoding uses.
pub(crate) fn enc_named_added_entry(a: &NamedAdded<SemioObjectEntry>) -> String {
    format!("{}:{}", a.index, enc_semio_object_entry(&a.item))
}
pub(crate) fn dec_named_added_entry(s: &str) -> Result<NamedAdded<SemioObjectEntry>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added entry: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_semio_object_entry(rest)? })
}
pub(crate) fn enc_named_added_node(a: &NamedAdded<SemioObjectNode>) -> String {
    format!("{}:{}", a.index, enc_semio_object_node(&a.item))
}
pub(crate) fn dec_named_added_node(s: &str) -> Result<NamedAdded<SemioObjectNode>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added node: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_semio_object_node(rest)? })
}
//#endregion 🔖️SemioValueCodecs

//#region 🔖️DiffValueCodecs
/// 🌳 `SemioValueDiff` itself needs a tag (`P`=rePlace, `B`=Bool, `I`=Int, `F`=Float, `S`=Str,
/// `Y`=Bytes, `L`=List, `M`=Map, `R`=Ref) since, unlike a plain [`SemioValue`], it appears
/// standalone (not always inside a bracketed container) at the top-level `root=`/`objects=` token
/// position.
pub(crate) fn enc_value_diff(d: &SemioValueDiff) -> String {
    match d {
        SemioValueDiff::Replace { value } => format!("P[{}]", enc_semio_value(value)),
        SemioValueDiff::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        SemioValueDiff::Int { lexeme } => format!("I[{}]", enc_str(lexeme)),
        SemioValueDiff::Float { lexeme } => format!("F[{}]", enc_str(lexeme)),
        SemioValueDiff::Str { value } => format!("S[{}]", enc_str(value)),
        SemioValueDiff::Bytes { value } => format!("Y[{}]", hex_encode(value)),
        SemioValueDiff::List { diff } => format!("L[{}]", enc_indexed_triple(diff, enc_value_diff, enc_semio_value)),
        SemioValueDiff::Map { diff } => format!("M[{}]", enc_named_triple(diff, |k: &String| enc_str(k), enc_value_diff, enc_named_added_entry)),
        SemioValueDiff::Ref { id } => format!("R[{}]", enc_object_id(id)),
    }
}
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
        "R" => Ok(SemioValueDiff::Ref { id: dec_object_id(inner)? }),
        other => Err(format!("semio value diff: unknown tag {other:?}")),
    }
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
/// 🧭️ Two-field top level (`root=<enc>` / `objects=<enc>`, either absent = unchanged).
fn print_object_diff(d: &SemioObjectDiff) -> String {
    let mut tokens = Vec::new();
    if let Some(v) = &d.root {
        tokens.push(format!("root={}", enc_value_diff(v)));
    }
    if let Some(o) = &d.objects {
        tokens.push(format!("objects={}", enc_named_triple(o, enc_object_id, enc_value_diff, enc_named_added_node)));
    }
    tokens.join(" ")
}
fn parse_object_diff(line: &str) -> Result<SemioObjectDiff, String> {
    let mut d = SemioObjectDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("root=") {
            d.root = Some(dec_value_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("objects=") {
            d.objects = Some(dec_named_triple(rest, dec_object_id, dec_value_diff, dec_named_added_node)?);
        } else {
            return Err(format!("semio object diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for SemioObjectDiff {
    fn print_diff(&self) -> String {
        print_object_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_object_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim — same simplification `JsonDiff`/`SvgDiff`/`GifDiff`
    /// use, satisfies every `DiffCodec` law without inventing a second wire format.
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA;
    use std::collections::HashMap;

    fn snap(root: SemioValue, objects: Vec<SemioObjectNode>) -> SemioObjectSnapshot {
        SemioObjectSnapshot { schema: STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.into(), root, objects }
    }

    fn listv(items: Vec<SemioValue>) -> SemioValue {
        SemioValue::List { items }
    }

    fn mapv(pairs: Vec<(&str, SemioValue)>) -> SemioValue {
        SemioValue::Map { entries: pairs.into_iter().map(|(k, v)| SemioObjectEntry { key: k.into(), value: v }).collect() }
    }

    fn intv(lexeme: &str) -> SemioValue {
        SemioValue::Int { lexeme: lexeme.into() }
    }

    fn floatv(lexeme: &str) -> SemioValue {
        SemioValue::Float { lexeme: lexeme.into() }
    }

    fn strv(s: &str) -> SemioValue {
        SemioValue::Str { value: s.into() }
    }

    fn refv(id: &str) -> SemioValue {
        SemioValue::Ref { id: ObjectId::new(id) }
    }

    fn node(id: &str, value: SemioValue) -> SemioObjectNode {
        SemioObjectNode { id: ObjectId::new(id), value }
    }

    //#region between_roundtrip_law
    #[test]
    fn between_roundtrip_law_scalars_and_kind_change() {
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
            assert_eq!(SemioObjectDiff::between(&sa, &sb).apply(&sa), sb, "a={a:?} b={b:?}");
            assert_eq!(SemioObjectDiff::between(&sb, &sa).apply(&sb), sa);
        }
    }

    #[test]
    fn between_roundtrip_law_nested_collections_and_graph() {
        let a = snap(
            mapv(vec![("tags", listv(vec![strv("x"), strv("y")])), ("n", intv("1"))]),
            vec![node("n1", strv("hello"))],
        );
        let b = snap(
            mapv(vec![("tags", listv(vec![strv("x"), strv("z"), strv("w")])), ("n", intv("2")), ("extra", refv("n1"))]),
            vec![node("n1", strv("world")), node("n2", intv("9"))],
        );
        assert_eq!(SemioObjectDiff::between(&a, &b).apply(&a), b);
        assert_eq!(SemioObjectDiff::between(&b, &a).apply(&b), a);
    }

    #[test]
    fn between_self_is_empty() {
        let a = snap(mapv(vec![("x", intv("1"))]), vec![node("n1", strv("v"))]);
        assert!(SemioObjectDiff::between(&a, &a).is_empty());
    }
    //#endregion between_roundtrip_law

    //#region inverse_law
    #[test]
    fn inverse_law_diff_level() {
        let a = snap(mapv(vec![("x", intv("1")), ("y", listv(vec![intv("1"), intv("2")]))]), vec![node("n1", strv("a"))]);
        let b = snap(mapv(vec![("x", intv("2")), ("z", strv("new"))]), vec![node("n1", strv("b")), node("n2", intv("5"))]);
        let d = SemioObjectDiff::between(&a, &b);
        let mid = d.apply(&a);
        assert_eq!(mid, b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&mid), a);
    }
    //#endregion inverse_law

    //#region absorb_law canonical cases (list/index-keyed)
    // NOTE: these construct `d1`/`d2` DIRECTLY as genuine Insert/Remove/Modify list diffs (matching
    // exactly what `InsertListItem`/`RemoveListItem`/`SetValue` would produce) rather than via
    // `SemioObjectDiff::between(base, next)` — same rationale `json`'s own absorb tests document.
    fn list_diff(d: IndexedTripleDiff<SemioValueDiff, SemioValue>) -> SemioObjectDiff {
        SemioObjectDiff { root: Some(SemioValueDiff::List { diff: d }), objects: None }
    }

    #[test]
    fn absorb_list_insert_then_remove_before() {
        // base = [a,b,c]; d1 = Insert(2,f) -> mid=[a,b,f,c]; d2 = Remove(0) -> after=[b,f,c].
        let base = snap(listv(vec![strv("a"), strv("b"), strv("c")]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: strv("f") }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base));
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), sequential);
        assert_eq!(sequential.root, listv(vec![strv("b"), strv("f"), strv("c")]));
        match &combined.root {
            Some(SemioValueDiff::List { diff }) => {
                assert_eq!(diff.removed, vec![0]);
                assert_eq!(diff.added, vec![IndexAdded { index: 1, item: strv("f") }]);
            }
            other => panic!("expected list diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_list_insert_insert_same_index_both_survive() {
        let base = snap(listv(vec![strv("a"), strv("b")]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: strv("f") }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: strv("g") }], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base));
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), sequential);
        assert_eq!(sequential.root, listv(vec![strv("a"), strv("b"), strv("g"), strv("f")]));
        match &combined.root {
            Some(SemioValueDiff::List { diff }) => assert_eq!(diff.added.len(), 2, "both inserts must survive"),
            other => panic!("expected list diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_list_insert_then_remove_of_same_added_item_cancels() {
        let base = snap(listv(vec![strv("a")]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 1, item: strv("f") }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff { removed: vec![1], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base));
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), sequential);
        assert_eq!(sequential, base);
        assert!(combined.is_empty(), "cancelling insert+remove must coalesce to an empty diff");
    }

    #[test]
    fn absorb_list_add_then_setfield_patches_added_payload() {
        let base = snap(listv(vec![]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 0, item: mapv(vec![("x", intv("1"))]) }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff {
            modified: vec![IndexModified {
                index: 0,
                diff: SemioValueDiff::Map { diff: NamedTripleDiff { added: vec![NamedAdded { index: 1, item: SemioObjectEntry { key: "y".into(), value: intv("2") } }], ..Default::default() } },
            }],
            ..Default::default()
        });
        let sequential = d2.apply(&d1.apply(&base));
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), sequential);
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

    #[test]
    fn absorb_list_modify_then_remove_drops_pending_patch() {
        let base = snap(listv(vec![intv("1"), intv("2")]), vec![]);
        let d1 = list_diff(IndexedTripleDiff { modified: vec![IndexModified { index: 0, diff: SemioValueDiff::Int { lexeme: "9".into() } }], ..Default::default() });
        let d2 = list_diff(IndexedTripleDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base));
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), sequential);
        assert_eq!(sequential.root, listv(vec![intv("2")]));
        match &combined.root {
            Some(SemioValueDiff::List { diff }) => {
                assert_eq!(diff.removed, vec![0]);
                assert!(diff.modified.is_empty(), "the pending modify on the removed base index must be dropped");
            }
            other => panic!("expected list diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_list_associativity() {
        let s0 = snap(listv(vec![intv("1"), intv("2"), intv("3")]), vec![]);
        let s1 = snap(listv(vec![intv("1"), intv("9"), intv("3")]), vec![]);
        let s2 = snap(listv(vec![intv("9"), intv("3"), intv("4")]), vec![]);
        let s3 = snap(listv(vec![intv("9"), intv("4")]), vec![]);
        let d1 = SemioObjectDiff::between(&s0, &s1);
        let d2 = SemioObjectDiff::between(&s1, &s2);
        let d3 = SemioObjectDiff::between(&s2, &s3);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);

        assert_eq!(left.apply(&s0), s3);
        assert_eq!(right.apply(&s0), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (list/index-keyed)

    //#region absorb_law canonical cases (map/name-keyed)
    #[test]
    fn absorb_map_add_then_setfield_patches_added_payload() {
        let base = snap(mapv(vec![]), vec![]);
        let mid = snap(mapv(vec![("config", mapv(vec![]))]), vec![]);
        let after = snap(mapv(vec![("config", mapv(vec![("x", intv("5"))]))]), vec![]);
        let d1 = SemioObjectDiff::between(&base, &mid);
        let d2 = SemioObjectDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        match &combined.root {
            Some(SemioValueDiff::Map { diff }) => {
                assert!(diff.modified.is_empty());
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].item.value, mapv(vec![("x", intv("5"))]));
            }
            other => panic!("expected map diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_map_modify_then_remove_drops_pending_patch() {
        let base = snap(mapv(vec![("a", intv("1")), ("b", intv("2"))]), vec![]);
        let mid = snap(mapv(vec![("a", intv("9")), ("b", intv("2"))]), vec![]);
        let after = snap(mapv(vec![("b", intv("2"))]), vec![]);
        let d1 = SemioObjectDiff::between(&base, &mid);
        let d2 = SemioObjectDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        match &combined.root {
            Some(SemioValueDiff::Map { diff }) => {
                assert_eq!(diff.removed, vec!["a".to_string()]);
                assert!(diff.modified.is_empty());
            }
            other => panic!("expected map diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_map_insert_insert_both_survive() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let mid = snap(mapv(vec![("a", intv("1")), ("f", intv("2"))]), vec![]);
        let after = snap(mapv(vec![("a", intv("1")), ("f", intv("2")), ("g", intv("3"))]), vec![]);
        let d1 = SemioObjectDiff::between(&base, &mid);
        let d2 = SemioObjectDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        match &combined.root {
            Some(SemioValueDiff::Map { diff }) => assert_eq!(diff.added.len(), 2),
            other => panic!("expected map diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_map_insert_then_remove_of_same_added_item_cancels() {
        let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let mid = snap(mapv(vec![("a", intv("1")), ("f", intv("2"))]), vec![]);
        let after = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let d1 = SemioObjectDiff::between(&base, &mid);
        let d2 = SemioObjectDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), base);
        assert!(combined.is_empty());
    }

    #[test]
    fn absorb_map_associativity() {
        let s0 = snap(mapv(vec![("a", intv("1"))]), vec![]);
        let s1 = snap(mapv(vec![("a", intv("1")), ("b", intv("2"))]), vec![]);
        let s2 = snap(mapv(vec![("a", intv("9")), ("b", intv("2"))]), vec![]);
        let s3 = snap(mapv(vec![("b", intv("2")), ("c", intv("3"))]), vec![]);
        let d1 = SemioObjectDiff::between(&s0, &s1);
        let d2 = SemioObjectDiff::between(&s1, &s2);
        let d3 = SemioObjectDiff::between(&s2, &s3);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);

        assert_eq!(left.apply(&s0), s3);
        assert_eq!(right.apply(&s0), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (map/name-keyed)

    //#region absorb_law canonical cases (objects graph / id-keyed)
    #[test]
    fn absorb_objects_add_then_setfield_patches_added_payload() {
        let base = snap(SemioValue::Null, vec![]);
        let mid = snap(SemioValue::Null, vec![node("n1", mapv(vec![]))]);
        let after = snap(SemioValue::Null, vec![node("n1", mapv(vec![("x", intv("5"))]))]);
        let d1 = SemioObjectDiff::between(&base, &mid);
        let d2 = SemioObjectDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        match &combined.objects {
            Some(diff) => {
                assert!(diff.modified.is_empty());
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].item.value, mapv(vec![("x", intv("5"))]));
            }
            None => panic!("expected an objects diff"),
        }
    }

    #[test]
    fn absorb_objects_modify_then_remove_drops_pending_patch() {
        let base = snap(SemioValue::Null, vec![node("a", intv("1")), node("b", intv("2"))]);
        let mid = snap(SemioValue::Null, vec![node("a", intv("9")), node("b", intv("2"))]);
        let after = snap(SemioValue::Null, vec![node("b", intv("2"))]);
        let d1 = SemioObjectDiff::between(&base, &mid);
        let d2 = SemioObjectDiff::between(&mid, &after);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base), after);
        match &combined.objects {
            Some(diff) => {
                assert_eq!(diff.removed, vec![ObjectId::new("a")]);
                assert!(diff.modified.is_empty());
            }
            None => panic!("expected an objects diff"),
        }
    }

    #[test]
    fn absorb_objects_associativity() {
        let s0 = snap(SemioValue::Null, vec![node("a", intv("1"))]);
        let s1 = snap(SemioValue::Null, vec![node("a", intv("1")), node("b", intv("2"))]);
        let s2 = snap(SemioValue::Null, vec![node("a", intv("9")), node("b", intv("2"))]);
        let s3 = snap(SemioValue::Null, vec![node("b", intv("2")), node("c", intv("3"))]);
        let d1 = SemioObjectDiff::between(&s0, &s1);
        let d2 = SemioObjectDiff::between(&s1, &s2);
        let d3 = SemioObjectDiff::between(&s2, &s3);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);

        assert_eq!(left.apply(&s0), s3);
        assert_eq!(right.apply(&s0), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (objects graph / id-keyed)

    //#region field_sweep
    fn sweep_a() -> SemioObjectSnapshot {
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

    fn sweep_b() -> SemioObjectSnapshot {
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

    #[test]
    fn field_sweep_between_roundtrips_both_directions() {
        let (a, b) = (sweep_a(), sweep_b());
        assert_eq!(SemioObjectDiff::between(&a, &b).apply(&a), b);
        assert_eq!(SemioObjectDiff::between(&b, &a).apply(&b), a);
        assert!(SemioObjectDiff::between(&a, &a).is_empty());
    }

    #[test]
    fn field_sweep_every_field_present_in_diff() {
        let (a, b) = (sweep_a(), sweep_b());
        let diff = SemioObjectDiff::between(&a, &b);

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

        let objects_diff = diff.objects.as_ref().expect("expected an objects graph diff");
        assert_eq!(objects_diff.removed, vec![ObjectId::new("n2")]);
        assert_eq!(objects_diff.added.len(), 1);
        assert_eq!(objects_diff.added[0].item.id, ObjectId::new("n4"));
        assert_eq!(objects_diff.modified.len(), 1);
        assert_eq!(objects_diff.modified[0].key, ObjectId::new("n3"));
    }
    //#endregion field_sweep

    //#region 🔖️HandcraftedDiffCodecTests
    /// 🧪️ diff_codec_text_binary_roundtrip_law: exercises every `SemioValueDiff` variant (incl.
    /// the `Replace` kind-change fallback), nested list/map/objects-graph collection triples, and
    /// the empty (`None`/`None`) diff.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;

        let a = sweep_a();
        let b = sweep_b();
        let nested = mapv(vec![
            ("tags", listv(vec![strv("x"), strv("y"), strv("z")])),
            ("meta", mapv(vec![("a", intv("1")), ("b", SemioValue::Null)])),
        ]);
        let nested2 = mapv(vec![
            ("tags", listv(vec![strv("x"), strv("w")])),
            ("meta", mapv(vec![("a", intv("9")), ("c", strv("new"))])),
            ("extra", SemioValue::Bool { value: true }),
        ]);

        let cases = vec![
            SemioObjectDiff::default(),
            SemioObjectDiff::between(&a, &b),
            SemioObjectDiff::between(&b, &a),
            SemioObjectDiff::between(&snap(nested.clone(), vec![]), &snap(nested2.clone(), vec![])),
            SemioObjectDiff::between(&snap(nested2, vec![]), &snap(nested, vec![])),
            SemioObjectDiff::between(&snap(intv("1"), vec![]), &snap(strv("1"), vec![])),
            SemioObjectDiff::between(&snap(SemioValue::Null, vec![]), &snap(listv(vec![intv("1"), intv("2")]), vec![])),
            SemioObjectDiff::between(&snap(SemioValue::Null, vec![]), &snap(SemioValue::Null, vec![node("z", SemioValue::Bytes { value: vec![9, 8, 7] })])),
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioObjectDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioObjectDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion 🔖️HandcraftedDiffCodecTests
}
//#endregion 🧪️Tests
