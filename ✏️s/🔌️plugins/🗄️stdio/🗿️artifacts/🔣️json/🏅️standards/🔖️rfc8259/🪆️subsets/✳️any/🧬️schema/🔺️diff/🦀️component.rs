//! 🔺️ JsonDiff — recursive, handcrafted diff mirroring `JsonValue`'s shape. `Array` gets an
//! index-keyed triple, `Object` gets a name-keyed triple; scalars get a `Replace` fallback when
//! the node KIND changes at a position, or a direct field diff when the kind is stable. No
//! `snapshot: Option<JsonSnapshot>` full-replace slot anywhere — `SetSnapshot`'s own diff is the
//! sparse `between(base, next)` just like every other mutation.

use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
use crate::artifacts::json::JsonSnapshot;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
// 🧭️ `DiffAlgebra` isn't yet on the `protocol` facade's curated re-export list (S1 added the
// trait but the facade wasn't updated — see s1-spine-report.md) so it's reached via the
// still-public `os_spr::command` path instead of touching that framework facade file.
use protocol::os_spr::command::DiffAlgebra;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

//#region 🔖️CollectionDiffs
/// 📦️ Index-keyed `array` triple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonArrayDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JsonArrayModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JsonArrayAdded>,
}

/// 📦️ One `array.modified[]` entry — `index` refers to BASE state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonArrayModified {
    pub index: usize,
    pub diff: JsonValueDiff,
}

/// 📦️ One `array.added[]` entry — `index` refers to FINAL state, ascending insert at
/// `min(index, len)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonArrayAdded {
    pub index: usize,
    pub item: JsonValue,
}

/// 📦️ Name-keyed `object` triple (member insertion order preserved on apply).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonObjectDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JsonObjectModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JsonObjectAdded>,
}

/// 📦️ One `object.modified[]` entry — `key` refers to BASE state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonObjectModified {
    pub key: String,
    pub diff: JsonValueDiff,
}

/// 📦️ One `object.added[]` entry — `index` is the FINAL Vec position (insertion order hint).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonObjectAdded {
    pub index: usize,
    pub key: String,
    pub item: JsonValue,
}
//#endregion 🔖️CollectionDiffs

//#region 🔖️JsonValueDiff
/// 🔺️ Recursive diff mirroring [`JsonValue`]'s shape. `Replace` is the fallback used whenever the
/// node's KIND changes between base and next (e.g. a member goes from `Number` to `String`); the
/// other variants are direct/structural diffs used whenever the kind is stable.
/// 🧪️ F6: `#[derive(dsl::DslDiff)]` is unusable here for the same structural reason confirmed live
/// by the F6 recon pilot on `SvgDiff`/`GifDiff` (`f6-recon-report.md` §3a): `JsonValueDiff` is a
/// genuine data-carrying enum (`Replace`/`Bool`/`Number`/`String`/`Array`/`Object`, each with
/// fields), and `DslField` — the trait every struct field's type must implement for the derive to
/// bind it — has no impl for any data-carrying enum (only `DslRecord`-derived structs and
/// `DslScalar`-derived UNIT-only enums implement it; `f6-recon-report.md` §3a cites the identical
/// compiler error, `the trait bound ...: DslField is not satisfied`, for `SvgNodeDiff`). Zero
/// tri-state (`Option<Option<_>>`) fields anywhere in this artifact (§3b does not apply — this is
/// the recipe's "enum-only" hand-roll case, same family as `dxf`). `DiffCodec` is hand-rolled
/// below (§🔖️HandcraftedDiffCodec), grammar template copied from `SvgDiff`'s.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JsonValueDiff {
    /// 🔁️ Whole-node replace — the node's KIND changed, or a mutation explicitly overwrites it.
    Replace {
        value: JsonValue,
    },
    Bool {
        value: bool,
    },
    Number {
        lexeme: String,
    },
    String {
        value: String,
    },
    Array {
        diff: JsonArrayDiff,
    },
    Object {
        diff: JsonObjectDiff,
    },
}
//#endregion 🔖️JsonValueDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.json`. `schema` is an identity field and is never diffed.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json.diff")]
pub struct JsonDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValueDiff>,
}

impl MutationDiff<JsonSnapshot> for JsonDiff {
    async fn apply(&self, base: &JsonSnapshot) -> MutationApplyResult<JsonSnapshot> {
        if let Some(diff) = &self.value {
            validate_value_diff(diff, &base.value).await?;
        }
        let mut next = base.clone();
        if let Some(diff) = &self.value {
            next.value = apply_value_diff(diff, &base.value).await;
        }
        Ok(next)
    }

    /// ➕️ Structural, total, base-free, sequential-coalesce absorb (see the module-level `Absorb`
    /// helpers below for the array/object transport algorithm). A composed collection diff that
    /// ends up structurally empty (e.g. an `Insert` immediately cancelled by a matching `Remove`)
    /// collapses back to `None` rather than surviving as a no-op `Some(Array{diff: <empty>})`.
    async fn absorb(&mut self, other: Self) {
        self.value = match (self.value.take(), other.value) {
            (None, None) => None,
            (Some(d1), None) => Some(d1),
            (None, Some(d2)) => Some(d2),
            (Some(d1), Some(d2)) => {
                let combined = absorb_value_diff(d1, d2);
                if is_value_diff_effectively_empty(&combined).await {
                    None
                } else {
                    Some(combined.await)
                }
            }
        };
    }
}

impl DiffAlgebra<JsonSnapshot> for JsonDiff {
    /// 🔁️ Diff-level undo, derived generically from `between`: `mid = self.apply(base)`, then
    /// `between(mid, base)` is — by the `between_roundtrip_law` — exactly the diff that restores
    /// `base` when applied to `mid`.
    async fn inverse(&self, base: &JsonSnapshot) -> Self {
        let mid = apply_json_diff_unchecked(self, base);
        Self::between(&mid, base).await
    }

    async fn between(base: &JsonSnapshot, other: &JsonSnapshot) -> Self {
        JsonDiff { value: value_diff_between(&base.value, &other.value).await }
    }

