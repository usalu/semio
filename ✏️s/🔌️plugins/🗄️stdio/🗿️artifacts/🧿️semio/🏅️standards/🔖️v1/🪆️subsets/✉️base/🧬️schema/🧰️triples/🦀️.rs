//! 🧰️ Shared generic diff/op-codec helpers for semio v1 subsets — hex-encoded, bracket-depth-
//! aware triple codecs for index-keyed and name-keyed collection diffs, ported from the bcf/docx
//! hand-rolled reference implementations
//! (`bcf/🏅️standards/🔖️2.1/🪆️subsets/✉️base/🧬️schema/🔺️diff/🦀️.rs`,
//! `docx/🏅️standards/🔖️ecma-376/🪆️subsets/✉️base/🧬️schema/🔺️diff/🦀️.rs`) so all 13 W2
//! subset agents import this ONE copy instead of reinventing it 13 times. REAL and tested
//! (round-trip below) — load-bearing shared infrastructure, not a scaffolded placeholder.
//!
//! 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
//! STATE-MACHINES) — pure generic diff/op-codec helpers with no snapshot dependency of their
//! own, so they land in `✉️base`'s own schema (the artifact-wide shared vocabulary every subset
//! already builds on), never an engine. Reached at `standards::v1::subsets::any::schema::triples`
//! (no shorter shim — every consumer now uses this full path).

//#region 🔖️IndexedTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexAdded<T> {
    pub index: usize,
    pub item: T,
}

// 🩹 `#[derive(ToValue, FromValue)]` synthesizes `D: ToValue + FromValue`/`T: ToValue + FromValue`
// automatically per own type parameter (see `🌱️value/✨️derive`'s module docs) — no explicit
// `#[value(bound = "...")]` override needed here, unlike `serde_derive`'s own inference which
// bcf's local `NamedTripleDiff` copy had to work around explicitly.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexedTripleDiff<D, T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexModified<D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexAdded<T>>,
}

impl<D, T> Default for IndexedTripleDiff<D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}
//#endregion 🔖️IndexedTriple

//#region 🔖️NamedTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedModified<K, D> {
    pub key: K,
    pub diff: D,
}

// 🩹 same auto-synthesized-bound story as `IndexedTripleDiff` above (see that struct's comment) —
// no explicit `#[value(bound = "...")]` needed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedTripleDiff<K, D, T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

/// 🛡️ Rejects malformed indexed collection operations before any candidate snapshot is changed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, base_len: usize, target: impl IntoIterator<Item = impl Into<String>>) -> protocol::MutationApplyResult<()> {
    let target: Vec<String> = target.into_iter().map(Into::into).collect();
    let mut removed = std::collections::BTreeSet::new();
    for &index in &diff.removed {
        if index >= base_len || !removed.insert(index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-remove-index", format!("remove index {index} is absent or duplicated")).at(target));
        }
    }
    let mut modified = std::collections::BTreeSet::new();
    for entry in &diff.modified {
        if entry.index >= base_len || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-modify-index", format!("modify index {} is absent, removed, or duplicated", entry.index)).at(target));
        }
    }
    let mut added = std::collections::BTreeSet::new();
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    for (length, index) in (base_len - removed.len()..).zip(additions) {
        if index > length || !added.insert(index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("add index {index} is out of range or duplicated")).at(target));
        }
    }
    Ok(())
}

/// 🛡️ Rejects missing, duplicate, overlapping, or colliding named collection operations.
///
/// 🐛️ A key the same diff REMOVES may be re-added: `apply_named` retains survivors first and pushes
/// `added` onto the tail, so `removed + added` of one key is the only spelling this container has for
/// "move this member", and the key is present exactly once afterwards. Testing `added` against the
/// raw base keys instead of the post-removal ones made the validator disagree with the applier it
/// guards, and made a whole-collection REPLACEMENT — the only faithful diff for a `set-snapshot`
/// that reorders surviving members — unrepresentable. Measured on the real Nakagin Capsule Tower by
/// `🏛️mutate-semio-model`’s `mutate-set-snapshot`, which the applier handles correctly and this
/// preflight rejected with `mutation.apply.invalid-add-key`. A key that is NOT removed still
/// collides, exactly as before.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_named_triple<K, D, T, A>(base: &[T], diff: &NamedTripleDiff<K, D, A>, key_of_base: impl Fn(&T) -> K, key_of_added: impl Fn(&A) -> K, target: impl IntoIterator<Item = impl Into<String>>) -> protocol::MutationApplyResult<()>
where
    K: PartialEq + Clone + std::fmt::Debug,
{
    let target: Vec<String> = target.into_iter().map(Into::into).collect();
    let mut base_keys = Vec::new();
    for item in base {
        let key = key_of_base(item);
        if base_keys.contains(&key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-base-key", format!("base key {key:?} is duplicated")).at(target));
        }
        base_keys.push(key);
    }
    let mut removed = Vec::new();
    for key in &diff.removed {
        if !base_keys.contains(key) || removed.contains(key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-remove-key", format!("remove key {key:?} is absent or duplicated")).at(target));
        }
        removed.push(key.clone());
    }
    let mut modified = Vec::new();
    for entry in &diff.modified {
        if !base_keys.contains(&entry.key) || removed.contains(&entry.key) || modified.contains(&entry.key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-modify-key", format!("modify key {:?} is absent, removed, or duplicated", entry.key)).at(target));
        }
        modified.push(entry.key.clone());
    }
    let mut added = Vec::new();
    for item in &diff.added {
        let key = key_of_added(item);
        if (base_keys.contains(&key) && !removed.contains(&key)) || added.contains(&key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-key", format!("add key {key:?} already exists or is duplicated")).at(target));
        }
        added.push(key);
    }
    Ok(())
}

/// 🧷 Position-carrying "added" wrapper for name/id-keyed collections, supplied as `T` in
/// `NamedTripleDiff<K, D, NamedAdded<T>>` by any consumer that needs a re-added interior member to
/// land back at its real position instead of always being appended last (`IndexedTripleDiff`
/// already gets this for free via `IndexAdded<T>`; `NamedTripleDiff`'s own `added: Vec<T>` field
/// intentionally stays position-agnostic — most named/keyed collections don't care about order —
/// so this is opt-in via `T`, not a change to the struct itself). Was independently reinvented by
/// every W2 subset that needed it (`value::NamedAdded`, `json::JsonObjectAdded`, …) before this
/// shared copy existed — see `s.stdio.value`'s own `🧬️schema/🔺️diff/🦀️.rs` for the
/// reference usage this was hoisted from. Existing per-subset local copies are untouched (still
/// correct); only new W4/W5 consumers should import this one instead of reinventing it again.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedAdded<T> {
    pub index: usize,
    pub item: T,
}
//#endregion 🔖️NamedTriple

//#region 🔖️Parsing



//#endregion 🔖️Parsing

//#region 🔖️IndexedCodec



//#endregion 🔖️IndexedCodec

//#region 🔖️NamedCodec






//#endregion 🔖️NamedCodec

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
