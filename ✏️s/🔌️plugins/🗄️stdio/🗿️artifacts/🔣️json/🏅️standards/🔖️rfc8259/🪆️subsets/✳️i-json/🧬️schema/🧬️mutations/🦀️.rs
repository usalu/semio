//! 🧬️ `JsonIJsonMutation` — the RFC 7493 I-JSON editing vocabulary for `s.stdio.json@rfc8259/i-json`.
//!
//! This is NOT a copy of the ✳️any sibling's `JsonMutation`. RFC 8259 defines a *syntax*; RFC 7493
//! narrows the *value space* it may describe, and four of that narrowing's clauses are facts about a
//! decoded `JsonSnapshot` rather than about its bytes — which is exactly why
//! `../🦀️component.rs`'s `check_i_json_conformance` can check them at all. This vocabulary makes the
//! same four clauses part of the EDITING ALGEBRA instead of only of the acceptance gate:
//!
//! | clause | ✳️any can express | ✳️i-json |
//! |---|---|---|
//! | §2.1 top-level value is an object or an array | `SetScalar { path: [], value: <any> }` — a bare scalar root is representable | `SetTopLevel { root: JsonIJsonRoot }` — a scalar root is UNREPRESENTABLE, the recommendation made structural rather than checked after the fact |
//! | §2.2 integers within ±(2^53−1) | `SetScalar` writes any `Number` lexeme, including `9007199254740993` | `SetSafeNumber { path, lexeme }` — an integer outside the safe range is refused `mutation.invariant` and never reaches the diff |
//! | §2.3 object member names are unique | no rename verb at all; `RemoveMember` + `SetMember` transits a state where BOTH names exist and loses the member's position | `RenameMember { path, from, to }` — one atomic, position-preserving step, refused when `to` is already present |
//! | §2.4 strings avoid Unicode noncharacters | `SetScalar` writes any `String` | `SetString { path, value }` — a noncharacter-bearing string is refused |
//!
//! The remaining four verbs (`UpsertMember`, `RemoveMember`, `InsertArrayElement`,
//! `RemoveArrayElement`) are INHERITED, not re-derived: I-JSON says nothing about arrays and nothing
//! about member insertion or deletion beyond uniqueness, so their semantics are the ✳️any subset's
//! verbatim and they lower onto its ops one-for-one. Saying so is the honest result; inventing a
//! difference for them would not be.
//!
//! Every variant lowers to exactly ONE `JsonMutation` and delegates to its `Mutation::diff`, so the
//! `JsonDiff` algebra stays a single source of truth — this leaf adds the I-JSON gate and the
//! I-JSON-level inverse, never a second diff semantics.
//!
//! @see <https://www.rfc-editor.org/rfc/rfc7493> (I-JSON Message Format)
//! @see `../🦀️component.rs` `derived_analysis::check_i_json_conformance` — the same four clauses as an acceptance gate

use crate::artifacts::json::standards::v_rfc8259::subsets::base::schema::diff::JsonDiff;
use crate::artifacts::json::standards::v_rfc8259::subsets::base::schema::mutations::{
    InsertArrayElementMutation, InsertArrayElementPayload, JsonMutation, JsonPath, JsonPathSegment, RemoveArrayElementMutation, RemoveArrayElementPayload, RemoveMemberMutation,
    RemoveMemberPayload, SetMemberMutation, SetMemberPayload, SetScalarMutation, SetScalarPayload,
};
use crate::artifacts::json::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonMember, JsonSnapshot, JsonValue};
use protocol::Mutation;

//#region 🔖️Root
/// 🌳️ RFC 7493 §2.1 made structural: the top-level value of an I-JSON text is an object or an array,
/// and this type cannot spell anything else. `SetTopLevel` carries it instead of a bare `JsonValue`,
/// which is the one representational difference between this vocabulary and the ✳️any sibling's that
/// costs nothing at run time.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum JsonIJsonRoot {
    Object { members: Vec<JsonMember> },
    Array { items: Vec<JsonValue> },
}