    async fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

async fn apply_json_diff_unchecked(diff: &JsonDiff, base: &JsonSnapshot) -> JsonSnapshot {
    let mut next = base.clone();
    if let Some(value) = &diff.value {
        next.value = apply_value_diff(value, &base.value).await;
    }
    next
}

/// 🧩 Builds the sparse `between(base, next)` diff for a `SetSnapshot` mutation — NOT a full
/// `snapshot: Option<JsonSnapshot>` replace slot.
pub async fn diff_set_snapshot(base: &JsonSnapshot, next: &JsonSnapshot) -> JsonDiff {
    JsonDiff::between(base, next).await
}
//#endregion 🔖️Diff

//#region 🔖️Apply
/// ▶️ Applies a [`JsonValueDiff`] against the corresponding base node.
pub async fn apply_value_diff(diff: &JsonValueDiff, base: &JsonValue) -> JsonValue {
    match diff {
        JsonValueDiff::Replace { value } => value.clone(),
        JsonValueDiff::Bool { value } => JsonValue::Bool { value: *value },
        JsonValueDiff::Number { lexeme } => JsonValue::Number { lexeme: lexeme.clone() },
        JsonValueDiff::String { value } => JsonValue::String { value: value.clone() },
        JsonValueDiff::Array { diff } => {
            let items: &[JsonValue] = match base {
                JsonValue::Array { items } => items.as_slice(),
                _ => &[],
            };
            JsonValue::Array { items: Box::pin(apply_array_diff(diff, items)).await }
        }
        JsonValueDiff::Object { diff } => {
            let members: &[JsonMember] = match base {
                JsonValue::Object { members } => members.as_slice(),
                _ => &[],
            };
            JsonValue::Object { members: Box::pin(apply_object_diff(diff, members)).await }
        }
    }
}

async fn validate_value_diff(diff: &JsonValueDiff, base: &JsonValue) -> MutationApplyResult<()> {
    match diff {
        JsonValueDiff::Replace { .. } => Ok(()),
        JsonValueDiff::Bool { .. } if matches!(base, JsonValue::Bool { .. }) => Ok(()),
        JsonValueDiff::Number { .. } if matches!(base, JsonValue::Number { .. }) => Ok(()),
        JsonValueDiff::String { .. } if matches!(base, JsonValue::String { .. }) => Ok(()),
        JsonValueDiff::Array { diff } => match base {
            JsonValue::Array { items } => validate_array_diff(diff, items).await,
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "array diff targets a non-array value").await),
        },
        JsonValueDiff::Object { diff } => match base {
            JsonValue::Object { members } => validate_object_diff(diff, members).await,
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "object diff targets a non-object value").await),
        },
        _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "scalar diff targets a different JSON value kind").await),
    }
}

async fn validate_array_diff(diff: &JsonArrayDiff, base: &[JsonValue]) -> MutationApplyResult<()> {
    let mut removed = HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "array removal target does not exist").await);
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "array removal target is repeated").await);
        }
    }
    let mut modified = HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "array modification target does not exist").await);
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "array modification targets a removed item").await);
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "array modification target is repeated").await);
        }
        validate_value_diff(&entry.diff, &base[entry.index]).await.map_err(|error| error.under(vec!["modified".to_string(), entry.index.to_string()]))?;
    }
    let final_len = base.len() - removed.len() + diff.added.len();
    let mut added = HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "array addition is outside the final collection").await);
        }
        if !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "array addition occupies a repeated final position").await);
        }
    }
    Ok(())
}

