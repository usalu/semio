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
    fn default() -> Self {
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
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
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
pub fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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

pub fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
//#endregion 🔖️Parsing

//#region 🔖️IndexedCodec
pub fn enc_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", m.index, enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|a| format!("{}:{}", a.index, enc_t(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

pub fn dec_indexed_triple<D, T>(body: &str, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<IndexedTripleDiff<D, T>, String> {
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
pub fn enc_named_triple<K, D, T>(triple: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K) -> String, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(|k| enc_k(k)).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", enc_k(&m.key), enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(|t| enc_t(t)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

pub fn dec_named_triple<K, D, T>(s: &str, dec_k: impl Fn(&str) -> Result<K, String>, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<K, D, T>, String> {
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
pub fn enc_named_added<T>(a: &NamedAdded<T>, enc_t: impl Fn(&T) -> String) -> String {
    format!("{}:{}", a.index, enc_t(&a.item))
}
pub fn dec_named_added<T>(s: &str, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedAdded<T>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_t(rest)? })
}
//#endregion 🔖️NamedCodec

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn enc_u32(v: &u32) -> String {
        v.to_string()
    }
    fn dec_u32(s: &str) -> Result<u32, String> {
        s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
    }
    fn enc_str(v: &String) -> String {
        v.clone()
    }
    fn dec_str(s: &str) -> Result<String, String> {
        Ok(s.to_string())
    }

    #[test]
    fn indexed_triple_round_trips_through_hex_shape() {
        let diff: IndexedTripleDiff<u32, String> = IndexedTripleDiff { removed: vec![2, 5], modified: vec![IndexModified { index: 1, diff: 7 }], added: vec![IndexAdded { index: 3, item: "new".to_string() }] };
        let encoded = enc_indexed_triple(&diff, enc_u32, enc_str);
        let decoded = dec_indexed_triple(&encoded, dec_u32, dec_str).expect("decode");
        assert_eq!(decoded, diff);
    }

    #[test]
    fn named_triple_round_trips_through_hex_shape() {
        let diff: NamedTripleDiff<String, u32, String> = NamedTripleDiff { removed: vec!["gone".to_string()], modified: vec![NamedModified { key: "kept".to_string(), diff: 9 }], added: vec!["fresh".to_string()] };
        let encoded = enc_named_triple(&diff, enc_str, enc_u32, enc_str);
        let decoded = dec_named_triple(&encoded, dec_str, dec_u32, dec_str).expect("decode");
        assert_eq!(decoded, diff);
    }

    #[test]
    fn named_added_round_trips_through_hex_shape() {
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
    fn serde_json_round_trips_a_non_default_item_type() {
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
    fn empty_triples_round_trip_to_empty_brackets() {
        let diff: IndexedTripleDiff<u32, String> = IndexedTripleDiff::default();
        let encoded = enc_indexed_triple(&diff, enc_u32, enc_str);
        assert_eq!(encoded, "[];[];[]");
        let decoded = dec_indexed_triple(&encoded, dec_u32, dec_str).expect("decode");
        assert_eq!(decoded, diff);
    }

    #[test]
    fn nested_bracket_payload_does_not_confuse_the_top_level_split() {
        // 🧪️ Depth-awareness proof: an item whose own encoding contains "[a,b]" must not be torn
        // apart by the outer added-list comma split.
        fn enc_pair(v: &(u32, u32)) -> String {
            format!("[{},{}]", v.0, v.1)
        }
        fn dec_pair(s: &str) -> Result<(u32, u32), String> {
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
}
//#endregion 🔖️Tests
