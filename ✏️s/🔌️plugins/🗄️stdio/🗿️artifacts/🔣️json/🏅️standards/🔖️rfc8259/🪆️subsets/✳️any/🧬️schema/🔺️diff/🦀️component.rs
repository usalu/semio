//! 🔺️ JsonDiff — recursive, handcrafted diff mirroring `JsonValue`'s shape. `Array` gets an
//! index-keyed triple, `Object` gets a name-keyed triple; scalars get a `Replace` fallback when
//! the node KIND changes at a position, or a direct field diff when the kind is stable. No
//! `snapshot: Option<JsonSnapshot>` full-replace slot anywhere — `SetSnapshot`'s own diff is the
//! sparse `between(base, next)` just like every other mutation.

use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
use crate::artifacts::json::JsonSnapshot;
use protocol::{DiffAlgebra, MutationDiff};
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JsonValueDiff {
    /// 🔁️ Whole-node replace — the node's KIND changed, or a mutation explicitly overwrites it.
    Replace { value: JsonValue },
    Bool { value: bool },
    Number { lexeme: String },
    String { value: String },
    Array { diff: JsonArrayDiff },
    Object { diff: JsonObjectDiff },
}
//#endregion 🔖️JsonValueDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.json`. `schema` is an identity field and is never diffed.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json.diff")]
pub struct JsonDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValueDiff>,
}

impl MutationDiff<JsonSnapshot> for JsonDiff {
    fn apply(&self, base: &JsonSnapshot) -> JsonSnapshot {
        let mut next = base.clone();
        if let Some(diff) = &self.value {
            next.value = apply_value_diff(diff, &base.value);
        }
        next
    }

    /// ➕️ Structural, total, base-free, sequential-coalesce absorb (see the module-level `Absorb`
    /// helpers below for the array/object transport algorithm).
    fn absorb(&mut self, other: Self) {
        self.value = match (self.value.take(), other.value) {
            (None, None) => None,
            (Some(d1), None) => Some(d1),
            (None, Some(d2)) => Some(d2),
            (Some(d1), Some(d2)) => Some(absorb_value_diff(d1, d2)),
        };
    }
}

impl DiffAlgebra<JsonSnapshot> for JsonDiff {
    /// 🔁️ Diff-level undo, derived generically from `between`: `mid = self.apply(base)`, then
    /// `between(mid, base)` is — by the `between_roundtrip_law` — exactly the diff that restores
    /// `base` when applied to `mid`.
    fn inverse(&self, base: &JsonSnapshot) -> Self {
        let mid = self.apply(base);
        Self::between(&mid, base)
    }