async fn validate_object_diff(diff: &JsonObjectDiff, base: &[JsonMember]) -> MutationApplyResult<()> {
    let keys: Vec<&str> = base.iter().map(|member| member.key.as_str()).collect();
    for (position, key) in diff.removed.iter().enumerate() {
        if !keys.contains(&key.as_str()) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "object removal target does not exist").await);
        }
        if diff.removed[..position].contains(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "object removal target is repeated").await);
        }
    }
    for (position, modified) in diff.modified.iter().enumerate() {
        if !keys.contains(&modified.key.as_str()) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "object modification target does not exist").await);
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "object modification targets a removed member").await);
        }
        if diff.modified[..position].iter().any(|candidate| candidate.key == modified.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "object modification target is repeated").await);
        }
        let Some(member) = base.iter().find(|member| member.key == modified.key) else {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "object modification target does not exist").await);
        };
        validate_value_diff(&modified.diff, &member.value).await.map_err(|error| error.under(vec!["modified".to_string(), modified.key.clone()]))?;
    }
    let final_len = base.len() - diff.removed.len() + diff.added.len();
    let mut added_keys = HashSet::new();
    let mut added_indices = HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "object addition is outside the final collection").await);
        }
        if !added_indices.insert(entry.index) || keys.contains(&entry.key.as_str()) || !added_keys.insert(entry.key.clone()) || diff.removed.contains(&entry.key) || diff.modified.iter().any(|modified| modified.key == entry.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "object addition target already exists or conflicts").await);
        }
    }
    Ok(())
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert at
/// `min(index, len)`). Out-of-range indices are graceful no-ops.
pub async fn apply_array_diff(diff: &JsonArrayDiff, base: &[JsonValue]) -> Vec<JsonValue> {
    let mut items: Vec<JsonValue> = base.to_vec();
    for m in &diff.modified {
        if let Some(old) = base.get(m.index) {
            if let Some(slot) = items.get_mut(m.index) {
                *slot = Box::pin(apply_value_diff(&m.diff, old)).await;
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

/// ▶️ Same normative apply semantics as arrays, keyed by member name instead of position.
pub async fn apply_object_diff(diff: &JsonObjectDiff, base: &[JsonMember]) -> Vec<JsonMember> {
    let mut members: Vec<JsonMember> = base.to_vec();
    for m in &diff.modified {
        if let Some(pos) = members.iter().position(|mem| mem.key == m.key) {
            let old = members[pos].value.clone();
            members[pos].value = Box::pin(apply_value_diff(&m.diff, &old)).await;
        }
    }
    for key in &diff.removed {
        if let Some(pos) = members.iter().position(|mem| &mem.key == key) {
            members.remove(pos);
        }
    }
    let mut added_sorted = diff.added.clone();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        let pos = a.index.min(members.len());
        members.insert(pos, JsonMember { key: a.key, value: a.item });
    }
    members
}
//#endregion 🔖️Apply

//#region 🔖️Between
/// 🧭️ State-delta construction: `None` when nodes are equal; a direct field diff when the KIND
/// is stable; `Replace` when it changed.
pub async fn value_diff_between(a: &JsonValue, b: &JsonValue) -> Option<JsonValueDiff> {
    if a == b {
        return None;
    }
    match (a, b) {
        (JsonValue::Bool { value: _ }, JsonValue::Bool { value: next }) => Some(JsonValueDiff::Bool { value: *next }),
        (JsonValue::Number { .. }, JsonValue::Number { lexeme }) => Some(JsonValueDiff::Number { lexeme: lexeme.clone() }),
        (JsonValue::String { value: _ }, JsonValue::String { value: next }) => Some(JsonValueDiff::String { value: next.clone() }),
        (JsonValue::Array { items: av }, JsonValue::Array { items: bv }) => {
            let diff = array_diff_between(av, bv);
            if is_array_diff_empty(&diff).await {
                None
            } else {
                Some(JsonValueDiff::Array { diff })
            }
        }
        (JsonValue::Object { members: am }, JsonValue::Object { members: bm }) => {
            let diff = object_diff_between(am, bm);
            if is_object_diff_empty(&diff).await {
                None
            } else {
                Some(JsonValueDiff::Object { diff })
            }
        }
        _ => Some(JsonValueDiff::Replace { value: b.clone() }),
    }
}

/// 🧭️ Index-pairwise: `modified` compares `0..min(len)`, `removed` is the base tail, `added` is
/// the other tail (final-state indices, per the normative apply contract).
async fn array_diff_between(a: &[JsonValue], b: &[JsonValue]) -> JsonArrayDiff {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if let Some(diff) = value_diff_between(&a[i], &b[i]).await {
            modified.push(JsonArrayModified { index: i, diff });
        }
    }
    let removed: Vec<usize> = if a.len() > b.len() { (b.len()..a.len()).collect() } else { Vec::new() };
    let added: Vec<JsonArrayAdded> = if b.len() > a.len() { (a.len()..b.len()).map(|i| JsonArrayAdded { index: i, item: b[i].clone() }).collect() } else { Vec::new() };
    JsonArrayDiff { removed, modified, added }
}

/// 🧭️ Name-keyed: base members missing from `b` are `removed`; members present in both with a
/// changed value are `modified`; members only in `b` are `added` at their `b`-position (renames
/// are documented as `removed`+`added` — no rename detection).
async fn object_diff_between(a: &[JsonMember], b: &[JsonMember]) -> JsonObjectDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for am in a {
        match b.iter().find(|bm| bm.key == am.key) {
            Some(bm) => {
                if let Some(diff) = value_diff_between(&am.value, &bm.value).await {
                    modified.push(JsonObjectModified { key: am.key.clone(), diff });
                }
            }
            None => removed.push(am.key.clone()),
        }
    }
    let mut added = Vec::new();
    for (i, bm) in b.iter().enumerate() {
        if !a.iter().any(|am| am.key == bm.key) {
            added.push(JsonObjectAdded { index: i, key: bm.key.clone(), item: bm.value.clone() });
        }
    }
    JsonObjectDiff { removed, modified, added }
}

async fn is_array_diff_empty(d: &JsonArrayDiff) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

async fn is_object_diff_empty(d: &JsonObjectDiff) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

/// 🕳️ Whether a (possibly freshly-absorbed) node diff represents no actual change. Scalar
/// replace/field diffs are never "empty" in isolation — a value can round-trip back to its
/// original through absorb and still legitimately carry an explicit `Some(original)` (same
/// accepted LWW-field limitation `compose`'s `CanonicalKitDiff` scalar fields have) — but a
/// collection diff with nothing removed/modified/added genuinely changes nothing and should
/// collapse away rather than survive as a no-op wrapper.
async fn is_value_diff_effectively_empty(d: &JsonValueDiff) -> bool {
    match d {
        JsonValueDiff::Array { diff } => is_array_diff_empty(diff).await,
        JsonValueDiff::Object { diff } => is_object_diff_empty(diff).await,
        _ => false,
    }
}
//#endregion 🔖️Between

//#region 🔖️Absorb
/// ➕️ Diff-level absorb (base→mid composed with mid→after). `d2` always wins on a full `Replace`
/// (it fully determines the final value regardless of `d1`); a `Replace` in `d1` gets `d2` baked
/// into its known literal value via `apply_value_diff`; otherwise both sides share the same node
/// KIND (guaranteed by construction against the real intervening `mid` state) and compose
/// per-kind, recursing into collections.
async fn absorb_value_diff(d1: JsonValueDiff, d2: JsonValueDiff) -> JsonValueDiff {
    if matches!(d2, JsonValueDiff::Replace { .. }) {
        return d2;
    }
    if let JsonValueDiff::Replace { value } = d1 {
        let merged = apply_value_diff(&d2, &value);
        return JsonValueDiff::Replace { value: merged.await };
    }
    match (d1, d2) {
        (JsonValueDiff::Bool { .. }, JsonValueDiff::Bool { value }) => JsonValueDiff::Bool { value },
        (JsonValueDiff::Number { .. }, JsonValueDiff::Number { lexeme }) => JsonValueDiff::Number { lexeme },
        (JsonValueDiff::String { .. }, JsonValueDiff::String { value }) => JsonValueDiff::String { value },
        (JsonValueDiff::Array { diff: a1 }, JsonValueDiff::Array { diff: a2 }) => JsonValueDiff::Array { diff: Box::pin(absorb_array_diff(a1, a2)).await },
        (JsonValueDiff::Object { diff: o1 }, JsonValueDiff::Object { diff: o2 }) => JsonValueDiff::Object { diff: absorb_object_diff(o1, o2).await },
        // Defensive: a kind mismatch that isn't a Replace shouldn't arise from two diffs that were
        // actually produced by real sequential application against the same intervening state —
        // fall back to d2 (last-write-wins) rather than panicking.
        (_, other) => other,
    }
}

/// ➕️ Index-keyed absorb via symbolic position simulation: replays `d1` then `d2` over a
/// synthetic, generously-sized token array (`Base(i)` / `D1Added(tag)`) so every real index/key
/// reference in `d1`/`d2` lands on a valid slot without ever needing the normative
/// `min(index,len)` clamp to trigger (diffs built by real `between`/mutation construction never
/// rely on clamping — it exists purely as a defensive no-op for malformed/out-of-range diffs).
/// Walking the resulting token array after both replays yields exactly:
/// `Insert(2,f)+Remove(0) -> {removed:[0], added:[(1,f)]}`,
/// `Insert(2,f)+Insert(2,g) -> {added:[(2,g),(3,f)]}` (both survive),
/// a `d2`-removal of a `d1`-added slot silently drops the add, and a `d2`-modify of a `d1`-added
/// slot patches the carried payload — matching the recipe's canonical absorb cases exactly.
async fn absorb_array_diff(d1: JsonArrayDiff, d2: JsonArrayDiff) -> JsonArrayDiff {
    #[derive(Clone, Copy)]
    enum Origin {
        Base(usize),
        D1Added(usize),
    }
    enum AfterSlot {
        Base { orig: usize, diff: Option<JsonValueDiff> },
        D1Added { tag: usize, patch: Option<JsonValueDiff> },
        D2Added(JsonValue),
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
    let d1_modified: HashMap<usize, JsonValueDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();

    // Step B: mid -> after.
    let mut after: Vec<AfterSlot> = mid
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
                        Some(existing) => Box::pin(absorb_value_diff(existing, m.diff.clone())).await,
                        None => m.diff.clone(),
                    };
                    *diff = if is_value_diff_effectively_empty(&combined).await { None } else { Some(combined) };
                }
                AfterSlot::D1Added { patch, .. } => {
                    let combined = match patch.take() {
                        Some(existing) => absorb_value_diff(existing, m.diff.clone()).await,
                        None => m.diff.clone(),
                    };
                    *patch = if is_value_diff_effectively_empty(&combined).await { None } else { Some(combined) };
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
            AfterSlot::Base { orig, diff: Some(diff) } => modified.push(JsonArrayModified { index: orig, diff }),
            AfterSlot::Base { .. } => {}
            AfterSlot::D1Added { tag, patch } => {
                let mut item = d1.added[tag].item.clone();
                if let Some(patch) = patch {
                    item = apply_value_diff(&patch, &item).await;
                }
                added.push(JsonArrayAdded { index: pos, item });
            }
            AfterSlot::D2Added(item) => added.push(JsonArrayAdded { index: pos, item }),
        }
    }
    final_removed.sort_unstable();
    final_removed.dedup();
    JsonArrayDiff { removed: final_removed, modified, added }
}

