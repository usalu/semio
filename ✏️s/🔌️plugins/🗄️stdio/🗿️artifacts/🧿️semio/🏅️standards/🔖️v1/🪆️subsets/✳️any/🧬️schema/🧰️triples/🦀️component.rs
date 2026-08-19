//! 🧰️ Shared generic diff/op-codec helpers for semio v1 subsets — hex-encoded, bracket-depth-
//! aware triple codecs for index-keyed and name-keyed collection diffs, ported from the bcf/docx
//! hand-rolled reference implementations
//! (`bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`,
//! `docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`) so all 13 W2
//! subset agents import this ONE copy instead of reinventing it 13 times. REAL and tested
//! (round-trip below) — load-bearing shared infrastructure, not a scaffolded placeholder.
//!
//! 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
//! STATE-MACHINES) — pure generic diff/op-codec helpers with no snapshot dependency of their
//! own, so they land in `✳️any`'s own schema (the artifact-wide shared vocabulary every subset
//! already builds on), never an engine. Reached at `standards::v1::subsets::any::schema::triples`
//! (no shorter shim — every consumer now uses this full path).

use serde::{Deserialize, Serialize};

//#region 🔖️IndexedTriple
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexAdded<T> {
    pub index: usize,
    pub item: T,
}

// 🩹 `bound(...)` override (same pattern as bcf's own local `NamedTripleDiff` copy, see that
// module's doc comment): without it, `#[derive(Deserialize)]` on a struct whose field is
// `#[serde(default)]` infers a spurious `D: Default`/`T: Default` bound via serde_derive's
// generic-parameter heuristic, even though `Vec<_>: Default` never actually needs `D`/`T: Default`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(serialize = "D: Serialize, T: Serialize", deserialize = "D: Deserialize<'de>, T: Deserialize<'de>"))]
pub struct IndexedTripleDiff<D, T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexModified<D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexAdded<T>>,
}

impl<D, T> Default for IndexedTripleDiff<D, T> {
    async fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}
//#endregion 🔖️IndexedTriple

//#region 🔖️NamedTriple
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedModified<K, D> {
    pub key: K,
    pub diff: D,
}

// 🩹 same `bound(...)` override as `IndexedTripleDiff` above (see that struct's comment) — required
// here too, `K`/`D`/`T` all appear only inside `#[serde(default)]` `Vec<_>` fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(serialize = "K: Serialize, D: Serialize, T: Serialize", deserialize = "K: Deserialize<'de>, D: Deserialize<'de>, T: Deserialize<'de>"))]
pub struct NamedTripleDiff<K, D, T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    async fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

/// 🛡️ Rejects malformed indexed collection operations before any candidate snapshot is changed.
pub async fn validate_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, base_len: usize, target: impl IntoIterator<Item = impl Into<String>>) -> protocol::MutationApplyResult<()> {
    let target: Vec<String> = target.into_iter().map(Into::into).collect();
    let mut removed = std::collections::BTreeSet::new();
    for &index in &diff.removed {
        if index >= base_len || !removed.insert(index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-remove-index", format!("remove index {index} is absent or duplicated")).at(target.clone()));
        }
    }
    let mut modified = std::collections::BTreeSet::new();
    for entry in &diff.modified {
        if entry.index >= base_len || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-modify-index", format!("modify index {} is absent, removed, or duplicated", entry.index)).at(target.clone()));
        }
    }
    let mut added = std::collections::BTreeSet::new();
    let mut length = base_len - removed.len();
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    for index in additions {
        if index > length || !added.insert(index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-index", format!("add index {index} is out of range or duplicated")).at(target.clone()));
        }
        length += 1;
    }
    Ok(())
}