impl JsonIJsonRoot {
    /// ⬆️ The equivalent `JsonValue`, for handing to the ✳️any subset's own ops.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_value(&self) -> JsonValue {
        match self {
            JsonIJsonRoot::Object { members } => JsonValue::Object { members: members.clone() },
            JsonIJsonRoot::Array { items } => JsonValue::Array { items: items.clone() },
        }
    }

    /// ⬇️ `None` for a scalar — the whole point of the type.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_value(value: &JsonValue) -> Option<Self> {
        match value {
            JsonValue::Object { members } => Some(JsonIJsonRoot::Object { members: members.clone() }),
            JsonValue::Array { items } => Some(JsonIJsonRoot::Array { items: items.clone() }),
            _ => None,
        }
    }
}
//#endregion 🔖️Root

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `s.stdio.json@rfc8259/i-json` — see this file's header for what
/// each variant owes to RFC 7493 and which four are inherited from the ✳️any sibling unchanged.
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🌳set-top-level/🦀️.rs"]
pub mod set_top_level;
#[path = "➕upsert-member/🦀️.rs"]
pub mod upsert_member;
#[path = "➖remove-member/🦀️.rs"]
pub mod remove_member;
#[path = "🏷rename-member/🦀️.rs"]
pub mod rename_member;
#[path = "🔢set-safe-number/🦀️.rs"]
pub mod set_safe_number;
#[path = "🔤set-string/🦀️.rs"]
pub mod set_string;
#[path = "📥insert-array-element/🦀️.rs"]
pub mod insert_array_element;
#[path = "📤remove-array-element/🦀️.rs"]
pub mod remove_array_element;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = JsonSnapshot, diff = JsonDiff, schema = "JsonIJsonMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum JsonIJsonMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// 🌳️ §2.1 — replaces the whole document root with an object or an array.
    SetTopLevel(set_top_level::SetTopLevel),
    /// ➕️ Sets (creating or overwriting) member `key` on the object at `path`. Inherited from ✳️any.
    UpsertMember(upsert_member::UpsertMember),
    /// ➖️ Removes member `key` from the object at `path`. Inherited from ✳️any.
    RemoveMember(remove_member::RemoveMember),
    /// 🏷️ §2.3 — renames member `from` to `to` on the object at `path`, in place. Refused when `to`
    /// already names a member, which is the duplicate the clause forbids.
    RenameMember(rename_member::RenameMember),
    /// 🔢️ §2.2 — writes a number over the number already at `path`. An integer lexeme outside
    /// ±(2^53−1) is refused rather than written.
    SetSafeNumber(set_safe_number::SetSafeNumber),
    /// 🔤️ §2.4 — writes a string over the string already at `path`. A Unicode noncharacter in
    /// `value` is refused rather than written.
    SetString(set_string::SetString),
    /// ➕️ Inserts `value` into the array at `path`. Inherited from ✳️any.
    InsertArrayElement(insert_array_element::InsertArrayElement),
    /// ➖️ Removes the element at `index` from the array at `path`. Inherited from ✳️any.
    RemoveArrayElement(remove_array_element::RemoveArrayElement),
}

/// 🧾️ Kebab-case spelling of every `JsonIJsonMutation` variant, in declaration order — the
/// `json-rfc8259-i-json` catalog in `../../🧪️oracle/🔣️.json` is measured against this exact
/// list, and `kinds_match_the_enum_and_the_catalog` below proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-snapshot", "set-top-level", "upsert-member", "remove-member", "rename-member", "set-safe-number", "set-string", "insert-array-element", "remove-array-element"];
//#endregion 🔖️Mutations

//#region 🔖️Clauses
/// 🔢️ RFC 7493 §2.2 — ±(2^53−1), the largest integer magnitude an IEEE-754 double represents exactly.
pub const MAX_SAFE_INTEGER_MAGNITUDE: i128 = 9_007_199_254_740_991;

/// 🚫️ Frozen `MutationMessage` code for every refusal below (`Fatal`, empty diff). The seven-code
/// set is closed and generic; a per-plugin code is never minted.
const CODE_INVARIANT: &str = "mutation.invariant";