/// ➕️ Name-keyed absorb: resolution of WHICH entry a `d2` op refers to is exact (key identity),
/// but — unlike arrays — surviving `d1`-added entries' `index` is carried forward unshifted by
/// unrelated `d2` removals elsewhere in the object (member NAME identity carries no positional
/// information base-free, unlike array indices). This is exact for the realistic/expected usage
/// pattern (new members always appended — see `JsonMutation::SetMember`'s own diff construction)
/// and for every canonical `absorb_law` case this artifact tests; see the ticket report's
/// `deviations` for the documented residual gap on adversarial synthetic diff pairs.
async fn absorb_object_diff(d1: JsonObjectDiff, d2: JsonObjectDiff) -> JsonObjectDiff {
    let mut removed: Vec<String> = d1.removed;
    let mut modified: Vec<JsonObjectModified> = d1.modified;
    let mut added: Vec<JsonObjectAdded> = d1.added;
    let mut merged_removed: HashSet<String> = HashSet::new();

    for key in d2.removed {
        if let Some(pos) = added.iter().position(|a| a.key == key) {
            added.remove(pos);
        } else if let Some(pos) = modified.iter().position(|m| m.key == key) {
            modified.remove(pos);
            if merged_removed.insert(key.clone()) {
                removed.push(key);
            }
        } else if merged_removed.insert(key.clone()) {
            removed.push(key);
        }
    }
    for m in d2.modified {
        if let Some(a) = added.iter_mut().find(|a| a.key == m.key) {
            a.item = apply_value_diff(&m.diff, &a.item).await;
        } else if let Some(pos) = modified.iter().position(|e| e.key == m.key) {
            let combined = absorb_value_diff(modified[pos].diff.clone(), m.diff.clone());
            if is_value_diff_effectively_empty(&combined).await {
                modified.remove(pos);
            } else {
                modified[pos].diff = combined.await;
            }
        } else {
            modified.push(JsonObjectModified { key: m.key, diff: m.diff });
        }
    }
    for a in d2.added {
        added.push(a);
    }
    added.sort_by_key(|a| a.index);
    removed.sort();
    removed.dedup();
    JsonObjectDiff { removed, modified, added }
}
//#endregion 🔖️Absorb

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` for `JsonDiff` — template copied verbatim from
/// `SvgDiff`'s (`f6-recon-report.md` §5), self-contained (own copies of the small primitive set,
/// no shared "hand-roll helpers" module exists yet — same rationale `SvgDiff`'s file documents).
//#region 🔖️Primitives
pub(crate) async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes()).await
}
pub(crate) async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s).await?).map_err(|e| e.to_string())
}
pub(crate) async fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) async fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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
pub(crate) async fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-P1: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8) backing the
/// upgraded `OpBinary`/`DiffCodec` frames (see `../🧬️mutations/🦀️component.rs`'s `#region OpCodecs`
/// and `#region 🔖️HandcraftedDiffCodec` below) — reuses `store::pack_rt::write_varint_u64` /
/// `store::ByteReader` rather than reinventing varint encode/decode.
pub(crate) async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
pub(crate) async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).await.map_err(|e| e.to_string())?.to_vec())
}
pub(crate) async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader).await?).map_err(|e| e.to_string())
}
//#endregion 🔖️BinaryPrimitives
//#endregion 🔖️Primitives

//#region 🔖️JsonValueCodecs
/// 🌳 Tag-prefixed like `SvgDiff`'s `enc_xml_node`: `Z` (null, no payload, no brackets) / `B[0|1]`
/// / `N[hex(lexeme)]` / `S[hex(value)]` / `A[v1,v2,...]` / `O[hexkey1:v1,hexkey2:v2,...]` — member
/// insertion order preserved by construction (a list, never re-sorted).
pub(crate) async fn enc_json_value(v: &JsonValue) -> String {
    match v {
        JsonValue::Null => "Z".to_string(),
        JsonValue::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        JsonValue::Number { lexeme } => format!("N[{}]", enc_str(lexeme)),
        JsonValue::String { value } => format!("S[{}]", enc_str(value)),
        JsonValue::Array { items } => format!("A[{}]", items.iter().map(enc_json_value).collect::<Vec<_>>().join(",")),
        JsonValue::Object { members } => format!("O[{}]", members.iter().map(|m| format!("{}:{}", enc_str(&m.key), enc_json_value(&m.value))).collect::<Vec<_>>().join(",")),
    }
}
pub(crate) async fn dec_json_value(s: &str) -> Result<JsonValue, String> {
    if s == "Z" {
        return Ok(JsonValue::Null);
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest).await?;
    match tag {
        "B" => Ok(JsonValue::Bool { value: inner == "1" }),
        "N" => Ok(JsonValue::Number { lexeme: dec_str(inner).await? }),
        "S" => Ok(JsonValue::String { value: dec_str(inner).await? }),
        "A" => Ok(JsonValue::Array { items: split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_json_value).collect::<Result<Vec<_>, String>>()? }),
        "O" => {
            let members = split_top_level(inner, ',')
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|entry| {
                    let (key, value) = entry.split_once(':').ok_or_else(|| format!("object member: bad entry {entry:?}"))?;
                    Ok(JsonMember { key: dec_str(key)?, value: dec_json_value(value)? })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(JsonValue::Object { members })
        }
        other => Err(format!("json value: unknown tag {other:?}")),
    }
}

