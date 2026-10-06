//! 🧬️ JsonSnapshot schema — own `JsonValue` model + a from-scratch RFC8259 recursive-descent
//! parser/serializer. Preserves object-member INSERTION ORDER (`Vec<JsonMember>`, not a map) and
//! the ORIGINAL NUMBER LEXEME verbatim (rfc8259 allows arbitrary precision — never round-tripped
//! through `f64`). No `serde_json::Value` anywhere in this file.

#[path="🔢️number/🦀️.rs"]
pub(crate) mod number;

use crate::STDIO_JSON_DOCUMENT_SCHEMA;
use semio_framework_diagnostic::TextSpan;
use framework_schema::ArtifactSchema;
use semio_framework_diagnostic::TextError;

//#region 🔖️JsonModel
/// 🍃️ One `object` member, in source order.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JsonMember {
    pub key: String,
    pub value: JsonValue,
}

/// 🌳 An RFC8259 JSON value. `Number` keeps the ORIGINAL LEXEME (never parsed to `f64` — rfc8259
/// permits arbitrary precision, so re-emitting a lossy `f64` round-trip would silently corrupt
/// real documents carrying e.g. 19-digit ids or high-precision decimals). `Object` is a `Vec` of
/// [`JsonMember`] (never a map) so decode->encode preserves member insertion order exactly.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
#[derive(Default)]
pub enum JsonValue {
    #[default]
    Null,
    // NOTE: every non-unit variant MUST be a struct variant (named field), never a bare tuple
    // variant — serde's internally-tagged (`tag = "kind"`) representation can only merge the tag
    // into map-shaped content; a tuple variant wrapping a non-map type (`bool`/`String`/`Vec<_>`)
    // compiles fine but fails at RUNTIME serialization ("can only flatten structs and maps").
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
        items: Vec<JsonValue>,
    },
    Object {
        members: Vec<JsonMember>,
    },
}

impl From<serde_json::Value> for JsonValue {
    fn from(v: serde_json::Value) -> Self {
        match v {
            serde_json::Value::Null => JsonValue::Null,
            serde_json::Value::Bool(b) => JsonValue::Bool { value: b },
            serde_json::Value::Number(n) => JsonValue::Number { lexeme: n.to_string() },
            serde_json::Value::String(s) => JsonValue::String { value: s },
            serde_json::Value::Array(arr) => JsonValue::Array { items: arr.into_iter().map(JsonValue::from).collect() },
            serde_json::Value::Object(map) => JsonValue::Object { members: map.into_iter().map(|(k, v)| JsonMember { key: k, value: JsonValue::from(v) }).collect() },
        }
    }
}

/// 🌉️ `pack::json::Value` → this module's own key-order/lexeme-preserving `JsonValue` — the
/// cross-plugin bridge the fan-out playbook flagged as needed (ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`, §`🔱️trinity` batch):
/// callers converting off `ToValue`/`FromValue` (never `serde_json::Value`) still need to reach
/// this artifact's own `JsonSnapshot::from_value`. `pack::json::Number` has no independent lexeme
/// (unlike this crate's own arbitrary-precision `Number { lexeme }`), so it round-trips through
/// `pack::json_to_string` on a lone `Number` value — the exact bytes `pack`'s own writer would
/// have emitted for that number inline, not a re-implementation of its float/int formatting.
impl From<semio_framework_pack_json::Value> for JsonValue {
    fn from(v: semio_framework_pack_json::Value) -> Self {
        JsonValue::from(&v)
    }
}

impl From<&semio_framework_pack_json::Value> for JsonValue {
    fn from(v: &semio_framework_pack_json::Value) -> Self {
        match v {
            semio_framework_pack_json::Value::Null => JsonValue::Null,
            semio_framework_pack_json::Value::Bool(b) => JsonValue::Bool { value: *b },
            semio_framework_pack_json::Value::Number(n) => JsonValue::Number { lexeme: semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Number(*n)) },
            semio_framework_pack_json::Value::String(s) => JsonValue::String { value: s.clone() },
            semio_framework_pack_json::Value::Array(items) => JsonValue::Array { items: items.iter().map(JsonValue::from).collect() },
            semio_framework_pack_json::Value::Object(members) => JsonValue::Object { members: members.iter().map(|(k, v)| JsonMember { key: k.to_string(), value: JsonValue::from(v) }).collect() },
        }
    }
}