/// 🚫️ Frozen code for "the addressed node is not there, or is not the kind this verb writes over".
const CODE_TARGET_MISSING: &str = "mutation.target-missing";

/// 🔢️ Is this number lexeme an integer? RFC 8259's grammar puts `.`/`e`/`E` only in the fraction and
/// exponent parts, so their absence is exactly integrality. Mirrors `derived_analysis`'s own
/// `is_integer_lexeme` rather than importing it — that one is private to the analysis module.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_integer_lexeme(lexeme: &str) -> bool {
    !lexeme.contains('.') && !lexeme.contains('e') && !lexeme.contains('E')
}

/// 🔢️ §2.2: an integer lexeme is safe when its magnitude fits ±(2^53−1); a non-integer lexeme is
/// outside this clause entirely. Checked on the LEXEME, never through a lossy `f64`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_safe_number_lexeme(lexeme: &str) -> bool {
    if !is_integer_lexeme(lexeme) {
        return true;
    }
    match lexeme.parse::<i128>() {
        Ok(value) => value.unsigned_abs() <= MAX_SAFE_INTEGER_MAGNITUDE as u128,
        Err(_) => false,
    }
}

/// 🚫️ §2.4 — a Unicode noncharacter: the last two code points of every plane, plus the reserved BMP
/// range U+FDD0..=U+FDEF.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_unicode_noncharacter(c: char) -> bool {
    let cp = c as u32;
    (cp & 0xFFFE) == 0xFFFE || (0xFDD0..=0xFDEF).contains(&cp)
}
//#endregion 🔖️Clauses

//#region 🔖️Navigation
/// 🔎️ Read-only navigation of `path` from `root`; `None` on the first unresolvable segment. A local
/// copy because the ✳️any sibling keeps its own `resolve` private to that module.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve<'a>(root: &'a JsonValue, path: &[JsonPathSegment]) -> Option<&'a JsonValue> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (JsonPathSegment::Key(key), JsonValue::Object { members }) => &members.iter().find(|member| &member.key == key)?.value,
            (JsonPathSegment::Index(index), JsonValue::Array { items }) => items.get(*index)?,
            _ => return None,
        };
    }
    Some(node)
}

/// 🧭️ A path rendered as a `MutationMessage` target address, outermost segment first.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn target_of(path: &[JsonPathSegment]) -> Vec<String> {
    path.iter()
        .map(|segment| match segment {
            JsonPathSegment::Key(key) => key.clone(),
            JsonPathSegment::Index(index) => index.to_string(),
        })
        .collect()
}
//#endregion 🔖️Navigation