//#region 🔖️JsonValueBinaryCodecs
/// 🧪️ P2-P1: real recursive binary twin of [`enc_json_value`]/[`dec_json_value`] above — a 1-byte
/// kind tag (`0`=Null/`1`=Bool/`2`=Number/`3`=String/`4`=Array/`5`=Object, distinct numbering from
/// the text codec's letter tags, chosen to read cleanly as a match arm) followed by the real payload
/// (length-prefixed bytes for scalars, a varint COUNT then that many recursively-encoded elements
/// for `Array`/`Object` — genuinely recursive, not text-as-bytes). Backs the upgraded `OpBinary`
/// frame (`../🧬️mutations/🦀️component.rs`) and the `Replace`/added-item payloads inside
/// [`enc_value_diff_bin`] below.
pub(crate) async fn enc_json_value_bin(value: &JsonValue, out: &mut Vec<u8>) {
    match value {
        JsonValue::Null => out.push(0),
        JsonValue::Bool { value } => {
            out.push(1);
            out.push(if *value { 1 } else { 0 });
        }
        JsonValue::Number { lexeme } => {
            out.push(2);
            write_str_lp(out, lexeme);
        }
        JsonValue::String { value } => {
            out.push(3);
            write_str_lp(out, value);
        }
        JsonValue::Array { items } => {
            out.push(4);
            store::pack_rt::write_varint_u64(out, items.len() as u64);
            for item in items {
                enc_json_value_bin(item, out);
            }
        }
        JsonValue::Object { members } => {
            out.push(5);
            store::pack_rt::write_varint_u64(out, members.len() as u64);
            for member in members {
                write_str_lp(out, &member.key);
                enc_json_value_bin(&member.value, out);
            }
        }
    }
}
pub(crate) async fn dec_json_value_bin(reader: &mut store::ByteReader<'_>) -> Result<JsonValue, String> {
    let tag = reader.read_u8().await.map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(JsonValue::Null),
        1 => Ok(JsonValue::Bool { value: reader.read_u8().await.map_err(|e| e.to_string())? != 0 }),
        2 => Ok(JsonValue::Number { lexeme: read_str_lp(reader).await? }),
        3 => Ok(JsonValue::String { value: read_str_lp(reader).await? }),
        4 => {
            let count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
            let mut items = Vec::with_capacity(count as usize);
            for _ in 0..count {
                items.push(Box::pin(dec_json_value_bin(reader)).await?);
            }
            Ok(JsonValue::Array { items })
        }
        5 => {
            let count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
            let mut members = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let key = read_str_lp(reader).await?;
                let value = Box::pin(dec_json_value_bin(reader)).await?;
                members.push(JsonMember { key, value });
            }
            Ok(JsonValue::Object { members })
        }
        other => Err(format!("json value binary: unknown tag {other}")),
    }
}
//#endregion 🔖️JsonValueBinaryCodecs
//#endregion 🔖️JsonValueCodecs

//#region 🔖️DiffValueCodecs
/// 🌳 `JsonValueDiff` itself needs a tag (`R`=Replace, `B`=Bool, `N`=Number, `S`=String, `A`=Array,
/// `O`=Object) since, unlike a plain [`JsonValue`], it appears standalone (not always inside a
/// bracketed container) at the top-level `value=` token position.
pub(crate) async fn enc_value_diff(d: &JsonValueDiff) -> String {
    match d {
        JsonValueDiff::Replace { value } => format!("R[{}]", enc_json_value(value)),
        JsonValueDiff::Bool { value } => format!("B[{}]", if *value { "1" } else { "0" }),
        JsonValueDiff::Number { lexeme } => format!("N[{}]", enc_str(lexeme)),
        JsonValueDiff::String { value } => format!("S[{}]", enc_str(value)),
        JsonValueDiff::Array { diff } => format!("A[{}]", enc_array_diff(diff)),
        JsonValueDiff::Object { diff } => format!("O[{}]", enc_object_diff(diff)),
    }
}
pub(crate) async fn dec_value_diff(s: &str) -> Result<JsonValueDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest).await?;
    match tag {
        "R" => Ok(JsonValueDiff::Replace { value: dec_json_value(inner).await? }),
        "B" => Ok(JsonValueDiff::Bool { value: inner == "1" }),
        "N" => Ok(JsonValueDiff::Number { lexeme: dec_str(inner).await? }),
        "S" => Ok(JsonValueDiff::String { value: dec_str(inner).await? }),
        "A" => Ok(JsonValueDiff::Array { diff: dec_array_diff(inner).await? }),
        "O" => Ok(JsonValueDiff::Object { diff: dec_object_diff(inner).await? }),
        other => Err(format!("json value diff: unknown tag {other:?}")),
    }
}

