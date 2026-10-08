//! 🔺️ JsonDiff — recursive, handcrafted diff mirroring `JsonValue`'s shape. `Array` gets an
//! index-keyed triple, `Object` gets a name-keyed triple; scalars get a `Replace` fallback when
//! the node KIND changes at a position, or a direct field diff when the kind is stable. No
//! `snapshot: Option<JsonSnapshot>` full-replace slot anywhere: every mutation names the value rows it changes.

use crate::schema::snapshot::JsonMember;
use crate::JsonSnapshot;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
// 🧭️ `DiffAlgebra` isn't yet on the `protocol` facade's curated re-export list (S1 added the
// trait but the facade wasn't updated — see s1-spine-report.md) so it's reached via the
// still-public `os_spr::command` path instead of touching that framework facade file.
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::{HashMap, HashSet};
//#region 🔖️CollectionDiffs
/// 📦️ Index-keyed `array` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JsonArrayDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JsonArrayModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JsonArrayAdded>,
}

/// 📦️ One `array.modified[]` entry — `index` refers to BASE state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JsonArrayModified {
    pub index: usize,
    pub diff: JsonValueDiff,
}

/// 📦️ One `array.added[]` entry — `index` refers to FINAL state, ascending insert at
/// `min(index, len)`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JsonArrayAdded {
    pub index: usize,
    pub item: JsonValue,
}

/// 📦️ Name-keyed `object` triple (member insertion order preserved on apply).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JsonObjectDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JsonObjectModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JsonObjectAdded>,
}

/// 📦️ One `object.modified[]` entry — `key` refers to BASE state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JsonObjectModified {
    pub key: String,
    pub diff: JsonValueDiff,
}

/// 📦️ One `object.added[]` entry — `index` is the FINAL Vec position (insertion order hint).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json.diff")]
pub struct JsonDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValueDiff>,
}