    fn between(base: &JsonSnapshot, other: &JsonSnapshot) -> Self {
        JsonDiff { value: value_diff_between(&base.value, &other.value) }
    }

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

/// 🧩 Builds the sparse `between(base, next)` diff for a `SetSnapshot` mutation — NOT a full
/// `snapshot: Option<JsonSnapshot>` replace slot.
pub fn diff_set_snapshot(base: &JsonSnapshot, next: &JsonSnapshot) -> JsonDiff {
    JsonDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️Apply
/// ▶️ Applies a [`JsonValueDiff`] against the corresponding base node.
pub fn apply_value_diff(diff: &JsonValueDiff, base: &JsonValue) -> JsonValue {
    match diff {
        JsonValueDiff::Replace { value } => value.clone(),
        JsonValueDiff::Bool { value } => JsonValue::Bool(*value),
        JsonValueDiff::Number { lexeme } => JsonValue::Number { lexeme: lexeme.clone() },
        JsonValueDiff::String { value } => JsonValue::String(value.clone()),
        JsonValueDiff::Array { diff } => {
            let items: &[JsonValue] = match base { JsonValue::Array(items) => items.as_slice(), _ => &[] };
            JsonValue::Array(apply_array_diff(diff, items))
        }
        JsonValueDiff::Object { diff } => {
            let members: &[JsonMember] = match base { JsonValue::Object(members) => members.as_slice(), _ => &[] };
            JsonValue::Object(apply_object_diff(diff, members))
        }
    }
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert at
/// `min(index, len)`). Out-of-range indices are graceful no-ops.
pub fn apply_array_diff(diff: &JsonArrayDiff, base: &[JsonValue]) -> Vec<JsonValue> {
    let mut items: Vec<JsonValue> = base.to_vec();
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

/// ▶️ Same normative apply semantics as arrays, keyed by member name instead of position.
pub fn apply_object_diff(diff: &JsonObjectDiff, base: &[JsonMember]) -> Vec<JsonMember> {
    let mut members: Vec<JsonMember> = base.to_vec();
    for m in &diff.modified {
        if let Some(pos) = members.iter().position(|mem| mem.key == m.key) {
            let old = members[pos].value.clone();
            members[pos].value = apply_value_diff(&m.diff, &old);
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
pub fn value_diff_between(a: &JsonValue, b: &JsonValue) -> Option<JsonValueDiff> {
    if a == b {
        return None;
    }
    match (a, b) {
        (JsonValue::Bool(_), JsonValue::Bool(next)) => Some(JsonValueDiff::Bool { value: *next }),
        (JsonValue::Number { .. }, JsonValue::Number { lexeme }) => Some(JsonValueDiff::Number { lexeme: lexeme.clone() }),
        (JsonValue::String(_), JsonValue::String(next)) => Some(JsonValueDiff::String { value: next.clone() }),
        (JsonValue::Array(av), JsonValue::Array(bv)) => {
            let diff = array_diff_between(av, bv);
            if is_array_diff_empty(&diff) { None } else { Some(JsonValueDiff::Array { diff }) }
        }
        (JsonValue::Object(am), JsonValue::Object(bm)) => {
            let diff = object_diff_between(am, bm);
            if is_object_diff_empty(&diff) { None } else { Some(JsonValueDiff::Object { diff }) }
        }
        _ => Some(JsonValueDiff::Replace { value: b.clone() }),
    }
}

/// 🧭️ Index-pairwise: `modified` compares `0..min(len)`, `removed` is the base tail, `added` is
/// the other tail (final-state indices, per the normative apply contract).
fn array_diff_between(a: &[JsonValue], b: &[JsonValue]) -> JsonArrayDiff {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if let Some(diff) = value_diff_between(&a[i], &b[i]) {
            modified.push(JsonArrayModified { index: i, diff });
        }
    }
    let removed: Vec<usize> = if a.len() > b.len() { (b.len()..a.len()).collect() } else { Vec::new() };
    let added: Vec<JsonArrayAdded> = if b.len() > a.len() {
        (a.len()..b.len()).map(|i| JsonArrayAdded { index: i, item: b[i].clone() }).collect()
    } else {
        Vec::new()
    };
    JsonArrayDiff { removed, modified, added }
}

/// 🧭️ Name-keyed: base members missing from `b` are `removed`; members present in both with a
/// changed value are `modified`; members only in `b` are `added` at their `b`-position (renames
/// are documented as `removed`+`added` — no rename detection).
fn object_diff_between(a: &[JsonMember], b: &[JsonMember]) -> JsonObjectDiff {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for am in a {
        match b.iter().find(|bm| bm.key == am.key) {
            Some(bm) => {
                if let Some(diff) = value_diff_between(&am.value, &bm.value) {
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

fn is_array_diff_empty(d: &JsonArrayDiff) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

fn is_object_diff_empty(d: &JsonObjectDiff) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}
//#endregion 🔖️Between

//#region 🔖️Absorb
/// ➕️ Diff-level absorb (base→mid composed with mid→after). `d2` always wins on a full `Replace`
/// (it fully determines the final value regardless of `d1`); a `Replace` in `d1` gets `d2` baked
/// into its known literal value via `apply_value_diff`; otherwise both sides share the same node
/// KIND (guaranteed by construction against the real intervening `mid` state) and compose
/// per-kind, recursing into collections.
fn absorb_value_diff(d1: JsonValueDiff, d2: JsonValueDiff) -> JsonValueDiff {
    if matches!(d2, JsonValueDiff::Replace { .. }) {
        return d2;
    }
    if let JsonValueDiff::Replace { value } = d1 {
        let merged = apply_value_diff(&d2, &value);
        return JsonValueDiff::Replace { value: merged };
    }
    match (d1, d2) {
        (JsonValueDiff::Bool { .. }, JsonValueDiff::Bool { value }) => JsonValueDiff::Bool { value },
        (JsonValueDiff::Number { .. }, JsonValueDiff::Number { lexeme }) => JsonValueDiff::Number { lexeme },
        (JsonValueDiff::String { .. }, JsonValueDiff::String { value }) => JsonValueDiff::String { value },
        (JsonValueDiff::Array { diff: a1 }, JsonValueDiff::Array { diff: a2 }) => JsonValueDiff::Array { diff: absorb_array_diff(a1, a2) },
        (JsonValueDiff::Object { diff: o1 }, JsonValueDiff::Object { diff: o2 }) => JsonValueDiff::Object { diff: absorb_object_diff(o1, o2) },
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
fn absorb_array_diff(d1: JsonArrayDiff, d2: JsonArrayDiff) -> JsonArrayDiff {
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
    let d1_modified: HashMap<usize, JsonValueDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();

    // Step B: mid -> after.
    let mut after: Vec<AfterSlot> = mid.iter().map(|origin| match origin {
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
                    *diff = Some(match diff.take() {
                        Some(existing) => absorb_value_diff(existing, m.diff.clone()),
                        None => m.diff.clone(),
                    });
                }
                AfterSlot::D1Added { patch, .. } => {
                    *patch = Some(match patch.take() {
                        Some(existing) => absorb_value_diff(existing, m.diff.clone()),
                        None => m.diff.clone(),
                    });
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
                    item = apply_value_diff(&patch, &item);
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
fn absorb_object_diff(d1: JsonObjectDiff, d2: JsonObjectDiff) -> JsonObjectDiff {
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
            a.item = apply_value_diff(&m.diff, &a.item);
        } else if let Some(existing) = modified.iter_mut().find(|e| e.key == m.key) {
            existing.diff = absorb_value_diff(existing.diff.clone(), m.diff.clone());
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

    fn snap(value: JsonValue) -> JsonSnapshot {
        JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
    }

    fn arr(items: Vec<JsonValue>) -> JsonValue {
        JsonValue::Array(items)
    }

    fn objv(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
        JsonValue::Object(pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect())
    }

    fn num(lexeme: &str) -> JsonValue {
        JsonValue::Number { lexeme: lexeme.into() }
    }

    fn str_(s: &str) -> JsonValue {
        JsonValue::String(s.into())
    }

    //#region between_roundtrip_law
    #[test]
    fn between_roundtrip_law_scalars_and_kind_change() {
        let cases = [
            (JsonValue::Null, JsonValue::Bool(true)),
            (JsonValue::Bool(true), JsonValue::Bool(false)),
            (num("1"), num("2.5e10")),
            (str_("a"), str_("b")),
            (num("1"), str_("1")),
        ];
        for (a, b) in cases {
            let (sa, sb) = (snap(a.clone()), snap(b.clone()));
            assert_eq!(JsonDiff::between(&sa, &sb).apply(&sa), sb, "a={a:?} b={b:?}");
            assert_eq!(JsonDiff::between(&sb, &sa).apply(&sb), sa);
        }
    }

    #[test]
    fn between_roundtrip_law_nested_collections() {
        let a = objv(vec![("tags", arr(vec![str_("x"), str_("y")])), ("n", num("1"))]);
        let b = objv(vec![("tags", arr(vec![str_("x"), str_("z"), str_("w")])), ("n", num("2")), ("extra", JsonValue::Bool(true))]);
        let (sa, sb) = (snap(a.clone()), snap(b.clone()));
        assert_eq!(JsonDiff::between(&sa, &sb).apply(&sa), sb);
        assert_eq!(JsonDiff::between(&sb, &sa).apply(&sb), sa);
    }

    #[test]
    fn between_self_is_empty() {
        let a = objv(vec![("x", num("1"))]);
        let sa = snap(a);
        assert!(JsonDiff::between(&sa, &sa).is_empty());
    }
    //#endregion between_roundtrip_law

    //#region inverse_law
    #[test]
    fn inverse_law_diff_level() {
        let a = objv(vec![("x", num("1")), ("y", arr(vec![num("1"), num("2")]))]);
        let b = objv(vec![("x", num("2")), ("z", str_("new"))]);
        let (sa, sb) = (snap(a), snap(b));
        let d = JsonDiff::between(&sa, &sb);
        let mid = d.apply(&sa);
        assert_eq!(mid, sb);
        let inv = d.inverse(&sa);
        assert_eq!(inv.apply(&mid), sa);
    }
    //#endregion inverse_law

    //#region absorb_law canonical cases (array/index-keyed)
    #[test]
    fn absorb_array_insert_then_remove_before() {
        // base = [a,b,c]; d1 = Insert(2,f) -> mid=[a,b,f,c]; d2 = Remove(0) -> after=[b,f,c].
        let base = arr(vec![str_("a"), str_("b"), str_("c")]);
        let mid = arr(vec![str_("a"), str_("b"), str_("f"), str_("c")]);
        let after = arr(vec![str_("b"), str_("f"), str_("c")]);
        let (sbase, smid, safter) = (snap(base), snap(mid.clone()), snap(after));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase), safter);
        assert_eq!(combined.apply(&sbase), d2.apply(&d1.apply(&sbase)));
        match &combined.value {
            Some(JsonValueDiff::Array { diff }) => {
                assert_eq!(diff.removed, vec![0]);
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].index, 1);
                assert_eq!(diff.added[0].item, str_("f"));
            }
            other => panic!("expected array diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_array_insert_insert_same_index_both_survive() {
        let base = arr(vec![str_("a"), str_("b")]);
        let mid = arr(vec![str_("a"), str_("b"), str_("f")]);
        let after = arr(vec![str_("a"), str_("b"), str_("g"), str_("f")]);
        let (sbase, smid, safter) = (snap(base), snap(mid), snap(after.clone()));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase), snap(after));
        match &combined.value {
            Some(JsonValueDiff::Array { diff }) => assert_eq!(diff.added.len(), 2, "both inserts must survive"),
            other => panic!("expected array diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_array_insert_then_remove_of_same_added_item_cancels() {
        let base = arr(vec![str_("a")]);
        let mid = arr(vec![str_("a"), str_("f")]);
        let after = arr(vec![str_("a")]);
        let (sbase, smid, safter) = (snap(base.clone()), snap(mid), snap(after));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase), snap(base));
        assert!(combined.is_empty(), "cancelling insert+remove must coalesce to an empty diff");
    }

    #[test]
    fn absorb_array_add_then_setfield_patches_added_payload() {
        let base = arr(vec![]);
        let mid = arr(vec![objv(vec![("x", num("1"))])]);
        let after = arr(vec![objv(vec![("x", num("1")), ("y", num("2"))])]);
        let (sbase, smid, safter) = (snap(base), snap(mid), snap(after.clone()));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase), snap(after));
        match &combined.value {
            Some(JsonValueDiff::Array { diff }) => {
                assert!(diff.modified.is_empty());
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].item, objv(vec![("x", num("1")), ("y", num("2"))]));
            }
            other => panic!("expected array diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_array_modify_then_remove_drops_pending_patch() {
        let base = arr(vec![num("1"), num("2")]);
        let mid = arr(vec![num("9"), num("2")]);
        let after = arr(vec![num("2")]);
        let (sbase, smid, safter) = (snap(base), snap(mid), snap(after));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase), safter);
        match &combined.value {
            Some(JsonValueDiff::Array { diff }) => {
                assert_eq!(diff.removed, vec![0]);
                assert!(diff.modified.is_empty(), "the pending modify on the removed base index must be dropped");
            }
            other => panic!("expected array diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_array_associativity() {
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

        assert_eq!(left.apply(&s0), s3);
        assert_eq!(right.apply(&s0), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (array/index-keyed)

    //#region absorb_law canonical cases (object/name-keyed)
    #[test]
    fn absorb_object_add_then_setfield_patches_added_payload() {
        let base = objv(vec![]);
        let mid = objv(vec![("config", objv(vec![]))]);
        let after = objv(vec![("config", objv(vec![("x", num("5"))]))]);
        let (sbase, smid, safter) = (snap(base), snap(mid), snap(after.clone()));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase), snap(after));
        match &combined.value {
            Some(JsonValueDiff::Object { diff }) => {
                assert!(diff.modified.is_empty());
                assert_eq!(diff.added.len(), 1);
                assert_eq!(diff.added[0].item, objv(vec![("x", num("5"))]));
            }
            other => panic!("expected object diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_object_modify_then_remove_drops_pending_patch() {
        let base = objv(vec![("a", num("1")), ("b", num("2"))]);
        let mid = objv(vec![("a", num("9")), ("b", num("2"))]);
        let after = objv(vec![("b", num("2"))]);
        let (sbase, smid, safter) = (snap(base), snap(mid), snap(after));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase), safter);
        match &combined.value {
            Some(JsonValueDiff::Object { diff }) => {
                assert_eq!(diff.removed, vec!["a".to_string()]);
                assert!(diff.modified.is_empty());
            }
            other => panic!("expected object diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_object_insert_insert_both_survive() {
        let base = objv(vec![("a", num("1"))]);
        let mid = objv(vec![("a", num("1")), ("f", num("2"))]);
        let after = objv(vec![("a", num("1")), ("f", num("2")), ("g", num("3"))]);
        let (sbase, smid, safter) = (snap(base), snap(mid), snap(after.clone()));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase), snap(after));
        match &combined.value {
            Some(JsonValueDiff::Object { diff }) => assert_eq!(diff.added.len(), 2),
            other => panic!("expected object diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_object_insert_then_remove_of_same_added_item_cancels() {
        let base = objv(vec![("a", num("1"))]);
        let mid = objv(vec![("a", num("1")), ("f", num("2"))]);
        let after = objv(vec![("a", num("1"))]);
        let (sbase, smid, safter) = (snap(base.clone()), snap(mid), snap(after));
        let d1 = JsonDiff::between(&sbase, &smid);
        let d2 = JsonDiff::between(&smid, &safter);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&sbase), snap(base));
        assert!(combined.is_empty());
    }

    #[test]
    fn absorb_object_associativity() {
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

        assert_eq!(left.apply(&s0), s3);
        assert_eq!(right.apply(&s0), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law canonical cases (object/name-keyed)

    //#region field_sweep
    fn sweep_a() -> JsonSnapshot {
        snap(objv(vec![
            ("keepBool", JsonValue::Bool(true)),
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

    fn sweep_b() -> JsonSnapshot {
        snap(objv(vec![
            ("keepBool", JsonValue::Bool(false)),
            ("keepNumber", num("2.5e3")),
            ("keepString", str_("changed")),
            ("kindChange", str_("now a string")),
            ("nullToValue", JsonValue::Bool(true)),
            ("modifiedMember", num("99")),
            ("nestedArray", arr(vec![num("1"), num("20"), num("30"), num("4")])),
            ("nestedObject", objv(vec![("inner", str_("y")), ("extra", JsonValue::Bool(true))])),
            ("addedMember", str_("new")),
        ]))
    }

    #[test]
    fn field_sweep_between_roundtrips_both_directions() {
        let (a, b) = (sweep_a(), sweep_b());
        assert_eq!(JsonDiff::between(&a, &b).apply(&a), b);
        assert_eq!(JsonDiff::between(&b, &a).apply(&b), a);
        assert!(JsonDiff::between(&a, &a).is_empty());
    }

    #[test]
    fn field_sweep_every_field_present_in_diff() {
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
}
//#endregion 🧪️Tests
