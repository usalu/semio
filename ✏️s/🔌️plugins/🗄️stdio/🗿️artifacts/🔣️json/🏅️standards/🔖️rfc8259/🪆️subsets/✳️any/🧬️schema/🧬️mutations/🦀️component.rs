//! 🧬️ JsonMutation — document mutation dispatch. Addresses a target node via a [`JsonPath`]
//! (mirrors the recipe's tree-nesting rule: `NodePath` stays mutation-level, each mutation's
//! `diff()` lowers it to a nested modified-chain via [`diff_at_path`]). Every variant's `diff()`
//! and `inverse()` is handcrafted directly against the sparse [`JsonDiff`] shape — never
//! apply-and-capture.

use crate::artifacts::json::schema::diff::{
    diff_set_snapshot, JsonArrayAdded, JsonArrayDiff, JsonArrayModified, JsonDiff, JsonObjectAdded, JsonObjectDiff, JsonObjectModified, JsonValueDiff,
};
use crate::artifacts::json::schema::diff::{dec_json_value, dec_str, enc_json_value, enc_str, parse_usize, split_top_level, strip_brackets};
use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
use crate::artifacts::json::JsonSnapshot;
use protocol::{Mutation, MutationDiff, OpText};
#[cfg(test)]
// 🧭️ `DiffAlgebra` isn't yet on the `protocol` facade's curated re-export list (S1 added the
// trait but the facade wasn't updated — see s1-spine-report.md) so it's reached via the
// still-public `os_spr::command` path instead of touching that framework facade file. Only the
// test module below calls `.is_empty()`/`.inverse()` via trait method syntax.
use protocol::os_spr::command::DiffAlgebra;
use serde::{Deserialize, Serialize};

//#region 🔖️JsonPath
/// 🧭️ One step of a [`JsonPath`] — a member name (object) or a position (array).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JsonPathSegment {
    Key(String),
    Index(usize),
}

/// 🧭️ Addresses a node inside a `JsonValue` tree, root-to-leaf.
pub type JsonPath = Vec<JsonPathSegment>;

/// 🔎️ Read-only navigation of `path` from `root`, `None` on the first unresolvable segment
/// (missing key, out-of-range index, or a segment applied to the wrong node kind).
fn resolve<'a>(root: &'a JsonValue, path: &[JsonPathSegment]) -> Option<&'a JsonValue> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (JsonPathSegment::Key(key), JsonValue::Object { members }) => &members.iter().find(|m| &m.key == key)?.value,
            (JsonPathSegment::Index(index), JsonValue::Array { items }) => items.get(*index)?,
            _ => return None,
        };
    }
    Some(node)
}
//#endregion 🔖️JsonPath

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.json`.
/// 🧪️ F6: `#[derive(dsl::DslOps)]` is unusable here for the same structural reason as `JsonDiff`
/// (see that file's doc comment, `f6-recon-report.md` §3a) — `SetSnapshot{snapshot: JsonSnapshot}`
/// recursively contains `JsonValue` (a data-carrying enum, no `DslField` impl exists or can exist
/// for it), and every other path-carrying variant carries a `JsonValue` directly AND a `JsonPath`
/// (`Vec<JsonPathSegment>`, itself a data-carrying enum: `Key(String)`/`Index(usize)`) — two
/// independent enum-shaped payloads per variant, not just one. `OpText`/`OpBinary` hand-rolled
/// below (§OpCodecs), grammar template copied from `SvgMutation`'s, reusing `JsonDiff`'s
/// `pub(crate)` grammar primitives (`hex_encode`/`enc_json_value`/`split_top_level`/...).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum JsonMutation {
    NoMutation,
    SetSnapshot { snapshot: JsonSnapshot },
    /// ➕️ Sets (creating or overwriting) member `key` on the object at `path` to `value`.
    SetMember { path: JsonPath, key: String, value: JsonValue },
    /// ➖️ Removes member `key` from the object at `path`, if present.
    RemoveMember { path: JsonPath, key: String },
    /// ➕️ Inserts `value` into the array at `path` at `index` (ascending-insert-clamped, per the
    /// normative apply contract).
    InsertArrayElement { path: JsonPath, index: usize, value: JsonValue },
    /// ➖️ Removes the element at `index` from the array at `path`, if present.
    RemoveArrayElement { path: JsonPath, index: usize },
    /// 🔁️ Replaces the whole node found at `path` (root, if empty) with `value`, regardless of
    /// its previous kind.
    SetScalar { path: JsonPath, value: JsonValue },
}

impl Default for JsonMutation {
    fn default() -> Self {
        JsonMutation::NoMutation
    }
}
//#endregion 🔖️Mutations

