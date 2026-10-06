//! 🔺️ SemioValueTreeDiff — recursive, handcrafted diff mirroring `SemioValue`'s shape. `List` gets an
//! index-keyed triple, `Map` gets a name-keyed triple, the top-level `nodes` graph gets an
//! id-keyed triple — all THREE built directly on the shared
//! `crate::standards::v1::subsets::base::schema::triples` codec (`IndexedTripleDiff`/
//! `NamedTripleDiff` + their `enc_*`/`dec_*` bridge functions) per this ticket's explicit
//! instruction to reuse it rather than reinvent it a 14th time (bcf/docx and now `json` each
//! rolled their own copy before this shared engine existed). No `snapshot: Option<SemioValueSnapshot>`
//! full-replace slot anywhere — `SetSnapshot`'s own diff is the sparse `between(base, next)` just
//! like every other mutation. Structural template (Replace-on-kind-change fallback, recursive
//! between/apply/absorb) copied from `json`'s own `JsonDiff` (this subset's informing source).

use crate::standards::v1::subsets::base::schema::triples::{dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple, IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};


use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, ValueId};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️NamedAdded
/// 🧷 Position-carrying "added" wrapper for name/id-keyed collections — the recipe's own
/// normative shape (`🧬️schema-design.md`'s `CAdded { pub index: usize, pub item: C }`, "full
/// payload + final position") mirrors `engine::triples::IndexAdded<T>` exactly, but the shared
/// `engine::triples::NamedTripleDiff<K,D,T>` only provides that for INDEXED collections
/// (`IndexAdded<T>`) — its named-triple counterpart's `added: Vec<T>` carries no position,
/// confirmed by direct inspection of `engine/🧰️triples/🦀️.rs`. Without a position, a
/// re-added interior member (e.g. `between(b, a)` re-adding a member `a` has but `b` doesn't) can
/// only be appended at the END, which breaks `between_roundtrip_law` in the REVERSE direction the
/// moment the member's real position isn't already last (caught live by this subset's own
/// standalone algorithm-verification harness before this fix landed). `json`'s own
/// `JsonObjectAdded{index,key,item}` independently carries the identical index field for the
/// identical reason — this is this subset's local instantiation of that same normative shape,
/// supplied as `T` for `Map`/`nodes`' own `NamedTripleDiff<K,D,T>` rather than editing the
/// shared engine file (out of scope — see this subset's own report's "shared infra gaps").
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
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
/// `Deserialize` derive needs `D: Default` — a `#[value(default)]`-triggered bound-inference
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.value.diff")]
pub struct SemioValueTreeDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<SemioValueDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
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
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&base.nodes, diff, |node| node.id.clone(), |added| added.item.id.clone(), ["nodes"])?;
            validate_added_positions(diff.added.iter().map(|added| added.index), base.nodes.len() - diff.removed.len(), ["nodes"])?;
            for modified in &diff.modified {
                let node = base.nodes.iter().find(|node| node.id == modified.key).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-node", format!("node {:?} is absent", modified.key)).at(["nodes"]))?;
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
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("add index {index} is out of range or duplicated")).at(target));
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
            crate::standards::v1::subsets::base::schema::triples::validate_indexed_triple(diff, items.len(), target.clone())?;
            for modified in &diff.modified {
                let mut nested = target.clone();
                nested.push(modified.index.to_string());
                validate_value_diff(&modified.diff, &items[modified.index], nested)?;
            }
        }
        (SemioValueDiff::Map { diff }, SemioValue::Map { entries }) => {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(entries, diff, |entry| entry.key.clone(), |added| added.item.key.clone(), target.clone())?;
            validate_added_positions(diff.added.iter().map(|added| added.index), entries.len() - diff.removed.len(), target.clone())?;
            for modified in &diff.modified {
                let entry = entries.iter().find(|entry| entry.key == modified.key).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-map-entry", format!("map entry {:?} is absent", modified.key)).at(target.clone()))?;
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
            SemioValue::List { items: apply_list_diff(diff, items) }
        }
        SemioValueDiff::Map { diff } => {
            let entries: &[SemioValueEntry] = match base {
                SemioValue::Map { entries } => entries.as_slice(),
                _ => &[],
            };
            SemioValue::Map { entries: apply_map_diff(diff, entries) }
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_map_diff(diff: &NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>, base: &[SemioValueEntry]) -> Vec<SemioValueEntry> {
    let mut entries: Vec<SemioValueEntry> = base.to_vec();
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
        (SemioValueDiff::List { diff: a1 }, SemioValueDiff::List { diff: a2 }) => SemioValueDiff::List { diff: absorb_indexed(a1, &a2, &absorb_value_diff, &apply_value_diff, &is_value_diff_effectively_empty) },
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
fn absorb_indexed<D: Clone, T: Clone>(d1: IndexedTripleDiff<D, T>, d2: &IndexedTripleDiff<D, T>, absorb_d: &impl Fn(D, D) -> D, apply_d_to_t: &impl Fn(&D, &T) -> T, is_d_empty: &impl Fn(&D) -> bool) -> IndexedTripleDiff<D, T> {
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
/// is exact (key/id identity). Every `added` entry carries its FINAL-state `index`, so composing
/// `d1` (base→mid) with `d2` (mid→after) re-derives each surviving `d1`-added entry's position in
/// `after`:
/// 1. a `d2` removal of a `d1`-added entry drops it and closes its slot (later `d1` adds move up);
/// 2. a `d2` removal of a BASE entry closes one slot before a surviving `d1`-added entry whenever a
///    base entry precedes it — base-free, the count of base entries before an added entry is known
///    (`index − rank`) but not WHICH ones, so each such removal is taken to precede it, capped at
///    that count. Exact for the realistic pattern (new entries appended, see `SetMapEntry`/
///    `SetNode`) and always yields distinct, in-range positions;
/// 3. `d2`'s own adds (already `after` positions) are inserted ascending, shifting every surviving
///    `d1`-added entry at or past each insert point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K: Clone + PartialEq, D, X: Clone>(
    d1: NamedTripleDiff<K, D, NamedAdded<X>>,
    d2: NamedTripleDiff<K, D, NamedAdded<X>>,
    key_of: &impl Fn(&NamedAdded<X>) -> K,
    absorb_d: &impl Fn(D, D) -> D,
    apply_d_to_t: &impl Fn(&D, &NamedAdded<X>) -> NamedAdded<X>,
    is_d_empty: &impl Fn(&D) -> bool,
) -> NamedTripleDiff<K, D, NamedAdded<X>> {
    let mut removed: Vec<K> = d1.removed;
    let mut modified: Vec<NamedModified<K, D>> = d1.modified;
    let mut added: Vec<NamedAdded<X>> = d1.added;
    added.sort_by_key(|a| a.index);
    let mut merged_removed: Vec<K> = Vec::new();
    let mut removed_base = 0usize;

    for key in d2.removed {
        if let Some(pos) = added.iter().position(|t| key_of(t) == key) {
            let closed = added.remove(pos).index;
            for later in added.iter_mut().filter(|a| a.index > closed) {
                later.index -= 1;
            }
        } else {
            if let Some(pos) = modified.iter().position(|m| m.key == key) {
                modified.remove(pos);
            }
            if !merged_removed.contains(&key) {
                merged_removed.push(key.clone());
                removed.push(key);
                removed_base += 1;
            }
        }
    }
    for (rank, entry) in added.iter_mut().enumerate() {
        let base_before = entry.index.saturating_sub(rank);
        entry.index -= removed_base.min(base_before);
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
    let mut incoming = d2.added;
    incoming.sort_by_key(|a| a.index);
    for a in incoming {
        for existing in added.iter_mut().filter(|existing| existing.index >= a.index) {
            existing.index += 1;
        }
        added.push(a);
    }
    added.sort_by_key(|a| a.index);
    NamedTripleDiff { removed, modified, added }
}
//#endregion 🔖️Absorb

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` for `SemioValueTreeDiff` — grammar template copied from
/// `JsonDiff`'s (this subset's own informing source).
//#region 🔖️Primitives






//#endregion 🔖️Primitives

//#region 🔖️SemioValueCodecs













//#endregion 🔖️SemioValueCodecs

//#region 🔖️DiffValueCodecs


//#endregion 🔖️DiffValueCodecs

//#region 🔖️BinaryPrimitives




//#endregion 🔖️BinaryPrimitives

//#region 🔖️SemioValueBinaryCodecs





//#endregion 🔖️SemioValueBinaryCodecs

//#region 🔖️DiffValueBinaryCodecs











//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ Representative `SemioValueTreeDiff` values (every `SemioValueDiff` variant incl. the `Replace`
/// kind-change fallback, nested list/map/nodes-graph collection triples, and the empty/`None`
/// diff) — the single source of truth reused by `diff_codec_text_binary_roundtrip_law` below AND by
/// `🎹️composer/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests, same convention json's own `demo_diff_cases` uses.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioValueTreeDiff> {
    use crate::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