/// 🛡️ Rejects missing, duplicate, overlapping, or colliding named collection operations.
pub async fn validate_named_triple<K, D, T, A>(base: &[T], diff: &NamedTripleDiff<K, D, A>, key_of_base: impl Fn(&T) -> K, key_of_added: impl Fn(&A) -> K, target: impl IntoIterator<Item = impl Into<String>>) -> protocol::MutationApplyResult<()>
where
    K: PartialEq + Clone + std::fmt::Debug,
{
    let target: Vec<String> = target.into_iter().map(Into::into).collect();
    let mut base_keys = Vec::new();
    for item in base {
        let key = key_of_base(item);
        if base_keys.contains(&key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-base-key", format!("base key {key:?} is duplicated")).at(target.clone()));
        }
        base_keys.push(key);
    }
    let mut removed = Vec::new();
    for key in &diff.removed {
        if !base_keys.contains(key) || removed.contains(key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-remove-key", format!("remove key {key:?} is absent or duplicated")).at(target.clone()));
        }
        removed.push(key.clone());
    }
    let mut modified = Vec::new();
    for entry in &diff.modified {
        if !base_keys.contains(&entry.key) || removed.contains(&entry.key) || modified.contains(&entry.key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-modify-key", format!("modify key {:?} is absent, removed, or duplicated", entry.key)).at(target.clone()));
        }
        modified.push(entry.key.clone());
    }
    let mut added = Vec::new();
    for item in &diff.added {
        let key = key_of_added(item);
        if base_keys.contains(&key) || added.contains(&key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-add-key", format!("add key {key:?} already exists or is duplicated")).at(target.clone()));
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
/// shared copy existed — see `s.stdio.value`'s own `🧬️schema/🔺️diff/🦀️component.rs` for the
/// reference usage this was hoisted from. Existing per-subset local copies are untouched (still
/// correct); only new W4/W5 consumers should import this one instead of reinventing it again.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedAdded<T> {
    pub index: usize,
    pub item: T,
}
//#endregion 🔖️NamedTriple

//#region 🔖️Parsing
/// 📐️ Bracket-depth-aware split — a `sep` inside `[...]` never splits (so a modified/added
/// entry's own nested `[...]` payload survives the outer `;`/`,` split intact).
pub async fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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

pub async fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
//#endregion 🔖️Parsing

//#region 🔖️IndexedCodec
pub async fn enc_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", m.index, enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|a| format!("{}:{}", a.index, enc_t(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

pub async fn dec_indexed_triple<D, T>(body: &str, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<IndexedTripleDiff<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("indexed triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed modified: bad entry {entry:?}"))?;
            Ok(IndexModified { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed added: bad entry {entry:?}"))?;
            Ok(IndexAdded { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_t(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IndexedTripleDiff { removed, modified, added })
}
//#endregion 🔖️IndexedCodec

//#region 🔖️NamedCodec
pub async fn enc_named_triple<K, D, T>(triple: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K) -> String, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(|k| enc_k(k)).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", enc_k(&m.key), enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(|t| enc_t(t)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

pub async fn dec_named_triple<K, D, T>(s: &str, dec_k: impl Fn(&str) -> Result<K, String>, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<K, D, T>, String> {
    let three = split_top_level(s, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|e| dec_k(e)).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (k, rest) = entry.split_once(':').ok_or_else(|| format!("named triple modified: bad entry {entry:?}"))?;
            Ok(NamedModified { key: dec_k(k)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|e| dec_t(e)).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}

/// 🧷 `NamedAdded<T>`-wrapping encode/decode pair (`index:item`, same `idx:rest` shape
/// `enc_indexed_triple`/`dec_indexed_triple` already use for `IndexAdded<T>` above) — for a
/// consumer instantiating `NamedTripleDiff<K, D, NamedAdded<T>>`, pass
/// `|a| enc_named_added(a, enc_t)`/`|s| dec_named_added(s, dec_t)` as `enc_named_triple`'s/
/// `dec_named_triple`'s own `enc_t`/`dec_t` argument.
pub async fn enc_named_added<T>(a: &NamedAdded<T>, enc_t: impl Fn(&T) -> String) -> String {
    format!("{}:{}", a.index, enc_t(&a.item))
}
pub async fn dec_named_added<T>(s: &str, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedAdded<T>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_t(rest)? })
}
//#endregion 🔖️NamedCodec

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn enc_u32(v: &u32) -> String {
        v.to_string()
    }
    async fn dec_u32(s: &str) -> Result<u32, String> {
        s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
    }
    async fn enc_str(v: &String) -> String {
        v.clone()
    }
    async fn dec_str(s: &str) -> Result<String, String> {
        Ok(s.to_string())
    }

    #[test]
    async fn indexed_triple_round_trips_through_hex_shape() {
        let diff: IndexedTripleDiff<u32, String> = IndexedTripleDiff { removed: vec![2, 5], modified: vec![IndexModified { index: 1, diff: 7 }], added: vec![IndexAdded { index: 3, item: "new".to_string() }] };
        let encoded = enc_indexed_triple(&diff, enc_u32, enc_str);
        let decoded = dec_indexed_triple(&encoded, dec_u32, dec_str).expect("decode");
        assert_eq!(decoded, diff);
    }

    #[test]
    async fn named_triple_round_trips_through_hex_shape() {
        let diff: NamedTripleDiff<String, u32, String> = NamedTripleDiff { removed: vec!["gone".to_string()], modified: vec![NamedModified { key: "kept".to_string(), diff: 9 }], added: vec!["fresh".to_string()] };
        let encoded = enc_named_triple(&diff, enc_str, enc_u32, enc_str);
        let decoded = dec_named_triple(&encoded, dec_str, dec_u32, dec_str).expect("decode");
        assert_eq!(decoded, diff);
    }

    #[test]
    async fn named_added_round_trips_through_hex_shape() {
        let diff: NamedTripleDiff<String, u32, NamedAdded<String>> = NamedTripleDiff { removed: vec![], modified: vec![], added: vec![NamedAdded { index: 2, item: "reinserted".to_string() }] };
        let encoded = enc_named_triple(&diff, enc_str, enc_u32, |a| enc_named_added(a, enc_str));
        let decoded = dec_named_triple(&encoded, dec_str, dec_u32, |s| dec_named_added(s, dec_str)).expect("decode");
        assert_eq!(decoded, diff);
    }

    /// 🩹 A non-`Default` item type proves the `bound(...)` override actually works: without it,
    /// `#[derive(Deserialize)]`'s spurious inferred `T: Default` bound would fail THIS test to
    /// even compile (not just at runtime) the moment `T` doesn't implement `Default` — exactly the
    /// shape every real `ArtifactSchema` diff-item type is in practice.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct NoDefault(u32);

    #[test]
    async fn serde_json_round_trips_a_non_default_item_type() {
        let diff: NamedTripleDiff<String, NoDefault, NoDefault> = NamedTripleDiff { removed: vec!["gone".to_string()], modified: vec![NamedModified { key: "kept".to_string(), diff: NoDefault(9) }], added: vec![NoDefault(3)] };
        let json = serde_json::to_string(&diff).expect("serialize");
        let decoded: NamedTripleDiff<String, NoDefault, NoDefault> = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded, diff);

        let idiff: IndexedTripleDiff<NoDefault, NoDefault> = IndexedTripleDiff { removed: vec![1], modified: vec![IndexModified { index: 0, diff: NoDefault(5) }], added: vec![IndexAdded { index: 2, item: NoDefault(7) }] };
        let ijson = serde_json::to_string(&idiff).expect("serialize");
        let idecoded: IndexedTripleDiff<NoDefault, NoDefault> = serde_json::from_str(&ijson).expect("deserialize");
        assert_eq!(idecoded, idiff);
    }

    #[test]
    async fn empty_triples_round_trip_to_empty_brackets() {
        let diff: IndexedTripleDiff<u32, String> = IndexedTripleDiff::default();
        let encoded = enc_indexed_triple(&diff, enc_u32, enc_str);
        assert_eq!(encoded, "[];[];[]");
        let decoded = dec_indexed_triple(&encoded, dec_u32, dec_str).expect("decode");
        assert_eq!(decoded, diff);
    }

    #[test]
    async fn nested_bracket_payload_does_not_confuse_the_top_level_split() {
        // 🧪️ Depth-awareness proof: an item whose own encoding contains "[a,b]" must not be torn
        // apart by the outer added-list comma split.
        async fn enc_pair(v: &(u32, u32)) -> String {
            format!("[{},{}]", v.0, v.1)
        }
        async fn dec_pair(s: &str) -> Result<(u32, u32), String> {
            let inner = strip_brackets(s)?;
            let parts = split_top_level(inner, ',');
            let [a, b] = parts.as_slice() else { return Err("expected 2 fields".to_string()) };
            Ok((dec_u32(a)?, dec_u32(b)?))
        }
        let diff: IndexedTripleDiff<u32, (u32, u32)> = IndexedTripleDiff { removed: vec![], modified: vec![], added: vec![IndexAdded { index: 0, item: (1, 2) }, IndexAdded { index: 1, item: (3, 4) }] };
        let encoded = enc_indexed_triple(&diff, enc_u32, enc_pair);
        let decoded = dec_indexed_triple(&encoded, dec_u32, dec_pair).expect("decode");
        assert_eq!(decoded, diff);
    }

    #[test]
    async fn indexed_preflight_rejects_missing_and_clamped_targets() {
        let missing: IndexedTripleDiff<(), ()> = IndexedTripleDiff { removed: vec![2], modified: Vec::new(), added: Vec::new() };
        let error = validate_indexed_triple(&missing, 1, ["items"]).unwrap_err();
        assert_eq!(error.code, "mutation.apply.invalid-remove-index");
        assert_eq!(error.target, vec!["items"]);

        let clamped: IndexedTripleDiff<(), ()> = IndexedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexAdded { index: 2, item: () }] };
        let error = validate_indexed_triple(&clamped, 0, ["items"]).unwrap_err();
        assert_eq!(error.code, "mutation.apply.invalid-add-index");
    }

    #[test]
    async fn named_preflight_rejects_missing_and_colliding_keys() {
        let missing: NamedTripleDiff<String, (), String> = NamedTripleDiff { removed: Vec::new(), modified: vec![NamedModified { key: "absent".into(), diff: () }], added: Vec::new() };
        let error = validate_named_triple(&["present".to_string()], &missing, Clone::clone, Clone::clone, ["items"]).unwrap_err();
        assert_eq!(error.code, "mutation.apply.invalid-modify-key");

        let collision: NamedTripleDiff<String, (), String> = NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec!["present".into()] };
        let error = validate_named_triple(&["present".to_string()], &collision, Clone::clone, Clone::clone, ["items"]).unwrap_err();
        assert_eq!(error.code, "mutation.apply.invalid-add-key");
    }
}
//#endregion 🔖️Tests