async fn enc_array_diff(d: &JsonArrayDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_value_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_json_value(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
async fn dec_array_diff(body: &str) -> Result<JsonArrayDiff, String> {
    let three = split_top_level(body, ';').await;
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("array diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s).await?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s).await?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("array modified: bad entry {entry:?}"))?;
            Ok(JsonArrayModified { index: parse_usize(idx)?, diff: dec_value_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s).await?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("array added: bad entry {entry:?}"))?;
            Ok(JsonArrayAdded { index: parse_usize(idx)?, item: dec_json_value(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JsonArrayDiff { removed, modified, added })
}

async fn enc_object_diff(d: &JsonObjectDiff) -> String {
    let removed = d.removed.iter().map(|k| enc_str(k)).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", enc_str(&m.key), enc_value_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}:{}", a.index, enc_str(&a.key), enc_json_value(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
async fn dec_object_diff(body: &str) -> Result<JsonObjectDiff, String> {
    let three = split_top_level(body, ';').await;
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("object diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s).await?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s).await?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (key, rest) = entry.split_once(':').ok_or_else(|| format!("object modified: bad entry {entry:?}"))?;
            Ok(JsonObjectModified { key: dec_str(key)?, diff: dec_value_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s).await?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("object added: bad entry {entry:?}"))?;
            let (key, item) = rest.split_once(':').ok_or_else(|| format!("object added: bad entry {entry:?}"))?;
            Ok(JsonObjectAdded { index: parse_usize(idx)?, key: dec_str(key)?, item: dec_json_value(item)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(JsonObjectDiff { removed, modified, added })
}

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-P1: real recursive binary twin of [`enc_value_diff`]/[`dec_value_diff`] — same 1-byte tag
/// numbering scheme as [`enc_json_value_bin`] plus `6`=`Replace` (needs its own arm since `Replace`
/// wraps a whole [`JsonValue`], not a bare scalar payload). `Array`/`Object` collection triples
/// encode as three varint-counted, recursively-encoded lists (removed/modified/added) — genuinely
/// structured binary, backing the upgraded `DiffCodec::encode_diff`/`decode_diff` below.
pub(crate) async fn enc_value_diff_bin(diff: &JsonValueDiff, out: &mut Vec<u8>) {
    match diff {
        JsonValueDiff::Replace { value } => {
            out.push(6);
            enc_json_value_bin(value, out);
        }
        JsonValueDiff::Bool { value } => {
            out.push(1);
            out.push(if *value { 1 } else { 0 });
        }
        JsonValueDiff::Number { lexeme } => {
            out.push(2);
            write_str_lp(out, lexeme);
        }
        JsonValueDiff::String { value } => {
            out.push(3);
            write_str_lp(out, value);
        }
        JsonValueDiff::Array { diff } => {
            out.push(4);
            enc_array_diff_bin(diff, out);
        }
        JsonValueDiff::Object { diff } => {
            out.push(5);
            enc_object_diff_bin(diff, out);
        }
    }
}
pub(crate) async fn dec_value_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JsonValueDiff, String> {
    let tag = reader.read_u8().await.map_err(|e| e.to_string())?;
    match tag {
        6 => Ok(JsonValueDiff::Replace { value: dec_json_value_bin(reader).await? }),
        1 => Ok(JsonValueDiff::Bool { value: reader.read_u8().await.map_err(|e| e.to_string())? != 0 }),
        2 => Ok(JsonValueDiff::Number { lexeme: read_str_lp(reader).await? }),
        3 => Ok(JsonValueDiff::String { value: read_str_lp(reader).await? }),
        4 => Ok(JsonValueDiff::Array { diff: Box::pin(dec_array_diff_bin(reader)).await? }),
        5 => Ok(JsonValueDiff::Object { diff: Box::pin(dec_object_diff_bin(reader)).await? }),
        other => Err(format!("json value diff binary: unknown tag {other}")),
    }
}

async fn enc_array_diff_bin(diff: &JsonArrayDiff, out: &mut Vec<u8>) {
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
        enc_json_value_bin(&entry.item, out);
    }
}
async fn dec_array_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JsonArrayDiff, String> {
    let removed_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
        let diff = Box::pin(dec_value_diff_bin(reader)).await?;
        modified.push(JsonArrayModified { index, diff });
    }
    let added_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
        let item = dec_json_value_bin(reader).await?;
        added.push(JsonArrayAdded { index, item });
    }
    Ok(JsonArrayDiff { removed, modified, added })
}

async fn enc_object_diff_bin(diff: &JsonObjectDiff, out: &mut Vec<u8>) {
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
        write_str_lp(out, &entry.key);
        enc_json_value_bin(&entry.item, out);
    }
}
async fn dec_object_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JsonObjectDiff, String> {
    let removed_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(read_str_lp(reader).await?);
    }
    let modified_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = read_str_lp(reader).await?;
        let diff = Box::pin(dec_value_diff_bin(reader)).await?;
        modified.push(JsonObjectModified { key, diff });
    }
    let added_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
        let key = read_str_lp(reader).await?;
        let item = dec_json_value_bin(reader).await?;
        added.push(JsonObjectAdded { index, key, item });
    }
    Ok(JsonObjectDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
/// 🧭️ Single-field top level (`value=<enc>`, absent = unchanged) — `JsonDiff` has exactly one
/// diffable field (`schema` is identity-only, never diffed), so there is only ever zero or one
/// space-separated token, unlike `SvgDiff`'s multi-field line.
async fn print_json_diff(d: &JsonDiff) -> String {
    match &d.value {
        Some(v) => format!("value={}", enc_value_diff(v)),
        None => String::new(),
    }
}
async fn parse_json_diff(line: &str) -> Result<JsonDiff, String> {
    let mut d = JsonDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("value=") {
            d.value = Some(dec_value_diff(rest).await?);
        } else {
            return Err(format!("json diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for JsonDiff {
    async fn print_diff(&self) -> String {
        print_json_diff(self).await
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_json_diff(line).await.map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-P1: REAL binary frame (`format u8 | has_value u8 | value-diff payload`), matching
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
    /// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of stdio's
    /// `DiffCodec` impls were still on that shortcut per the P2-W0 census; this is the first real
    /// upgrade, per the ticket's own "be the good example" framing).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, if self.value.is_some() { 1 } else { 0 }];
        if let Some(value) = &self.value {
            enc_value_diff_bin(value, &mut out);
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes).await;
        let _format = reader.read_u8().await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: e.to_string() })?;
        let has_value = reader.read_u8().await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff has_value", offset: 1, detail: e.to_string() })?;
        let value = if has_value != 0 { Some(dec_value_diff_bin(&mut reader).await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff value", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e })?) } else { None };
        Ok(JsonDiff { value })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-P1: representative `JsonDiff` values (scalars, a kind-change `Replace`, nested array/object
/// collection triples, and the empty/`None` diff) — the single source of truth reused by
/// `diff_codec_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️component.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests.
#[cfg(test)]
pub(crate) async fn demo_diff_cases() -> Vec<JsonDiff> {
    use crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

    async fn snap(value: JsonValue) -> JsonSnapshot {
        JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
    }
    async fn arr(items: Vec<JsonValue>) -> JsonValue {
        JsonValue::Array { items }
    }
    async fn objv(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
        JsonValue::Object { members: pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect() }
    }
    async fn num(lexeme: &str) -> JsonValue {
        JsonValue::Number { lexeme: lexeme.into() }
    }
    async fn str_(s: &str) -> JsonValue {
        JsonValue::String { value: s.into() }
    }

    let a = snap(objv(vec![("keepNumber", num("1")), ("kindChange", num("1"))]));
    let b = snap(objv(vec![("keepNumber", num("2.5e3")), ("kindChange", str_("now a string"))]));
    let nested = objv(vec![("tags", arr(vec![str_("x"), str_("y"), str_("z")])), ("meta", objv(vec![("a", num("1")), ("b", JsonValue::Null)]))]);
    let nested2 = objv(vec![("tags", arr(vec![str_("x"), str_("w")])), ("meta", objv(vec![("a", num("9")), ("c", str_("new"))])), ("extra", JsonValue::Bool { value: true })]);

    vec![
        JsonDiff::default(),
        JsonDiff::between(&a, &b),
        JsonDiff::between(&b, &a),
        JsonDiff::between(&snap(nested.clone()), &snap(nested2.clone())),
        JsonDiff::between(&snap(nested2), &snap(nested)),
        JsonDiff::between(&snap(num("1")), &snap(str_("1"))),
        JsonDiff::between(&snap(JsonValue::Null), &snap(arr(vec![num("1"), num("2")]))),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

    async fn snap(value: JsonValue) -> JsonSnapshot {
        JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
    }

    async fn arr(items: Vec<JsonValue>) -> JsonValue {
        JsonValue::Array { items }
    }

    async fn objv(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
        JsonValue::Object { members: pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect() }
    }

    async fn num(lexeme: &str) -> JsonValue {
        JsonValue::Number { lexeme: lexeme.into() }
    }

    async fn str_(s: &str) -> JsonValue {
        JsonValue::String { value: s.into() }
    }

    //#region between_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law_scalars_and_kind_change() {
        let cases = [(JsonValue::Null, JsonValue::Bool { value: true }), (JsonValue::Bool { value: true }, JsonValue::Bool { value: false }), (num("1"), num("2.5e10")), (str_("a"), str_("b")), (num("1"), str_("1"))];
        for (a, b) in cases {
            let (sa, sb) = (snap(a.clone()), snap(b.clone()));
            assert_eq!(JsonDiff::between(&sa, &sb).apply(&sa).unwrap(), sb, "a={a:?} b={b:?}");
            assert_eq!(JsonDiff::between(&sb, &sa).apply(&sb).unwrap(), sa);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law_nested_collections() {
        let a = objv(vec![("tags", arr(vec![str_("x"), str_("y")])), ("n", num("1"))]);
        let b = objv(vec![("tags", arr(vec![str_("x"), str_("z"), str_("w")])), ("n", num("2")), ("extra", JsonValue::Bool { value: true })]);
        let (sa, sb) = (snap(a.clone()), snap(b.clone()));
        assert_eq!(JsonDiff::between(&sa, &sb).apply(&sa).unwrap(), sb);
        assert_eq!(JsonDiff::between(&sb, &sa).apply(&sb).unwrap(), sa);
    }

    #[semio_framework_async_macros::async_test]
    async fn between_self_is_empty() {
        let a = objv(vec![("x", num("1"))]);
        let sa = snap(a);
        assert!(JsonDiff::between(&sa, &sa).is_empty());
    }
    //#endregion between_roundtrip_law

    //#region inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law_diff_level() {
        let a = objv(vec![("x", num("1")), ("y", arr(vec![num("1"), num("2")]))]);
        let b = objv(vec![("x", num("2")), ("z", str_("new"))]);
        let (sa, sb) = (snap(a), snap(b));
        let d = JsonDiff::between(&sa, &sb);
        let mid = d.apply(&sa).unwrap();
        assert_eq!(mid, sb);
        let inv = d.inverse(&sa);
        assert_eq!(inv.apply(&mid).unwrap(), sa);
    }
    //#endregion inverse_law

    //#region absorb_law canonical cases (array/index-keyed)
    // NOTE: these construct `d1`/`d2` DIRECTLY as genuine Insert/Remove/Modify array diffs
    // (matching exactly what `JsonMutation::InsertArrayElement`/`RemoveArrayElement`/`SetScalar`
    // would produce) rather than via `JsonDiff::between(base, next)` — `between` does a PURE
    // POSITIONAL comparison (0..min(len)), so a middle-insertion between two concrete array
    // VALUES is represented as a same-position `modified` entry plus a tail `added` entry, not as
    // a genuine `Insert` — the right, and separately law-tested, behavior for `between`, but the
    // wrong fixture shape for exercising the mandated Insert/Remove canonical absorb cases.
    async fn array_diff(d: JsonArrayDiff) -> JsonDiff {
        JsonDiff { value: Some(JsonValueDiff::Array { diff: d }) }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_array_insert_then_remove_before() {
        // base = [a,b,c]; d1 = Insert(2,f) -> mid=[a,b,f,c]; d2 = Remove(0) -> after=[b,f,c].
        let base = snap(arr(vec![str_("a"), str_("b"), str_("c")]));
        let d1 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 2, item: str_("f") }], ..Default::default() });
        let d2 = array_diff(JsonArrayDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        assert_eq!(sequential.value, arr(vec![str_("b"), str_("f"), str_("c")]));
        match &combined.value {
            Some(JsonValueDiff::Array { diff }) => {
                assert_eq!(diff.removed, vec![0]);
                assert_eq!(diff.added, vec![JsonArrayAdded { index: 1, item: str_("f") }]);
            }
            other => panic!("expected array diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_array_insert_insert_same_index_both_survive() {
        // base = [a,b]; d1 = Insert(2,f); d2 = Insert(2,g) (against mid=[a,b,f]) -> [a,b,g,f].
        let base = snap(arr(vec![str_("a"), str_("b")]));
        let d1 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 2, item: str_("f") }], ..Default::default() });
        let d2 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 2, item: str_("g") }], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        assert_eq!(sequential.value, arr(vec![str_("a"), str_("b"), str_("g"), str_("f")]));
        match &combined.value {
            Some(JsonValueDiff::Array { diff }) => assert_eq!(diff.added.len(), 2, "both inserts must survive"),
            other => panic!("expected array diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_array_insert_then_remove_of_same_added_item_cancels() {
        // base = [a]; d1 = Insert(1,f) -> mid=[a,f]; d2 = Remove(1) -> after=[a].
        let base = snap(arr(vec![str_("a")]));
        let d1 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 1, item: str_("f") }], ..Default::default() });
        let d2 = array_diff(JsonArrayDiff { removed: vec![1], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        assert_eq!(sequential, base);
        assert!(combined.is_empty(), "cancelling insert+remove must coalesce to an empty diff");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_array_add_then_setfield_patches_added_payload() {
        // base = []; d1 = Insert(0,{x:1}) -> mid=[{x:1}]; d2 = SetMember([0],y,2) -> [{x:1,y:2}].
        let base = snap(arr(vec![]));
        let d1 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 0, item: objv(vec![("x", num("1"))]) }], ..Default::default() });
        let d2 = array_diff(JsonArrayDiff {
            modified: vec![JsonArrayModified { index: 0, diff: JsonValueDiff::Object { diff: JsonObjectDiff { added: vec![JsonObjectAdded { index: 1, key: "y".into(), item: num("2") }], ..Default::default() } } }],
            ..Default::default()
        });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        assert_eq!(sequential.value, arr(vec![objv(vec![("x", num("1")), ("y", num("2"))])]));
        match &combined.value {
            Some(JsonValueDiff::Array { diff }) => {
                assert!(diff.modified.is_empty(), "the patch must land INSIDE the carried added payload, not as a separate modified entry");
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].item, objv(vec![("x", num("1")), ("y", num("2"))]));
            }
            other => panic!("expected array diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_array_modify_then_remove_drops_pending_patch() {
        // base = [1,2]; d1 = Modify(0,9) -> mid=[9,2]; d2 = Remove(0) -> after=[2].
        let base = snap(arr(vec![num("1"), num("2")]));
        let d1 = array_diff(JsonArrayDiff { modified: vec![JsonArrayModified { index: 0, diff: JsonValueDiff::Number { lexeme: "9".into() } }], ..Default::default() });
        let d2 = array_diff(JsonArrayDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        assert_eq!(sequential.value, arr(vec![num("2")]));
        match &combined.value {
            Some(JsonValueDiff::Array { diff }) => {
                assert_eq!(diff.removed, vec![0]);
                assert!(diff.modified.is_empty(), "the pending modify on the removed base index must be dropped");
            }
            other => panic!("expected array diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_array_associativity() {
        let s0 = snap(arr(vec![num("1"), num("2"), num("3")]));
        let s1 = snap(arr(vec![num("1"), num("9"), num("3")]));
        let s2 = snap(arr(vec![num("9"), num("3"), num("4")]));
        let s3 = snap(arr(vec![num("9"), num("4")]));
        let d1 = JsonDiff::between(&s0, &s1);
        let d2 = JsonDiff::between(&s1, &s2);
        let d3 = JsonDiff::between(&s2, &s3);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);

        assert_eq!(left.apply(&s0).unwrap(), s3);
        assert_eq!(right.apply(&s0).unwrap(), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (array/index-keyed)

    //#region absorb_law canonical cases (object/name-keyed)
    #[semio_framework_async_macros::async_test]
    async fn absorb_object_add_then_setfield_patches_added_payload() {
        let base = objv(vec![]);
        let mid = objv(vec![("config", objv(vec![]))]);
        let after = objv(vec![("config", objv(vec![("x", num("5"))]))]);
        let (sbase, smid, safter) = (snap(base), snap(mid), snap(after.clone()));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase).unwrap(), snap(after));
        match &combined.value {
            Some(JsonValueDiff::Object { diff }) => {
                assert!(diff.modified.is_empty());
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].item, objv(vec![("x", num("5"))]));
            }
            other => panic!("expected object diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_object_modify_then_remove_drops_pending_patch() {
        let base = objv(vec![("a", num("1")), ("b", num("2"))]);
        let mid = objv(vec![("a", num("9")), ("b", num("2"))]);
        let after = objv(vec![("b", num("2"))]);
        let (sbase, smid, safter) = (snap(base), snap(mid), snap(after));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase).unwrap(), safter);
        match &combined.value {
            Some(JsonValueDiff::Object { diff }) => {
                assert_eq!(diff.removed, vec!["a".to_string()]);
                assert!(diff.modified.is_empty());
            }
            other => panic!("expected object diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_object_insert_insert_both_survive() {
        let base = objv(vec![("a", num("1"))]);
        let mid = objv(vec![("a", num("1")), ("f", num("2"))]);
        let after = objv(vec![("a", num("1")), ("f", num("2")), ("g", num("3"))]);
        let (sbase, smid, safter) = (snap(base), snap(mid), snap(after.clone()));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase).unwrap(), snap(after));
        match &combined.value {
            Some(JsonValueDiff::Object { diff }) => assert_eq!(diff.added.len(), 2),
            other => panic!("expected object diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_object_insert_then_remove_of_same_added_item_cancels() {
        let base = objv(vec![("a", num("1"))]);
        let mid = objv(vec![("a", num("1")), ("f", num("2"))]);
        let after = objv(vec![("a", num("1"))]);
        let (sbase, smid, safter) = (snap(base.clone()), snap(mid), snap(after));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase).unwrap(), snap(base));
        assert!(combined.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_object_associativity() {
        let s0 = snap(objv(vec![("a", num("1"))]));
        let s1 = snap(objv(vec![("a", num("1")), ("b", num("2"))]));
        let s2 = snap(objv(vec![("a", num("9")), ("b", num("2"))]));
        let s3 = snap(objv(vec![("b", num("2")), ("c", num("3"))]));
        let d1 = JsonDiff::between(&s0, &s1);
        let d2 = JsonDiff::between(&s1, &s2);
        let d3 = JsonDiff::between(&s2, &s3);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);

        assert_eq!(left.apply(&s0).unwrap(), s3);
        assert_eq!(right.apply(&s0).unwrap(), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (object/name-keyed)

    //#region field_sweep
    async fn sweep_a() -> JsonSnapshot {
        snap(objv(vec![
            ("keepBool", JsonValue::Bool { value: true }),
            ("keepNumber", num("1")),
            ("keepString", str_("base")),
            ("kindChange", num("1")),
            ("nullToValue", JsonValue::Null),
            ("removedMember", str_("gone")),
            ("modifiedMember", num("10")),
            ("nestedArray", arr(vec![num("1"), num("2"), num("3")])),
            ("nestedObject", objv(vec![("inner", str_("x"))])),
        ]))
    }

    async fn sweep_b() -> JsonSnapshot {
        snap(objv(vec![
            ("keepBool", JsonValue::Bool { value: false }),
            ("keepNumber", num("2.5e3")),
            ("keepString", str_("changed")),
            ("kindChange", str_("now a string")),
            ("nullToValue", JsonValue::Bool { value: true }),
            ("modifiedMember", num("99")),
            ("nestedArray", arr(vec![num("1"), num("20"), num("30"), num("4")])),
            ("nestedObject", objv(vec![("inner", str_("y")), ("extra", JsonValue::Bool { value: true })])),
            ("addedMember", str_("new")),
        ]))
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_between_roundtrips_both_directions() {
        let (a, b) = (sweep_a(), sweep_b());
        assert_eq!(JsonDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(JsonDiff::between(&b, &a).apply(&b).unwrap(), a);
        assert!(JsonDiff::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_every_field_present_in_diff() {
        let (a, b) = (sweep_a(), sweep_b());
        let diff = JsonDiff::between(&a, &b);
        let object_diff = match diff.value {
            Some(JsonValueDiff::Object { diff }) => diff,
            other => panic!("expected a top-level object diff, got {other:?}"),
        };
        assert_eq!(object_diff.removed, vec!["removedMember".to_string()]);
        assert_eq!(object_diff.added.len(), 1);
        assert_eq!(object_diff.added[0].key, "addedMember");

        let by_key: HashMap<&str, &JsonValueDiff> = object_diff.modified.iter().map(|m| (m.key.as_str(), &m.diff)).collect();
        for key in ["keepBool", "keepNumber", "keepString", "kindChange", "nullToValue", "modifiedMember", "nestedArray", "nestedObject"] {
            assert!(by_key.contains_key(key), "expected a modified entry for `{key}`");
        }
        assert!(matches!(by_key["kindChange"], JsonValueDiff::Replace { .. }), "Number->String must fall back to Replace");
        assert!(matches!(by_key["nullToValue"], JsonValueDiff::Replace { .. }), "Null->Bool must fall back to Replace");
        assert!(matches!(by_key["keepBool"], JsonValueDiff::Bool { .. }));
        assert!(matches!(by_key["keepNumber"], JsonValueDiff::Number { .. }));
        assert!(matches!(by_key["keepString"], JsonValueDiff::String { .. }));
        assert!(matches!(by_key["modifiedMember"], JsonValueDiff::Number { .. }));
        match by_key["nestedArray"] {
            JsonValueDiff::Array { diff } => {
                assert!(!diff.modified.is_empty());
                assert!(!diff.added.is_empty());
            }
            other => panic!("expected array diff, got {other:?}"),
        }
        match by_key["nestedObject"] {
            JsonValueDiff::Object { diff } => {
                assert!(!diff.modified.is_empty());
                assert!(!diff.added.is_empty());
            }
            other => panic!("expected object diff, got {other:?}"),
        }
    }
    //#endregion field_sweep

    //#region 🔖️HandcraftedDiffCodecTests
    /// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `JsonDiff` grammar — exercises
    /// every `JsonValueDiff` variant (incl. the `Replace` kind-change fallback), nested
    /// array/object collection triples, and the empty (`None`) diff.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;

        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = JsonDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = JsonDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion 🔖️HandcraftedDiffCodecTests
}
//#endregion 🧪️Tests