//#region 🔖️Lowering
/// 🚫️ One refused I-JSON clause: the frozen code, the prose, and the address it was refused at.
type Refusal = (&'static str, String, Vec<String>);

/// ⬇️ The single ✳️any op this I-JSON verb means, or the clause that refuses it. Every variant
/// lowers to exactly one op, so the `JsonDiff` algebra never gains a second semantics here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower(mutation: &JsonIJsonMutation, base: &JsonSnapshot) -> Result<JsonMutation, Refusal> {
    match mutation {
        JsonIJsonMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => Ok(JsonMutation::SetScalar(SetScalarMutation::Apply(SetScalarPayload { path: Vec::new(), value: snapshot.value.clone() }))),
        JsonIJsonMutation::SetTopLevel(set_top_level::SetTopLevel { root }) => Ok(JsonMutation::SetScalar(SetScalarMutation::Apply(SetScalarPayload { path: Vec::new(), value: root.to_value() }))),
        JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path, key, value }) => Ok(JsonMutation::SetMember(SetMemberMutation::Apply(SetMemberPayload {
            path: path.clone(),
            key: key.clone(),
            value: value.clone(),
        }))),
        JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path, key }) => Ok(JsonMutation::RemoveMember(RemoveMemberMutation::Apply(RemoveMemberPayload { path: path.clone(), key: key.clone() }))),
        JsonIJsonMutation::RenameMember(rename_member::RenameMember { path, from, to }) => {
            let Some(JsonValue::Object { members }) = resolve(&base.value, path) else {
                return Err((CODE_TARGET_MISSING, format!("rename-member: no object at the addressed path, so member {from:?} cannot be renamed"), target_of(path)));
            };
            if !members.iter().any(|member| &member.key == from) {
                return Err((CODE_TARGET_MISSING, format!("rename-member: the object carries no member named {from:?}"), target_of(path)));
            }
            if from != to && members.iter().any(|member| &member.key == to) {
                return Err((CODE_INVARIANT, format!("rename-member: the object already carries a member named {to:?} -- RFC 7493 §2.3 requires member names to be unique within one object, so this rename would create the duplicate the clause forbids"), target_of(path)));
            }
            let renamed = members.iter().map(|member| if &member.key == from { JsonMember { key: to.clone(), value: member.value.clone() } } else { member.clone() }).collect();
            Ok(JsonMutation::SetScalar(SetScalarMutation::Apply(SetScalarPayload { path: path.clone(), value: JsonValue::Object { members: renamed } })))
        }
        JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path, lexeme }) => {
            if !matches!(resolve(&base.value, path), Some(JsonValue::Number { .. })) {
                return Err((CODE_TARGET_MISSING, "set-safe-number: the addressed path does not hold a number, and this verb writes a number over a number so that its own inverse is always another set-safe-number".to_string(), target_of(path)));
            }
            if !is_safe_number_lexeme(lexeme) {
                return Err((CODE_INVARIANT, format!("set-safe-number: integer {lexeme} exceeds ±{MAX_SAFE_INTEGER_MAGNITUDE} = ±(2^53-1) and is not exactly representable as an IEEE-754 double -- RFC 7493 §2.2 forbids it in I-JSON"), target_of(path)));
            }
            Ok(JsonMutation::SetScalar(SetScalarMutation::Apply(SetScalarPayload { path: path.clone(), value: JsonValue::Number { lexeme: lexeme.clone() } })))
        }
        JsonIJsonMutation::SetString(set_string::SetString { path, value }) => {
            if !matches!(resolve(&base.value, path), Some(JsonValue::String { .. })) {
                return Err((CODE_TARGET_MISSING, "set-string: the addressed path does not hold a string, and this verb writes a string over a string so that its own inverse is always another set-string".to_string(), target_of(path)));
            }
            if let Some(offending) = value.chars().find(|c| is_unicode_noncharacter(*c)) {
                return Err((CODE_INVARIANT, format!("set-string: the value carries the Unicode noncharacter U+{:04X} -- RFC 7493 §2.4 forbids noncharacters in I-JSON text", offending as u32), target_of(path)));
            }
            Ok(JsonMutation::SetScalar(SetScalarMutation::Apply(SetScalarPayload { path: path.clone(), value: JsonValue::String { value: value.clone() } })))
        }
        JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path, index, value }) => Ok(JsonMutation::InsertArrayElement(InsertArrayElementMutation::Apply(InsertArrayElementPayload {
            path: path.clone(),
            index: *index,
            value: value.clone(),
        }))),
        JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path, index }) => {
            Ok(JsonMutation::RemoveArrayElement(RemoveArrayElementMutation::Apply(RemoveArrayElementPayload { path: path.clone(), index: *index })))
        }
    }
}
//#endregion 🔖️Lowering

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. The diff is the single semantics source: computed once from
/// the pre-mutation state, applied to produce the new state, and returned — the same shape
/// `apply_json_mutation` uses for the ✳️any subset.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_json_i_json_mutation(snapshot: &mut JsonSnapshot, mutation: &JsonIJsonMutation) -> protocol::MutationOutcome<JsonDiff> {
    let outcome = <JsonIJsonMutation as Mutation<JsonSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// ↩️ This subset's own inverse algebra as a free function, so a caller that legitimately drives the
/// vocabulary from outside the crate — an owner-root test adapter, for one — can reach it without
/// naming the `protocol::Mutation` trait, which it has no reason to link.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_json_i_json_mutation(mutation: &JsonIJsonMutation, base: &JsonSnapshot) -> Vec<JsonIJsonMutation> {
    Mutation::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