impl MutationDiff<JsonSnapshot> for JsonDiff {
    fn apply(&self, base: &JsonSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<JsonSnapshot> {
        if let Some(diff) = &self.value {
            validate_value_diff(diff, &base.value)?;
        }
        let mut next = base.clone();
        if let Some(diff) = &self.value {
            next.value = apply_value_diff(diff, &base.value);
        }
        Ok(next)
    }

    /// ➕️ Structural, total, base-free, sequential-coalesce absorb (see the module-level `Absorb`
    /// helpers below for the array/object transport algorithm). A composed collection diff that
    /// ends up structurally empty (e.g. an `Insert` immediately cancelled by a matching `Remove`)
    /// collapses back to `None` rather than surviving as a no-op `Some(Array{diff: <empty>})`.
    fn absorb(&mut self, other: Self) {
        self.value = match (self.value.take(), other.value) {
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
    }
}

impl DiffAlgebra<JsonSnapshot> for JsonDiff {
    /// 🔁️ The negative diff: walked structurally against the base value, never by applying the diff and differencing the result.
    fn inverse(&self, base: &JsonSnapshot) -> Self {
        JsonDiff { value: self.value.as_ref().map(|diff| inverse_value_diff(diff, &base.value)) }
    }

    fn between(base: &JsonSnapshot, other: &JsonSnapshot) -> Self {
        JsonDiff { value: value_diff_between(&base.value, &other.value) }
    }

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_value_diff(diff: &JsonValueDiff, base: &JsonValue) -> JsonValueDiff {
    match (diff, base) {
        (JsonValueDiff::Bool { .. }, JsonValue::Bool { value }) => JsonValueDiff::Bool { value: *value },
        (JsonValueDiff::Number { .. }, JsonValue::Number { lexeme }) => JsonValueDiff::Number { lexeme: lexeme.clone() },
        (JsonValueDiff::String { .. }, JsonValue::String { value }) => JsonValueDiff::String { value: value.clone() },
        (JsonValueDiff::Array { diff }, JsonValue::Array { items }) => JsonValueDiff::Array { diff: inverse_array_diff(diff, items) },
        (JsonValueDiff::Object { diff }, JsonValue::Object { members }) => JsonValueDiff::Object { diff: inverse_object_diff(diff, members) },
        _ => JsonValueDiff::Replace { value: base.clone() },
    }
}

/// ↩️ Negative array rows against the base items: added rows become removals at their final index, removed rows return at their base
/// index, and each modified row inverts against its base item at the index the row has after the diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_array_diff(diff: &JsonArrayDiff, base: &[JsonValue]) -> JsonArrayDiff {
    let mut removed_base = diff.removed.clone();
    removed_base.sort_unstable();
    removed_base.dedup();
    let mut added_final: Vec<usize> = diff.added.iter().map(|added| added.index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_base.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut modified: Vec<JsonArrayModified> = diff.modified.iter().filter_map(|m| base.get(m.index).map(|item| JsonArrayModified { index: after_index(m.index), diff: inverse_value_diff(&m.diff, item) })).collect();
    modified.sort_by_key(|m| m.index);
    let added = removed_base.iter().filter_map(|index| base.get(*index).map(|item| JsonArrayAdded { index: *index, item: item.clone() })).collect();
    JsonArrayDiff { removed: added_final, modified, added }
}

/// ↩️ Negative object rows against the base members: added members are removed by key, removed members return at their base
/// position, and each modified member inverts against its base value.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_object_diff(diff: &JsonObjectDiff, base: &[JsonMember]) -> JsonObjectDiff {
    let mut added_by_position: Vec<&JsonObjectAdded> = diff.added.iter().collect();
    added_by_position.sort_by_key(|added| added.index);
    let removed = added_by_position.iter().map(|added| added.key.clone()).collect();
    let modified = diff.modified.iter().filter_map(|m| base.iter().find(|member| member.key == m.key).map(|member| JsonObjectModified { key: m.key.clone(), diff: inverse_value_diff(&m.diff, &member.value) })).collect();
    let mut dropped: Vec<(usize, &JsonMember)> = base.iter().enumerate().filter(|(_, member)| diff.removed.contains(&member.key)).collect();
    dropped.sort_by_key(|(position, _)| *position);
    let added = dropped.into_iter().map(|(position, member)| JsonObjectAdded { index: position, key: member.key.clone(), item: member.value.clone() }).collect();
    JsonObjectDiff { removed, modified, added }
}

//#endregion 🔖️Diff

//#region 🔖️Apply
/// ▶️ Applies a [`JsonValueDiff`] against the corresponding base node.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_value_diff(diff: &JsonValueDiff, base: &JsonValue) -> JsonValue {
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
            JsonValue::Array { items: apply_array_diff(diff, items) }
        }
        JsonValueDiff::Object { diff } => {
            let members: &[JsonMember] = match base {
                JsonValue::Object { members } => members.as_slice(),
                _ => &[],
            };
            JsonValue::Object { members: apply_object_diff(diff, members) }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_value_diff(diff: &JsonValueDiff, base: &JsonValue) -> MutationApplyResult<()> {
    match diff {
        JsonValueDiff::Replace { .. } => Ok(()),
        JsonValueDiff::Bool { .. } if matches!(base, JsonValue::Bool { .. }) => Ok(()),
        JsonValueDiff::Number { .. } if matches!(base, JsonValue::Number { .. }) => Ok(()),
        JsonValueDiff::String { .. } if matches!(base, JsonValue::String { .. }) => Ok(()),
        JsonValueDiff::Array { diff } => match base {
            JsonValue::Array { items } => validate_array_diff(diff, items),
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "array diff targets a non-array value")),
        },
        JsonValueDiff::Object { diff } => match base {
            JsonValue::Object { members } => validate_object_diff(diff, members),
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "object diff targets a non-object value")),
        },
        _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "scalar diff targets a different JSON value kind")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_array_diff(diff: &JsonArrayDiff, base: &[JsonValue]) -> MutationApplyResult<()> {
    let mut removed = HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "array removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "array removal target is repeated"));
        }
    }
    let mut modified = HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "array modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "array modification targets a removed item"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "array modification target is repeated"));
        }
        validate_value_diff(&entry.diff, &base[entry.index]).map_err(|error| error.under(vec!["modified".to_string(), entry.index.to_string()]))?;
    }
    let final_len = base.len() - removed.len() + diff.added.len();
    let mut added = HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "array addition is outside the final collection"));
        }
        if !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "array addition occupies a repeated final position"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_object_diff(diff: &JsonObjectDiff, base: &[JsonMember]) -> MutationApplyResult<()> {
    let keys: Vec<&str> = base.iter().map(|member| member.key.as_str()).collect();
    for (position, key) in diff.removed.iter().enumerate() {
        if !keys.contains(&key.as_str()) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "object removal target does not exist"));
        }
        if diff.removed[..position].contains(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "object removal target is repeated"));
        }
    }
    for (position, modified) in diff.modified.iter().enumerate() {
        if !keys.contains(&modified.key.as_str()) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "object modification target does not exist"));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "object modification targets a removed member"));
        }
        if diff.modified[..position].iter().any(|candidate| candidate.key == modified.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "object modification target is repeated"));
        }
        let Some(member) = base.iter().find(|member| member.key == modified.key) else {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "object modification target does not exist"));
        };
        validate_value_diff(&modified.diff, &member.value).map_err(|error| error.under(vec!["modified".to_string(), modified.key.clone()]))?;
    }
    let final_len = base.len() - diff.removed.len() + diff.added.len();
    let mut added_keys = HashSet::new();
    let mut added_indices = HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "object addition is outside the final collection"));
        }
        if !added_indices.insert(entry.index) || keys.contains(&entry.key.as_str()) || !added_keys.insert(entry.key.clone()) || diff.removed.contains(&entry.key) || diff.modified.iter().any(|modified| modified.key == entry.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "object addition target already exists or conflicts"));
        }
    }
    Ok(())
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert at
/// `min(index, len)`). Out-of-range indices are graceful no-ops.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn value_diff_between(a: &JsonValue, b: &JsonValue) -> Option<JsonValueDiff> {
    if a == b {
        return None;
    }
    match (a, b) {
        (JsonValue::Bool { value: _ }, JsonValue::Bool { value: next }) => Some(JsonValueDiff::Bool { value: *next }),
        (JsonValue::Number { .. }, JsonValue::Number { lexeme }) => Some(JsonValueDiff::Number { lexeme: lexeme.clone() }),
        (JsonValue::String { value: _ }, JsonValue::String { value: next }) => Some(JsonValueDiff::String { value: next.clone() }),
        (JsonValue::Array { items: av }, JsonValue::Array { items: bv }) => {
            let diff = array_diff_between(av, bv);
            if is_array_diff_empty(&diff) {
                None
            } else {
                Some(JsonValueDiff::Array { diff })
            }
        }
        (JsonValue::Object { members: am }, JsonValue::Object { members: bm }) => {
            let diff = object_diff_between(am, bm);
            if is_object_diff_empty(&diff) {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn array_diff_between(a: &[JsonValue], b: &[JsonValue]) -> JsonArrayDiff {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if let Some(diff) = value_diff_between(&a[i], &b[i]) {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_array_diff_empty(d: &JsonArrayDiff) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_object_diff_empty(d: &JsonObjectDiff) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

/// 🕳️ Whether a (possibly freshly-absorbed) node diff represents no actual change. Scalar
/// replace/field diffs are never "empty" in isolation — a value can round-trip back to its
/// original through absorb and still legitimately carry an explicit `Some(original)` (same
/// accepted LWW-field limitation `compose`'s `CanonicalKitDiff` scalar fields have) — but a
/// collection diff with nothing removed/modified/added genuinely changes nothing and should
/// collapse away rather than survive as a no-op wrapper.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_value_diff_effectively_empty(d: &JsonValueDiff) -> bool {
    match d {
        JsonValueDiff::Array { diff } => is_array_diff_empty(diff),
        JsonValueDiff::Object { diff } => is_object_diff_empty(diff),
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
        (JsonValueDiff::Array { diff: a1 }, JsonValueDiff::Array { diff: a2 }) => JsonValueDiff::Array { diff: absorb_array_diff(a1, &a2) },
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_array_diff(d1: JsonArrayDiff, d2: &JsonArrayDiff) -> JsonArrayDiff {
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
                        Some(existing) => absorb_value_diff(existing, m.diff.clone()),
                        None => m.diff.clone(),
                    };
                    *diff = if is_value_diff_effectively_empty(&combined) { None } else { Some(combined) };
                }
                AfterSlot::D1Added { patch, .. } => {
                    let combined = match patch.take() {
                        Some(existing) => absorb_value_diff(existing, m.diff.clone()),
                        None => m.diff.clone(),
                    };
                    *patch = if is_value_diff_effectively_empty(&combined) { None } else { Some(combined) };
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

/// ➕️ Name-keyed absorb: resolution of WHICH entry a `d2` op refers to is exact (key identity).
/// Positions need the same replay arrays get, with one asymmetry: a `d1`-added entry's `index`
/// counts positions in `mid`, a `d2`-added entry's counts positions in `after`, and member NAME
/// identity carries no positional information base-free — so where a `d2` removal of a BASE member
/// sat inside `mid` is unknowable here. Two sound facts close the gap:
/// - each `d2` removal deletes exactly one slot from `mid`, so a surviving `d1`-added entry's final
///   index is at most `index - <removals>`; dropping it by the count of `d2` removals of `mid`
///   slots is EXACT whenever the removed members precede it (the expected pattern — new members are
///   appended, see `JsonMutation::SetMember`'s own diff construction) and otherwise stays an upper
///   bound that [`apply_object_diff`]'s normative `min(index, len)` clamp absorbs;
/// - a `d1`-added index is therefore never authoritatively BELOW a `d2`-added one it ties with, so
///   the stable sort keeps `d1`'s entry first and the tie is broken upwards.
///
/// The result is always a valid triple (strictly ascending, collision-free `added` indices), which
/// the unshifted carry-forward this replaced was not: `{a:1} -> {a:1,b:2} -> {a:9,b:2} -> {b:2,c:3}`
/// produced two additions claiming index 1 and failed `validate_object_diff` outright.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_object_diff(d1: JsonObjectDiff, d2: JsonObjectDiff) -> JsonObjectDiff {
    let mut removed: Vec<String> = d1.removed;
    let mut modified: Vec<JsonObjectModified> = d1.modified;
    let mut added: Vec<JsonObjectAdded> = d1.added;
    let mut merged_removed: HashSet<String> = HashSet::new();
    let mut base_slots_removed = 0usize;

    for key in d2.removed {
        if let Some(pos) = added.iter().position(|a| a.key == key) {
            let vacated = added[pos].index;
            added.remove(pos);
            for entry in added.iter_mut() {
                if entry.index > vacated {
                    entry.index -= 1;
                }
            }
        } else if let Some(pos) = modified.iter().position(|m| m.key == key) {
            modified.remove(pos);
            if merged_removed.insert(key.clone()) {
                removed.push(key);
                base_slots_removed += 1;
            }
        } else if merged_removed.insert(key.clone()) {
            removed.push(key);
            base_slots_removed += 1;
        }
    }
    for entry in added.iter_mut() {
        entry.index = entry.index.saturating_sub(base_slots_removed);
    }
    for m in d2.modified {
        if let Some(a) = added.iter_mut().find(|a| a.key == m.key) {
            a.item = apply_value_diff(&m.diff, &a.item);
        } else if let Some(pos) = modified.iter().position(|e| e.key == m.key) {
            let combined = absorb_value_diff(modified[pos].diff.clone(), m.diff.clone());
            if is_value_diff_effectively_empty(&combined) {
                modified.remove(pos);
            } else {
                modified[pos].diff = combined;
            }
        } else {
            modified.push(JsonObjectModified { key: m.key, diff: m.diff });
        }
    }
    for a in d2.added {
        added.push(a);
    }
    added.sort_by_key(|a| a.index);
    let mut next_free = 0usize;
    for entry in added.iter_mut() {
        entry.index = entry.index.max(next_free);
        next_free = entry.index + 1;
    }
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








//#region 🔖️BinaryPrimitives




//#endregion 🔖️BinaryPrimitives
//#endregion 🔖️Primitives

//#region 🔖️JsonValueCodecs



//#region 🔖️JsonValueBinaryCodecs


//#endregion 🔖️JsonValueBinaryCodecs
//#endregion 🔖️JsonValueCodecs

//#region 🔖️DiffValueCodecs









//#region 🔖️DiffValueBinaryCodecs








//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-P1: representative `JsonDiff` values (scalars, a kind-change `Replace`, nested array/object
/// collection triples, and the empty/`None` diff) — the single source of truth reused by
/// `diff_codec_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<JsonDiff> {
    use crate::STDIO_JSON_DOCUMENT_SCHEMA;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snap(value: JsonValue) -> JsonSnapshot {
        JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn arr(items: Vec<JsonValue>) -> JsonValue {
        JsonValue::Array { items }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn objv(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
        JsonValue::Object { members: pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn num(lexeme: &str) -> JsonValue {
        JsonValue::Number { lexeme: lexeme.into() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn str_(s: &str) -> JsonValue {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::JsonValue;
//#endregion 🔁️Re-exports


