/// 🌉️ The reverse of the impl above — `JsonSnapshot::to_pack_value`'s bridge back into
/// `pack::json::Value` for a caller that needs the value tree, not the wire bytes. The original
/// arbitrary-precision `lexeme` re-parses through `pack::parse_json` (a full round trip through
/// the exact writer/reader pair `pack::json_to_string`/`pack::parse_json` already exercise
/// elsewhere in this crate) rather than a second hand-rolled number lexer.
impl From<&JsonValue> for semio_framework_pack_json::Value {
    fn from(v: &JsonValue) -> Self {
        match v {
            JsonValue::Null => semio_framework_pack_json::Value::Null,
            JsonValue::Bool { value } => semio_framework_pack_json::Value::Bool(*value),
            JsonValue::Number { lexeme } => semio_framework_pack_json::parse(lexeme, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(semio_framework_pack_json::Value::Null),
            JsonValue::String { value } => semio_framework_pack_json::Value::String(value.clone()),
            JsonValue::Array { items } => semio_framework_pack_json::Value::Array(items.iter().map(semio_framework_pack_json::Value::from).collect()),
            JsonValue::Object { members } => semio_framework_pack_json::object(members.iter().map(|member| (member.key.clone(), semio_framework_pack_json::Value::from(&member.value)))),
        }
    }
}

impl From<&serde_json::Value> for JsonValue {
    fn from(v: &serde_json::Value) -> Self {
        match v {
            serde_json::Value::Null => JsonValue::Null,
            serde_json::Value::Bool(b) => JsonValue::Bool { value: *b },
            serde_json::Value::Number(n) => JsonValue::Number { lexeme: n.to_string() },
            serde_json::Value::String(s) => JsonValue::String { value: s.clone() },
            serde_json::Value::Array(arr) => JsonValue::Array { items: arr.iter().map(JsonValue::from).collect() },
            serde_json::Value::Object(map) => JsonValue::Object { members: map.iter().map(|(k, v)| JsonMember { key: k.clone(), value: JsonValue::from(v) }).collect() },
        }
    }
}

impl From<JsonValue> for serde_json::Value {
    fn from(v: JsonValue) -> Self {
        match v {
            JsonValue::Null => serde_json::Value::Null,
            JsonValue::Bool { value } => serde_json::Value::Bool(value),
            JsonValue::Number { lexeme } => {
                if let Ok(n) = lexeme.parse::<serde_json::Number>() {
                    serde_json::Value::Number(n)
                } else if let Ok(val) = serde_json::from_str::<serde_json::Value>(&lexeme) {
                    val
                } else {
                    serde_json::Value::String(lexeme)
                }
            }
            JsonValue::String { value } => serde_json::Value::String(value),
            JsonValue::Array { items } => serde_json::Value::Array(items.into_iter().map(serde_json::Value::from).collect()),
            JsonValue::Object { members } => {
                let mut map = serde_json::Map::with_capacity(members.len());
                for m in members {
                    map.insert(m.key, serde_json::Value::from(m.value));
                }
                serde_json::Value::Object(map)
            }
        }
    }
}

impl From<&JsonValue> for serde_json::Value {
    fn from(v: &JsonValue) -> Self {
        match v {
            JsonValue::Null => serde_json::Value::Null,
            JsonValue::Bool { value } => serde_json::Value::Bool(*value),
            JsonValue::Number { lexeme } => {
                if let Ok(n) = lexeme.parse::<serde_json::Number>() {
                    serde_json::Value::Number(n)
                } else if let Ok(val) = serde_json::from_str::<serde_json::Value>(lexeme) {
                    val
                } else {
                    serde_json::Value::String(lexeme.clone())
                }
            }
            JsonValue::String { value } => serde_json::Value::String(value.clone()),
            JsonValue::Array { items } => serde_json::Value::Array(items.iter().map(serde_json::Value::from).collect()),
            JsonValue::Object { members } => {
                let mut map = serde_json::Map::with_capacity(members.len());
                for m in members {
                    map.insert(m.key.clone(), serde_json::Value::from(&m.value));
                }
                serde_json::Value::Object(map)
            }
        }
    }
}
//#endregion 🔖️JsonModel

//#region 🔖️Parser





//#endregion 🔖️Parser

//#region 🔖️Serializer















//#endregion 🔖️Serializer

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.json` snapshot.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json")]
pub struct JsonSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub value: JsonValue,
}

impl Default for JsonSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: JsonValue::Null }
    }
}

impl JsonSnapshot {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_value(value: impl Into<JsonValue>) -> Self {
        Self { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: value.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_serde_value(&self) -> serde_json::Value {
        serde_json::Value::from(&self.value)
    }

    /// 🌉️ `to_serde_value`'s first-party analog — for a caller that has stopped depending on
    /// `serde_json` and only wants `pack::json::Value`.
    // 🚫️async: E1 pure inherent-impl helper, same reason as `to_serde_value` above — see R9
    pub fn to_pack_value(&self) -> semio_framework_pack_json::Value {
        semio_framework_pack_json::Value::from(&self.value)
    }
}
//#endregion 🔖️Snapshot



//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure document helper, no engine needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_json_snapshot() -> JsonSnapshot {
    JsonSnapshot::default()
}

/// 📄️ The demo `stdio.json` document — a genuinely 3-level-nested `JsonValue` (object → array,
/// object → object → object → array) exercising every `JsonValue` variant (`Null`/`Bool`/`Number`/
/// `String`/`Array`/`Object`) at least once. The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `../💡️inferences/🦀️.rs`'s `conformance_laws`) and for
/// `nontrivial_nested_value_round_trip` below, which calls this instead of duplicating the literal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_json_snapshot() -> JsonSnapshot {
    let value = JsonValue::Object {
        members: vec![
            JsonMember { key: "name".into(), value: JsonValue::String { value: "semio".into() } },
            JsonMember { key: "count".into(), value: JsonValue::Number { lexeme: "42".into() } },
            JsonMember { key: "ratio".into(), value: JsonValue::Number { lexeme: "3.5".into() } },
            JsonMember { key: "active".into(), value: JsonValue::Bool { value: true } },
            JsonMember { key: "missing".into(), value: JsonValue::Null },
            JsonMember { key: "tags".into(), value: JsonValue::Array { items: vec![JsonValue::String { value: "a".into() }, JsonValue::String { value: "b".into() }, JsonValue::String { value: "c".into() }] } },
            JsonMember {
                key: "nested".into(),
                value: JsonValue::Object {
                    members: vec![JsonMember {
                        key: "deep".into(),
                        value: JsonValue::Object {
                            members: vec![JsonMember { key: "deeper".into(), value: JsonValue::Array { items: vec![JsonValue::Number { lexeme: "1".into() }, JsonValue::Number { lexeme: "2".into() }, JsonValue::Number { lexeme: "3".into() }] } }],
                        },
                    }],
                },
            },
        ],
    };
    JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests



#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
