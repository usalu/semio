//! 🧬️ `JsonIJsonMutation` — the RFC 7493 I-JSON editing vocabulary for `s.stdio.json@rfc8259/i-json`.
//!
//! This is NOT a copy of the ✳️any sibling's `JsonMutation`. RFC 8259 defines a *syntax*; RFC 7493
//! narrows the *value space* it may describe, and four of that narrowing's clauses are facts about a
//! decoded `JsonSnapshot` rather than about its bytes — which is exactly why
//! `../🦀️.rs`'s `check_i_json_conformance` can check them at all. This vocabulary makes the
//! same four clauses part of the EDITING ALGEBRA instead of only of the acceptance gate:
//!
//! | clause | ✳️any can express | 🛜️i-json |
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
//! @see `../🦀️.rs` `derived_analysis::check_i_json_conformance` — the same four clauses as an acceptance gate

use crate::standards::v_rfc8259::subsets::base::schema::diff::JsonDiff;
use crate::standards::v_rfc8259::subsets::base::schema::mutations::{
    InsertArrayElementMutation, InsertArrayElementPayload, JsonMutation, JsonPath, JsonPathSegment, RemoveArrayElementMutation, RemoveArrayElementPayload, RemoveMemberMutation, RemoveMemberPayload, SetMemberMutation, SetMemberPayload,
    SetScalarMutation, SetScalarPayload,
};
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonMember, JsonSnapshot, JsonValue};
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
#[path = "📥insert-array-element/🦀️.rs"]
pub mod insert_array_element;
#[path = "📤remove-array-element/🦀️.rs"]
pub mod remove_array_element;
#[path = "➖remove-member/🦀️.rs"]
pub mod remove_member;
#[path = "🏷️rename-member/🦀️.rs"]
pub mod rename_member;
#[path = "🔢set-safe-number/🦀️.rs"]
pub mod set_safe_number;
/// 📐️ Typed content mutation for `s.stdio.json@rfc8259/i-json` — see this file's header for what
/// each variant owes to RFC 7493 and which four are inherited from the ✳️any sibling unchanged.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔤set-string/🦀️.rs"]
pub mod set_string;
#[path = "🌳set-top-level/🦀️.rs"]
pub mod set_top_level;
#[path = "➕upsert-member/🦀️.rs"]
pub mod upsert_member;
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
/// `json-rfc8259-i-json` catalog in `../../🔣️oracle.json` is measured against this exact
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
        JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path, key, value }) => Ok(JsonMutation::SetMember(SetMemberMutation::Apply(SetMemberPayload { path: path.clone(), key: key.clone(), value: value.clone() }))),
        JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path, key }) => Ok(JsonMutation::RemoveMember(RemoveMemberMutation::Apply(RemoveMemberPayload { path: path.clone(), key: key.clone() }))),
        JsonIJsonMutation::RenameMember(rename_member::RenameMember { path, from, to }) => {
            let Some(JsonValue::Object { members }) = resolve(&base.value, path) else {
                return Err((CODE_TARGET_MISSING, format!("rename-member: no object at the addressed path, so member {from:?} cannot be renamed"), target_of(path)));
            };
            if !members.iter().any(|member| &member.key == from) {
                return Err((CODE_TARGET_MISSING, format!("rename-member: the object carries no member named {from:?}"), target_of(path)));
            }
            if from != to && members.iter().any(|member| &member.key == to) {
                return Err((
                    CODE_INVARIANT,
                    format!("rename-member: the object already carries a member named {to:?} -- RFC 7493 §2.3 requires member names to be unique within one object, so this rename would create the duplicate the clause forbids"),
                    target_of(path),
                ));
            }
            let renamed = members.iter().map(|member| if &member.key == from { JsonMember { key: to.clone(), value: member.value.clone() } } else { member.clone() }).collect();
            Ok(JsonMutation::SetScalar(SetScalarMutation::Apply(SetScalarPayload { path: path.clone(), value: JsonValue::Object { members: renamed } })))
        }
        JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path, lexeme }) => {
            if !matches!(resolve(&base.value, path), Some(JsonValue::Number { .. })) {
                return Err((CODE_TARGET_MISSING, "set-safe-number: the addressed path does not hold a number, and this verb writes a number over a number so that its own inverse is always another set-safe-number".to_string(), target_of(path)));
            }
            if !is_safe_number_lexeme(lexeme) {
                return Err((
                    CODE_INVARIANT,
                    format!("set-safe-number: integer {lexeme} exceeds ±{MAX_SAFE_INTEGER_MAGNITUDE} = ±(2^53-1) and is not exactly representable as an IEEE-754 double -- RFC 7493 §2.2 forbids it in I-JSON"),
                    target_of(path),
                ));
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
        JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path, index, value }) => {
            Ok(JsonMutation::InsertArrayElement(InsertArrayElementMutation::Apply(InsertArrayElementPayload { path: path.clone(), index: *index, value: value.clone() })))
        }
        JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path, index }) => Ok(JsonMutation::RemoveArrayElement(RemoveArrayElementMutation::Apply(RemoveArrayElementPayload { path: path.clone(), index: *index }))),
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