//#region 🔖️DiffAtPath
/// 🧩 Lowers a leaf [`JsonValueDiff`] (addressing the node found at `path`) into the nested
/// modified-chain matching the recipe's tree-nesting rule — no path addressing inside diffs
/// themselves, only at the mutation level.
fn diff_at_path(path: &[JsonPathSegment], leaf: Option<JsonValueDiff>) -> JsonDiff {
    JsonDiff { value: leaf.map(|leaf| wrap_at_path(path, leaf)) }
}

fn wrap_at_path(path: &[JsonPathSegment], leaf: JsonValueDiff) -> JsonValueDiff {
    match path.split_first() {
        None => leaf,
        Some((JsonPathSegment::Key(key), rest)) => JsonValueDiff::Object {
            diff: JsonObjectDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![JsonObjectModified { key: key.clone(), diff: wrap_at_path(rest, leaf) }],
            },
        },
        Some((JsonPathSegment::Index(index), rest)) => JsonValueDiff::Array {
            diff: JsonArrayDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![JsonArrayModified { index: *index, diff: wrap_at_path(rest, leaf) }],
            },
        },
    }
}
//#endregion 🔖️DiffAtPath

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. The diff is the single semantics source: it's computed
/// once from the pre-mutation state, applied to produce the new state, and returned.
pub fn apply_json_mutation(snapshot: &mut JsonSnapshot, mutation: &JsonMutation) -> JsonDiff {
    let diff = <JsonMutation as Mutation<JsonSnapshot>>::diff(mutation, snapshot);
    *snapshot = <JsonDiff as MutationDiff<JsonSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<JsonSnapshot> for JsonMutation {
    type Diff = JsonDiff;

    fn diff(&self, base: &JsonSnapshot) -> Self::Diff {
        match self {
            JsonMutation::NoMutation => JsonDiff::default(),
            JsonMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),

            JsonMutation::SetMember { path, key, value } => match resolve(&base.value, path) {
                Some(JsonValue::Object { members }) => match members.iter().find(|m| &m.key == key) {
                    Some(existing) => {
                        let leaf = crate::artifacts::json::schema::diff::value_diff_between(&existing.value, value);
                        diff_at_path(path, leaf.map(|diff| JsonValueDiff::Object {
                            diff: JsonObjectDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonObjectModified { key: key.clone(), diff }] },
                        }))
                    }
                    None => diff_at_path(path, Some(JsonValueDiff::Object {
                        diff: JsonObjectDiff {
                            removed: Vec::new(),
                            modified: Vec::new(),
                            added: vec![JsonObjectAdded { index: members.len(), key: key.clone(), item: value.clone() }],
                        },
                    })),
                },
                _ => JsonDiff::default(),
            },

            JsonMutation::RemoveMember { path, key } => match resolve(&base.value, path) {
                Some(JsonValue::Object { members }) if members.iter().any(|m| &m.key == key) => diff_at_path(path, Some(JsonValueDiff::Object {
                    diff: JsonObjectDiff { removed: vec![key.clone()], modified: Vec::new(), added: Vec::new() },
                })),
                _ => JsonDiff::default(),
            },

            JsonMutation::InsertArrayElement { path, index, value } => match resolve(&base.value, path) {
                Some(JsonValue::Array { items }) => diff_at_path(path, Some(JsonValueDiff::Array {
                    diff: JsonArrayDiff {
                        removed: Vec::new(),
                        modified: Vec::new(),
                        added: vec![JsonArrayAdded { index: (*index).min(items.len()), item: value.clone() }],
                    },
                })),
                _ => JsonDiff::default(),
            },

            JsonMutation::RemoveArrayElement { path, index } => match resolve(&base.value, path) {
                Some(JsonValue::Array { items }) if *index < items.len() => diff_at_path(path, Some(JsonValueDiff::Array {
                    diff: JsonArrayDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() },
                })),
                _ => JsonDiff::default(),
            },

            JsonMutation::SetScalar { path, value } => match resolve(&base.value, path) {
                Some(old) if old != value => diff_at_path(path, Some(JsonValueDiff::Replace { value: value.clone() })),
                _ => JsonDiff::default(),
            },
        }
    }

    /// ↩️ Handcrafted mutation-level inverse, key/index-aware — reads the pre-mutation `base`
    /// state to recover the exact undo (e.g. `SetMember` on an existing key inverts to a
    /// `SetMember` restoring the OLD value; on a fresh key it inverts to `RemoveMember`).
    fn inverse(&self, base: &JsonSnapshot) -> Vec<Self> {
        match self {
            JsonMutation::NoMutation => vec![JsonMutation::NoMutation],
            JsonMutation::SetSnapshot { .. } => vec![JsonMutation::SetSnapshot { snapshot: base.clone() }],

            JsonMutation::SetMember { path, key, .. } => match resolve(&base.value, path) {
                Some(JsonValue::Object { members }) => match members.iter().find(|m| &m.key == key) {
                    Some(existing) => vec![JsonMutation::SetMember { path: path.clone(), key: key.clone(), value: existing.value.clone() }],
                    None => vec![JsonMutation::RemoveMember { path: path.clone(), key: key.clone() }],
                },
                _ => vec![JsonMutation::NoMutation],
            },

            // ↩️ `SetMember` on an absent key always APPENDS (see `diff()` above), so naively
            // reinverting to a single `SetMember` would restore the VALUE but lose the ORIGINAL
            // POSITION whenever other members follow it. Restore exact position (required for
            // `inverse_law`'s exact-state-equality, since member order is significant) by first
            // removing every member that originally followed `key`, then re-adding `key` and each
            // of them back in original order — every re-add is an append, landing them exactly
            // where they started.
            JsonMutation::RemoveMember { path, key } => match resolve(&base.value, path) {
                Some(JsonValue::Object { members }) => match members.iter().position(|m| &m.key == key) {
                    Some(pos) => {
                        let tail: Vec<JsonMember> = members[pos + 1..].to_vec();
                        let mut steps: Vec<JsonMutation> = tail.iter().rev()
                            .map(|m| JsonMutation::RemoveMember { path: path.clone(), key: m.key.clone() })
                            .collect();
                        steps.push(JsonMutation::SetMember { path: path.clone(), key: key.clone(), value: members[pos].value.clone() });
                        steps.extend(tail.into_iter().map(|m| JsonMutation::SetMember { path: path.clone(), key: m.key, value: m.value }));
                        steps
                    }
                    None => vec![JsonMutation::NoMutation],
                },
                _ => vec![JsonMutation::NoMutation],
            },

            JsonMutation::InsertArrayElement { path, index, .. } => match resolve(&base.value, path) {
                Some(JsonValue::Array { items }) => vec![JsonMutation::RemoveArrayElement { path: path.clone(), index: (*index).min(items.len()) }],
                _ => vec![JsonMutation::NoMutation],
            },

            JsonMutation::RemoveArrayElement { path, index } => match resolve(&base.value, path) {
                Some(JsonValue::Array { items }) => match items.get(*index) {
                    Some(item) => vec![JsonMutation::InsertArrayElement { path: path.clone(), index: *index, value: item.clone() }],
                    None => vec![JsonMutation::NoMutation],
                },
                _ => vec![JsonMutation::NoMutation],
            },

            JsonMutation::SetScalar { path, .. } => match resolve(&base.value, path) {
                Some(old) => vec![JsonMutation::SetScalar { path: path.clone(), value: old.clone() }],
                None => vec![JsonMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `JsonMutation` (`#[derive(dsl::DslOps)]`
/// blocked, see the enum doc comment above) — reuses `JsonDiff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_json_value`/`split_top_level`/...) rather than duplicating them a second
/// time in this file. Grammar: `keyword arg=value ...` (space-separated, same shape the derive's
/// own handcrafted-wrapper convention uses, `f6-recon-report.md` §2), one match arm per variant.
fn enc_json_path_segment(seg: &JsonPathSegment) -> String {
    match seg {
        JsonPathSegment::Key(key) => format!("K[{}]", enc_str(key)),
        JsonPathSegment::Index(index) => format!("I[{index}]"),
    }
}
fn dec_json_path_segment(s: &str) -> Result<JsonPathSegment, String> {
    let (tag, rest) = s.split_at(1);
    match tag {
        "K" => Ok(JsonPathSegment::Key(dec_str(strip_brackets(rest)?)?)),
        "I" => Ok(JsonPathSegment::Index(parse_usize(strip_brackets(rest)?)?)),
        other => Err(format!("json path segment: unknown tag {other:?}")),
    }
}
fn enc_json_path(p: &JsonPath) -> String {
    format!("[{}]", p.iter().map(enc_json_path_segment).collect::<Vec<_>>().join(","))
}
fn dec_json_path(s: &str) -> Result<JsonPath, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_json_path_segment).collect()
}
fn enc_json_snapshot(s: &JsonSnapshot) -> String {
    format!("[{},{}]", enc_str(&s.schema), enc_json_value(&s.value))
}
fn dec_json_snapshot(s: &str) -> Result<JsonSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, value] = parts.as_slice() else { return Err(format!("json snapshot: expected 2 fields, got {}", parts.len())) };
    Ok(JsonSnapshot { schema: dec_str(schema)?, value: dec_json_value(value)? })
}

fn print_json_mutation(m: &JsonMutation) -> String {
    match m {
        JsonMutation::NoMutation => "no-mutation".to_string(),
        JsonMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_json_snapshot(snapshot)),
        JsonMutation::SetMember { path, key, value } => {
            format!("set-member path={} key={} value={}", enc_json_path(path), enc_str(key), enc_json_value(value))
        }
        JsonMutation::RemoveMember { path, key } => format!("remove-member path={} key={}", enc_json_path(path), enc_str(key)),
        JsonMutation::InsertArrayElement { path, index, value } => {
            format!("insert-array-element path={} index={index} value={}", enc_json_path(path), enc_json_value(value))
        }
        JsonMutation::RemoveArrayElement { path, index } => format!("remove-array-element path={} index={index}", enc_json_path(path)),
        JsonMutation::SetScalar { path, value } => format!("set-scalar path={} value={}", enc_json_path(path), enc_json_value(value)),
    }
}
fn parse_json_mutation(line: &str) -> Result<JsonMutation, String> {
    if line == "no-mutation" {
        return Ok(JsonMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("json mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("json mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(JsonMutation::SetSnapshot { snapshot: dec_json_snapshot(arg("snapshot")?)? }),
        "set-member" => Ok(JsonMutation::SetMember { path: dec_json_path(arg("path")?)?, key: dec_str(arg("key")?)?, value: dec_json_value(arg("value")?)? }),
        "remove-member" => Ok(JsonMutation::RemoveMember { path: dec_json_path(arg("path")?)?, key: dec_str(arg("key")?)? }),
        "insert-array-element" => Ok(JsonMutation::InsertArrayElement {
            path: dec_json_path(arg("path")?)?,
            index: usize_arg("index")?,
            value: dec_json_value(arg("value")?)?,
        }),
        "remove-array-element" => Ok(JsonMutation::RemoveArrayElement { path: dec_json_path(arg("path")?)?, index: usize_arg("index")? }),
        "set-scalar" => Ok(JsonMutation::SetScalar { path: dec_json_path(arg("path")?)?, value: dec_json_value(arg("value")?)? }),
        other => Err(format!("json mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for JsonMutation {
    fn print_op(&self) -> String {
        print_json_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_json_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification `JsonDiff`'s hand-rolled codec uses.
impl protocol::OpBinary for JsonMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

    fn snap(value: JsonValue) -> JsonSnapshot {
        JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
    }

    fn objv(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
        JsonValue::Object { members: pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect() }
    }

    fn arr(items: Vec<JsonValue>) -> JsonValue {
        JsonValue::Array { items }
    }

    fn num(lexeme: &str) -> JsonValue {
        JsonValue::Number { lexeme: lexeme.into() }
    }

    fn str_(s: &str) -> JsonValue {
        JsonValue::String { value: s.into() }
    }

    fn apply_and_check(base: &JsonSnapshot, mutation: JsonMutation) -> (JsonSnapshot, JsonDiff) {
        let mut via_apply = base.clone();
        let returned = apply_json_mutation(&mut via_apply, &mutation);
        let expected_diff = mutation.diff(base);
        assert_eq!(returned, expected_diff, "apply_json_mutation must return mutation.diff(base)");
        let via_diff_apply = expected_diff.apply(base);
        assert_eq!(via_apply, via_diff_apply, "m.diff(base).apply(base) must equal apply_json_mutation's result");
        (via_apply, returned)
    }

    //#region mutation_diff_law
    #[test]
    fn mutation_diff_law_all_variants() {
        let base = snap(objv(vec![("a", num("1")), ("list", arr(vec![num("1"), num("2")]))]));

        apply_and_check(&base, JsonMutation::NoMutation);
        apply_and_check(&base, JsonMutation::SetSnapshot { snapshot: snap(JsonValue::Bool { value: true }) });
        apply_and_check(&base, JsonMutation::SetMember { path: vec![], key: "a".into(), value: num("2") });
        apply_and_check(&base, JsonMutation::SetMember { path: vec![], key: "new".into(), value: str_("fresh") });
        apply_and_check(&base, JsonMutation::RemoveMember { path: vec![], key: "a".into() });
        apply_and_check(&base, JsonMutation::InsertArrayElement { path: vec![JsonPathSegment::Key("list".into())], index: 1, value: num("99") });
        apply_and_check(&base, JsonMutation::RemoveArrayElement { path: vec![JsonPathSegment::Key("list".into())], index: 0 });
        apply_and_check(&base, JsonMutation::SetScalar { path: vec![JsonPathSegment::Key("a".into())], value: str_("replaced") });
    }

    #[test]
    fn set_member_on_missing_key_adds_at_end() {
        let base = snap(objv(vec![("a", num("1"))]));
        let (result, _) = apply_and_check(&base, JsonMutation::SetMember { path: vec![], key: "b".into(), value: num("2") });
        assert_eq!(result.value, objv(vec![("a", num("1")), ("b", num("2"))]));
    }

    #[test]
    fn remove_member_missing_key_is_noop() {
        let base = snap(objv(vec![("a", num("1"))]));
        let (result, diff) = apply_and_check(&base, JsonMutation::RemoveMember { path: vec![], key: "missing".into() });
        assert_eq!(result, base);
        assert!(diff.is_empty());
    }

    #[test]
    fn nested_path_targets_inner_member() {
        let base = snap(objv(vec![("outer", objv(vec![("inner", num("1"))]))]));
        let (result, _) = apply_and_check(&base, JsonMutation::SetMember {
            path: vec![JsonPathSegment::Key("outer".into())],
            key: "inner".into(),
            value: num("42"),
        });
        assert_eq!(result.value, objv(vec![("outer", objv(vec![("inner", num("42"))]))]));
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    #[test]
    fn inverse_law_mutation_level_round_trips() {
        let base = snap(objv(vec![("a", num("1")), ("list", arr(vec![num("1"), num("2")]))]));
        let mutations = vec![
            JsonMutation::SetMember { path: vec![], key: "a".into(), value: num("2") },
            JsonMutation::SetMember { path: vec![], key: "new".into(), value: str_("fresh") },
            JsonMutation::RemoveMember { path: vec![], key: "a".into() },
            JsonMutation::InsertArrayElement { path: vec![JsonPathSegment::Key("list".into())], index: 1, value: num("99") },
            JsonMutation::RemoveArrayElement { path: vec![JsonPathSegment::Key("list".into())], index: 0 },
            JsonMutation::SetScalar { path: vec![JsonPathSegment::Key("a".into())], value: str_("replaced") },
        ];
        for mutation in mutations {
            let mut state = base.clone();
            apply_json_mutation(&mut state, &mutation);
            for undo in <JsonMutation as Mutation<JsonSnapshot>>::inverse(&mutation, &base) {
                apply_json_mutation(&mut state, &undo);
            }
            assert_eq!(state, base, "mutation {mutation:?} did not round-trip via its inverse");
        }
    }

    #[test]
    fn inverse_law_diff_level_matches_mutation_diff() {
        let base = snap(objv(vec![("a", num("1"))]));
        let mutation = JsonMutation::SetMember { path: vec![], key: "a".into(), value: num("2") };
        let diff = mutation.diff(&base);
        let mid = diff.apply(&base);
        let inv = diff.inverse(&base);
        assert_eq!(inv.apply(&mid), base);
    }
    //#endregion inverse_law

    //#region 🔖️OpCodecTests
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws over the hand-rolled `JsonMutation` grammar —
    /// exercises every variant, incl. nested/array/object payload values and a multi-segment
    /// `JsonPath` (mixing both `Key`/`Index` segment kinds).
    #[test]
    fn op_text_binary_roundtrip_law() {
        use protocol::{OpBinary, OpText};

        let mixed_path = vec![JsonPathSegment::Key("outer".into()), JsonPathSegment::Index(2), JsonPathSegment::Key("inner".into())];
        let cases = vec![
            JsonMutation::NoMutation,
            JsonMutation::SetSnapshot { snapshot: snap(objv(vec![("a", num("1")), ("b", arr(vec![str_("x"), JsonValue::Null, JsonValue::Bool { value: true }]))])) },
            JsonMutation::SetMember { path: vec![], key: "a".into(), value: num("2.5e10") },
            JsonMutation::SetMember { path: mixed_path.clone(), key: "k".into(), value: objv(vec![("nested", str_("v"))]) },
            JsonMutation::RemoveMember { path: vec![JsonPathSegment::Key("outer".into())], key: "gone".into() },
            JsonMutation::InsertArrayElement { path: vec![JsonPathSegment::Key("list".into())], index: 1, value: arr(vec![num("1"), num("2")]) },
            JsonMutation::RemoveArrayElement { path: vec![JsonPathSegment::Index(0)], index: 3 },
            JsonMutation::SetScalar { path: mixed_path, value: JsonValue::Null },
        ];
        for m in cases {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <JsonMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <JsonMutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch");
        }
    }
    //#endregion 🔖️OpCodecTests
}
//#endregion 🧪️Tests