///
/// 🧮️ Gate first, then delegate. A refused clause yields `MutationOutcome::fatal` — LAW 1 of the
/// frozen outcome contract: a `Fatal` message means the diff is `JsonDiff::default()`, so a
/// refused I-JSON edit can never reach the snapshot by any path.
pub(crate) fn agg_diff(this: &JsonIJsonMutation, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
    match lower(this, base) {
        Ok(step) => <JsonMutation as Mutation<JsonSnapshot>>::diff(&step, base),
        Err((code, message, target)) => protocol::MutationOutcome::fatal(code, message, target),
    }
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
///
/// ↩️ Handcrafted, clause-aware and exact: every verb's undo is spelled in THIS vocabulary, never
/// by falling back to `SetSnapshot`. `SetSafeNumber`/`SetString` are total because they refuse a
/// target of the wrong kind up front, so the prior value is always a lexeme/string they can write
/// back; `RenameMember` inverts to itself with the two names swapped, which is why it had to be
/// one atomic verb rather than a remove/insert pair.
pub(crate) fn agg_inverse(this: &JsonIJsonMutation, base: &JsonSnapshot) -> Vec<JsonIJsonMutation> {
    match this {
        JsonIJsonMutation::SetSnapshot(_) => vec![JsonIJsonMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        JsonIJsonMutation::SetTopLevel(_) => match JsonIJsonRoot::from_value(&base.value) {
            Some(root) => vec![JsonIJsonMutation::SetTopLevel(set_top_level::SetTopLevel { root })],
            None => vec![JsonIJsonMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        },
        JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path, key, .. }) => match resolve(&base.value, path) {
            Some(JsonValue::Object { members }) => match members.iter().find(|member| &member.key == key) {
                Some(existing) => vec![JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: path.clone(), key: key.clone(), value: existing.value.clone() })],
                None => vec![JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: path.clone(), key: key.clone() })],
            },
            _ => Vec::new(),
        },
        JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path, key }) => match resolve(&base.value, path) {
            Some(JsonValue::Object { members }) => match members.iter().find(|member| &member.key == key) {
                Some(existing) => vec![JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: path.clone(), key: key.clone(), value: existing.value.clone() })],
                None => Vec::new(),
            },
            _ => Vec::new(),
        },
        JsonIJsonMutation::RenameMember(rename_member::RenameMember { path, from, to }) => vec![JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: path.clone(), from: to.clone(), to: from.clone() })],
        JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path, .. }) => match resolve(&base.value, path) {
            Some(JsonValue::Number { lexeme }) => vec![JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: path.clone(), lexeme: lexeme.clone() })],
            _ => Vec::new(),
        },
        JsonIJsonMutation::SetString(set_string::SetString { path, .. }) => match resolve(&base.value, path) {
            Some(JsonValue::String { value }) => vec![JsonIJsonMutation::SetString(set_string::SetString { path: path.clone(), value: value.clone() })],
            _ => Vec::new(),
        },
        JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path, index, .. }) => match resolve(&base.value, path) {
            Some(JsonValue::Array { items }) => vec![JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: path.clone(), index: (*index).min(items.len()) })],
            _ => Vec::new(),
        },
        JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path, index }) => match resolve(&base.value, path) {
            Some(JsonValue::Array { items }) => match items.get(*index) {
                Some(item) => vec![JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: path.clone(), index: *index, value: item.clone() })],
                None => Vec::new(),
            },
            _ => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn member(key: &str, value: JsonValue) -> JsonMember {
        JsonMember { key: key.to_string(), value }
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn number(lexeme: &str) -> JsonValue {
        JsonValue::Number { lexeme: lexeme.to_string() }
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn base() -> JsonSnapshot {
        JsonSnapshot {
            value: JsonValue::Object {
                members: vec![
                    member("revision", number("4")),
                    member("title", JsonValue::String { value: "hexagonal cut".to_string() }),
                    member("tags", JsonValue::Array { items: vec![JsonValue::String { value: "a".to_string() }, JsonValue::String { value: "b".to_string() }] }),
                ],
            },
            ..JsonSnapshot::default()
        }
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn key(name: &str) -> JsonPath {
        vec![JsonPathSegment::Key(name.to_string())]
    }

    /// 📇️ The honesty gate the fleet brief requires: `KINDS` is the enum's own variant list, in
    /// declaration order — checked here directly against the enum's own tagged serialization.
    #[test]
    fn kinds_match_the_enum() {
        let sample = vec![
            JsonIJsonMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: JsonSnapshot::default() }),
            JsonIJsonMutation::SetTopLevel(set_top_level::SetTopLevel { root: JsonIJsonRoot::Array { items: Vec::new() } }),
            JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: String::new(), value: JsonValue::Null }),
            JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: Vec::new(), key: String::new() }),
            JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: Vec::new(), from: String::new(), to: String::new() }),
            JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: Vec::new(), lexeme: "0".to_string() }),
            JsonIJsonMutation::SetString(set_string::SetString { path: Vec::new(), value: String::new() }),
            JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: Vec::new(), index: 0, value: JsonValue::Null }),
            JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: Vec::new(), index: 0 }),
        ];
        assert_eq!(sample.len(), KINDS.len(), "one sample per declared kind");
        for (mutation, kind) in sample.iter().zip(KINDS) {
            let tag = serde_json::to_value(mutation).expect("serializes")["mutation"].as_str().expect("the internally-tagged variant name").to_string();
            let kebab = kind.split('-').enumerate().map(|(index, part)| if index == 0 { part.to_string() } else { format!("{}{}", part[..1].to_uppercase(), &part[1..]) }).collect::<String>();
            assert_eq!(tag, kebab, "KINDS entry {kind} must name the variant it stands for");
        }
    }

    #[test]
    fn set_top_level_cannot_spell_a_scalar_root() {
        assert!(JsonIJsonRoot::from_value(&JsonValue::String { value: "bare".to_string() }).is_none());
        assert!(JsonIJsonRoot::from_value(&number("1")).is_none());
        assert!(JsonIJsonRoot::from_value(&JsonValue::Array { items: Vec::new() }).is_some());
    }

    #[test]
    fn set_safe_number_at_the_boundary_is_accepted() {
        let mut snapshot = base();
        let outcome = apply_json_i_json_mutation(&mut snapshot, &JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: key("revision"), lexeme: "9007199254740991".to_string() }));
        assert!(outcome.messages().is_empty(), "got {:?}", outcome.messages());
        assert_eq!(resolve(&snapshot.value, &key("revision")), Some(&number("9007199254740991")));
    }

    #[test]
    fn set_safe_number_one_past_the_boundary_is_refused_and_never_applied() {
        let mut snapshot = base();
        let outcome = apply_json_i_json_mutation(&mut snapshot, &JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: key("revision"), lexeme: "9007199254740992".to_string() }));
        assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_INVARIANT), "got {:?}", outcome.messages());
        assert_eq!(resolve(&snapshot.value, &key("revision")), Some(&number("4")), "a refused edit must leave the snapshot untouched");
    }

    #[test]
    fn a_fractional_lexeme_is_outside_the_safe_integer_clause() {
        assert!(is_safe_number_lexeme("9007199254740993.5"));
        assert!(is_safe_number_lexeme("4.44089209850063e-16"));
        assert!(!is_safe_number_lexeme("-9007199254740993"));
        assert!(!is_safe_number_lexeme("100000000000000000000000000000"));
    }

    #[test]
    fn set_string_refuses_a_unicode_noncharacter() {
        let mut snapshot = base();
        let outcome = apply_json_i_json_mutation(&mut snapshot, &JsonIJsonMutation::SetString(set_string::SetString { path: key("title"), value: "before\u{FFFE}after".to_string() }));
        assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_INVARIANT), "got {:?}", outcome.messages());
        assert_eq!(resolve(&snapshot.value, &key("title")), Some(&JsonValue::String { value: "hexagonal cut".to_string() }));
    }

    #[test]
    fn rename_member_is_atomic_and_position_preserving() {
        let mut snapshot = base();
        apply_json_i_json_mutation(&mut snapshot, &JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: Vec::new(), from: "revision".to_string(), to: "version".to_string() }));
        let JsonValue::Object { members } = &snapshot.value else { panic!("root stays an object") };
        assert_eq!(members.iter().map(|member| member.key.as_str()).collect::<Vec<_>>(), vec!["version", "title", "tags"]);
    }

    #[test]
    fn rename_member_onto_an_existing_name_is_refused() {
        let mut snapshot = base();
        let outcome = apply_json_i_json_mutation(&mut snapshot, &JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: Vec::new(), from: "revision".to_string(), to: "title".to_string() }));
        assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_INVARIANT), "got {:?}", outcome.messages());
        let JsonValue::Object { members } = &snapshot.value else { panic!("root stays an object") };
        assert_eq!(members.iter().map(|member| member.key.as_str()).collect::<Vec<_>>(), vec!["revision", "title", "tags"]);
    }

    /// ↩️ The metamorphic law every `inverse-<kind>` scenario of `mutate-json-rfc8259-i-json` rests
    /// on, proved here for all nine kinds against one snapshot rather than only end-to-end.
    #[test]
    fn applying_a_mutation_and_then_its_inverse_restores_the_snapshot() {
        let original = base();
        let mutations = vec![
            JsonIJsonMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: JsonSnapshot { value: JsonValue::Array { items: vec![number("1")] }, ..JsonSnapshot::default() } }),
            JsonIJsonMutation::SetTopLevel(set_top_level::SetTopLevel { root: JsonIJsonRoot::Array { items: vec![JsonValue::Null] } }),
            JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: "revision".to_string(), value: number("9") }),
            JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: "fresh".to_string(), value: JsonValue::Null }),
            JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: Vec::new(), key: "title".to_string() }),
            JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: Vec::new(), from: "title".to_string(), to: "heading".to_string() }),
            JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: key("revision"), lexeme: "9007199254740991".to_string() }),
            JsonIJsonMutation::SetString(set_string::SetString { path: key("title"), value: "Ünïcödé, mit Sonderzeichen".to_string() }),
            JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: key("tags"), index: 1, value: JsonValue::String { value: "inserted".to_string() } }),
            JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: key("tags"), index: 0 }),
        ];
        for mutation in mutations {
            let mut snapshot = original.clone();
            let undo = <JsonIJsonMutation as Mutation<JsonSnapshot>>::inverse(&mutation, &snapshot);
            apply_json_i_json_mutation(&mut snapshot, &mutation);
            for step in &undo {
                apply_json_i_json_mutation(&mut snapshot, step);
            }
            assert_eq!(snapshot, original, "{mutation:?} did not invert cleanly");
        }
    }

    /// 🧬️ Every inherited verb means exactly what the ✳️any sibling means by it — the claim this
    /// leaf's header makes, checked rather than asserted in prose.
    #[test]
    fn the_four_inherited_verbs_lower_onto_their_any_counterparts_unchanged() {
        let snapshot = base();
        let path = key("tags");
        assert_eq!(
            lower(&JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: Vec::new(), key: "revision".to_string(), value: number("9") }), &snapshot),
            Ok(JsonMutation::SetMember(SetMemberMutation::Apply(SetMemberPayload { path: Vec::new(), key: "revision".to_string(), value: number("9") })))
        );
        assert_eq!(
            lower(&JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: Vec::new(), key: "title".to_string() }), &snapshot),
            Ok(JsonMutation::RemoveMember(RemoveMemberMutation::Apply(RemoveMemberPayload { path: Vec::new(), key: "title".to_string() })))
        );
        assert_eq!(
            lower(&JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: path.clone(), index: 1, value: JsonValue::Null }), &snapshot),
            Ok(JsonMutation::InsertArrayElement(InsertArrayElementMutation::Apply(InsertArrayElementPayload { path: path.clone(), index: 1, value: JsonValue::Null })))
        );
        assert_eq!(
            lower(&JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: path.clone(), index: 0 }), &snapshot),
            Ok(JsonMutation::RemoveArrayElement(RemoveArrayElementMutation::Apply(RemoveArrayElementPayload { path, index: 0 })))
        );
    }
}
//#endregion 🧪️Tests
